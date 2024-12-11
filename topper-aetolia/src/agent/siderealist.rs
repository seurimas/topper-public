use super::*;
use serde::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SiderealistBoard {
    dustring: Timer,
    asterism: Timer,
    moonlet: Timer,
    magic_weakness: Option<(Timer, i32)>,
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

    pub fn dustring_hit(&mut self) {
        self.dustring.reset();
    }

    pub fn asterism_hit(&mut self) {
        self.asterism.reset();
    }

    pub fn moonlet_hit(&mut self) {
        self.moonlet.reset();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            "Eja Kodosa" => Some(Self::EjaKodosa),
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
            Self::EjaKodosa => "Eja Kodosa",
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
            Self::EjaKodosa => "tome",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub enum Vibration {
    Dissipate,
    Palpitation,
    Alarm,
    Tremors,
    Reverbation,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GleamStars {
    red: Timer,
    orange: Timer,
    yellow: Timer,
    green: Timer,
    blue: Timer,
    indigo: Timer,
    violet: Timer,
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SiderealistClassState {
    mend: Timer,
    parallax: Option<(Timer, String, String)>,
    gleam_cooldown: Timer,
    gleam_stars: Option<GleamStars>,
}

impl SiderealistClassState {
    pub fn new() -> Self {
        Self {
            mend: Timer::count_down_seconds(30.),
            parallax: None,
            gleam_cooldown: Timer::count_up_seconds_off(30.),
            gleam_stars: None,
        }
    }

    pub fn wait(&mut self, time: CType) {
        self.mend.wait(time);
        if let Some((timer, _, _)) = &mut self.parallax {
            timer.wait(time);
        }
        self.gleam_cooldown.wait(time);
        if let Some(gleam_stars) = &mut self.gleam_stars {
            gleam_stars.red.wait(time);
            gleam_stars.orange.wait(time);
            gleam_stars.yellow.wait(time);
            gleam_stars.green.wait(time);
            gleam_stars.blue.wait(time);
            gleam_stars.indigo.wait(time);
            gleam_stars.violet.wait(time);
        }
    }

    pub fn use_mend(&mut self) {
        self.mend.reset();
    }

    pub fn weave_parallax(&mut self, time: f32, spell: String, target: String) {
        self.parallax = Some((Timer::count_down_seconds(time), spell, target));
    }

    pub fn release_parallax(&mut self) {
        self.parallax = None;
    }

    pub fn use_gleam(&mut self) {
        self.gleam_stars = Some(GleamStars::default());
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
    }
}
