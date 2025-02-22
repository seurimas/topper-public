use std::collections::HashMap;

use crate::{
    AppealType, Appeals, PersuasionAff, PersuasionState, PersuasionStatus, PERSUASION_AFFS,
};

#[derive(Debug, Clone, Copy)]
enum EvaluatorResult {
    Add(i32),
    Zero,
}
type EvaluatorAccumulator = i32;
type Evaluator<'a> = Box<dyn Fn(&PersuasionState, &PersuasionStatus) -> EvaluatorResult + 'a>;
type EvalPair<'a> = (Appeals, Evaluator<'a>);

fn apply_result(result: EvaluatorResult, acc: EvaluatorAccumulator) -> EvaluatorAccumulator {
    match result {
        EvaluatorResult::Add(x) => acc + x,
        EvaluatorResult::Zero => acc,
    }
}

pub fn simple_strategy(
    my_state: &PersuasionState,
    target_state: &PersuasionStatus,
) -> Result<String, String> {
    if my_state.appeals_in_hand.is_empty() {
        return Err("No appeals in hand".to_string());
    }
    let evaluators: Vec<EvalPair> = vec![
        // Basic evals...
        (
            Appeals::Authority,
            Box::new(move |me: &PersuasionState, denizen: &PersuasionStatus| {
                if denizen.resolve() < 2000 {
                    return EvaluatorResult::Zero;
                }
                if !me.is(PersuasionAff::Gravitas) && !me.would_unrhetoric(Appeals::Authority) {
                    return EvaluatorResult::Add(5);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        (
            Appeals::Morality,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if denizen.resolve() < 2000 {
                    return EvaluatorResult::Zero;
                }
                if me.reasoned() {
                    return EvaluatorResult::Add(2);
                } else {
                    return EvaluatorResult::Add(1);
                }
            }),
        ),
        (
            Appeals::Reputation,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.influence_stacks > 4 {
                    return EvaluatorResult::Zero;
                }
                let assumed_value = denizen.resolve() / 21;
                let assumed_value = if me.reasoned() {
                    assumed_value * 2
                } else {
                    assumed_value
                };
                return EvaluatorResult::Add(assumed_value / 300);
            }),
        ),
        (
            Appeals::Tradition,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.has_pathos_bonus() || me.analogizing() || me.is(PersuasionAff::Revelation) {
                    return EvaluatorResult::Add(2);
                }
                let mut aff_count = 0;
                for aff in PERSUASION_AFFS.iter() {
                    if me.is(*aff) {
                        aff_count += 1;
                    }
                }
                return EvaluatorResult::Add(aff_count);
            }),
        ),
        (
            Appeals::Reason,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.reasoned() {
                    return EvaluatorResult::Add(-5);
                } else if me
                    .appeals_in_hand
                    .iter()
                    .any(|appeal| appeal == &Appeals::Inspiration || appeal == &Appeals::Analogy)
                {
                    return EvaluatorResult::Add(2);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        (
            Appeals::Evidence,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.reasoned()
                    && me
                        .appeals_in_hand
                        .iter()
                        .any(|appeal| denizen.personality().is_weak_to(*appeal))
                {
                    return EvaluatorResult::Add(2);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        (
            Appeals::Analogy,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.rhetoric_just_started() {
                    return EvaluatorResult::Add(-2);
                } else if me.would_finish_rhetoric(Appeals::Analogy) {
                    return EvaluatorResult::Add(2);
                } else if me.in_rhetoric() {
                    return EvaluatorResult::Add(-2);
                }
                if me.reasoned()
                    && me
                        .appeals_in_hand
                        .iter()
                        .any(|appeal| appeal.appeal_type() != AppealType::Logos)
                {
                    return EvaluatorResult::Add(2);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        (
            Appeals::Causality,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.has_pathos_bonus() {
                    return EvaluatorResult::Add(2);
                }
                if me.reasoned() {
                    return EvaluatorResult::Add(3);
                } else if me.appeals_in_hand.iter().any(|appeal| {
                    appeal == &Appeals::Tradition
                        || (appeal == &Appeals::Provocation && me.acumen > 2000)
                }) {
                    return EvaluatorResult::Add(3);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        (
            Appeals::Intimidation,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.acumen < 2000 {
                    return EvaluatorResult::Zero;
                } else {
                    return EvaluatorResult::Add(1);
                }
            }),
        ),
        (
            Appeals::Reassurance,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.cyclic == Some(Appeals::Reassurance) && me.acumen < 4000 {
                    return EvaluatorResult::Add(5);
                } else if me.acumen < 2000 {
                    return EvaluatorResult::Add(3);
                } else if me.acumen < 3000 {
                    return EvaluatorResult::Add(2);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        (
            Appeals::Inspiration,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.reasoned() {
                    return EvaluatorResult::Add(3);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        (
            Appeals::Provocation,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if (me.has_pathos_bonus() || me.is(PersuasionAff::Revelation) || me.analogizing())
                    && me.acumen > 2000
                {
                    return EvaluatorResult::Add(2);
                }
                if denizen.max_resolve() > 11000 {
                    return EvaluatorResult::Add(-3);
                } else if me.acumen < 2000 {
                    return EvaluatorResult::Add(-2);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        // Affliction evals
        (
            Appeals::Provocation,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.is(PersuasionAff::Pressured) {
                    return EvaluatorResult::Add(-3);
                } else if me.is(PersuasionAff::Confounded) {
                    return EvaluatorResult::Add(-1);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        (
            Appeals::Tradition,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.is(PersuasionAff::Pressured) && !me.is(PersuasionAff::Conviction) {
                    return EvaluatorResult::Add(-3);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        (
            Appeals::Causality,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.is(PersuasionAff::Pressured) && !me.is(PersuasionAff::Conviction) {
                    return EvaluatorResult::Add(-3);
                } else if me.is(PersuasionAff::Confounded) {
                    return EvaluatorResult::Add(-1);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
        (
            Appeals::Authority,
            Box::new(|me: &PersuasionState, denizen: &PersuasionStatus| {
                if me.is(PersuasionAff::Pressured) && !me.is(PersuasionAff::Conviction) {
                    return EvaluatorResult::Add(-2);
                } else {
                    return EvaluatorResult::Zero;
                }
            }),
        ),
    ];
    if my_state.cyclic.is_none() || my_state.cyclic_in_discard() {
        if let Some(new_cyclic) = my_state.first_not_in_discard(&[
            Appeals::Reassurance,
            Appeals::Morality,
            Appeals::Reason,
            Appeals::Authority,
        ]) {
            return Ok(format!("cyclic {}", new_cyclic.to_name()));
        }
    } else if my_state.cyclic != Some(Appeals::Reassurance)
        && my_state.acumen < 4000
        && my_state
            .first_not_in_discard(&[Appeals::Reassurance])
            .is_some()
    {
        return Ok("cyclic reassurance".to_string());
    } else if my_state.could_follow_any_rhetoric()
        && !my_state.in_rhetoric()
        && my_state.acumen > 3000
    {
        return Ok("rhetoric".to_string());
    }
    let mut values = HashMap::new();
    let mut best_dps = 0;
    let mut best_dps_appeal = my_state.appeals_in_hand[0];
    for appeal in my_state.appeals_in_hand.iter() {
        values.insert(appeal, 0);
        for (ev_appeal, evaluator) in evaluators.iter() {
            if appeal == ev_appeal {
                let result = evaluator(my_state, target_state);
                values.insert(
                    appeal,
                    apply_result(result, values.get(appeal).copied().unwrap_or(0)),
                );
            }
        }
        if my_state.would_conflict(*appeal) {
            values.insert(appeal, -2);
        }
        if my_state.would_entrench(*appeal) {
            values.insert(appeal, -2);
        }
        if my_state.rhetoric_just_started() && !my_state.could_follow_rhetoric(appeal.appeal_type())
        {
            values.insert(appeal, -2);
        } else if my_state.would_unrhetoric(*appeal) {
            values.insert(appeal, -2);
        } else if my_state.would_finish_rhetoric(*appeal) {
            values.insert(
                appeal,
                apply_result(
                    EvaluatorResult::Add(3),
                    values.get(appeal).copied().unwrap_or(0),
                ),
            );
        } else if my_state.would_follow_rhetoric(*appeal) {
            values.insert(
                appeal,
                apply_result(
                    EvaluatorResult::Add(2),
                    values.get(appeal).copied().unwrap_or(0),
                ),
            );
        }
        let guessed_damage = appeal.guess_resolve_damage(target_state.personality(), my_state);
        if guessed_damage > best_dps {
            best_dps = guessed_damage;
            best_dps_appeal = *appeal;
        }
    }
    if let Some(mut v) = values.get_mut(&best_dps_appeal) {
        *v += 1;
    }
    println!(
        "Values: {:?} - My Acumen: {} - Resolve: {} - Rhetoric: {:?}",
        values,
        my_state.acumen,
        target_state.resolve(),
        my_state.get_rhetoric_state()
    );
    values
        .iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, _)| format!("appeal to {}", k.to_name()))
        .ok_or("No appeals in hand".to_string())
}
