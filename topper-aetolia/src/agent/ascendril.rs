use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use topper_core::timeline::{BALANCE_SCALE, CType};

use crate::{
    agent::{AgentState, FType},
    types::Timer,
};

use super::CooldownEffect;

const ECHOES_TIME: f32 = 60.0 * 10.0 * BALANCE_SCALE;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub enum PhenomenaKind {
    Blazewhirl,
    Glazeflow,
    Electrosphere,
}

impl PhenomenaKind {
    pub fn name(&self) -> &str {
        match self {
            PhenomenaKind::Blazewhirl => BLAZEWHIRL_NAME,
            PhenomenaKind::Glazeflow => GLAZEFLOW_NAME,
            PhenomenaKind::Electrosphere => ELECTROSPHERE_NAME,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Phenomena {
    pub room_id: i64,
    pub id: i64,
    pub stacks: i32,
    pub state: PhenomenaKind,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AscendrilBoard {
    sunspot: Timer,
    icicles: i32,
    icicle_timer: Timer,
    shattered: i32,
    shattered_timer: Timer,
    aeroblast: Timer,
    aeroblast_stun: Timer,
}

impl AscendrilBoard {
    pub fn wait(&mut self, time: CType) {
        self.sunspot.wait(time);
        self.icicle_timer.wait(time);
        self.shattered_timer.wait(time);
        self.aeroblast.wait(time);
        self.aeroblast_stun.wait(time);
    }

    pub fn sunspot(&mut self) {
        self.sunspot = Timer::count_down_seconds(5.);
    }

    pub fn sunspot_active(&self) -> bool {
        self.sunspot.is_active()
    }

    pub fn icicles_spawn(&mut self) {
        self.icicles = 3;
        self.icicle_timer = Timer::count_down_seconds(3.5);
    }

    pub fn icicles_hit(&mut self) {
        self.icicles -= 1;
        self.icicle_timer = Timer::count_down_seconds(3.5);
        if self.icicles < 0 {
            self.icicles = 0;
        }
    }

    pub fn icicles_active(&self) -> bool {
        self.icicles > 0
    }

    pub fn shatter(&mut self) {
        if self.icicles > 0 {
            self.shattered = self.icicles * 3;
            self.shattered_timer = self.icicle_timer.clone();
            self.icicles = 0;
            self.icicle_timer.expire();
        }
    }

    pub fn shatter_down(&mut self) {
        self.shattered = 0;
        self.shattered_timer.expire();
    }

    pub fn shattering_active(&self) -> bool {
        self.shattered > 0
    }

    pub fn aeroblast(&mut self, fast: bool) {
        self.aeroblast = Timer::count_down_seconds(if fast { 4.0 } else { 8.0 });
    }

    pub fn aeroblast_hit(&mut self) {
        self.aeroblast.expire();
    }

    pub fn aeroblast_down(&mut self) {
        self.aeroblast.expire();
    }

    pub fn aeroblast_active(&self) -> bool {
        self.aeroblast.is_active()
    }

    pub fn aeroblast_stun(&mut self) {
        self.aeroblast_stun = Timer::count_down_seconds(4.25);
    }

    pub fn aeroblast_stun_hit(&mut self) {
        self.aeroblast_stun.expire();
    }

    pub fn aeroblast_stun_active(&self) -> bool {
        self.aeroblast_stun.is_active()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Water,
    Air,
    Spirit,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapacitanceState {
    #[default]
    NoCapacitance,
    CapacitanceComing {
        timer: Timer,
    },
    CapacitanceUp {
        timer: Timer,
        count: i32,
    },
}

impl CapacitanceState {
    pub fn wait(&mut self, time: CType) {
        match self {
            CapacitanceState::CapacitanceComing { timer } => {
                timer.wait(time);
                if !timer.is_active() {
                    *self = CapacitanceState::CapacitanceUp {
                        timer: Timer::count_down_seconds(80.),
                        count: 0,
                    };
                }
            }
            CapacitanceState::CapacitanceUp { timer, .. } => {
                timer.wait(time);
                if !timer.is_active() {
                    *self = CapacitanceState::NoCapacitance;
                }
            }
            CapacitanceState::NoCapacitance => {}
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FulcrumState {
    #[default]
    NoFulcrum,
    FulcrumOnMe {
        echoes: Timer,
        schism: bool,
        imbalance: bool,
        inactive_degradation: bool,
        inactive_spiritrift: bool,
        resonance: Option<(Element, i32)>,
    },
    FulcrumExpanded {
        room_id: i64,
        echoes: Timer,
        schism: bool,
        imbalance: bool,
        degradation: bool,
        spiritrift: bool,
        resonance: Option<(Element, i32)>,
        pushing: Option<(String, Timer)>,
    },
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhenomenaState {
    #[default]
    NoPhenomena,
    Unclaimed {
        kind: PhenomenaKind,
        target: String,
    },
    Claimed {
        id: i64,
        room_id: i64,
        stacks: i32,
        state: PhenomenaKind,
        target: String,
    },
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AscendrilClassState {
    fulcrum: FulcrumState,
    enrich_timer: Timer,
    fireburst: Option<(Timer, i32)>,
    afterburn_raising: Timer,
    afterburn_up: Timer,
    capacitance: CapacitanceState,
    my_phenomenon: PhenomenaState,
    freshest_phenomenon: Option<(i64, i64, PhenomenaKind)>,
    shift_cooldown: Timer,
}

impl AscendrilClassState {
    pub fn wait(&mut self, time: CType, cooldown_effect: CooldownEffect) {
        if self.afterburn_raising.is_active() {
            self.afterburn_raising.wait(time);
            if !self.afterburn_raising.is_active() {
                self.afterburn_up = Timer::count_down_seconds(40.);
            }
        }
        self.afterburn_up.wait(time);
        if !cooldown_effect {
            self.enrich_timer.wait(time);
        }
        if let Some((timer, _)) = &mut self.fireburst {
            timer.wait(time);
        }
        self.capacitance.wait(time);
        self.shift_cooldown.wait(time);
    }

    pub fn try_claim(&mut self, phenomenon: PhenomenaKind) {
        if let PhenomenaState::Unclaimed { kind, target } = self.my_phenomenon.clone() {
            if let Some((id, room_id, seen_state)) = self.freshest_phenomenon {
                if seen_state == phenomenon {
                    self.my_phenomenon = PhenomenaState::Claimed {
                        id,
                        room_id,
                        state: seen_state,
                        stacks: 0,
                        target,
                    };
                }
            }
        }
    }

    pub fn cast_spell(&mut self, element: Element) {
        if element == Element::Air {
            self.count_air_cast();
        } else if element == Element::Fire {
            self.lose_afterburn();
        }
        let resonance = match &mut self.fulcrum {
            FulcrumState::FulcrumOnMe { resonance, .. }
            | FulcrumState::FulcrumExpanded { resonance, .. } => resonance,
            FulcrumState::NoFulcrum => return,
        };
        if let Some((res, stacks)) = resonance {
            if *res == element {
                *stacks += 1;
            } else {
                *res = element;
                *stacks = 1;
            }
        } else {
            *resonance = Some((element, 1));
        }
    }

    pub fn fulcrum_construct(&mut self) {
        self.fulcrum = FulcrumState::FulcrumOnMe {
            echoes: Timer::count_down_seconds(0.),
            schism: false,
            imbalance: false,
            inactive_degradation: false,
            inactive_spiritrift: false,
            resonance: None,
        };
    }

    pub fn fulcrum_expand(&mut self, room_id: i64) {
        match self.fulcrum.clone() {
            FulcrumState::FulcrumOnMe {
                echoes,
                schism,
                imbalance,
                inactive_degradation,
                inactive_spiritrift,
                resonance,
            } => {
                self.fulcrum = FulcrumState::FulcrumExpanded {
                    room_id,
                    echoes,
                    schism,
                    imbalance,
                    degradation: inactive_degradation,
                    spiritrift: inactive_spiritrift,
                    resonance,
                    pushing: None,
                };
            }
            FulcrumState::FulcrumExpanded {
                echoes,
                schism,
                imbalance,
                degradation,
                spiritrift,
                resonance,
                ..
            } => {
                self.fulcrum = FulcrumState::FulcrumExpanded {
                    room_id,
                    echoes,
                    schism,
                    imbalance,
                    degradation,
                    spiritrift,
                    resonance,
                    pushing: None,
                };
            }
            FulcrumState::NoFulcrum => {
                self.fulcrum = FulcrumState::FulcrumExpanded {
                    room_id,
                    echoes: Timer::count_down_seconds(0.),
                    schism: false,
                    imbalance: false,
                    degradation: false,
                    spiritrift: false,
                    resonance: None,
                    pushing: None,
                };
            }
        }
    }

    pub fn fulcrum_push_start(&mut self, target_name: String) {
        if let FulcrumState::FulcrumExpanded { pushing, .. } = &mut self.fulcrum {
            *pushing = Some((target_name, Timer::count_down_seconds(3.)));
        }
    }

    pub fn fulcrum_push_end(&mut self) {
        if let FulcrumState::FulcrumExpanded { pushing, .. } = &mut self.fulcrum {
            *pushing = None;
        }
    }

    pub fn fulcrum_contract(&mut self) {
        match self.fulcrum.clone() {
            FulcrumState::FulcrumOnMe { .. } => {}
            FulcrumState::FulcrumExpanded {
                echoes,
                schism,
                imbalance,
                degradation,
                spiritrift,
                resonance,
                ..
            } => {
                self.fulcrum = FulcrumState::FulcrumOnMe {
                    echoes,
                    schism,
                    imbalance,
                    inactive_degradation: degradation,
                    inactive_spiritrift: spiritrift,
                    resonance,
                };
            }
            FulcrumState::NoFulcrum => {}
        }
    }

    pub fn fulcrum_active(&self) -> bool {
        !matches!(self.fulcrum, FulcrumState::NoFulcrum)
    }

    pub fn fulcrum_active_here(&self, room_id: i64) -> bool {
        match &self.fulcrum {
            FulcrumState::FulcrumOnMe { .. } => true,
            FulcrumState::FulcrumExpanded { room_id: r, .. } => *r == room_id,
            FulcrumState::NoFulcrum => false,
        }
    }

    pub fn fulcrum_expanded(&self, room_id: i64) -> bool {
        match &self.fulcrum {
            FulcrumState::FulcrumExpanded { room_id: r, .. } => *r == room_id,
            _ => false,
        }
    }

    pub fn fulcrum_interfused(&self) -> bool {
        matches!(self.fulcrum, FulcrumState::FulcrumOnMe { .. })
    }

    pub fn echoes_on(&mut self) {
        match &mut self.fulcrum {
            FulcrumState::FulcrumOnMe { echoes, .. }
            | FulcrumState::FulcrumExpanded { echoes, .. } => {
                *echoes = Timer::count_down_seconds(ECHOES_TIME);
            }
            FulcrumState::NoFulcrum => {}
        }
    }

    pub fn schism_on(&mut self) {
        match &mut self.fulcrum {
            FulcrumState::FulcrumOnMe { schism, .. }
            | FulcrumState::FulcrumExpanded { schism, .. } => *schism = true,
            FulcrumState::NoFulcrum => {}
        }
    }

    pub fn imbalance_on(&mut self) {
        match &mut self.fulcrum {
            FulcrumState::FulcrumOnMe { imbalance, .. }
            | FulcrumState::FulcrumExpanded { imbalance, .. } => *imbalance = true,
            FulcrumState::NoFulcrum => {}
        }
    }

    pub fn schism_active(&self, room_id: Option<i64>) -> bool {
        match &self.fulcrum {
            FulcrumState::FulcrumOnMe { schism, .. } => *schism,
            FulcrumState::FulcrumExpanded {
                room_id: r, schism, ..
            } => *schism && room_id.map_or(true, |id| *r == id),
            FulcrumState::NoFulcrum => false,
        }
    }

    pub fn imbalance_active(&self, room_id: Option<i64>) -> bool {
        match &self.fulcrum {
            FulcrumState::FulcrumOnMe { imbalance, .. } => *imbalance,
            FulcrumState::FulcrumExpanded {
                room_id: r,
                imbalance,
                ..
            } => *imbalance && room_id.map_or(true, |id| *r == id),
            FulcrumState::NoFulcrum => false,
        }
    }

    pub fn has_no_resonance(&self) -> bool {
        let resonance = match &self.fulcrum {
            FulcrumState::FulcrumOnMe { resonance, .. }
            | FulcrumState::FulcrumExpanded { resonance, .. } => resonance,
            FulcrumState::NoFulcrum => return true,
        };
        resonance.is_none()
    }

    pub fn resonance_active(&self, element: &Element) -> bool {
        let resonance = match &self.fulcrum {
            FulcrumState::FulcrumOnMe { resonance, .. }
            | FulcrumState::FulcrumExpanded { resonance, .. } => resonance,
            FulcrumState::NoFulcrum => return false,
        };
        matches!(resonance, Some((res, stacks)) if res == element && *stacks >= 2)
    }

    pub fn half_resonance_active(&self, element: &Element) -> bool {
        let resonance = match &self.fulcrum {
            FulcrumState::FulcrumOnMe { resonance, .. }
            | FulcrumState::FulcrumExpanded { resonance, .. } => resonance,
            FulcrumState::NoFulcrum => return false,
        };
        matches!(resonance, Some((res, stacks)) if res == element && *stacks == 1)
    }

    pub fn use_up_resonance(&mut self) {
        match &mut self.fulcrum {
            FulcrumState::FulcrumOnMe { resonance, .. }
            | FulcrumState::FulcrumExpanded { resonance, .. } => *resonance = None,
            FulcrumState::NoFulcrum => {}
        }
    }

    pub fn fireburst_fill(&mut self) {
        self.fireburst = Some((Timer::count_down_seconds(60.), 4));
    }

    pub fn fireburst_decrement(&mut self) {
        if let Some((timer, stacks)) = &mut self.fireburst {
            *stacks -= 1;
            if *stacks == 0 {
                timer.expire();
            }
        }
    }

    pub fn fireburst_stacks(&self) -> i32 {
        if let Some((timer, stacks)) = &self.fireburst {
            if timer.is_active() { *stacks } else { 0 }
        } else {
            0
        }
    }

    pub fn raise_capacitance(&mut self) {
        self.capacitance = CapacitanceState::CapacitanceComing {
            timer: Timer::count_down_seconds(5.),
        };
    }

    pub fn lose_capacitance(&mut self) {
        self.capacitance = CapacitanceState::NoCapacitance;
    }

    pub fn capacitance_coming_up(&self) -> bool {
        matches!(self.capacitance, CapacitanceState::CapacitanceComing { .. })
    }

    pub fn capacitance_active(&self) -> bool {
        matches!(self.capacitance, CapacitanceState::CapacitanceUp { .. })
    }

    pub fn capacitance_will_disrupt(&self) -> bool {
        matches!(self.capacitance, CapacitanceState::CapacitanceUp { count, .. } if count >= 4)
    }

    fn count_air_cast(&mut self) {
        if let CapacitanceState::CapacitanceUp { count, .. } = &mut self.capacitance {
            *count += 1;
            if *count >= 5 {
                self.capacitance = CapacitanceState::NoCapacitance;
            }
        }
    }

    pub fn raise_afterburn(&mut self) {
        self.afterburn_raising = Timer::count_down_seconds(5.);
    }

    pub fn get_afterburn(&mut self) {
        self.afterburn_raising.expire();
        self.afterburn_up = Timer::count_down_seconds(40.);
    }

    pub fn lose_afterburn(&mut self) {
        self.afterburn_up.expire();
    }

    pub fn afterburn_coming_up(&self) -> bool {
        self.afterburn_raising.is_active()
    }

    pub fn afterburn_active(&self) -> bool {
        self.afterburn_up.is_active()
    }

    pub fn enrich(&mut self, element: Element) {
        self.enrich_timer = Timer::count_down_seconds(30.);
        match &mut self.fulcrum {
            FulcrumState::FulcrumOnMe { resonance, .. }
            | FulcrumState::FulcrumExpanded { resonance, .. } => *resonance = Some((element, 2)),
            FulcrumState::NoFulcrum => {}
        }
    }

    pub fn can_enrich(&self, room_id: i64, element: &Element) -> bool {
        if self.enrich_timer.is_active() {
            return false;
        }
        let resonance = match &self.fulcrum {
            FulcrumState::FulcrumOnMe { resonance, .. } => resonance,
            FulcrumState::FulcrumExpanded {
                resonance,
                room_id: r,
                ..
            } if *r == room_id => resonance,
            FulcrumState::NoFulcrum => return false,
            _ => return false,
        };
        !matches!(resonance, Some((res, stacks)) if res == element && *stacks >= 2)
    }

    pub fn can_catalyze(&self, room_id: i64, target: &AgentState, element: &Element) -> bool {
        target.is(FType::Etherflux)
            && ((*element == Element::Fire && target.is(FType::Emberbrand))
                || (*element == Element::Water && target.is(FType::Frostbrand))
                || (*element == Element::Air && target.is(FType::Thunderbrand)))
            && self.fulcrum_active_here(room_id)
    }

    pub fn degradation_on(&mut self) {
        if let FulcrumState::FulcrumExpanded { degradation, .. } = &mut self.fulcrum {
            *degradation = true;
        }
    }

    pub fn degradation_active(&self, room_id: Option<i64>) -> bool {
        match &self.fulcrum {
            FulcrumState::FulcrumExpanded {
                room_id: r,
                degradation,
                ..
            } => *degradation && room_id.map_or(true, |id| *r == id),
            _ => false,
        }
    }

    pub fn spiritrift_on(&mut self) {
        if let FulcrumState::FulcrumExpanded { spiritrift, .. } = &mut self.fulcrum {
            *spiritrift = true;
        }
    }

    pub fn spiritrift_active(&self, room_id: Option<i64>) -> bool {
        match &self.fulcrum {
            FulcrumState::FulcrumExpanded {
                room_id: r,
                spiritrift,
                ..
            } => *spiritrift && room_id.map_or(true, |id| *r == id),
            _ => false,
        }
    }

    pub fn use_shift(&mut self) {
        self.shift_cooldown = Timer::count_down_seconds(25.);
    }

    pub fn can_shift(&self) -> bool {
        !self.shift_cooldown.is_active()
    }

    pub fn fulcrum_destroy(&mut self) {
        self.fulcrum = FulcrumState::NoFulcrum;
    }

    pub fn phenomenon_active(&self, phenomenon: Option<PhenomenaKind>) -> bool {
        if let PhenomenaState::Claimed { state, .. } = &self.my_phenomenon {
            if let Some(p) = phenomenon {
                return *state == p;
            } else {
                return true;
            }
        }
        false
    }

    pub fn phenomenon_in_room(&self, room_id: i64, phenomenon: Option<PhenomenaKind>) -> bool {
        if let PhenomenaState::Claimed {
            state, room_id: r, ..
        } = &self.my_phenomenon
        {
            if let Some(p) = phenomenon {
                return *r == room_id && *state == p;
            } else {
                return *r == room_id;
            }
        }
        false
    }

    pub fn phenomenon_chasing(
        &self,
        room_id: Option<i64>,
        target_name: &String,
        phenomenon: Option<PhenomenaKind>,
    ) -> bool {
        if let PhenomenaState::Claimed {
            state,
            room_id: r,
            target,
            ..
        } = &self.my_phenomenon
        {
            if let Some(room_id) = room_id {
                if *r != room_id {
                    return false;
                }
            }
            if let Some(p) = phenomenon {
                return *state == p && target_name.eq(target);
            } else {
                return target_name.eq(target);
            }
        }
        false
    }

    pub fn enrapture_accelerated(&self, target: &AgentState) -> bool {
        self.resonance_active(&Element::Fire) && target.is(FType::Emberbrand)
            || self.resonance_active(&Element::Water) && target.is(FType::Frostbrand)
            || self.resonance_active(&Element::Air) && target.is(FType::Thunderbrand)
    }
}

pub const GLAZEFLOW_NAME: &str = "a flow of icy glaze";
pub const BLAZEWHIRL_NAME: &str = "a raging whirl of fire";
pub const ELECTROSPHERE_NAME: &str = "a shocking, electric sphere";

pub fn ascendril_add_item(
    id: i64,
    name: &str,
    in_room: Option<i64>,
) -> Box<dyn Fn(&mut AscendrilClassState)> {
    if in_room.is_none() {
        Box::new(move |_| {})
    } else if name.contains(GLAZEFLOW_NAME)
        || name.contains(BLAZEWHIRL_NAME)
        || name.contains(ELECTROSPHERE_NAME)
    {
        let state = match name {
            x if x.contains(GLAZEFLOW_NAME) => PhenomenaKind::Glazeflow,
            x if x.contains(BLAZEWHIRL_NAME) => PhenomenaKind::Blazewhirl,
            x if x.contains(ELECTROSPHERE_NAME) => PhenomenaKind::Electrosphere,
            _ => unreachable!(),
        };
        Box::new(move |me| {
            me.freshest_phenomenon = Some((id, in_room.unwrap(), state));
        })
    } else {
        Box::new(move |_| {})
    }
}

pub fn ascendril_remove_item(
    id: i64,
    name: &str,
    in_room: Option<i64>,
) -> Box<dyn Fn(&mut AscendrilClassState)> {
    if in_room.is_none() {
        Box::new(move |_| {})
    } else if name.contains(GLAZEFLOW_NAME)
        || name.contains(BLAZEWHIRL_NAME)
        || name.contains(ELECTROSPHERE_NAME)
    {
        let name = name.to_string();
        Box::new(move |me| {})
    } else {
        Box::new(move |_| {})
    }
}
