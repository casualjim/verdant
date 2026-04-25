use std::collections::HashMap;

use verdant::{renderer::TerminalRenderer, Highlights, Processor};
use verdant_core::theme::ResolvedTheme;
use verdant_parsers_git::{Lang, LanguageSetImpl};

fn main() {
    let examples: HashMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();

    let highlights =
        Processor::process_once(&examples["rust"], Lang::Rust, &LanguageSetImpl::new()).unwrap();

    for theme in verdant_themes::THEMES {
        example(
            &highlights,
            verdant_themes::from_str(theme).unwrap(),
            theme,
        );
    }
}

fn example(highlights: &Highlights, theme: ResolvedTheme, name: &str) {
    let bg_color = theme.bg();
    println!(
        "\n\x1b[1m{name}:\x1b[0m\n{0}\n{1}{0}",
        "-".repeat(50),
        verdant::render(highlights, &mut TerminalRenderer::new(bg_color), theme),
    );
}
