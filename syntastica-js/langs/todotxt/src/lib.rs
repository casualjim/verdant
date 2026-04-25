#![no_std]
#![warn(rust_2018_idioms)]

use tree_sitter::Language;

verdant_macros::js_lang_info!();
verdant_macros::js_lang_lib!("todotxt");
