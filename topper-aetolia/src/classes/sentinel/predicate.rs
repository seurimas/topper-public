use behavior_bark::unpowered::*;
use serde::*;

use crate::{
    agent::sentinel::{Resin, SentinelBeast},
    bt::*,
    types::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SentinelPredicate {
    // Beast predicates
    HasBeast(SentinelBeast),
    BeastCountOver(usize),
    // Resin predicates
    HasColdResin,
    HasHotResin,
    ColdResinIs(Resin),
    HotResinIs(Resin),
    IsBurning,
    ResinTicksOver(u8),
    // Alacrity
    HasAlacrity,
    AlacrityOver(u32),
    // Spike
    HasSpike,
    // First strike follow-up window
    HasFirstStrike,
    FirstStrikeExpiring,
}

impl TargetPredicate for SentinelPredicate {
    fn check(
        &self,
        aet_target: &AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        if let Some(target) = aet_target.get_target(model, controller) {
            match self {
                // ── Beast predicates ─────────────────────────────────────
                SentinelPredicate::HasBeast(beast) => target
                    .check_if_sentinel(&|s| s.has_beast(*beast))
                    .unwrap_or(false),
                SentinelPredicate::BeastCountOver(count) => target
                    .check_if_sentinel(&|s| s.beast_count() >= *count)
                    .unwrap_or(false),

                // ── Resin predicates ─────────────────────────────────────
                SentinelPredicate::HasColdResin => target.resin_state.cold.is_some(),
                SentinelPredicate::HasHotResin => target.resin_state.hot.is_some(),
                SentinelPredicate::ColdResinIs(resin) => {
                    target.resin_state.cold.as_ref() == Some(resin)
                }
                SentinelPredicate::HotResinIs(resin) => {
                    target.resin_state.hot.as_ref() == Some(resin)
                }
                SentinelPredicate::IsBurning => target.resin_state.burning.is_active(),
                SentinelPredicate::ResinTicksOver(n) => target.resin_state.ticks_left >= *n,

                // ── Alacrity ─────────────────────────────────────────────
                SentinelPredicate::HasAlacrity => target
                    .check_if_sentinel(&|s| s.alacrity > 0)
                    .unwrap_or(false),
                SentinelPredicate::AlacrityOver(n) => target
                    .check_if_sentinel(&|s| s.alacrity >= *n)
                    .unwrap_or(false),

                // ── Spike ────────────────────────────────────────────────
                SentinelPredicate::HasSpike => target
                    .check_if_sentinel(&|s| s.spike.is_some())
                    .unwrap_or(false),

                // ── First strike window ─────────────────────────────────
                SentinelPredicate::HasFirstStrike => {
                    let result = target.check_if_sentinel(&|s| {
                        let active = s.has_first_strike(false) || s.has_first_strike(true);
                        active
                    });
                    result.unwrap_or(false)
                }
                SentinelPredicate::FirstStrikeExpiring => target
                    .check_if_sentinel(&|s| {
                        s.first_strike_timer.is_active()
                            && s.first_strike_timer.get_time_left() <= 20
                    })
                    .unwrap_or(false),
            }
        } else {
            false
        }
    }
}
