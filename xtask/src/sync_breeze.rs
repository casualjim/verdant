use std::{
    collections::{HashMap, HashSet, VecDeque},
    env,
    fs,
    path::PathBuf,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use anyhow::{bail, Context, Result};
use serde::Deserialize;

use crate::{add_lang, fetch_queries, schema, WORKSPACE_DIR};

#[derive(Debug, Deserialize)]
struct BreezeConfig {
    grammars: Vec<BreezeGrammar>,
}

#[derive(Debug, Deserialize)]
struct BreezeGrammar {
    name: String,
    repo: String,
    rev: String,
    #[serde(default)]
    path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GrammarMapping {
    grammar: String,
    #[serde(default)]
    nvim_filetype: Option<String>,
    #[serde(default)]
    effective_filetype: Option<String>,
    // helix_file_types is not used anymore - we use effective_filetype instead
}

#[derive(Debug, Deserialize)]
struct GithubContentEntry {
    name: String,
    #[serde(rename = "type")]
    kind: String,
}

pub fn run() -> Result<()> {
    let mut dry_run = false;
    let mut limit: Option<usize> = None;
    let mut jobs: Option<usize> = None;

    let mut args = env::args().skip(2);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dry-run" => dry_run = true,
            "--jobs" | "-j" => {
                let value = args.next().with_context(|| "missing value for --jobs")?;
                jobs = Some(value.parse::<usize>().context("invalid --jobs value")?);
            }
            "--limit" => {
                let value = args
                    .next()
                    .with_context(|| "missing value for --limit")?;
                limit = Some(value.parse::<usize>().context("invalid --limit value")?);
            }
            "--help" | "-h" => {
                println!(
                    "{}",
                    r###"
Usage: cargo xtask sync-breeze [--dry-run] [--limit N]
       cargo xtask sync-breeze [--jobs N]

Reads ../breeze-tree-sitter-parsers/grammars.json and adds missing languages to
syntastica-macros/languages.toml, but only when nvim-treesitter provides a
runtime query directory with a highlights.scm for the language.
"###
                    .trim()
                );
                return Ok(());
            }
            other => bail!("unknown arg '{other}' (try --help)"),
        }
    }

    let breeze_grammars_path = breeze_grammars_path()?;
    let breeze_json = fs::read_to_string(&breeze_grammars_path).with_context(|| {
        format!(
            "failed to read breeze grammars at {}",
            breeze_grammars_path.display()
        )
    })?;
    let breeze: BreezeConfig = serde_json::from_str(&breeze_json).context("invalid breeze json")?;

    let grammar_mappings = load_grammar_mappings()?;

    let nvim_query_dirs = fetch_nvim_runtime_query_dirs()?;

    let langs_toml_path = WORKSPACE_DIR.join("syntastica-macros/languages.toml");
    let langs_toml = fs::read_to_string(&langs_toml_path)?;
    let existing_cfg: schema::LanguageConfig =
        toml::from_str(&langs_toml).expect("invalid `languages.toml`");
    let existing_names = existing_cfg
        .languages
        .iter()
        .map(|l| l.name.clone())
        .collect::<HashSet<_>>();
    let mut reserved_names = existing_names.clone();

    let mut to_add = Vec::new();
    for grammar in breeze.grammars.into_iter() {
        if should_skip_grammar(&grammar.name) {
            continue;
        }
        if let Some(limit) = limit {
            if to_add.len() >= limit {
                break;
            }
        }

        let mut target_name = canonical_syntastica_name(&grammar.name, &grammar_mappings);
        if reserved_names.contains(&target_name) {
            let fallback = sanitize_lang_name(&grammar.name);
            if reserved_names.contains(&fallback) {
                println!(
                    "sync-breeze: skipping {} (target name '{}' already exists)",
                    grammar.name, target_name
                );
                continue;
            }
            target_name = fallback;
        }
        reserved_names.insert(target_name.clone());

        let Some(nvim_name) = resolve_nvim_query_name(&grammar.name, &target_name, &nvim_query_dirs, &grammar_mappings)
        else {
            continue;
        };

        to_add.push((grammar, target_name, nvim_name));
    }

    if to_add.is_empty() {
        println!("sync-breeze: nothing to do");
        return Ok(());
    }

    println!("sync-breeze: adding {} languages", to_add.len());

    let total_count = to_add.len();
    let job_count = jobs
        .unwrap_or_else(default_job_count)
        .max(1)
        .min(total_count);
    let queue = Arc::new(Mutex::new(VecDeque::from(to_add)));
    let (result_tx, result_rx) = mpsc::channel();
    let mappings = Arc::new(grammar_mappings);

    for _ in 0..job_count {
        let queue = Arc::clone(&queue);
        let result_tx = result_tx.clone();
        let mappings = Arc::clone(&mappings);
        thread::spawn(move || loop {
            let item = {
                let mut guard = queue.lock().expect("sync-breeze queue poisoned");
                guard.pop_front()
            };
            let Some((grammar, target_name, nvim_name)) = item else {
                break;
            };

            let result = build_language_block(
                &grammar,
                &target_name,
                &nvim_name,
                &mappings,
                dry_run,
            )
            .map(|block| (target_name, block));

            if result_tx.send(result).is_err() {
                break;
            }
        });
    }
    drop(result_tx);

    let mut new_blocks: Vec<(String, String)> = Vec::with_capacity(total_count);
    let mut errors: Vec<anyhow::Error> = Vec::new();
    for _ in 0..total_count {
        match result_rx.recv() {
            Ok(Ok(item)) => new_blocks.push(item),
            Ok(Err(err)) => errors.push(err),
            Err(err) => bail!("sync-breeze worker hung up: {err}"),
        }
    }

    if !errors.is_empty() {
        let first = errors.remove(0);
        bail!(
            "sync-breeze failed with {} error(s); first error: {first:?}",
            errors.len() + 1
        );
    }

    if dry_run {
        println!("sync-breeze: dry-run, not writing {}", langs_toml_path.display());
        return Ok(());
    }

    let mut blocks = langs_toml
        .split("\n\n")
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    blocks.extend(new_blocks.into_iter().map(|(_, block)| block));
    blocks.sort_unstable_by_key(|block| extract_lang_name(block).unwrap_or_default().to_owned());
    fs::write(&langs_toml_path, blocks.join("\n\n"))?;

    Ok(())
}

fn breeze_grammars_path() -> Result<PathBuf> {
    let Some(parent) = WORKSPACE_DIR.parent() else {
        bail!("workspace dir has no parent: {}", WORKSPACE_DIR.display());
    };
    Ok(parent.join("breeze-tree-sitter-parsers/grammars.json"))
}

fn grammar_mappings_path() -> Result<PathBuf> {
    Ok(WORKSPACE_DIR.join("grammars-mapping-filetypes.json"))
}

fn load_grammar_mappings() -> Result<HashMap<String, GrammarMapping>> {
    let path = grammar_mappings_path()?;
    let content = fs::read_to_string(&path).with_context(|| {
        format!(
            "failed to read grammar mappings at {}",
            path.display()
        )
    })?;
    let mappings: Vec<GrammarMapping> = serde_json::from_str(&content)
        .context("invalid grammar mappings json")?;
    Ok(mappings
        .into_iter()
        .map(|m| (m.grammar.clone(), m))
        .collect())
}

fn get_file_types(grammar_name: &str, mappings: &HashMap<String, GrammarMapping>) -> Option<Vec<String>> {
    mappings.get(grammar_name).and_then(|m| {
        // Use the effective_filetype as the FileType enum name
        m.effective_filetype.as_ref().map(|ft| vec![format!("\"{ft}\"")])
    })
}

fn fetch_nvim_runtime_query_dirs() -> Result<std::collections::BTreeSet<String>> {
    let url = "https://api.github.com/repos/nvim-treesitter/nvim-treesitter/contents/runtime/queries?ref=main";
    let client = reqwest::blocking::Client::builder()
        .user_agent("syntastica xtask (sync-breeze)")
        .build()
        .context("failed to build http client")?;
    let res = client.get(url).send().context("failed to query github api")?;
    if !res.status().is_success() {
        bail!("github api returned {}", res.status());
    }
    let entries: Vec<GithubContentEntry> =
        res.json().context("failed to parse github api response")?;
    Ok(entries
        .into_iter()
        .filter(|e| e.kind == "dir")
        .map(|e| e.name)
        .collect())
}

fn canonical_syntastica_name(breeze_name: &str, mappings: &HashMap<String, GrammarMapping>) -> String {
    // First check the grammar mappings for effective_filetype
    if let Some(mapping) = mappings.get(breeze_name) {
        if let Some(effective) = &mapping.effective_filetype {
            if is_valid_lang_name(effective) {
                return effective.clone();
            }
        }
    }

    // Fallback to hardcoded mappings
    match sanitize_lang_name(breeze_name).as_str() {
        "csharp" => "c_sharp".to_owned(),
        other => other.to_owned(),
    }
}

fn is_valid_lang_name(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if first.is_ascii_digit() {
        return false;
    }
    if !(first.is_ascii_lowercase() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn sanitize_lang_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

fn resolve_nvim_query_name(
    breeze_name: &str,
    syntastica_name: &str,
    nvim_dirs: &std::collections::BTreeSet<String>,
    mappings: &HashMap<String, GrammarMapping>,
) -> Option<String> {
    // First check grammar mappings for nvim_filetype
    if let Some(mapping) = mappings.get(breeze_name) {
        if let Some(nvim_ft) = &mapping.nvim_filetype {
            if nvim_dirs.contains(nvim_ft) {
                return Some(nvim_ft.clone());
            }
        }
    }

    // Fallback to candidate names
    let candidates = [
        syntastica_name.to_owned(),
        breeze_name.to_owned(),
        breeze_name.replace('-', "_"),
    ];
    candidates
        .into_iter()
        .find(|name| nvim_dirs.contains(name))
}

fn should_skip_grammar(name: &str) -> bool {
    matches!(name, "fsharp_signature")
}

fn default_job_count() -> usize {
    let detected = thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    detected.min(8)
}

fn build_language_block(
    grammar: &BreezeGrammar,
    target_name: &str,
    nvim_name: &str,
    mappings: &HashMap<String, GrammarMapping>,
    dry_run: bool,
) -> Result<String> {
    println!(">>> {target_name} (queries: {nvim_name})");

    let (external_c, external_cpp, generate) =
        detect_parser_metadata(&grammar.repo, &grammar.rev, grammar.path.as_deref())?;
    let wasm_unknown = !external_cpp;

    let (queries_injections, queries_locals) = write_queries(target_name, nvim_name, dry_run)?;

    let file_types = get_file_types(&grammar.name, mappings);

    Ok(format_language_block(
        target_name,
        &grammar.repo,
        &grammar.rev,
        grammar.path.as_deref(),
        external_c,
        external_cpp,
        wasm_unknown,
        generate,
        queries_injections,
        queries_locals,
        file_types,
    ))
}

fn detect_parser_metadata(url: &str, rev: &str, path: Option<&str>) -> Result<(bool, bool, bool)> {
    let content_url = add_lang::url_to_content_url(url, rev);
    let path_in_url = match path {
        Some(path) => format!("/{path}"),
        None => String::new(),
    };

    let external_c = content_url.as_ref().is_some_and(|base| {
        reqwest::blocking::get(format!("{base}{path_in_url}/src/scanner.c"))
            .is_ok_and(|response| response.status().is_success())
    });
    let external_cpp = content_url.as_ref().is_some_and(|base| {
        reqwest::blocking::get(format!("{base}{path_in_url}/src/scanner.cc"))
            .is_ok_and(|response| response.status().is_success())
    });

    let generate = content_url.as_ref().is_some_and(|base| {
        reqwest::blocking::get(format!("{base}{path_in_url}/src/parser.c"))
            .ok()
            .is_some_and(|response| response.status() == reqwest::StatusCode::NOT_FOUND)
    });

    Ok((external_c, external_cpp, generate))
}

fn write_queries(target_name: &str, nvim_name: &str, dry_run: bool) -> Result<(bool, bool)> {
    let mut queries_injections = false;
    let mut queries_locals = false;

    let out_dir = WORKSPACE_DIR.join(format!("queries/{target_name}"));
    if !dry_run {
        fs::create_dir_all(&out_dir)?;
    }

    let highlights = fetch_queries::fetch_query(nvim_name, "highlights")?
        .with_context(|| format!("missing highlights query for {nvim_name}"))?;
    if !dry_run {
        fs::write(out_dir.join("highlights.scm"), highlights)?;
    }

    for kind in ["injections", "locals"] {
        let Some(text) = fetch_queries::fetch_query(nvim_name, kind)? else {
            continue;
        };
        if kind == "injections" {
            queries_injections = true;
        } else {
            queries_locals = true;
        }
        if !dry_run {
            fs::write(out_dir.join(format!("{kind}.scm")), text)?;
        }
    }

    Ok((queries_injections, queries_locals))
}

#[allow(clippy::too_many_arguments)]
fn format_language_block(
    name: &str,
    url: &str,
    rev: &str,
    path: Option<&str>,
    external_c: bool,
    external_cpp: bool,
    wasm_unknown: bool,
    generate: bool,
    queries_injections: bool,
    queries_locals: bool,
    file_types: Option<Vec<String>>,
) -> String {
    let mut out = String::new();
    out.push_str("[[languages]]\n");
    out.push_str(&format!("name = \"{name}\"\n"));
    out.push_str("group = \"all\"\n");
    if let Some(types) = file_types {
        if types.is_empty() {
            out.push_str("file-types = []\n");
        } else {
            out.push_str(&format!("file-types = [{}]\n", types.join(", ")));
        }
    } else {
        out.push_str("file-types = []\n");
    }
    if !wasm_unknown {
        out.push_str("wasm-unknown = false\n");
    }
    out.push_str("[languages.parser]\n");
    out.push_str(&format!(
        "git = {{ url = \"{url}\", rev = \"{rev}\"{} }}\n",
        path.map_or_else(String::new, |p| format!(", path = \"{p}\""))
    ));
    out.push_str(&format!(
        "external-scanner = {{ c = {external_c}, cpp = {external_cpp} }}\n"
    ));
    out.push_str(&format!("ffi-func = \"tree_sitter_{name}\"\n"));
    out.push_str(&format!("package = \"tree-sitter-{}\"\n", name.replace('_', "-")));
    if generate {
        out.push_str("generate = true\n");
    }
    out.push_str("[languages.queries]\n");
    out.push_str("nvim-like = true\n");
    out.push_str(&format!("injections = {queries_injections}\n"));
    out.push_str(&format!("locals = {queries_locals}\n"));
    out
}

fn extract_lang_name(block: &str) -> Option<&str> {
    let (_, rest) = block.split_once("name = \"")?;
    let (name, _) = rest.split_once('"')?;
    Some(name)
}
