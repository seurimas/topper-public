use crate::models::PassiveDef;

const ASCENDRIL_PASSIVES: &[PassiveDef] = &[
    PassiveDef {
        id: "ascendril_attunement",
        label: "Ascendril Attunement",
        category: "Passive",
        skill: "AscendrilAttunementPulse",
        targeted: false,
        period_ms: 50,
    },
    PassiveDef {
        id: "ascendril_flux",
        label: "Ascendril Flux",
        category: "Passive",
        skill: "AscendrilFluxPulse",
        targeted: true,
        period_ms: 100,
    },
];

pub(crate) fn passives_for_class(class_name: &str) -> &'static [PassiveDef] {
    match class_name {
        "Ascendril" => ASCENDRIL_PASSIVES,
        _ => ASCENDRIL_PASSIVES,
    }
}
