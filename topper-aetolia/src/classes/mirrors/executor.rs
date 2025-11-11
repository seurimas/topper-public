use crate::timeline::*;
use std::collections::HashMap;

pub fn add_mappings(mut mapping: &mut HashMap<(String, String), (String, String)>) {
    // Artifice to Woodlore
    mapping.insert(
        ("Artifice".to_string(), "Grit".to_string()),
        ("Woodlore".to_string(), "Might".to_string()),
    );
    mapping.insert(
        ("Artifice".to_string(), "Terrifier".to_string()),
        ("Woodlore".to_string(), "Cockatrice".to_string()),
    );
    mapping.insert(
        ("Artifice".to_string(), "Eviscerator".to_string()),
        ("Woodlore".to_string(), "Crocodile".to_string()),
    );
    mapping.insert(
        ("Artifice".to_string(), "Brutaliser".to_string()),
        ("Woodlore".to_string(), "Raloth".to_string()),
    );
    mapping.insert(
        ("Artifice".to_string(), "Verglas".to_string()),
        ("Woodlore".to_string(), "Icebreath".to_string()),
    );
    mapping.insert(
        ("Artifice".to_string(), "Rimestalker".to_string()),
        ("Woodlore".to_string(), "Icewyrm".to_string()),
    );
    mapping.insert(
        ("Artifice".to_string(), "Wardpeeler".to_string()),
        ("Woodlore".to_string(), "Weasel".to_string()),
    );
    mapping.insert(
        ("Artifice".to_string(), "Throatripper".to_string()),
        ("Woodlore".to_string(), "Gyrfalcon".to_string()),
    );
    mapping.insert(
        ("Artifice".to_string(), "Monstrosity".to_string()),
        ("Woodlore".to_string(), "Elk".to_string()),
    );
    // Shadowdancing to Dhuriv
    mapping.insert(
        ("Shadowdancing".to_string(), "Contrive".to_string()),
        ("Dhuriv".to_string(), "Slash".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Beguile".to_string()),
        ("Dhuriv".to_string(), "Stab".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Wile".to_string()),
        ("Dhuriv".to_string(), "Slice".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Inveigle".to_string()),
        ("Dhuriv".to_string(), "Thrust".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Waylay".to_string()),
        ("Dhuriv".to_string(), "Ambush".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Brandish".to_string()),
        ("Dhuriv".to_string(), "Flourish".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Incise".to_string()),
        ("Dhuriv".to_string(), "Pierce".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Dissever".to_string()),
        ("Dhuriv".to_string(), "Sever".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Dualraze".to_string()),
        ("Dhuriv".to_string(), "Dualraze".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Shave".to_string()),
        ("Dhuriv".to_string(), "Reave".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Ruse".to_string()),
        ("Dhuriv".to_string(), "Twirl".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Terminate".to_string()),
        ("Dhuriv".to_string(), "Spinecut".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Stifle".to_string()),
        ("Dhuriv".to_string(), "Throatcrush".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Phlebotomise".to_string()),
        ("Dhuriv".to_string(), "Crosscut".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Gambol".to_string()),
        ("Dhuriv".to_string(), "Trip".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Perplex".to_string()),
        ("Dhuriv".to_string(), "Slam".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Muddle".to_string()),
        ("Dhuriv".to_string(), "Gouge".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Desolate".to_string()),
        ("Dhuriv".to_string(), "Heartbreaker".to_string()),
    );
    mapping.insert(
        ("Shadowdancing".to_string(), "Razor".to_string()),
        ("Dhuriv".to_string(), "Slit".to_string()),
    );
}
