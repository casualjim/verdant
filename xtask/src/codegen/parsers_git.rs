use std::fs;

use anyhow::Result;

use crate::schema::Group;

pub fn write() -> Result<()> {
    let toml_path = crate::WORKSPACE_DIR.join("syntastica-parsers-git/Cargo.toml");
    let mut toml = fs::read_to_string(&toml_path)?;

    if let Some((preserve, _)) = toml.split_once(super::TOML_AUTOGEN_HEADER) {
        toml.truncate(preserve.len());
    }
    toml += super::TOML_AUTOGEN_HEADER;

    toml += r###"
[features]
#! ## Features
default = []
"###;
    toml += super::TOML_FEATURES_INTRO;

    toml += &super::parsers_toml_feature(Group::Some);
    toml += super::TOML_FEATURES_MOST;
    toml += &super::parsers_toml_feature(Group::Most);
    toml += super::TOML_FEATURES_ALL;
    toml += &super::parsers_toml_feature(Group::All);
    toml += &super::parsers_toml_family_features(super::ParserCollection::Git);

    toml += super::TOML_FEATURES_DOCS;

    toml += &super::parsers_toml_lang_features(super::ParserCollection::Git);

    super::check_crates_io_feature_limit(&toml, "verdant-parsers-git")?;

    fs::write(&toml_path, toml)?;

    Ok(())
}
