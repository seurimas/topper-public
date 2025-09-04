use super::super::statics::*;
use super::FirstAidAction;
use super::FirstAidSetting;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use regex::{Regex, RegexSet};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use topper_core::observations::strip_ansi;
use topper_core::timeline::BaseAgentState;

static FIRST_AID_BLOCK: &'static str = "\x1b[48;5;232mYour affliction curing priorities:
1)
2)  poultice: [anorexia, gorged, destroyed_throat]
    pill:     [paralysis]
    special:  [asleep, fear, itchy]

3)  poultice: [head_mangled, crushed_chest, burnt_skin]
    pill:     [asthma]
    special:  [voyria, writhe_impaled, ice_encased]

4)  pipe:     [aeon]
    poultice: [left_leg_bruised_critical, right_leg_bruised_critical,
               right_arm_bruised_critical, left_arm_bruised_critical,
               torso_bruised_critical, head_bruised_critical]
    pill:     [physical_disruption, mental_disruption, paresis, rot_body]
    special:  [writhe_thighlock, writhe_armpitlock, writhe_necklock,
               writhe_hoist, writhe_lure]

5)  pipe:     [slickness]
    poultice: [hypothermia, cracked_ribs, gloom, voidgaze]
    pill:     [slough, blood_curse, blood_poison, thin_blood, mirroring]
    special:  [writhe_gunk, writhe_grappled, writhe_web, writhe_vines,
               writhe_bind, writhe_transfix, writhe_ropes, writhe_dartpinned]

6)  pipe:     [besilence]
    poultice: [left_leg_amputated, right_leg_amputated, left_leg_broken,
               right_leg_broken, right_leg_mangled, left_leg_mangled]
    pill:     [stupidity, impatience, stormtouched, patterns]
    special:  [writhe_stasis]

7)  pipe:     [hellsight, withering]
    poultice: [left_arm_amputated, right_arm_amputated, left_arm_broken,
               right_arm_broken, right_arm_mangled, left_arm_mangled]
    pill:     [sensitivity, recklessness, nyctophobia, accursed, agony]

8)  pipe:     [migraine]
    poultice: [shivering, frozen, mauled_face, frigid]
    pill:     [heartflutter, confusion, epilepsy, dissonance, lethargy,
               rot_wither]

9)  pipe:     [disfigurement, squelched]
    poultice: [ablaze, firstaid_predict_any_limb]
    pill:     [clumsiness, weariness, berserking, laxity, faintness, hollow,
               perplexed]
    special:  [disrupted, dazed]

10) pipe:     [deadening]
    poultice: [left_leg_crippled, right_leg_crippled, firstaid_predict_legs]
    pill:     [limp_veins, masochism, haemophilia, vomiting, allergies,
               ringing_ears]
    special:  [vinethorns, dread]

11) poultice: [right_arm_crippled, left_arm_crippled, firstaid_predict_arms]
    pill:     [dizziness, crippled, crippled_body, merciful]
    special:  [oiled]

12) poultice: [head_broken, collapsed_lung]
    pill:     [dementia, paranoia, hallucinations, addiction, impairment]

13) poultice: [left_leg_dislocated, right_leg_dislocated]
    pill:     [hypochondria, agoraphobia, loneliness, vertigo, claustrophobia,
               justice, blisters, self_loathing]

14) poultice: [left_arm_dislocated, right_arm_dislocated]
    pill:     [hatred, rend, blighted, infested, egocentric, magnanimity,
               exhausted, rot_heat, narcolepsy]

15) poultice: [spinal_rip, torso_broken, torso_mangled, deepwound, heatspear]
    pill:     [hypersomnia, shyness, pacifism, peace, generosity,
               lovers_effect, superstition, misery]

16) poultice: [left_leg_bruised_moderate, right_leg_bruised_moderate,
               right_arm_bruised_moderate, left_arm_bruised_moderate,
               torso_bruised_moderate, head_bruised_moderate]
    pill:     [rot_benign, rot_spirit]

17) poultice: [left_arm_bruised, right_arm_bruised, right_leg_bruised,
               left_leg_bruised, head_bruised, torso_bruised]
    pill:     [sadness, self-pity, baldness, commitment_fear, hubris,
               body_odor, worrywart]

18) poultice: [indifference, blurry_vision, smashed_throat]
    pill:     [plodding, idiocy]

19) poultice: [whiplash, backstrain, sore_wrist, sore_ankle, muscle_spasms,
               stiffness, weak_grip]

20) poultice: [stuttering, crippled_throat, burnt_eyes, lightwound]

21) poultice: [void, weakvoid]

22) poultice: [pre_restore_left_leg, pre_restore_right_leg,
               pre-restore torso (22%), pre-restore head (22%)]

23) poultice: [pre_restore_head, pre_restore_left_arm, pre_restore_right_arm,
               pre-restore left leg (21%), pre-restore right leg (21%)]

24) poultice: [pre_restore_torso, pre-restore right arm (21%),
               pre-restore left arm (21%)]

25) poultice: [effused_blood]

26)";

lazy_static! {
    static ref UNNAMED_HEADER: Regex = Regex::new(r"Your affliction curing priorities:").unwrap();
    static ref NAMED_HEADER: Regex =
        Regex::new(r"Your affliction curing priorities for the priority set (\w+):").unwrap();
    static ref PRIORITY_NUM_LINE: Regex =
        Regex::new(r"^(\d+)\)\s+(pipe|poultice|pill|special):\s+\[([a-z_, ]+)\]?$").unwrap();
    static ref PRIORITY_TYPE_LINE: Regex =
        Regex::new(r"^\s+(pipe|poultice|pill|special):\s+\[([a-z_, ]+)\]?$").unwrap();
    static ref PRIORITY_CONTINUITY_LINE: Regex = Regex::new(r"^\s+([a-z_, ]+)\]?$").unwrap();
    static ref DEFAULT_PRIORITIES: HashMap<FType, u32> = HashMap::from_iter(vec![
        (FType::Anorexia, 2),
        (FType::Gorged, 2),
        (FType::DestroyedThroat, 2),
        (FType::Paralysis, 2),
        (FType::Asleep, 2),
        (FType::Fear, 2),
        (FType::Itchy, 2),
        (FType::HeadMangled, 3),
        (FType::CrushedChest, 3),
        (FType::BurntSkin, 3),
        (FType::Asthma, 3),
        (FType::Voyria, 3),
        (FType::WritheImpaled, 3),
        (FType::Mindfog, 3),
        (FType::IceEncased, 3),
        (FType::Aeon, 4),
        (FType::LeftLegBruisedCritical, 4),
        (FType::RightLegBruisedCritical, 4),
        (FType::RightArmBruisedCritical, 4),
        (FType::LeftArmBruisedCritical, 4),
        (FType::TorsoBruisedCritical, 4),
        (FType::HeadBruisedCritical, 4),
        (FType::Extravasation, 4),
        (FType::Delirium, 4),
        (FType::Paresis, 4),
        (FType::RotBody, 4),
        (FType::WritheThighlock, 4),
        (FType::WritheArmpitlock, 4),
        (FType::WritheNecklock, 4),
        (FType::WritheHoist, 4),
        (FType::WritheLure, 4),
        (FType::Slickness, 5),
        (FType::Hypothermia, 5),
        (FType::CrackedRibs, 5),
        (FType::Gloom, 5),
        (FType::Voidgaze, 5),
        (FType::Slough, 5),
        (FType::Psychosis, 5),
        (FType::Sepsis, 5),
        (FType::Dyscrasia, 5),
        (FType::Mirroring, 5),
        (FType::WritheGunk, 5),
        (FType::WritheGrappled, 5),
        (FType::WritheWeb, 5),
        (FType::WritheVines, 5),
        (FType::WritheBind, 5),
        (FType::WritheTransfix, 5),
        (FType::WritheRopes, 5),
        (FType::WritheDartpinned, 5),
        (FType::Besilence, 6),
        (FType::LeftLegAmputated, 6),
        (FType::RightLegAmputated, 6),
        (FType::LeftLegBroken, 6),
        (FType::RightLegBroken, 6),
        (FType::RightLegMangled, 6),
        (FType::LeftLegMangled, 6),
        (FType::Stupidity, 6),
        (FType::Impatience, 6),
        (FType::Stormtouched, 6),
        (FType::Patterns, 6),
        (FType::WritheStasis, 6),
        (FType::Hellsight, 7),
        (FType::Withering, 7),
        (FType::LeftArmAmputated, 7),
        (FType::RightArmAmputated, 7),
        (FType::LeftArmBroken, 7),
        (FType::RightArmBroken, 7),
        (FType::RightArmMangled, 7),
        (FType::LeftArmMangled, 7),
        (FType::Sensitivity, 7),
        (FType::Hopelessness, 7),
        (FType::Recklessness, 7),
        (FType::Nyctophobia, 7),
        (FType::Accursed, 7),
        (FType::Agony, 7),
        (FType::Migraine, 8),
        (FType::Shivering, 8),
        (FType::Frozen, 8),
        (FType::MauledFace, 8),
        (FType::Frigid, 8),
        (FType::Arrhythmia, 8),
        (FType::Confusion, 8),
        (FType::Epilepsy, 8),
        (FType::Dissonance, 8),
        (FType::Lethargy, 8),
        (FType::RotWither, 8),
        (FType::Disfigurement, 9),
        (FType::Dazed, 9),
        (FType::Echoes, 9),
        (FType::Phosphenes, 9),
        (FType::Squelched, 9),
        (FType::Ablaze, 9),
        (FType::FirstaidPredictAnyLimb, 9),
        (FType::Clumsiness, 9),
        (FType::Weariness, 9),
        (FType::Mania, 9),
        (FType::Laxity, 9),
        (FType::Faintness, 9),
        (FType::Hollow, 9),
        (FType::Perplexity, 9),
        (FType::Disrupted, 9),
        // (FType::Dazed, 9),
        (FType::Deadening, 10),
        (FType::LeftLegCrippled, 10),
        (FType::RightLegCrippled, 10),
        (FType::FirstaidPredictLegs, 10),
        (FType::Hypotension, 10),
        (FType::Masochism, 10),
        (FType::Haemophilia, 10),
        (FType::Vomiting, 10),
        (FType::Allergies, 10),
        (FType::RingingEars, 10),
        // (FType::Vinethorns, 10),
        // (FType::Dread, 10),
        (FType::RightArmCrippled, 11),
        (FType::LeftArmCrippled, 11),
        (FType::FirstaidPredictArms, 11),
        (FType::Dizziness, 11),
        (FType::Crippled, 11),
        (FType::CrippledBody, 11),
        (FType::Mercy, 11),
        // (FType::Oiled, 11),
        (FType::HeadBroken, 12),
        (FType::CollapsedLung, 12),
        (FType::Dementia, 12),
        (FType::Paranoia, 12),
        (FType::Hallucinations, 12),
        (FType::Addiction, 12),
        (FType::Impairment, 12),
        (FType::LeftLegDislocated, 13),
        (FType::RightLegDislocated, 13),
        (FType::Hypochondria, 13),
        (FType::Agoraphobia, 13),
        (FType::Loneliness, 13),
        (FType::Vertigo, 13),
        (FType::Claustrophobia, 13),
        (FType::Justice, 13),
        (FType::Blisters, 13),
        (FType::SelfLoathing, 13),
        (FType::LeftArmDislocated, 14),
        (FType::RightArmDislocated, 14),
        (FType::Hatred, 14),
        (FType::Rend, 14),
        (FType::Blight, 14),
        (FType::Infestation, 14),
        (FType::Egocentrism, 14),
        (FType::Magnanimity, 14),
        (FType::Exhaustion, 14),
        (FType::RotHeat, 14),
        (FType::Narcolepsy, 14),
        (FType::SpinalRip, 15),
        (FType::TorsoBroken, 15),
        (FType::TorsoMangled, 15),
        (FType::Deepwound, 15),
        (FType::Heatspear, 15),
        (FType::Hypersomnia, 15),
        (FType::Shyness, 15),
        (FType::Pacifism, 15),
        (FType::Peace, 15),
        (FType::Generosity, 15),
        (FType::Infatuation, 15),
        (FType::Superstition, 15),
        (FType::Misery, 15),
        (FType::LeftLegBruisedModerate, 16),
        (FType::RightLegBruisedModerate, 16),
        (FType::RightArmBruisedModerate, 16),
        (FType::LeftArmBruisedModerate, 16),
        (FType::TorsoBruisedModerate, 16),
        (FType::HeadBruisedModerate, 16),
        (FType::Blindness, 16),
        (FType::RotBenign, 16),
        (FType::RotSpirit, 16),
        (FType::LeftArmBruised, 17),
        (FType::RightArmBruised, 17),
        (FType::RightLegBruised, 17),
        (FType::LeftLegBruised, 17),
        (FType::HeadBruised, 17),
        (FType::TorsoBruised, 17),
        (FType::Sadness, 17),
        (FType::SelfPity, 17),
        (FType::Hubris, 17),
        (FType::FeebleLegs, 18),
        (FType::Indifference, 18),
        (FType::Gnawing, 18),
        (FType::BlurryVision, 18),
        (FType::SmashedThroat, 18),
        (FType::Plodding, 18),
        (FType::Idiocy, 18),
        (FType::Whiplash, 19),
        (FType::Backstrain, 19),
        (FType::SoreWrist, 19),
        (FType::SoreAnkle, 19),
        (FType::MuscleSpasms, 19),
        (FType::Stiffness, 19),
        (FType::WeakGrip, 19),
        (FType::Stuttering, 20),
        (FType::CrippledThroat, 20),
        (FType::Deafness, 20),
        (FType::FeebleArms, 20),
        (FType::Lightwound, 20),
        (FType::Manablight, 20),
        (FType::Void, 21),
        (FType::Weakvoid, 21),
        (FType::PreRestoreLeftLeg, 22),
        (FType::PreRestoreRightLeg, 22),
        (FType::PreRestoreHead, 23),
        (FType::PreRestoreLeftArm, 23),
        (FType::PreRestoreRightArm, 23),
        (FType::PreRestoreTorso, 24),
        (FType::EffusedBlood, 25),
        (FType::Dead, 26),
    ]);
}

fn add_priorities(priorities: &mut FirstAidPriorities, priority: u32, aff_list: &str) {
    for mut aff_str in aff_list.split(", ") {
        aff_str = aff_str.trim_end_matches(&[',', ' '][..]);
        if let Some(aff) = FType::from_name(&aff_str.to_string()) {
            priorities.0.insert(aff, priority);
        }
    }
}

pub fn parse_priorities(priority_lines: &Vec<String>) -> FirstAidPriorities {
    let mut priorities = FirstAidPriorities(HashMap::new());
    let mut priority = 0;
    for line in priority_lines.iter() {
        if let Some(captures) = PRIORITY_NUM_LINE.captures(&line) {
            priority = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            add_priorities(&mut priorities, priority, captures.get(3).unwrap().as_str());
        } else if let Some(captures) = PRIORITY_TYPE_LINE.captures(&line) {
            add_priorities(&mut priorities, priority, captures.get(2).unwrap().as_str());
        } else if let Some(captures) = PRIORITY_CONTINUITY_LINE.captures(&line) {
            add_priorities(&mut priorities, priority, captures.get(1).unwrap().as_str());
        }
    }
    priorities
}

pub fn parse_priority_set(lines: &Vec<(String, u32)>) -> Option<(String, FirstAidPriorities)> {
    let mut priority_lines = Vec::new();
    let mut priority_name = None;
    for (line, _num) in lines.iter() {
        if let Some(captures) = NAMED_HEADER.captures(&line) {
            priority_name = Some(captures.get(1).unwrap().as_str().to_string());
        } else if let Some(captures) = UNNAMED_HEADER.find(&line) {
            priority_name = Some("".to_string());
        } else if priority_name.is_some() {
            priority_lines.push(strip_ansi(line));
        }
    }
    priority_name.map(|name| (name, parse_priorities(&priority_lines)))
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct FirstAidPriorities(HashMap<FType, u32>);

impl From<HashMap<FType, u32>> for FirstAidPriorities {
    fn from(map: HashMap<FType, u32>) -> Self {
        FirstAidPriorities(map)
    }
}

impl FirstAidPriorities {
    fn best_cure(&self, who_am_i: &str, state: &AgentState, aff: &FType) -> FirstAidAction {
        if let Some(herb) = AFFLICTION_SMOKES.get(aff) {
            if state.can_smoke(false) {
                return FirstAidAction::Simple(SimpleCureAction::smoke(&who_am_i, &herb));
            }
        }
        if let Some(pill) = AFFLICTION_PILLS.get(aff) {
            if state.can_pill(false) {
                return FirstAidAction::Simple(SimpleCureAction::pill(&who_am_i, &pill));
            }
        }
        if let Some((salve, location)) = AFFLICTION_SALVES.get(aff) {
            if state.can_salve(false) {
                return FirstAidAction::Simple(SimpleCureAction::salve(
                    &who_am_i, &salve, &location,
                ));
            }
        }
        return FirstAidAction::Wait;
    }

    pub fn get_cure(&self, who_am_i: &str, state: &AgentState) -> Option<(FType, FirstAidAction)> {
        let mut top_priority: Option<(FType, u32, FirstAidAction)> = None;
        for aff in state.flags.aff_iter() {
            if let Some(priority) = self.0.get(&aff) {
                match top_priority {
                    Some((aff, top, _)) => {
                        if *priority < top {
                            match self.best_cure(&who_am_i, state, &aff) {
                                FirstAidAction::Wait => {}
                                cure => {
                                    top_priority = Some((aff, *priority, cure));
                                }
                            }
                        }
                    }
                    None => match self.best_cure(&who_am_i, state, &aff) {
                        FirstAidAction::Wait => {}
                        cure => {
                            top_priority = Some((aff, *priority, cure));
                        }
                    },
                }
            }
        }
        top_priority.map(|(aff, _, cure)| (aff, cure))
    }

    pub fn get_next_cure(
        &self,
        who_am_i: &str,
        state: &AgentState,
    ) -> Option<(FType, FirstAidAction)> {
        if let Some(cure) = self.get_cure(&who_am_i, &state) {
            return Some(cure);
        }
        let mut viable_balances = vec![];
        if state.can_pill(true) && !state.balanced(BType::Pill) {
            viable_balances.push(BType::Pill);
        }
        if state.can_salve(true) && !state.balanced(BType::Salve) {
            viable_balances.push(BType::Salve);
        }
        if state.can_smoke(true) && !state.balanced(BType::Smoke) {
            viable_balances.push(BType::Smoke);
        }
        if state.can_tree(true) && !state.balanced(BType::Tree) {
            viable_balances.push(BType::Tree);
        }
        if state.can_focus(true) && !state.balanced(BType::Focus) {
            viable_balances.push(BType::Focus);
        }
        if let Some(balance) = state.next_balance(viable_balances.iter()) {
            let mut state = state.clone();
            state.wait(state.get_raw_balance(balance));
            return self.get_next_cure(&who_am_i, &state);
        } else {
            None
        }
    }

    pub fn insert(&mut self, aff: FType, priority: u32) -> Option<u32> {
        self.0.insert(aff, priority)
    }

    /**
         * Your affliction curing priorities:
    1)
    2)  poultice: [anorexia, gorged, destroyed_throat]
        pill:     [paralysis]
        special:  [asleep, fear, itchy]

    3)  poultice: [head_mangled, crushed_chest, burnt_skin]
        pill:     [asthma]
        special:  [voyria, writhe_impaled, ice_encased]

    4)  pipe:     [aeon]
        poultice: [left_leg_bruised_critical, right_leg_bruised_critical,
                   right_arm_bruised_critical, left_arm_bruised_critical,
                   torso_bruised_critical, head_bruised_critical]
        pill:     [physical_disruption, mental_disruption, paresis, rot_body]
        special:  [writhe_thighlock, writhe_armpitlock, writhe_necklock,
                   writhe_hoist, writhe_lure]

    5)  pipe:     [slickness]
        poultice: [hypothermia, cracked_ribs, gloom, voidgaze]
        pill:     [slough, blood_curse, blood_poison, thin_blood, mirroring]
        special:  [writhe_gunk, writhe_grappled, writhe_web, writhe_vines,
                   writhe_bind, writhe_transfix, writhe_ropes, writhe_dartpinned]

    6)  pipe:     [besilence]
        poultice: [left_leg_amputated, right_leg_amputated, left_leg_broken,
                   right_leg_broken, right_leg_mangled, left_leg_mangled]
        pill:     [stupidity, impatience, stormtouched, patterns]
        special:  [writhe_stasis]

    7)  pipe:     [hellsight, withering]
        poultice: [left_arm_amputated, right_arm_amputated, left_arm_broken,
                   right_arm_broken, right_arm_mangled, left_arm_mangled]
        pill:     [sensitivity, recklessness, nyctophobia, accursed, agony]

    8)  pipe:     [migraine]
        poultice: [shivering, frozen, mauled_face, frigid]
        pill:     [heartflutter, confusion, epilepsy, dissonance, lethargy,
                   rot_wither]

    9)  pipe:     [disfigurement, squelched]
        poultice: [ablaze, firstaid_predict_any_limb]
        pill:     [clumsiness, weariness, berserking, laxity, faintness, hollow,
                   perplexed]
        special:  [disrupted, dazed]

    10) pipe:     [deadening]
        poultice: [left_leg_crippled, right_leg_crippled, firstaid_predict_legs]
        pill:     [limp_veins, masochism, haemophilia, vomiting, allergies,
                   ringing_ears]
        special:  [vinethorns, dread]

    11) poultice: [right_arm_crippled, left_arm_crippled, firstaid_predict_arms]
        pill:     [dizziness, crippled, crippled_body, merciful]
        special:  [oiled]

    12) poultice: [head_broken, collapsed_lung]
        pill:     [dementia, paranoia, hallucinations, addiction, impairment]

    13) poultice: [left_leg_dislocated, right_leg_dislocated]
        pill:     [hypochondria, agoraphobia, loneliness, vertigo, claustrophobia,
                   justice, blisters, self_loathing]

    14) poultice: [left_arm_dislocated, right_arm_dislocated]
        pill:     [hatred, rend, blighted, infested, egocentric, magnanimity,
                   exhausted, rot_heat, narcolepsy]

    15) poultice: [spinal_rip, torso_broken, torso_mangled, deepwound, heatspear]
        pill:     [hypersomnia, shyness, pacifism, peace, generosity,
                   lovers_effect, superstition, misery]

    16) poultice: [left_leg_bruised_moderate, right_leg_bruised_moderate,
                   right_arm_bruised_moderate, left_arm_bruised_moderate,
                   torso_bruised_moderate, head_bruised_moderate]
        pill:     [rot_benign, rot_spirit]

    17) poultice: [left_arm_bruised, right_arm_bruised, right_leg_bruised,
                   left_leg_bruised, head_bruised, torso_bruised]
        pill:     [sadness, self-pity, baldness, commitment_fear, hubris,
                   body_odor, worrywart]

    18) poultice: [indifference, blurry_vision, smashed_throat]
        pill:     [plodding, idiocy]

    19) poultice: [whiplash, backstrain, sore_wrist, sore_ankle, muscle_spasms,
                   stiffness, weak_grip]

    20) poultice: [stuttering, crippled_throat, burnt_eyes, lightwound]

    21) poultice: [void, weakvoid]

    22) poultice: [pre_restore_left_leg, pre_restore_right_leg]

    23) poultice: [pre_restore_head, pre_restore_left_arm, pre_restore_right_arm]

    24) poultice: [pre_restore_torso]

    25) poultice: [effused_blood]

    26)
         */
    pub fn reset(&mut self) -> Option<(FType, u32)> {
        let mut result = None;
        for (aff, priority) in &self.0 {
            if aff.is_affliction()
                && *priority != DEFAULT_PRIORITIES.get(&aff).cloned().unwrap_or(*priority)
            {
                result = Some((*aff, *priority));
            }
        }
        for (aff, priority) in DEFAULT_PRIORITIES.iter() {
            self.0.insert(*aff, *priority);
        }
        result
    }

    pub fn reset_defence(&mut self) -> Option<(FType, u32)> {
        let mut result = None;
        for (aff, priority) in &self.0 {
            if aff.is_general_defence() {
                if *priority != DEFAULT_PRIORITIES.get(&aff).cloned().unwrap_or(*priority) {
                    result = Some((*aff, *priority));
                }
            }
        }
        for aff_idx in 0..(FType::Sadness as u16) {
            let Ok(aff) = FType::try_from(aff_idx) else {
                continue;
            };
            if aff.is_general_defence() {
                self.0.insert(aff, 0);
            }
        }
        result
    }

    pub fn to_settings(&self) -> Vec<FirstAidSetting> {
        self.0
            .iter()
            .map(|(ft, prio)| FirstAidSetting::SimplePriority(*ft, *prio))
            .collect()
    }
}
