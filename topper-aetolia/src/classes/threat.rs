use serde::{Deserialize, Serialize};

use crate::classes::Class;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Display, PartialEq, Eq, Hash)]
pub enum ClassThreat {
    VitalsPressure, // Class does some damage. (e.g. Infiltrator classically does not have this)
    HighVitalsPressure, // Class does a lot of damage. (e.g. Carnifex, Indorani, Ascendril)
    Locks,
    FastLocks,
    PillsPressure,
    SalvePressure,
    HiddenAfflictions, // Their hidden afflictions can be very dangerous.
    Mentals,
    PhysicalAttacks,
    HighBleed,
    CrippleEscalations, // Can turn crippled limbs into more dangerous afflictions.
    // Instakills
    ComboInstaKill, // Has combos that can instantly kill
    ManaKill,
}

impl Class {
    pub fn get_threats(&self) -> Vec<ClassThreat> {
        match self {
            Class::Carnifex => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::HighVitalsPressure,
                ClassThreat::PillsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
                ClassThreat::Locks,
            ],
            Class::Indorani => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::HighVitalsPressure,
                ClassThreat::PillsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
                ClassThreat::Locks,
            ],
            Class::Praenomen => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::ManaKill,
                ClassThreat::PillsPressure,
                ClassThreat::Mentals,
                ClassThreat::Locks,
                ClassThreat::HiddenAfflictions,
            ],
            Class::Teradrim => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
            ],
            Class::Monk => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
            ],
            Class::Sentinel => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
                ClassThreat::Locks,
            ],
            Class::Shaman => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
                ClassThreat::Locks,
                ClassThreat::HiddenAfflictions,
            ],
            Class::Ascendril => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::HighVitalsPressure,
                ClassThreat::PillsPressure,
                ClassThreat::ManaKill,
                ClassThreat::SalvePressure,
            ],
            Class::Luminary => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::PillsPressure,
                ClassThreat::Mentals,
                ClassThreat::PhysicalAttacks,
                ClassThreat::Locks,
                ClassThreat::ManaKill,
            ],
            Class::Templar => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::PillsPressure,
                ClassThreat::Mentals,
                ClassThreat::PhysicalAttacks,
                ClassThreat::Locks,
                ClassThreat::FastLocks,
            ],
            Class::Zealot => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
                ClassThreat::ComboInstaKill,
            ],
            Class::Archivist => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
                ClassThreat::Locks,
                ClassThreat::ComboInstaKill,
                ClassThreat::HiddenAfflictions,
            ],
            Class::Sciomancer => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::PillsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::Mentals,
                ClassThreat::ComboInstaKill,
                ClassThreat::HiddenAfflictions,
            ],
            Class::Infiltrator => vec![
                ClassThreat::Locks,
                ClassThreat::FastLocks,
                ClassThreat::HiddenAfflictions,
            ],
            Class::Shapeshifter => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
                ClassThreat::Locks,
                ClassThreat::CrippleEscalations,
            ],
            Class::Wayfarer => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::PillsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
                ClassThreat::Locks,
                ClassThreat::FastLocks,
            ],
            Class::Bard => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::PillsPressure,
                ClassThreat::Mentals,
                ClassThreat::Locks,
                ClassThreat::ComboInstaKill,
            ],
            Class::Predator => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::PillsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::PhysicalAttacks,
                ClassThreat::Locks,
            ],
            Class::Siderealist => vec![
                ClassThreat::VitalsPressure,
                ClassThreat::PillsPressure,
                ClassThreat::SalvePressure,
                ClassThreat::Mentals,
                ClassThreat::ComboInstaKill,
            ],
            Class::Lord => vec![ClassThreat::VitalsPressure, ClassThreat::HiddenAfflictions],
            _ => {
                if *self != self.normal() {
                    self.normal().get_threats()
                } else {
                    vec![]
                }
            }
        }
    }

    pub fn threatens(&self, threat: ClassThreat) -> bool {
        self.get_threats().contains(&threat)
    }
}
