use std::collections::HashMap;

pub mod assets;
pub mod classpath;
pub mod download;
pub mod game;
pub mod install;
pub mod manifest;

#[cfg(test)]
pub mod tests;

pub fn get_features() -> Vec<String> {
    // I'll just get the values that are true here, this is just for clarity. :)
    let raw = HashMap::from([
        ("is_demo_user", false),
        ("has_custom_resolution", false),
        ("has_quick_plays_support", false),
        ("is_quick_play_singleplayer", false),
        ("is_quick_play_multiplayer", false),
        ("is_quick_play_realms", false),
    ]);

    raw.iter()
        .filter(|(_, v)| **v)
        .map(|(k, _)| k.to_string())
        .collect::<Vec<_>>()
}
