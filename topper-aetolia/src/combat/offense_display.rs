use crate::{timeline::*, types::*};

fn expect_heals_per_second(timeline: &AetTimeline) -> f32 {
    let me = timeline.state.borrow_me();
    let average_max_stat =
        (me.get_max_stat(SType::Health) as f32 + me.get_max_stat(SType::Mana) as f32) / 2.0;
    // Elixir heals 100 + 15% over a 4.75 second balance cycle.
    (100.0 + average_max_stat * 0.15) / 4.75
}

pub fn heal_pressure_row(timeline: &AetTimeline) -> String {
    let pressure_duration = (20.0 * BALANCE_SCALE) as CType;
    let heal_duration = (30.0 * BALANCE_SCALE) as CType;
    let avg_damage = timeline
        .state
        .average_stat_over_last(DAMAGED, pressure_duration)
        .unwrap_or(0.0);
    let avg_mana_damage = timeline
        .state
        .average_stat_over_last(MANA_DAMAGED, pressure_duration)
        .unwrap_or(0.0);
    let avg_total_damage = avg_damage + avg_mana_damage;

    let avg_heal = timeline
        .state
        .average_stat_over_last(HEALED, heal_duration)
        .unwrap_or(0.0);
    let avg_mana_heal = timeline
        .state
        .average_stat_over_last(MANA_HEALED, heal_duration)
        .unwrap_or(0.0);
    let expected_hps = expect_heals_per_second(timeline).max(avg_heal + avg_mana_heal);

    let heal_color = if avg_total_damage > expected_hps * 1.5 {
        "magenta"
    } else if avg_total_damage > expected_hps * 1.2 {
        "red"
    } else if avg_total_damage >= expected_hps * 0.8 {
        "yellow"
    } else {
        "green"
    };

    format!(
        "<{}>Stat Pressure: {:.1}/s (exp {:.1}/s)\n",
        heal_color, avg_total_damage, expected_hps
    )
}
