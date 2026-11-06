use std::fmt::{self, Display, Formatter};

use palate::FileType;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LanguageConfig {
    pub languages: Vec<Language>,
}

impl LanguageConfig {
    /// The language whose Cargo feature enables `lang`: `lang` itself, or the language it is
    /// bundled with.
    pub fn owner<'a>(&'a self, lang: &'a Language) -> &'a Language {
        match &lang.bundled_with {
            None => lang,
            Some(owner) => self
                .languages
                .iter()
                .find(|other| other.name == *owner)
                .unwrap_or_else(|| {
                    panic!("`{}` is bundled with unknown language `{owner}`", lang.name)
                }),
        }
    }

    /// The Cargo feature that enables `lang`: the feature named after it, or, for `all` languages
    /// in a family, that family's feature.
    pub fn feature<'a>(&'a self, lang: &'a Language) -> &'a str {
        let owner = self.owner(lang);
        match &owner.family {
            Some(family) if owner.group == Group::All => family,
            _ => &owner.name,
        }
    }

    /// Whether a Cargo feature named after `lang` exists.
    #[allow(dead_code)] // is used in `xtask/src/codegen.rs`
    pub fn has_own_feature(&self, lang: &Language) -> bool {
        self.feature(lang) == lang.name
    }
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Language {
    pub name: String,
    pub group: Group,
    /// A family feature (like `shell`) that enables this language along with the rest of the
    /// family. `all` languages in a family have no feature of their own, `some` and `most`
    /// languages keep theirs and are additionally listed by the family.
    pub family: Option<String>,
    /// A language from the same parser release that provides the Cargo feature (and group) for
    /// this one, so they are enabled together.
    pub bundled_with: Option<String>,
    pub file_types: Vec<FileType>,
    #[serde(default = "default_true")]
    pub wasm: bool,
    #[serde(default = "default_true")]
    pub wasm_unknown: bool,
    // TODO: injection regex
    pub parser: Parser,
    pub queries: Queries,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Group {
    Some,
    Most,
    All,
}

impl Group {
    #[allow(dead_code)] // is used in `xtask/src/codegen.rs`
    pub fn next_smaller(&self) -> Option<Self> {
        match self {
            Group::Some => None,
            Group::Most => Some(Group::Some),
            Group::All => Some(Group::Most),
        }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Some => write!(f, "some"),
            Self::Most => write!(f, "most"),
            Self::All => write!(f, "all"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Parser {
    pub git: ParserGit,
    pub external_scanner: ParserExternal,
    pub ffi_func: String,
    pub rust_const: Option<String>,
    pub rust_func: Option<String>,
    pub package: String,
    pub crates_io: Option<String>,
    #[serde(default)]
    pub generate: bool,
}

impl Parser {
    pub fn supports_dep(&self) -> bool {
        (self.rust_const.is_some() || self.rust_func.is_some()) && self.crates_io.is_some()
    }

    pub fn supports_gitdep(&self) -> bool {
        (self.rust_const.is_some() || self.rust_func.is_some()) && !self.generate
    }

    pub fn supports(&self, git: bool) -> bool {
        match git {
            true => self.supports_gitdep(),
            false => self.supports_dep(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ParserGit {
    pub url: String,
    pub rev: String,
    #[serde(default)]
    pub branch: Option<String>,
    pub path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ParserExternal {
    pub c: bool,
    pub cpp: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Queries {
    pub nvim_like: bool,
    pub injections: bool,
    pub locals: bool,
}
