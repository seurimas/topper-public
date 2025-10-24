use crate::timeline::*;
use std::collections::HashMap;

pub fn add_mappings(mut mapping: &mut HashMap<(String, String), (String, String)>) {
    mapping.insert(
        ("Extirpation".to_string(), "Thrash".to_string()),
        ("Corpus".to_string(), "Gash".to_string()),
    );
    mapping.insert(
        ("Extirpation".to_string(), "Naturalise".to_string()),
        ("Corpus".to_string(), "Purify".to_string()),
    );

    mapping.insert(
        ("Caprice".to_string(), "Bewitch".to_string()),
        ("Mentis".to_string(), "Mesmerize".to_string()),
    );

    mapping.insert(
        ("Glamours".to_string(), "Awe".to_string()),
        ("Sanguis".to_string(), "Trepidation".to_string()),
    );
    mapping.insert(
        ("Glamours".to_string(), "Winnow".to_string()),
        ("Sanguis".to_string(), "Spew".to_string()),
    );
    mapping.insert(
        ("Glamours".to_string(), "Disfavour".to_string()),
        ("Sanguis".to_string(), "Poison".to_string()),
    );
}

pub fn map_mentis(combat_action: &CombatAction) -> Option<(String, String)> {
    if combat_action.category == "Caprice" {
        Some(("Mentis".to_string(), combat_action.skill.to_string()))
    } else {
        None
    }
}
