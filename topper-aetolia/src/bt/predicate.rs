use behavior_bark::unpowered::*;
use serde::Deserialize;
use serde::Serialize;

use crate::classes::Class;
use crate::classes::LockType;
use crate::classes::VenomPlan;
use crate::classes::ascendril::AscendrilPredicate;
use crate::classes::bard::BardPredicate;
use crate::classes::get_affs_from_plan;
use crate::classes::infiltrator::InfiltratorPredicate;
use crate::classes::is_affected_by;
use crate::classes::predator::PredatorPredicate;
use crate::classes::sentinel::SentinelPredicate;
use crate::classes::siderealist::SiderealistPredicate;
use crate::classes::zealot::ZealotPredicate;
use crate::curatives::MENTAL_AFFLICTIONS;
use crate::curatives::RANDOM_CURES;
use crate::curatives::get_cure_depth;
use crate::defense::DEFENSE_DATABASE;
use crate::defense::get_preferred_parry;
use crate::non_agent::AetTimelineRoomExt;
use crate::timeline::*;
use crate::types::*;
use crate::with_defense_db;

use super::BehaviorController;
use super::BehaviorModel;
use super::LimbDescriptor;

pub const QUEUE_TIME: f32 = 0.15;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum AetTarget {
    Me,
    Target,
}

impl AetTarget {
    pub fn get_target<'a>(
        &self,
        model: &'a BehaviorModel,
        controller: &BehaviorController,
    ) -> Option<&'a AgentState> {
        match self {
            AetTarget::Me => model
                .state
                .get_agent(&model.who_am_i())
                .and_then(|branches| branches.get(0)),
            AetTarget::Target => controller
                .target
                .as_ref()
                .and_then(|target| model.state.get_agent(&target))
                .and_then(|branches| branches.get(0))
                .or(Some(&model.default_agent)),
        }
    }

    pub fn get_name<'a>(
        &self,
        model: &'a BehaviorModel,
        controller: &BehaviorController,
    ) -> String {
        match self {
            AetTarget::Me => model.who_am_i(),
            AetTarget::Target => controller
                .target
                .clone()
                .unwrap_or_else(|| "enemy".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AetPredicate {
    NonceModEqual {
        modulus: i32,
        remainder: i32,
    },
    Persuading(AetTarget),
    // Affs
    AllAffs(AetTarget, Vec<FType>),
    SomeAffs(AetTarget, Vec<FType>),
    NoAffs(AetTarget, Vec<FType>),
    AffCountEqual(AetTarget, usize, Vec<FType>),
    AffCountOver(AetTarget, usize, Vec<FType>),
    AffCountUnder(AetTarget, usize, Vec<FType>),
    RandomCuresOver(AetTarget, usize),
    MentalAffsOver(AetTarget, usize),
    AffStacksOver(AetTarget, usize, FType),
    IsProne(AetTarget),
    // Limbs
    LimbsEqual(AetTarget, LimbDescriptor, LimbDescriptor),
    IsRestoringAny(AetTarget),
    IsRestoring(AetTarget, LimbDescriptor),
    IsOverRestoring(AetTarget, LimbDescriptor),
    CanBreak(AetTarget, LimbDescriptor, f32),
    RestoredBreak(AetTarget, LimbDescriptor, f32),
    CanMangled(AetTarget, LimbDescriptor, f32),
    RestoredMangle(AetTarget, LimbDescriptor, f32),
    LimbOver(AetTarget, LimbDescriptor, f32, bool),
    LimbsOver(AetTarget, Vec<LimbDescriptor>, f32, bool),
    CanMend(AetTarget, LimbDescriptor),
    AtLeastNLimbsOver(AetTarget, Vec<LimbDescriptor>, usize, f32, bool),
    LimbsBreakableCount {
        target: AetTarget,
        #[serde(default)]
        head_damage: Option<f32>,
        #[serde(default)]
        torso_damage: Option<f32>,
        #[serde(default)]
        left_arm_damage: Option<f32>,
        #[serde(default)]
        right_arm_damage: Option<f32>,
        #[serde(default)]
        left_leg_damage: Option<f32>,
        #[serde(default)]
        right_leg_damage: Option<f32>,
        min_count: usize,
        #[serde(default)]
        assume_restoration: bool,
    },
    // Priorities
    PriorityAffIs(AetTarget, FType),
    // Buffer/locks
    CannotCure(AetTarget, FType),
    Buffered(AetTarget, FType),
    Locked(AetTarget, bool),
    NearLocked(AetTarget, LockType, usize),
    PipeEmpty(AetTarget, String),
    // Timing
    ReboundingWindow(AetTarget, CType),
    SalveBlocked(AetTarget, CType),
    Channeling(AetTarget, Option<ChannelType>),
    ChannelStoppedBy(AetTarget, FType),
    // Hints
    LimbHintIs(String, LType),
    HintSet(String, String),
    // Stats
    StatUnderPercent(SType, AetTarget, f32),
    // Balances
    HasBalanceEquilibrium(AetTarget),
    HasBalance(AetTarget),
    HasEquilibrium(AetTarget),
    HasHandBalance(AetTarget),
    BalanceUnder(AetTarget, BType, f32),
    BalanceOver(AetTarget, BType, f32),
    HasTree(AetTarget, f32),
    HasFocus(AetTarget, f32),
    HasFitness(AetTarget, f32),
    HasClassCure(AetTarget, f32),
    CanDodge(AetTarget),
    // Elevation
    IsGrounded(AetTarget),
    IsFlying(AetTarget),
    IsClimbing(AetTarget),
    // Room tags
    RoomIsTagged(String),
    // Parries
    KnownParry(AetTarget, LimbDescriptor),
    ExpectedParry(AetTarget, LimbDescriptor),
    CanParry(AetTarget),
    // Class-specific
    IsAffectedBy(AetTarget, FType),
    ClassIn(AetTarget, Vec<Class>),
    // Class predicates
    BardPredicate(AetTarget, BardPredicate),
    PredatorPredicate(AetTarget, PredatorPredicate),
    InfiltratorPredicate(AetTarget, InfiltratorPredicate),
    AscendrilPredicate(AetTarget, AscendrilPredicate),
    SentinelPredicate(AetTarget, SentinelPredicate),
    SiderealistPredicate(AetTarget, SiderealistPredicate),
    ZealotPredicate(AetTarget, ZealotPredicate),
}

pub trait TargetPredicate {
    fn check(
        &self,
        target: &AetTarget,
        world: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool;
}

fn all_affs(
    target: &AetTarget,
    world: &BehaviorModel,
    controller: &BehaviorController,
    affs: &Vec<FType>,
) -> bool {
    if let Some(target) = target.get_target(world, controller) {
        for aff in affs {
            if !target.is(*aff) {
                return false;
            }
        }
        return true;
    }
    return false;
}

fn some_affs(
    target: &AetTarget,
    world: &BehaviorModel,
    controller: &BehaviorController,
    affs: &Vec<FType>,
) -> bool {
    if let Some(target) = target.get_target(world, controller) {
        for aff in affs {
            if target.is(*aff) {
                return true;
            }
        }
        return false;
    }
    return false;
}

fn no_affs(
    target: &AetTarget,
    world: &BehaviorModel,
    controller: &BehaviorController,
    affs: &Vec<FType>,
) -> bool {
    if let Some(target) = target.get_target(world, controller) {
        for aff in affs {
            if target.is(*aff) {
                return false;
            }
        }
        return true;
    }
    return true;
}

fn aff_counts(
    target: &AetTarget,
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    affs: &Vec<FType>,
) -> Option<usize> {
    target.get_target(model, controller).map(|target| {
        if affs.len() > 0 {
            target.affs_count(affs)
        } else {
            target.aff_count()
        }
    })
}

pub fn get_priority_aff(
    target: &AetTarget,
    model: &BehaviorModel,
    controller: &BehaviorController,
    stack: Option<Vec<VenomPlan>>,
) -> Option<FType> {
    if let (Some(target), Some(stack)) = (target.get_target(model, controller), stack) {
        get_affs_from_plan(&stack, 1, target, &vec![])
            .get(0)
            .cloned()
    } else {
        None
    }
}

impl UnpoweredFunction for AetPredicate {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            AetPredicate::NonceModEqual { modulus, remainder } => {
                if controller.nonce % *modulus == *remainder {
                    return UnpoweredFunctionState::Complete;
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::Persuading(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.persuasion_state.get_target().is_some() {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::AllAffs(target, affs) => {
                if all_affs(target, model, controller, affs) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::SomeAffs(target, affs) => {
                if some_affs(target, model, controller, affs) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::NoAffs(target, affs) => {
                if no_affs(target, model, controller, affs) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::AffCountEqual(target, count, affs) => {
                if let Some(aff_count) = aff_counts(target, model, controller, affs) {
                    if aff_count == *count {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::AffCountOver(target, min_count, affs) => {
                if let Some(aff_count) = aff_counts(target, model, controller, affs) {
                    if aff_count >= *min_count {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::AffCountUnder(target, min_count, affs) => {
                if let Some(aff_count) = aff_counts(target, model, controller, affs) {
                    if aff_count <= *min_count {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::RandomCuresOver(target, min_cures) => {
                if let Some(aff_count) =
                    aff_counts(target, model, controller, RANDOM_CURES.as_ref())
                {
                    if aff_count >= *min_cures {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::MentalAffsOver(target, min_count) => {
                if let Some(aff_count) =
                    aff_counts(target, model, controller, MENTAL_AFFLICTIONS.as_ref())
                {
                    if aff_count >= *min_count {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::AffStacksOver(target, min_stacks, aff) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_count(*aff) >= *min_stacks as u8 {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::IsProne(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.is_prone() {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::LimbsEqual(target, limb_a, limb_b) => {
                if let Some(limb_a) = limb_a.get_limb(model, controller, target) {
                    if let Some(limb_b) = limb_b.get_limb(model, controller, target) {
                        return if limb_a == limb_b {
                            UnpoweredFunctionState::Complete
                        } else {
                            UnpoweredFunctionState::Failed
                        };
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::IsRestoringAny(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_restoring().is_some() {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::IsRestoring(target, limb_descriptor) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        if target.get_limb_state(limb).is_restoring {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::IsOverRestoring(target, limb_descriptor) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        let limb_state = target.get_limb_state(limb);
                        if limb_state.is_restoring && limb_state.damage <= 33. {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CanBreak(target, limb_descriptor, damage) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        if target.get_limb_state(limb).hits_to_break(*damage) == 1 {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::RestoredBreak(target, limb_descriptor, damage) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        let mut limb_state = target.get_limb_state(limb);
                        if limb_state.is_restoring {
                            limb_state.assume_restore();
                        }
                        if limb_state.hits_to_break(*damage) == 1 {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CanMangled(target, limb_descriptor, damage) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        if target.get_limb_state(limb).hits_to_mangle(*damage) == 1 {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::RestoredMangle(target, limb_descriptor, damage) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        let mut limb_state = target.get_limb_state(limb);
                        if limb_state.is_restoring {
                            limb_state.assume_restore();
                        }
                        if limb_state.hits_to_mangle(*damage) == 1 {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::LimbOver(target, limb_descriptor, damage, apply_restoration) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        let mut limb_state = target.get_limb_state(limb);
                        if limb_state.is_restoring && *apply_restoration {
                            limb_state.assume_restore();
                        }
                        if limb_state.damage > *damage {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::LimbsOver(target, limbs, damage, apply_restoration) => {
                let limbs = if limbs.len() == 0 {
                    vec![
                        LimbDescriptor::Static(LType::LeftArmDamage),
                        LimbDescriptor::Static(LType::RightArmDamage),
                        LimbDescriptor::Static(LType::LeftLegDamage),
                        LimbDescriptor::Static(LType::RightLegDamage),
                        LimbDescriptor::Static(LType::HeadDamage),
                        LimbDescriptor::Static(LType::TorsoDamage),
                    ]
                } else {
                    limbs.clone()
                };
                let mut total_damage = 0.0;
                if let Some(target_state) = target.get_target(model, controller) {
                    for limb_descriptor in limbs {
                        if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                            let mut limb_state = target_state.get_limb_state(limb);
                            if limb_state.is_restoring && *apply_restoration {
                                limb_state.assume_restore();
                            }
                            total_damage += limb_state.damage;
                        }
                    }
                }
                if total_damage > *damage {
                    return UnpoweredFunctionState::Complete;
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CanMend(target, limb_descriptor) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        if target.get_limb_state(limb).crippled
                            && !target.get_limb_state(limb).broken
                        {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::AtLeastNLimbsOver(
                target,
                limbs,
                min_count,
                damage,
                apply_restoration,
            ) => {
                let limbs = if limbs.len() == 0 {
                    vec![
                        LimbDescriptor::Static(LType::LeftArmDamage),
                        LimbDescriptor::Static(LType::RightArmDamage),
                        LimbDescriptor::Static(LType::LeftLegDamage),
                        LimbDescriptor::Static(LType::RightLegDamage),
                    ]
                } else {
                    limbs.clone()
                };
                if let Some(target_state) = target.get_target(model, controller) {
                    let mut count = 0;
                    for limb_descriptor in limbs {
                        if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                            let mut limb_state = target_state.get_limb_state(limb);
                            if limb_state.is_restoring && *apply_restoration {
                                limb_state.assume_restore();
                            }
                            if limb_state.damage > *damage {
                                count += 1;
                            }
                        }
                    }
                    if count >= *min_count {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::LimbsBreakableCount {
                target,
                head_damage,
                torso_damage,
                left_arm_damage,
                right_arm_damage,
                left_leg_damage,
                right_leg_damage,
                min_count,
                assume_restoration,
            } => {
                let Some(target_state) = target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                let mut count = 0;
                if let Some(damage) = *head_damage {
                    let mut limb_state = target_state.get_limb_state(LType::HeadDamage);
                    if limb_state.is_restoring && *assume_restoration {
                        limb_state.assume_restore();
                    }
                    if limb_state.hits_to_break(damage) == 1 {
                        count += 1;
                    }
                }
                if let Some(damage) = *torso_damage {
                    let mut limb_state = target_state.get_limb_state(LType::TorsoDamage);
                    if limb_state.is_restoring && *assume_restoration {
                        limb_state.assume_restore();
                    }
                    if limb_state.hits_to_break(damage) == 1 {
                        count += 1;
                    }
                }
                if let Some(damage) = *left_arm_damage {
                    let mut limb_state = target_state.get_limb_state(LType::LeftArmDamage);
                    if limb_state.is_restoring && *assume_restoration {
                        limb_state.assume_restore();
                    }
                    if limb_state.hits_to_break(damage) == 1 {
                        count += 1;
                    }
                }
                if let Some(damage) = *right_arm_damage {
                    let mut limb_state = target_state.get_limb_state(LType::RightArmDamage);
                    if limb_state.is_restoring && *assume_restoration {
                        limb_state.assume_restore();
                    }
                    if limb_state.hits_to_break(damage) == 1 {
                        count += 1;
                    }
                }
                if let Some(damage) = *left_leg_damage {
                    let mut limb_state = target_state.get_limb_state(LType::LeftLegDamage);
                    if limb_state.is_restoring && *assume_restoration {
                        limb_state.assume_restore();
                    }
                    if limb_state.hits_to_break(damage) == 1 {
                        count += 1;
                    }
                }
                if let Some(damage) = *right_leg_damage {
                    let mut limb_state = target_state.get_limb_state(LType::RightLegDamage);
                    if limb_state.is_restoring && *assume_restoration {
                        limb_state.assume_restore();
                    }
                    if limb_state.hits_to_break(damage) == 1 {
                        count += 1;
                    }
                }
                if count >= *min_count {
                    return UnpoweredFunctionState::Complete;
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::Locked(target, hard_only) => {
                if let Some(target) = target.get_target(model, controller) {
                    if let Some(lock) = target.lock_duration() {
                        if !*hard_only || lock >= 10.0 {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::NearLocked(target, lock_type, aff_count) => {
                if let Some(target) = target.get_target(model, controller) {
                    let affs_to_lock = lock_type.affs_to_lock(target);
                    if affs_to_lock <= *aff_count {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::PipeEmpty(target, pipe) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.pipe_state.get_empties().contains(&pipe) {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CannotCure(target, aff) => {
                if let Some(target) = target.get_target(model, controller) {
                    let mut afflicted = target.clone();
                    afflicted.set_flag(*aff, true);
                    let cure_depth = get_cure_depth(&afflicted, *aff);
                    let minimum_depth =
                        if let Some(me) = AetTarget::Me.get_target(model, controller) {
                            110 + (BALANCE_SCALE * me.get_qeb_balance()) as CType
                        } else {
                            110
                        };
                    if cure_depth.time > minimum_depth {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::PriorityAffIs(target, aff) => {
                if let Some(priority_aff) =
                    get_priority_aff(target, model, controller, controller.aff_priorities.clone())
                {
                    if priority_aff == *aff {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::Buffered(target, aff) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.is(*aff) && get_cure_depth(target, *aff).cures > 1 {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::ReboundingWindow(target, minimum) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Rebounding) > (*minimum as f32 / BALANCE_SCALE) {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::SalveBlocked(target, minimum) => {
                if let Some(target) = target.get_target(model, controller) {
                    if let Some(restore) = target.limb_damage.restore_timer {
                        if restore.get_time_left() > *minimum {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::Channeling(target, channel) => {
                if let Some(target) = target.get_target(model, controller) {
                    if channel.is_some() {
                        if target.channel_state.is_channeling(channel.unwrap()) {
                            return UnpoweredFunctionState::Complete;
                        }
                    } else {
                        if target.channel_state.is_channeling_any() {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::ChannelStoppedBy(target, aff) => {
                if let Some(target) = target.get_target(model, controller) {
                    if let Some(channel) = target.channel_state.get_channel_type() {
                        if channel.stopped_by(*aff) {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasBalance(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Balance) < QUEUE_TIME {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasEquilibrium(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Equil) < QUEUE_TIME {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasHandBalance(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::LeftHandBalance) < QUEUE_TIME
                        || target.get_balance(BType::RightHandBalance) < QUEUE_TIME
                    {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasFocus(target, buffer) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Focus) < QUEUE_TIME + *buffer
                        && target.can_focus(true)
                    {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::KnownParry(target, limb_descriptor) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        if target.get_parrying() == Some(limb) && target.can_parry() {
                            if model.state.borrow_me().is(FType::Wrath) {
                                return UnpoweredFunctionState::Complete;
                            } else if target.parry_known {
                                return UnpoweredFunctionState::Complete;
                            }
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::ExpectedParry(aet_target, limb_descriptor) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, aet_target) {
                    if let Some(target) = aet_target.get_target(model, controller) {
                        if !target.can_parry() {
                            return UnpoweredFunctionState::Failed;
                        } else if model.state.borrow_me().is(FType::Wrath) || target.parry_known {
                            // If we are wrath, we can know the parry directly. We can also know if the target is off balance.
                            if target.get_parrying() == Some(limb) {
                                return UnpoweredFunctionState::Complete;
                            } else {
                                return UnpoweredFunctionState::Failed;
                            }
                        }
                        if target.parrying == Some(limb) {
                            // Maybe controversial, but let's avoid whatever they were last parrying.
                            return UnpoweredFunctionState::Complete;
                        }
                        if controller.get_expected_parry(model).is_some() {
                            if controller.get_expected_parry(model) == Some(limb) {
                                return UnpoweredFunctionState::Complete;
                            } else {
                                return UnpoweredFunctionState::Failed;
                            }
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CanParry(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.can_parry() {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::BalanceUnder(target, balance, maximum) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(*balance) <= *maximum {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::BalanceOver(target, balance, minimum) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(*balance) > *minimum {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasTree(target, buffer) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Tree) < QUEUE_TIME + *buffer
                        && target.can_tree(true)
                    {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasFitness(target, buffer) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Fitness) < QUEUE_TIME + *buffer {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasClassCure(target, buffer) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::ClassCure1) < QUEUE_TIME + *buffer {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CanDodge(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.dodge_state.can_dodge() {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasBalanceEquilibrium(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Balance) < QUEUE_TIME
                        && target.get_balance(BType::Equil) < QUEUE_TIME
                    {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::BardPredicate(target, bard_predicate) => {
                if bard_predicate.check(target, model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::PredatorPredicate(target, predator_predicate) => {
                if predator_predicate.check(target, model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::InfiltratorPredicate(target, infiltrator_predicate) => {
                if infiltrator_predicate.check(target, model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::AscendrilPredicate(target, ascendril_predicate) => {
                if ascendril_predicate.check(target, model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::SentinelPredicate(target, sentinel_predicate) => {
                if sentinel_predicate.check(target, model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::SiderealistPredicate(target, siderealist_predicate) => {
                if siderealist_predicate.check(target, model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::ZealotPredicate(target, zealot_predicate) => {
                if zealot_predicate.check(target, model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::LimbHintIs(hint, limb) => {
                if let Some(hint) = controller.get_hint(hint) {
                    if hint.eq_ignore_ascii_case(&limb.to_string()) {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::HintSet(hint, value) => {
                if let Some(hint) = controller.get_hint(hint) {
                    if hint.eq_ignore_ascii_case(value) {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::IsGrounded(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.elevation == Elevation::Ground {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::IsFlying(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.elevation == Elevation::Flying {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::IsClimbing(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.elevation == Elevation::Trees || target.elevation == Elevation::Roof {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::RoomIsTagged(tag) => {
                if let Some(room) = model.state.get_my_room() {
                    if room.has_tag(tag) {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::StatUnderPercent(stat, target, percent) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_stat_percent(*stat) < *percent {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::IsAffectedBy(target, aff) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if let Some(class) =
                        target_agent.class_state.get_normalized_class().or_else(|| {
                            with_defense_db!(db, {
                                return db.get_class(&target.get_name(model, controller));
                            });
                            None
                        })
                    {
                        if is_affected_by(class, *aff) {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::ClassIn(target, classes) => {
                if let Some(target) = target.get_target(model, controller) {
                    if let Some(class) = target.class_state.get_normalized_class() {
                        if classes.contains(&class) {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Nothing to do
    }
}
