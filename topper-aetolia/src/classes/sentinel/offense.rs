#[macro_use(affliction_stacker, affliction_plan_stacker)]
use crate::{affliction_stacker, affliction_plan_stacker};
use super::actions::*;
use super::constants::*;
use crate::alpha_beta::ActionPlanner;
use crate::classes::*;
use crate::defense::*;
use crate::timeline::*;
use crate::types::*;

fn get_stack<'s>(
    timeline: &AetTimeline,
    you: &AgentState,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Vec<VenomPlan> {
    let mut vec = db
        .and_then(|db| db.get_venom_plan(&format!("sentinel_{}", strategy)))
        .unwrap_or(get_simple_plan(DEFAULT_STACK.to_vec()));
    vec.retain(move |aff| match aff.affliction() {
        FType::Impatience
        | FType::Epilepsy
        | FType::Laxity
        | FType::Arrhythmia
        | FType::Impairment => !you.can_parry(),
        _ => true,
    });
    vec
}

fn assume_hit(who: &mut AgentState, strike: &FirstStrike) {
    if let Some(affs) = FIRST_STRIKE_AFFS.get(strike) {
        for aff in affs {
            who.set_flag(*aff, true);
        }
    }
}

/**
 * Planning
 **/

lazy_static! {
    static ref DEFAULT_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Impatience,
        FType::Epilepsy,
        FType::Asthma,
        FType::Clumsiness,
        FType::Slickness,
        FType::Anorexia,
        FType::Stupidity,
        FType::Confusion,
        FType::Arrhythmia,
        FType::LeftLegCrippled,
        FType::RightLegCrippled,
        FType::Vomiting,
        FType::Impairment,
        FType::LeftArmCrippled,
        FType::RightArmCrippled,
        FType::Dizziness,
        FType::Epilepsy,
        FType::RingingEars,
        FType::Recklessness,
    ];
}

fn want_fitness(me: &AgentState) -> bool {
    me.balanced(BType::Fitness)
        && me.is(FType::Asthma)
        && (me.is(FType::Hellsight) || me.is(FType::Slickness))
}

fn want_might(me: &AgentState) -> bool {
    me.balanced(BType::ClassCure1)
        && me.affs_count(&vec![FType::Anorexia, FType::Asthma, FType::Slickness]) >= 2
}

fn want_spinecut(you: &AgentState) -> bool {
    you.affs_count(&vec![
        FType::LeftLegCrippled,
        FType::RightLegCrippled,
        FType::Confusion,
        FType::Arrhythmia,
    ]) >= 4
}

fn want_pierce(you: &AgentState) -> Option<String> {
    if you.can_parry()
        || you.is(FType::Rebounding)
        || you.is(FType::Shielded)
        || you.is(FType::Confusion)
    {
        return None;
    } else if you.limb_damage.crippled(LType::LeftLegDamage)
        && !you.limb_damage.broken(LType::LeftLegDamage)
        && you.limb_damage.restoring != Some(LType::LeftLegDamage)
    {
        return Some("left".to_string());
    } else if you.limb_damage.crippled(LType::RightLegDamage)
        && !you.limb_damage.broken(LType::RightLegDamage)
        && you.limb_damage.restoring != Some(LType::RightLegDamage)
    {
        return Some("right".to_string());
    } else {
        return None;
    }
}

fn want_sever(you: &AgentState) -> Option<String> {
    if you.can_parry()
        || you.is(FType::Rebounding)
        || you.is(FType::Shielded)
        || you.is(FType::Confusion)
    {
        return None;
    } else if you.limb_damage.crippled(LType::LeftArmDamage)
        && !you.limb_damage.broken(LType::LeftArmDamage)
        && you.limb_damage.restoring != Some(LType::LeftArmDamage)
    {
        return Some("left".to_string());
    } else if you.limb_damage.crippled(LType::RightArmDamage)
        && !you.limb_damage.broken(LType::RightArmDamage)
        && you.limb_damage.restoring != Some(LType::RightArmDamage)
    {
        return Some("right".to_string());
    } else {
        return None;
    }
}

// pub fn get_balance_attack<'s>(
//     timeline: &AetTimeline,
//     who_am_i: &String,
//     target: &String,
//     strategy: &String,
//     db: Option<&impl AetDatabaseModule>,
// ) -> Box<dyn ActiveTransition> {
//     if strategy.eq("damage") {
//         return Box::new(Inactivity);
//     } else {
//         let me = timeline.state.borrow_agent(who_am_i);
//         let mut you = timeline.state.borrow_agent(target);
//         let mut stack = get_stack(timeline, &you, strategy, db);
//         if want_spinecut(&you) {
//             return Box::new(SpinecutAction::new(who_am_i.to_string(), target.clone()));
//         } else if want_fitness(&me) {
//             return Box::new(FitnessAction::new(who_am_i.to_string()));
//         } else if want_might(&me) {
//             return Box::new(MightAction::new(who_am_i.to_string()));
//         } else if you.is(FType::Shielded) && you.is(FType::Rebounding) {
//             return Box::new(DualrazeAction::new(who_am_i.to_string(), target.clone()));
//         } else if let Some(side) = want_pierce(&you) {
//             match side.as_str() {
//                 "left" => {
//                     return Box::new(PierceActionLeft::new(who_am_i.to_string(), target.clone()));
//                 }
//                 "right" => {
//                     return Box::new(PierceActionRight::new(who_am_i.to_string(), target.clone()));
//                 }
//                 _ => (),
//             }
//         } else if let Some(side) = want_sever(&you) {
//             match side.as_str() {
//                 "left" => {
//                     return Box::new(SeverActionLeft::new(who_am_i.to_string(), target.clone()));
//                 }
//                 "right" => {
//                     return Box::new(SeverActionRight::new(who_am_i.to_string(), target.clone()));
//                 }
//                 _ => (),
//             }
//         } else {
//             let first_strike = get_first_strike_from_plan(
//                 &stack,
//                 1,
//                 &you,
//                 &vec![
//                     FType::LeftLegCrippled,
//                     FType::RightLegCrippled,
//                     FType::LeftArmCrippled,
//                     FType::RightArmCrippled,
//                 ],
//             )
//             .pop();
//             if let Some(mut first_strike) = first_strike {
//                 if you.is(FType::Rebounding) && !first_strike.ignores_rebounding() {
//                     first_strike = FirstStrike::Reave;
//                 }
//                 assume_hit(&mut you, &first_strike);
//                 stack = get_stack(timeline, &you, strategy, db);
//                 let second_strike = if first_strike.flourish() {
//                     get_venoms_from_plan(
//                         &stack,
//                         1,
//                         &you,
//                         &vec![
//                             FType::LeftLegCrippled,
//                             FType::RightLegCrippled,
//                             FType::LeftArmCrippled,
//                             FType::RightArmCrippled,
//                         ],
//                     )
//                     .pop()
//                     .map(|venom| SecondStrike::Flourish(venom))
//                 } else {
//                     get_second_strike_from_plan(
//                         &stack,
//                         1,
//                         &you,
//                         &vec![
//                             FType::LeftLegCrippled,
//                             FType::RightLegCrippled,
//                             FType::LeftArmCrippled,
//                             FType::RightArmCrippled,
//                         ],
//                     )
//                     .pop()
//                 };
//                 if let Some(second_strike) = second_strike {
//                     return Box::new(ComboAction::new(
//                         who_am_i.to_string(),
//                         target.clone(),
//                         first_strike,
//                         second_strike,
//                     ));
//                 }
//             }
//         }
//         return Box::new(Inactivity);
//     }
// }

// pub fn get_action_plan(
//     timeline: &AetTimeline,
//     me: &String,
//     target: &String,
//     strategy: &String,
//     db: Option<&impl AetDatabaseModule>,
// ) -> ActionPlan {
//     let mut action_plan = ActionPlan::new(me);
//     let mut balance = get_balance_attack(timeline, me, target, strategy, db);
//     if let Some(parry) = get_needed_parry(timeline, me, target, db) {
//         balance = Box::new(SeparatorAction::pair(
//             Box::new(ParryAction::by_class(
//                 me.to_string(),
//                 parry,
//                 Class::Sentinel,
//             )),
//             balance,
//         ));
//     }
//     if let Ok(_activation) = balance.act(&timeline) {
//         action_plan.add_to_qeb(balance);
//     }
//     action_plan
// }

// pub fn get_attack(
//     timeline: &AetTimeline,
//     target: &String,
//     strategy: &String,
//     db: Option<&impl AetDatabaseModule>,
// ) -> String {
//     let action_plan = get_action_plan(&timeline, &timeline.who_am_i(), &target, &strategy, db);
//     action_plan.get_inputs(&timeline)
// }
