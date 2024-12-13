use std::{collections::HashMap, hash::Hash};

use super::*;
use serde::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SiderealistBoard {
    dustring: Timer,
    asterism: Timer,
    moonlet: Timer,
    magic_weakness: Option<(Timer, i32)>,
    irradiated_limb: Option<LType>,
}

impl Default for SiderealistBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl SiderealistBoard {
    pub fn new() -> Self {
        Self {
            dustring: Timer::count_up_seconds_off(150.),
            asterism: Timer::count_up_seconds_off(150.),
            moonlet: Timer::count_up_seconds_off(150.),
            magic_weakness: None,
            irradiated_limb: None,
        }
    }

    pub fn wait(&mut self, time: CType) {
        self.dustring.wait(time);
        self.asterism.wait(time);
        self.moonlet.wait(time);
        if let Some((timer, _)) = &mut self.magic_weakness {
            timer.wait(time);
        }
    }

    pub fn ray(&mut self) {
        let mut anomalies = 0;
        if self.dustring.is_active() {
            anomalies += 1;
        }
        if self.asterism.is_active() {
            anomalies += 1;
        }
        if self.moonlet.is_active() {
            anomalies += 1;
        }
        let time = 8. + (anomalies as f32 * 3.);
        self.magic_weakness = Some((Timer::count_down_seconds(time), anomalies));
    }

    pub fn irradiate(&mut self, limb: LType) {
        self.irradiated_limb = Some(limb);
    }

    pub fn has_irradiated_limb(&self, limb: LType) -> bool {
        self.irradiated_limb == Some(limb)
    }

    pub fn dustring_hit(&mut self) {
        self.dustring.reset();
    }

    pub fn asterism_hit(&mut self) {
        self.asterism.reset();
    }

    pub fn moonlet_hit(&mut self) {
        self.moonlet.reset();
    }

    pub fn has_dustring(&self) -> bool {
        self.dustring.is_active()
    }

    pub fn has_asterism(&self) -> bool {
        self.asterism.is_active()
    }

    pub fn has_moonlet(&self) -> bool {
        self.moonlet.is_active()
    }

    pub fn syzygy_hit(&mut self) {
        self.dustring.expire();
        self.asterism.expire();
        self.moonlet.expire();
    }

    pub fn expire_oldest_anomaly(&mut self) {
        let soonest = self
            .dustring
            .get_time_left_seconds()
            .min(self.asterism.get_time_left_seconds())
            .min(self.moonlet.get_time_left_seconds());
        if soonest == self.dustring.get_time_left_seconds() {
            self.dustring.expire();
        } else if soonest == self.asterism.get_time_left_seconds() {
            self.asterism.expire();
        } else {
            self.moonlet.expire();
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Regalia {
    Artos,
    Cthalut,
    Treyes,
    IzuAri,
    Vayua,
    Loskiou,
    Peripleko,
    Umbrael,
    Ontesme,
    Ulgar,
    Drobia,
    Gulbedim,
    Averroes,
    EjaKodosa,
}

impl Regalia {
    pub fn try_from_name(name: &str) -> Option<Self> {
        match name {
            "Artos" => Some(Self::Artos),
            "Cthalut" => Some(Self::Cthalut),
            "Treyes" => Some(Self::Treyes),
            "Izu-Ari" => Some(Self::IzuAri),
            "Vayua" => Some(Self::Vayua),
            "Loskiou" => Some(Self::Loskiou),
            "Peripleko" => Some(Self::Peripleko),
            "Umbrael" => Some(Self::Umbrael),
            "Ontesme" => Some(Self::Ontesme),
            "Ulgar" => Some(Self::Ulgar),
            "Drobia" => Some(Self::Drobia),
            "Gulbedim" => Some(Self::Gulbedim),
            "Averroes" => Some(Self::Averroes),
            "Eja Kodosa" | "EjaKodosa" => Some(Self::EjaKodosa),
            _ => None,
        }
    }

    pub fn try_from_type(regalia_type: &str) -> Option<Self> {
        match regalia_type {
            "glasses" => Some(Self::Artos),
            "bow" => Some(Self::Cthalut),
            "candle" => Some(Self::Treyes),
            "coin" => Some(Self::IzuAri),
            "sword" => Some(Self::Vayua),
            "robe" => Some(Self::Loskiou),
            "shoes" => Some(Self::Peripleko),
            "cloak" => Some(Self::Umbrael),
            "mask" => Some(Self::Ontesme),
            "hammer" => Some(Self::Ulgar),
            "gear" => Some(Self::Drobia),
            "mantle" => Some(Self::Gulbedim),
            "staff" => Some(Self::Averroes),
            "book" => Some(Self::EjaKodosa),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Artos => "Artos",
            Self::Cthalut => "Cthalut",
            Self::Treyes => "Treyes",
            Self::IzuAri => "Izu-Ari",
            Self::Vayua => "Vayua",
            Self::Loskiou => "Loskiou",
            Self::Peripleko => "Peripleko",
            Self::Umbrael => "Umbrael",
            Self::Ontesme => "Ontesme",
            Self::Ulgar => "Ulgar",
            Self::Drobia => "Drobia",
            Self::Gulbedim => "Gulbedim",
            Self::Averroes => "Averroes",
            Self::EjaKodosa => "EjaKodosa", // WHY
        }
    }

    pub fn regalia_type(&self) -> &str {
        match self {
            Self::Artos => "glasses",
            Self::Cthalut => "bow",
            Self::Treyes => "candle",
            Self::IzuAri => "coin",
            Self::Vayua => "sword",
            Self::Loskiou => "robe",
            Self::Peripleko => "shoes",
            Self::Umbrael => "cloak",
            Self::Ontesme => "mask",
            Self::Ulgar => "hammer",
            Self::Drobia => "gear",
            Self::Gulbedim => "mantle",
            Self::Averroes => "staff",
            Self::EjaKodosa => "book",
        }
    }

    pub fn from_item_name(name: &str) -> Option<Self> {
        Self::try_from_type(name.trim_matches(char::is_numeric))
    }
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Vibration {
    Dissipate,
    Palpitation,
    Alarm,
    Tremors,
    Reverberation,
    Revelation,
    Adduction,
    Harmony,
    Creeps,
    Din,
    Skyfall,
    Oscillate,
    Altering,
    Focus,
    Disorientation,
    Energize,
    Stridulation,
    Cavitation,
    Crystalforest,
    Dissension,
    Plague,
    Lullaby,
    Retrogradation,
    Cataclysm,
}

impl Vibration {
    pub fn from_name(name: String) -> Option<Vibration> {
        match name.to_lowercase().as_ref() {
            "dissipate" => Some(Self::Dissipate),
            "palpitation" => Some(Self::Palpitation),
            "alarm" => Some(Self::Alarm),
            "tremors" => Some(Self::Tremors),
            "reverberation" => Some(Self::Reverberation),
            "revelation" => Some(Self::Revelation),
            "adduction" => Some(Self::Adduction),
            "harmony" => Some(Self::Harmony),
            "creeps" => Some(Self::Creeps),
            "din" => Some(Self::Din),
            "skyfall" => Some(Self::Skyfall),
            "oscillate" => Some(Self::Oscillate),
            "altering" => Some(Self::Altering),
            "focus" => Some(Self::Focus),
            "disorientation" => Some(Self::Disorientation),
            "energize" => Some(Self::Energize),
            "stridulation" => Some(Self::Stridulation),
            "cavitation" => Some(Self::Cavitation),
            "crystalforest" | "forest" => Some(Self::Crystalforest),
            "dissension" => Some(Self::Dissension),
            "plague" => Some(Self::Plague),
            "lullaby" => Some(Self::Lullaby),
            "retrogradation" => Some(Self::Retrogradation),
            "cataclysm" => Some(Self::Cataclysm),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Copy, PartialEq, Eq, Hash)]
pub enum VibrationState {
    Dormant(Timer),
    Active(Timer),
}

impl VibrationState {
    pub fn fresh() -> Self {
        Self::Active(Timer::count_down_seconds(1200.))
    }

    pub fn active_seconds(seconds: i32) -> Self {
        Self::Active(Timer::count_down_seconds(seconds as f32))
    }

    pub fn dormant_seconds(seconds: i32) -> Self {
        Self::Dormant(Timer::count_down_seconds(seconds as f32))
    }

    pub fn activate(&self) -> Self {
        match self {
            Self::Dormant(time) => Self::Active(time.clone()),
            Self::Active(_) => self.clone(),
        }
    }

    pub fn wait(&mut self, time: CType) {
        match self {
            Self::Dormant(timer) => {}
            Self::Active(timer) => timer.wait(time),
        }
    }

    pub fn is_dormant(&self) -> bool {
        match self {
            Self::Dormant(_) => true,
            _ => false,
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            Self::Active(timer) => timer.is_active(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum SummonState {
    #[default]
    None,
    Glimmercrest(bool, Option<String>, Timer),
    Sprite(bool, Option<String>, Timer),
}

impl SummonState {
    pub fn wait(&mut self, time: CType) {
        match self {
            Self::None => {}
            Self::Glimmercrest(_, _, timer) => timer.wait(time),
            Self::Sprite(_, _, timer) => timer.wait(time),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub enum GleamColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
}

impl GleamColor {
    pub fn name(&self) -> &str {
        match self {
            Self::Red => "red",
            Self::Orange => "orange",
            Self::Yellow => "yellow",
            Self::Green => "green",
            Self::Blue => "blue",
            Self::Indigo => "indigo",
            Self::Violet => "violet",
        }
    }

    pub fn from_annotation(annotation: &str) -> Option<Self> {
        match annotation {
            "burning" => Some(Self::Red),
            "crushing" => Some(Self::Orange),
            "radiant" => Some(Self::Yellow),
            "noxious" => Some(Self::Green),
            "frigid" => Some(Self::Blue),
            "temporal" => Some(Self::Indigo),
            "twisted" => Some(Self::Violet),
            _ => None,
        }
    }

    pub fn affliction(&self) -> FType {
        match self {
            Self::Red => FType::Recklessness,
            Self::Orange => FType::Gnawing,
            Self::Yellow => FType::Dizziness,
            Self::Green => FType::Clumsiness,
            Self::Blue => FType::Mindfog,
            Self::Indigo => FType::Paresis,
            Self::Violet => FType::Paranoia,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GleamStars {
    pub red: Timer,
    pub orange: Timer,
    pub yellow: Timer,
    pub green: Timer,
    pub blue: Timer,
    pub indigo: Timer,
    pub violet: Timer,
}

impl Default for GleamStars {
    fn default() -> Self {
        Self {
            red: Timer::count_up_seconds_off(12.),
            orange: Timer::count_up_seconds_off(12.),
            yellow: Timer::count_up_seconds_off(12.),
            green: Timer::count_up_seconds_off(12.),
            blue: Timer::count_up_seconds_off(12.),
            indigo: Timer::count_up_seconds_off(12.),
            violet: Timer::count_up_seconds_off(12.),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SiderealistClassState {
    mend: Timer,
    parallax: Option<(Timer, String, String)>,
    gleam_cooldown: Timer,
    foresight_cooldown: Timer,
    centrum_cooldown: Timer,
    crystalforest_cooldown: Timer,
    pub gleam_stars: Option<GleamStars>,
    last_vibration_location: HashMap<Vibration, (i64, VibrationState)>,
    summon_state: SummonState,
    first_regalia: Option<Regalia>,
    second_regalia: Option<Regalia>,
}

impl Hash for SiderealistClassState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.mend.hash(state);
        self.parallax.hash(state);
        self.gleam_cooldown.hash(state);
        self.centrum_cooldown.hash(state);
        self.crystalforest_cooldown.hash(state);
        self.gleam_stars.hash(state);
        // self.last_vibration_location.hash(state);
        self.summon_state.hash(state);
        self.first_regalia.hash(state);
        self.second_regalia.hash(state);
        self.foresight_cooldown.hash(state);
    }
}

impl SiderealistClassState {
    pub fn new() -> Self {
        Self {
            mend: Timer::count_down_seconds(30.),
            parallax: None,
            gleam_cooldown: Timer::count_up_seconds_off(30.),
            foresight_cooldown: Timer::count_up_seconds_off(30.),
            centrum_cooldown: Timer::count_up_seconds_off(30.),
            crystalforest_cooldown: Timer::count_up_seconds_off(20.),
            gleam_stars: None,
            last_vibration_location: HashMap::new(),
            summon_state: SummonState::None,
            first_regalia: None,
            second_regalia: None,
        }
    }

    pub fn wait(&mut self, time: CType) {
        self.mend.wait(time);
        if let Some((timer, _, _)) = &mut self.parallax {
            timer.wait(time);
        }
        self.gleam_cooldown.wait(time);
        self.foresight_cooldown.wait(time);
        self.centrum_cooldown.wait(time);
        self.crystalforest_cooldown.wait(time);
        if let Some(gleam_stars) = &mut self.gleam_stars {
            gleam_stars.red.wait(time);
            gleam_stars.orange.wait(time);
            gleam_stars.yellow.wait(time);
            gleam_stars.green.wait(time);
            gleam_stars.blue.wait(time);
            gleam_stars.indigo.wait(time);
            gleam_stars.violet.wait(time);
        }
        for (_, (_, state)) in self.last_vibration_location.iter_mut() {
            state.wait(time);
        }
        self.summon_state.wait(time);
    }

    pub fn weave_parallax(&mut self, time: f32, spell: String, target: String) {
        self.parallax = Some((Timer::count_down_seconds(time), spell, target));
    }

    pub fn get_parallax(&self) -> Option<&(Timer, String, String)> {
        self.parallax.as_ref()
    }

    pub fn has_parallax(&self) -> bool {
        self.parallax.is_some()
    }

    pub fn is_parallaxing(&self, spell: &str) -> bool {
        if let Some((_, parallax_spell, _)) = &self.parallax {
            parallax_spell == spell
        } else {
            false
        }
    }

    pub fn release_parallax(&mut self) {
        self.parallax = None;
    }

    pub fn use_gleam(&mut self) {
        self.gleam_stars = Some(GleamStars::default());
    }

    pub fn has_gleam(&self) -> bool {
        self.gleam_stars.is_some()
    }

    pub fn can_gleam(&self) -> bool {
        !self.gleam_cooldown.is_active()
    }

    pub fn has_gleam_star(&self, color: GleamColor) -> bool {
        if let Some(gleam_stars) = &self.gleam_stars {
            match color {
                GleamColor::Red => !gleam_stars.red.is_active(),
                GleamColor::Orange => !gleam_stars.orange.is_active(),
                GleamColor::Yellow => !gleam_stars.yellow.is_active(),
                GleamColor::Green => !gleam_stars.green.is_active(),
                GleamColor::Blue => !gleam_stars.blue.is_active(),
                GleamColor::Indigo => !gleam_stars.indigo.is_active(),
                GleamColor::Violet => !gleam_stars.violet.is_active(),
            }
        } else {
            false
        }
    }

    pub fn inflict(&mut self, color: GleamColor) {
        if self.gleam_stars.is_none() {
            self.gleam_stars = Some(GleamStars::default());
        }
        if let Some(gleam_stars) = &mut self.gleam_stars {
            match color {
                GleamColor::Red => gleam_stars.red.reset(),
                GleamColor::Orange => gleam_stars.orange.reset(),
                GleamColor::Yellow => gleam_stars.yellow.reset(),
                GleamColor::Green => gleam_stars.green.reset(),
                GleamColor::Blue => gleam_stars.blue.reset(),
                GleamColor::Indigo => gleam_stars.indigo.reset(),
                GleamColor::Violet => gleam_stars.violet.reset(),
            }
        }
    }

    pub fn chromaflare(&mut self) {
        self.gleam_stars = None;
        self.gleam_cooldown.reset();
    }

    pub fn vibration_states(&self) -> Vec<(Vibration, VibrationState)> {
        self.last_vibration_location
            .iter()
            .map(|(vibration, (_, state))| (*vibration, *state))
            .collect()
    }

    pub fn has_vibration(&self, vibration: Vibration) -> bool {
        if let Some((room_id, v_state)) = self.last_vibration_location.get(&vibration) {
            match v_state {
                VibrationState::Active(timer) => timer.is_active(),
                VibrationState::Dormant(_) => true,
            }
        } else {
            false
        }
    }

    pub fn embed(&mut self, vibration: Vibration, room_id: i64) {
        self.last_vibration_location
            .insert(vibration, (room_id, VibrationState::fresh()));
    }

    pub fn set_vibration(&mut self, vibration: Vibration, room_id: i64, state: VibrationState) {
        self.last_vibration_location
            .insert(vibration, (room_id, state));
    }

    pub fn has_glimmercrest(&self) -> bool {
        matches!(self.summon_state, SummonState::Glimmercrest(_, _, _))
    }

    pub fn has_sprite(&self) -> bool {
        matches!(self.summon_state, SummonState::Sprite(_, _, _))
    }

    pub fn enigmaed(&mut self) {
        self.summon_state = SummonState::Glimmercrest(true, None, Timer::count_down_seconds(30.));
    }

    pub fn embodied(&mut self) {
        self.summon_state = SummonState::Sprite(true, None, Timer::count_down_seconds(30.));
    }

    pub fn order_glimmercrest_attack(&mut self, target: String) {
        self.summon_state =
            SummonState::Glimmercrest(true, Some(target), Timer::count_down_seconds(30.));
    }

    pub fn order_sprite_attack(&mut self, target: String) {
        self.summon_state = SummonState::Sprite(true, Some(target), Timer::count_down_seconds(30.));
    }

    pub fn order_glimmercrest_passive(&mut self) {
        self.summon_state = SummonState::Glimmercrest(true, None, Timer::count_down_seconds(30.));
    }

    pub fn order_sprite_passive(&mut self) {
        self.summon_state = SummonState::Sprite(true, None, Timer::count_down_seconds(30.));
    }

    pub fn cannot_see_glimmercrest(&mut self) {
        if let SummonState::Glimmercrest(in_room, _, timer) = &mut self.summon_state {
            *in_room = false;
        }
    }

    pub fn cannot_see_sprite(&mut self) {
        if let SummonState::Sprite(in_room, _, timer) = &mut self.summon_state {
            *in_room = false;
        }
    }

    pub fn glimmercrest_attacking(&self, who: &str) -> bool {
        if let SummonState::Glimmercrest(_, Some(target), last_seen) = &self.summon_state {
            target == who && last_seen.is_active()
        } else {
            false
        }
    }

    pub fn sprite_attacking(&self, who: &str) -> bool {
        if let SummonState::Sprite(_, Some(target), last_seen) = &self.summon_state {
            target == who && last_seen.is_active()
        } else {
            false
        }
    }

    pub fn summon_state(&self) -> &SummonState {
        &self.summon_state
    }

    pub fn kill_companion(&mut self) {
        self.summon_state = SummonState::None;
    }

    pub fn get_regalia(&self) -> Vec<Option<Regalia>> {
        vec![self.first_regalia, self.second_regalia]
    }

    pub fn has_regalia(&self, regalia: Regalia) -> bool {
        self.first_regalia == Some(regalia) || self.second_regalia == Some(regalia)
    }

    pub fn has_two_regalias(&self) -> bool {
        self.first_regalia.is_some() && self.second_regalia.is_some()
    }

    pub fn first_regalia_of(&self, regalias: &Vec<Regalia>) -> Option<Regalia> {
        for regalia in regalias {
            if self.first_regalia == Some(*regalia) {
                return Some(*regalia);
            } else if self.second_regalia == Some(*regalia) {
                return Some(*regalia);
            }
        }
        None
    }

    pub fn observe_regalia(&mut self, first: Option<Regalia>, second: Option<Regalia>) {
        self.first_regalia = first;
        self.second_regalia = second;
    }

    pub fn foresighted(&mut self) {
        self.foresight_cooldown.reset();
    }

    pub fn can_foresight(&self) -> bool {
        !self.foresight_cooldown.is_active()
    }

    pub fn centrumed(&mut self) {
        self.centrum_cooldown.reset();
    }

    pub fn can_centrum(&self) -> bool {
        !self.centrum_cooldown.is_active()
    }

    pub fn mended(&mut self) {
        self.mend.reset();
    }

    pub fn can_mend(&self) -> bool {
        !self.mend.is_active()
    }

    pub fn shatter_crystalforest(&mut self) {
        self.crystalforest_cooldown.reset();
    }

    pub fn can_crystalforest(&self) -> bool {
        !self.crystalforest_cooldown.is_active()
    }
}
