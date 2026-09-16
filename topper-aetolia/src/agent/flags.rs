use super::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use structdiff::StructDiff;
use topper_persuasion::PersuasionAff;

// Flags
#[derive(
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Clone,
    Copy,
    TryFromPrimitive,
    EnumString,
    Serialize,
    Deserialize,
    Display,
)]
#[repr(u16)]
pub enum FType {
    Dead,

    // Control
    Player,
    Ally,
    Enemy,

    // Defences
    Shielded,
    Deathsight,
    Insomnia,
    Instawake,
    Courage,
    Thirdeye,
    Daydreams,
    Fangbarrier,
    Waterbreathing,
    Waterwalking,
    // Reishi
    Rebounding,
    AssumedRebounding,
    // Elixirs
    Levitation,
    Arcane,
    Speed,
    Vigor,
    // Salves
    Insulation,
    Density,
    // Tattoos
    Flame,
    Cloak,
    // General
    Reflection,
    Clarity,

    // Ascendril defences
    FulcrumShift,

    // Infiltrator defences
    Shroud,
    Ghosted,
    Shadowslip,
    Weaving,
    Hiding,
    Shadowsight,

    // Zealot defences
    Mindspark,
    Zenith,
    Firefist,
    Swagger,
    Wrath,

    // Bard defences
    SongDestiny,
    Sheath,
    Aurora,
    Equipoise,
    Stretching,
    Halfbeat,
    Discordance,
    Euphonia,

    // Bard uncurable
    Manabarbs, // Don't count as an affliction, it's not in database.

    // Siderealist defences
    Luminesce,
    Foresight,
    Centrum,

    // Antipsychotic
    Sadness, // MUST BE FIRST AFFLICTION
    Confusion,
    Dementia,
    Hallucinations,
    Paranoia,
    Hatred,
    Addiction,
    Hypersomnia,
    Psychosis,
    Blight,

    // Euphoriant
    SelfPity,
    Stupidity,
    Dizziness,
    Faintness,
    Shyness,
    Epilepsy,
    Impatience,
    Dissonance,
    Infestation,
    // Insomnia,

    // Eucrasia
    Misery,
    Hopelessness,
    Echoes,
    Hollow,
    Narcolepsy,
    Loneliness,
    Perplexity,

    // Decongestant
    Clumsiness,
    Hypochondria,
    Weariness,
    Asthma,
    RingingEars,
    Sensitivity,
    Impairment,
    Sepsis,

    // Depressant
    Mercy,
    Recklessness,
    Egocentrism,
    Masochism,
    Agoraphobia,
    Mania,
    Vertigo,
    Claustrophobia,
    Nyctophobia,

    // Coagulation
    Lethargy,
    Delirium,
    Extravasation,
    Vomiting,
    Exhaustion,
    Dyscrasia,
    Rend,
    Haemophilia,

    // Steroid
    Hubris,
    Pacifism,
    Peace,
    Agony,
    Accursed,
    Hypotension,
    Infatuation,
    Laxity,
    Superstition,
    Generosity,
    Justice,
    Magnanimity,

    // Opiate
    Paresis,
    Paralysis,
    Mirroring,
    CrippledBody,
    Crippled,
    Blisters,
    Slickness,
    Arrhythmia,
    Slough,

    // Anabiotic
    Plodding,
    Idiocy,

    // Panacea
    Stormtouched,
    Patterns,
    Rot,
    RotBenign,
    RotSpirit,
    RotHeat,
    RotWither,
    RotBody,

    // Reishi
    Besilence,

    // Willow
    Aeon,
    Hellsight,
    Deadening,

    // Yarrow
    // Slickness,
    Withering,
    Disfigurement,
    Migraine,

    // Elixir
    Etherflux,
    Squelched,

    // Epidermal Head
    Indifference,
    Stuttering,
    WateryEyes,
    BlurryVision,
    Blindness,
    Gloom,
    Deafness,

    // Epidermal Toros
    Anorexia,
    Gorged,
    Gnawing,
    Manablight,
    EffusedBlood,

    // Mending Head
    HeadBruisedCritical,
    DestroyedThroat,
    CrippledThroat,
    HeadBruisedModerate,
    HeadBruised,

    // Mending Torso
    TorsoBruisedCritical,
    Lightwound,
    CrackedRibs,
    TorsoBruisedModerate,
    TorsoBruised,

    // Mending Arms
    FeebleArms,

    // Mending Legs
    FeebleLegs,

    // Mending Left Arm
    LeftArmBruisedCritical,
    LeftArmBruisedModerate,
    LeftArmBruised,
    LeftArmDislocated,

    // Mending Right Arm
    RightArmBruisedCritical,
    RightArmBruisedModerate,
    RightArmBruised,
    RightArmDislocated,

    // Mending Left Leg
    LeftLegBruisedCritical,
    LeftLegBruisedModerate,
    LeftLegBruised,
    LeftLegDislocated,

    // Mending Right Leg
    RightLegBruisedCritical,
    RightLegBruisedModerate,
    RightLegBruised,
    RightLegDislocated,

    // Restoration Head
    Voidgaze,
    Voidtrapped,
    MauledFace,
    SmashedThroat,

    // Restoration Torso
    CollapsedLung,
    SpinalRip,
    BurntSkin,
    CrushedChest,
    Heatspear,
    Deepwound,

    // Soothing
    Whiplash, // Head
    Phosphenes,
    Backstrain, // Torso
    MuscleSpasms,
    Stiffness,
    SoreWrist, // Arms
    WeakGrip,
    SoreAnkle, // Legs

    // Caloric
    Mindfog,
    Hypothermia,
    IceEncased,
    Frozen,
    Shivering,
    Frigid,

    // Monk Uncurable
    NumbArms,

    // Infiltrator Uncurable
    Void,
    Weakvoid,
    Backstabbed,
    NumbedSkin,
    MentalFatigue,
    Thorns,
    Marked,

    // Zealot Uncurable
    InfernalSeal,
    InfernalShroud,

    // Scio Uncurable
    Imbued,
    Impeded,
    Shadowbrand,
    Shadowsphere,

    // Praenomen Uncurable
    Seduction,
    Temptation,

    // Newscendril Uncurable
    AshenFeet,
    Emberbrand,
    Frostbrand,
    Thunderbrand,
    FrozenFeet,
    Direfrost,

    // Shapeshifter
    RippedThroat,

    // Special
    Asleep,
    Unconsciousness,
    Fear,
    Fallen,
    Itchy,

    // Writhes
    WritheImpaled,
    WritheArmpitlock,
    WritheNecklock,
    WritheThighlock,
    WritheTransfix,
    WritheBind,
    WritheGunk,
    WritheRopes,
    WritheVines,
    WritheWeb,
    WritheDartpinned,
    WritheHoist,
    WritheGrappled,
    WritheLure,
    WritheStasis,

    SIZE,
    // Afflictions that stack.
    Allergies,
    Ablaze,
    SappedStrength,
    SelfLoathing,
    Hobbled,
    Illgrasp,

    // Timed affs
    TIMED,
    Blackout,
    Stun,
    Shock,
    Burnout,
    Muddled,
    Disrupted,
    Dazed,
    // Immunity
    Voyria,

    FULL,
    // Afflictions stored elsewhere
    HeadMangled,
    HeadBroken,
    TorsoMangled,
    TorsoBroken,
    LeftLegCrippled,
    RightLegCrippled,
    LeftArmCrippled,
    RightArmCrippled,
    LeftLegAmputated,
    RightLegAmputated,
    LeftArmAmputated,
    RightArmAmputated,
    LeftLegMangled,
    RightLegMangled,
    LeftArmMangled,
    RightArmMangled,
    LeftLegBroken,
    RightLegBroken,
    LeftArmBroken,
    RightArmBroken,

    // Predator special affs
    Acid,
    Fleshbane,
    Bloodscourge,
    Cirisosis,
    Veinrip,
    Negated,
    Intoxicated,

    // Mirrored affs
    Remorse,
    Contrition,

    // Firstaid flags
    FirstaidPredictAnyLimb,
    FirstaidPredictArms,
    FirstaidPredictLegs,
    FirstaidFocusMuddled,
    PreRestoreHead,
    PreRestoreTorso,
    PreRestoreLeftArm,
    PreRestoreRightArm,
    PreRestoreLeftLeg,
    PreRestoreRightLeg,

    // Persuasion
    Conflicted,
    Confounded,
    Engrossed,
    Entrenched,
    Fatigued,
    Pressured,
    LimitedAppeals,
    Slandered,
    Revelation,
    Gravitas,
    Influence,
    Conviction,
    Tradition,

    // Fleeing
    Fleeing,
}

lazy_static! {
    static ref AFFLICTIONS: Vec<FType> = {
        let mut afflictions = Vec::new();
        for aff_idx in (FType::Sadness as u16).. {
            if let Ok(affliction) = FType::try_from(aff_idx) {
                if affliction == FType::SIZE
                    || affliction == FType::FULL
                    || affliction == FType::TIMED
                {
                    continue;
                }
                afflictions.push(affliction);
            } else {
                break;
            }
        }
        afflictions
    };
}

impl FType {
    pub fn to_persuasion_aff(&self) -> Option<PersuasionAff> {
        match self {
            FType::Conflicted => Some(PersuasionAff::Conflicted),
            FType::Confounded => Some(PersuasionAff::Confounded),
            FType::Engrossed => Some(PersuasionAff::Engrossed),
            FType::Entrenched => Some(PersuasionAff::Entrenched),
            FType::Fatigued => Some(PersuasionAff::Fatigued),
            FType::Pressured => Some(PersuasionAff::Pressured),
            FType::LimitedAppeals => Some(PersuasionAff::LimitedAppeals),
            FType::Slandered => Some(PersuasionAff::Slandered),
            FType::Revelation => Some(PersuasionAff::Revelation),
            FType::Gravitas => Some(PersuasionAff::Gravitas),
            FType::Influence => Some(PersuasionAff::Influence),
            FType::Conviction => Some(PersuasionAff::Conviction),
            FType::Tradition => Some(PersuasionAff::Tradition),
            _ => None,
        }
    }

    pub fn is_affliction(&self) -> bool {
        self >= &FType::Sadness
    }

    pub fn is_general_defence(&self) -> bool {
        match self {
            FType::Courage
            | FType::Insomnia
            | FType::Fangbarrier
            | FType::Levitation
            | FType::Instawake
            | FType::Speed
            | FType::Arcane
            | FType::Density
            | FType::Vigor => true,
            _ => false,
        }
    }

    pub fn from_name(aff_name: &String) -> Option<FType> {
        let pretty = aff_name
            .split(|c| c == ' ' || c == '_' || c == '-')
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<String>();
        match pretty.as_ref() {
            "Heartflutter" => Some(FType::Arrhythmia),
            "Inoculated" => Some(FType::Imbued),
            "FungalInvasion" => Some(FType::Impeded),
            "Preymark" => Some(FType::Shadowbrand),
            "WoeCurse" => Some(FType::Shadowsphere),
            "Mystified" => Some(FType::Voidtrapped),
            _ => pretty.parse::<FType>().ok().map(|aff| aff.normalize()),
        }
    }

    pub fn to_name(&self) -> String {
        let mut words = vec![];
        let mut word = String::from("");
        self.to_string().chars().for_each(|letter| {
            if word.len() == 0 {
                word.push_str(&letter.to_lowercase().to_string());
            } else if letter.is_uppercase() {
                words.push(word.clone());
                word = letter.to_lowercase().to_string();
            } else {
                word.push_str(&letter.to_string());
            }
        });
        words.push(word.clone());
        words.join("_")
    }

    pub fn try_from_counter_idx(
        idx: usize,
    ) -> Result<FType, num_enum::TryFromPrimitiveError<FType>> {
        FType::try_from(FType::SIZE as u16 + 1 + idx as u16)
    }

    pub fn is_counter(&self) -> bool {
        self > &FType::SIZE && self < &FType::TIMED
    }

    pub fn is_timed(&self) -> bool {
        self > &FType::TIMED && self < &FType::FULL
    }

    pub fn is_controller(&self) -> bool {
        match self {
            FType::FirstaidPredictAnyLimb
            | FType::FirstaidPredictArms
            | FType::FirstaidPredictLegs
            | FType::FirstaidFocusMuddled
            | FType::PreRestoreHead
            | FType::PreRestoreTorso
            | FType::PreRestoreLeftArm
            | FType::PreRestoreRightArm
            | FType::PreRestoreLeftLeg
            | FType::PreRestoreRightLeg => true,
            _ => false,
        }
    }

    pub fn default_time(&self) -> f32 {
        match self {
            FType::Blackout => 3.0,
            FType::Stun => 1.0,
            FType::Shock => SHOCK_TIME,
            FType::Burnout => BURNOUT_TIME,
            FType::Muddled => 8.0,
            FType::Disrupted => 1.8,
            FType::Dazed => 2.8,
            FType::Voyria => 22.0,
            _ => 0.0,
        }
    }

    pub fn afflictions() -> Vec<Self> {
        AFFLICTIONS.to_vec()
    }

    pub fn is_mirror(&self) -> bool {
        self == &FType::Remorse || self == &FType::Contrition
    }

    pub fn normalize(&self) -> Self {
        match self {
            FType::Remorse => FType::Seduction,
            FType::Contrition => FType::Temptation,
            other => *other,
        }
    }
}

const COUNTERS_SIZE: usize = FType::TIMED as usize - FType::SIZE as usize - 1;
const TIMERS_SIZE: usize = FType::FULL as usize - FType::TIMED as usize - 1;

#[derive(PartialEq, Eq, Hash)]
pub struct FlagSet {
    simple: [bool; FType::SIZE as usize],
    counters: [u8; COUNTERS_SIZE],
    timed: [Timer; TIMERS_SIZE as usize],
}

/// A single semantic change to a [`FlagSet`], grouped by the kind of flag it targets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlagSetDiff {
    /// A normal flag was set.
    Add(FType),
    /// A normal flag was cleared.
    Remove(FType),
    /// A counter flag's count changed to a nonzero value.
    Set(FType, u8),
    /// A counter flag dropped to zero.
    Unset(FType),
    /// A timed flag was (re)armed with a fresh timer.
    Start(FType, Timer),
    /// A timed flag's existing timer advanced by this much time.
    Tick(FType, CType),
}

fn timed_flag(idx: usize) -> FType {
    FType::try_from(idx as u16 + FType::TIMED as u16 + 1).unwrap()
}

// Only CountDown-CountDown and matching CountUpObserve-CountUpObserve pairs can be
// expressed as a single elapsed-time tick; anything else needs a fresh Start.
fn tick_delta(old: &Timer, new: &Timer) -> Option<CType> {
    if !old.is_active() || !new.is_active() {
        return None;
    }
    match (old, new) {
        (Timer::CountDown(_), Timer::CountDown(_)) => {
            Some(old.get_time_left() - new.get_time_left())
        }
        (
            Timer::CountUpObserve {
                up_to: u1,
                expire_at: e1,
                ..
            },
            Timer::CountUpObserve {
                up_to: u2,
                expire_at: e2,
                ..
            },
        ) if u1 == u2 && e1 == e2 => Some(old.get_time_left() - new.get_time_left()),
        _ => None,
    }
}

impl StructDiff for FlagSet {
    type Diff = FlagSetDiff;
    type DiffRef<'target> = FlagSetDiff;

    fn diff(&self, updated: &Self) -> Vec<Self::Diff> {
        let mut diffs = Vec::new();

        for idx in 0..self.simple.len() {
            if self.simple[idx] != updated.simple[idx] {
                if let Ok(flag) = FType::try_from(idx as u16) {
                    diffs.push(if updated.simple[idx] {
                        FlagSetDiff::Add(flag)
                    } else {
                        FlagSetDiff::Remove(flag)
                    });
                }
            }
        }

        for idx in 0..self.counters.len() {
            if self.counters[idx] != updated.counters[idx] {
                if let Ok(flag) = FType::try_from_counter_idx(idx) {
                    diffs.push(if updated.counters[idx] > 0 {
                        FlagSetDiff::Set(flag, updated.counters[idx])
                    } else {
                        FlagSetDiff::Unset(flag)
                    });
                }
            }
        }

        for idx in 0..self.timed.len() {
            let (old, new) = (&self.timed[idx], &updated.timed[idx]);
            if old != new {
                let flag = timed_flag(idx);
                diffs.push(match tick_delta(old, new) {
                    Some(delta) => FlagSetDiff::Tick(flag, delta),
                    None => FlagSetDiff::Start(flag, *new),
                });
            }
        }

        diffs
    }

    fn diff_ref<'target>(&'target self, updated: &'target Self) -> Vec<Self::DiffRef<'target>> {
        self.diff(updated)
    }

    fn apply_single(&mut self, diff: Self::Diff) {
        match diff {
            FlagSetDiff::Add(flag) => self.simple[flag as usize] = true,
            FlagSetDiff::Remove(flag) => self.simple[flag as usize] = false,
            FlagSetDiff::Set(flag, value) => *self.get_counter_mut(flag) = value,
            FlagSetDiff::Unset(flag) => *self.get_counter_mut(flag) = 0,
            FlagSetDiff::Start(flag, timer) => *self.get_timer_mut(flag) = timer,
            FlagSetDiff::Tick(flag, delta) => self.get_timer_mut(flag).wait(delta),
        }
    }
}

impl FlagSet {
    pub fn wait(&mut self, duration: CType) {
        let confused = self.is_flag_set(FType::Confusion);
        let thin_blooded = self.is_flag_set(FType::Dyscrasia);
        for (i, timer) in self.timed.iter_mut().enumerate() {
            let flag = FType::try_from(i as u16 + FType::TIMED as u16 + 1).unwrap();
            if flag == FType::Disrupted && confused {
                continue;
            } else if flag == FType::Voyria && thin_blooded {
                timer.wait(duration); // Tick down twice as fast.
            }
            timer.wait(duration);
        }
    }

    fn get_counter(&self, flag: FType) -> u8 {
        self.counters[flag as usize - FType::SIZE as usize - 1]
    }

    fn get_counter_mut(&mut self, flag: FType) -> &mut u8 {
        &mut self.counters[flag as usize - FType::SIZE as usize - 1]
    }

    fn get_timer(&self, flag: FType) -> &Timer {
        &self.timed[flag as usize - FType::TIMED as usize - 1]
    }

    fn get_timer_mut(&mut self, flag: FType) -> &mut Timer {
        &mut self.timed[flag as usize - FType::TIMED as usize - 1]
    }

    pub fn is_flag_set(&self, flag: FType) -> bool {
        if flag.is_mirror() {
            self.is_flag_set(flag.normalize())
        } else if flag.is_counter() {
            self.get_counter(flag) > 0
        } else if flag.is_timed() {
            self.get_timer(flag).is_active()
        } else if flag.is_controller() {
            // Ignore controller flags.
            false
        } else {
            self.simple[flag as usize]
        }
    }

    pub fn get_flag_count(&self, flag: FType) -> u8 {
        if flag.is_mirror() {
            self.get_flag_count(flag.normalize())
        } else if flag.is_counter() {
            self.get_counter(flag)
        } else if flag.is_timed() {
            if self.get_timer(flag).is_active() {
                1
            } else {
                0
            }
        } else if flag.is_controller() {
            // Ignore controller flags.
            0
        } else {
            if self.simple[flag as usize] { 1 } else { 0 }
        }
    }

    pub fn set_flag(&mut self, flag: FType, value: bool) {
        if flag.is_mirror() {
            self.set_flag(flag.normalize(), value);
        } else if flag.is_counter() {
            let old_value = self.get_counter(flag);
            if value && old_value < 1 {
                *self.get_counter_mut(flag) = 1;
            } else if !value && old_value > 0 {
                *self.get_counter_mut(flag) = 0;
            }
        } else if flag.is_timed() {
            *self.get_timer_mut(flag) = if value {
                Timer::count_down_seconds(flag.default_time())
            } else {
                Timer::default()
            };
        } else if flag.is_controller() {
            // Ignore controller flags.
        } else {
            self.simple[flag as usize] = value;
        }
    }

    pub fn set_flag_count(&mut self, flag: FType, value: u8) {
        if flag.is_mirror() {
            self.set_flag_count(flag.normalize(), value);
        } else if flag.is_counter() {
            *self.get_counter_mut(flag) = value;
        } else if flag.is_timed() {
            *self.get_timer_mut(flag) = if value > 0 {
                Timer::count_down_seconds(flag.default_time())
            } else {
                Timer::default()
            };
        } else if flag.is_controller() {
            // Ignore controller flags.
        } else if flag != FType::Fleshbane {
            self.simple[flag as usize] = value > 0;
        }
    }

    pub fn tick_counter_up(&mut self, flag: FType) {
        if flag.is_mirror() {
            self.tick_counter_up(flag.normalize());
        } else if flag.is_counter() {
            *self.get_counter_mut(flag) += 1;
        } else {
            println!("Tried to tick up non-counter.");
        }
    }

    pub fn tick_counter_down(&mut self, flag: FType) {
        if flag.is_mirror() {
            self.tick_counter_down(flag.normalize());
        } else if flag.is_counter() {
            let old_value = self.get_counter(flag);
            if old_value > 0 {
                *self.get_counter_mut(flag) -= 1;
            }
        } else {
            println!("Tried to tick down non-counter.");
        }
    }
}

impl fmt::Debug for FlagSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut wrote = false;
        for idx in 0..self.simple.len() {
            if self.simple[idx] {
                if wrote {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", FType::try_from(idx as u16))?;
                wrote = true;
            }
        }
        for idx in 0..self.counters.len() {
            if self.counters[idx] > 0 {
                if wrote {
                    write!(f, ", ")?;
                }
                write!(
                    f,
                    "{:?}x{}",
                    FType::try_from_counter_idx(idx),
                    self.counters[idx]
                )?;
                wrote = true;
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for FlagSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote = false;
        for idx in 0..self.simple.len() {
            if self.simple[idx] {
                if let Ok(ftype) = FType::try_from(idx as u16) {
                    if ftype.is_affliction() {
                        if wrote {
                            write!(f, ", ")?;
                        }
                        write!(f, "{:?}", ftype)?;
                        wrote = true;
                    }
                }
            }
        }
        for idx in 0..self.counters.len() {
            if self.counters[idx] > 0 {
                if let Ok(ftype) = FType::try_from_counter_idx(idx) {
                    if ftype.is_affliction() {
                        if wrote {
                            write!(f, ", ")?;
                        }
                        write!(f, "{:?}x{}", ftype, self.counters[idx])?;
                        wrote = true;
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct FlagSetIterator<'s> {
    index: usize,
    simple: bool,
    set: &'s FlagSet,
    predicate: &'s dyn Fn(FType) -> bool,
}

impl<'s> FlagSetIterator<'s> {
    fn next_simple(&mut self) -> Option<FType> {
        while self.index < self.set.simple.len() && !self.set.simple[self.index] {
            self.index += 1;
        }
        if self.index < self.set.simple.len() {
            let ftype = FType::try_from(self.index as u16).unwrap();
            self.index += 1;
            if (self.predicate)(ftype) {
                Some(ftype)
            } else {
                self.next()
            }
        } else {
            None
        }
    }
    fn next_counter(&mut self) -> Option<FType> {
        while self.index < self.set.counters.len() && self.set.counters[self.index] == 0 {
            self.index += 1;
        }
        if self.index < self.set.counters.len() {
            let ftype = FType::try_from_counter_idx(self.index).unwrap();
            self.index += 1;
            if (self.predicate)(ftype) {
                Some(ftype)
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

impl<'s> Iterator for FlagSetIterator<'s> {
    type Item = FType;
    fn next(&mut self) -> Option<Self::Item> {
        if self.simple {
            if let Some(simple) = self.next_simple() {
                Some(simple)
            } else {
                self.simple = false;
                self.index = 0;
                self.next()
            }
        } else {
            self.next_counter()
        }
    }
}

impl<'s> FlagSetIterator<'s> {
    fn new(flagset: &'s FlagSet, predicate: &'s impl Fn(FType) -> bool) -> Self {
        FlagSetIterator {
            simple: true,
            index: 0,
            set: flagset,
            predicate,
        }
    }
}

impl FlagSet {
    pub fn aff_iter<'s>(&'s self) -> FlagSetIterator<'s> {
        FlagSetIterator::new(self, &|ftype: FType| ftype.is_affliction())
    }
}

impl Default for FlagSet {
    fn default() -> Self {
        FlagSet {
            simple: [false; FType::SIZE as usize],
            counters: [0; COUNTERS_SIZE],
            timed: [Timer::default(); TIMERS_SIZE as usize],
        }
    }
}

impl Clone for FlagSet {
    fn clone(&self) -> Self {
        FlagSet {
            simple: self.simple,
            counters: self.counters,
            timed: self.timed.clone(),
        }
    }
}
