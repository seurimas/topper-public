use crate::timeline::*;
use std::collections::HashMap;

pub fn add_mappings(mut mapping: &mut HashMap<(String, String), (String, String)>) {
    mapping.insert(
        ("Hyalincuru".to_string(), "Sphere".to_string()),
        ("Tarot".to_string(), "Sun".to_string()),
    );
    mapping.insert(
        ("Hyalincuru".to_string(), "Hypercube".to_string()),
        ("Tarot".to_string(), "Moon".to_string()),
    );
    mapping.insert(
        ("Hyalincuru".to_string(), "Raven".to_string()),
        ("Tarot".to_string(), "Death".to_string()),
    );
}
