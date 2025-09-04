use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use topper_core::timeline::CType;

use crate::types::Timer;

use super::CooldownEffect;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum PhenomenaState {
    Blazewhirl,
    Glazeflow,
    Electrosphere,
}

impl PhenomenaState {
    pub fn name(&self) -> &str {
        match self {
            PhenomenaState::Blazewhirl => BLAZEWHIRL_NAME,
            PhenomenaState::Glazeflow => GLAZEFLOW_NAME,
            PhenomenaState::Electrosphere => ELECTROSPHERE_NAME,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Phenomena {
    pub room_id: i64,
    pub id: i64,
    pub stacks: i32,
    pub state: PhenomenaState,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AscendrilBoard {
    sunspot: Timer,
    icicles: i32,
    icicle_timer: Timer,
    shattering: bool,
    aeroblast: Timer,
}

impl AscendrilBoard {
    pub fn wait(&mut self, time: CType) {
        self.sunspot.wait(time);
        self.icicle_timer.wait(time);
        self.aeroblast.wait(time);
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
            self.shattering = true;
        }
    }

    pub fn shatter_down(&mut self) {
        self.shattering = false;
        self.icicles = 0;
        self.icicle_timer.expire();
    }

    pub fn shattering_active(&self) -> bool {
        self.shattering && self.icicles > 0
    }

    pub fn aeroblast(&mut self) {
        self.aeroblast = Timer::count_down_seconds(4.25);
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Water,
    Air,
    Spirit,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AscendrilClassState {
    fulcrum_up: bool,
    fulcrum_expanded: Option<i64>,
    fulcrum_glimpse: Option<(Timer, Element)>,
    schism: bool,
    imbalance: bool,
    enrich_timer: Timer,
    resonance: Option<(Element, i32)>,
    fireburst: Option<(Timer, i32)>,
    afterburn_raising: Timer,
    afterburn_up: Timer,
    my_phenomenon: Option<Phenomena>,
    freshest_phenomenon: Option<(i64, i64, PhenomenaState)>,
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
        if let Some((timer, _)) = &mut self.fulcrum_glimpse {
            timer.wait(time);
        }
        if let Some((timer, _)) = &mut self.fireburst {
            timer.wait(time);
        }
    }

    pub fn try_claim(&mut self, phenomenon: PhenomenaState) {
        if self.my_phenomenon.is_none() {
            if let Some((id, room_id, seen_state)) = self.freshest_phenomenon {
                if seen_state == phenomenon {
                    self.my_phenomenon = Some(Phenomena {
                        id,
                        room_id,
                        state: seen_state,
                        stacks: 0,
                    });
                }
            }
        }
    }

    pub fn cast_spell(&mut self, element: Element) {
        if let Some((resonance, stacks)) = &mut self.resonance {
            if *resonance == element {
                *stacks += 1;
            } else {
                *resonance = element;
                *stacks = 1;
            }
        } else {
            self.resonance = Some((element, 1));
        }
    }

    pub fn fulcrum_construct(&mut self) {
        self.fulcrum_up = true;
    }

    pub fn fulcrum_expand(&mut self, room_id: i64) {
        self.fulcrum_expanded = Some(room_id);
    }

    pub fn fulcrum_contract(&mut self) {
        self.fulcrum_up = true;
        self.fulcrum_expanded = None;
    }

    pub fn fulcrum_active(&self) -> bool {
        self.fulcrum_up
    }

    pub fn fulcrum_expanded(&self, room_id: i64) -> bool {
        self.fulcrum_expanded == Some(room_id)
    }

    pub fn fulcrum_glimpse(&mut self, element: Element) {
        self.fulcrum_glimpse = Some((Timer::count_down_seconds(60.), element));
    }

    pub fn is_glimpse_active(&self, element: Option<Element>) -> bool {
        if let Some((timer, glimpse)) = &self.fulcrum_glimpse {
            if let Some(element) = element {
                if *glimpse == element {
                    timer.is_active()
                } else {
                    false
                }
            } else {
                timer.is_active()
            }
        } else {
            false
        }
    }

    pub fn schism_on(&mut self) {
        self.schism = true;
    }

    pub fn imbalance_on(&mut self) {
        self.imbalance = true;
    }

    pub fn schism_active(&self, room_id: i64) -> bool {
        self.schism && (self.fulcrum_expanded.is_none() || self.fulcrum_expanded == Some(room_id))
    }

    pub fn imbalance_active(&self, room_id: i64) -> bool {
        self.imbalance
            && (self.fulcrum_expanded.is_none() || self.fulcrum_expanded == Some(room_id))
    }

    pub fn resonance_active(&self, element: &Element) -> bool {
        if let Some((resonance, stacks)) = &self.resonance {
            resonance == element && *stacks >= 2
        } else {
            false
        }
    }

    pub fn half_resonance_active(&self, element: &Element) -> bool {
        if let Some((resonance, stacks)) = &self.resonance {
            resonance == element && *stacks == 1
        } else {
            false
        }
    }

    pub fn use_up_resonance(&mut self) {
        self.resonance = None;
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
            if timer.is_active() {
                *stacks
            } else {
                0
            }
        } else {
            0
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
        self.resonance = Some((element, 2));
    }

    pub fn can_enrich(&self, element: &Element) -> bool {
        if let Some((resonance, stacks)) = &self.resonance {
            if resonance == element && *stacks >= 2 {
                false
            } else {
                !self.enrich_timer.is_active()
            }
        } else {
            !self.enrich_timer.is_active()
        }
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
            x if x.contains(GLAZEFLOW_NAME) => PhenomenaState::Glazeflow,
            x if x.contains(BLAZEWHIRL_NAME) => PhenomenaState::Blazewhirl,
            x if x.contains(ELECTROSPHERE_NAME) => PhenomenaState::Electrosphere,
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
