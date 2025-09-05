use std::convert::TryFrom;

use topper_core::{observations::strip_ansi, timeline::db::DatabaseModule};

use crate::{
    agent::Hypnosis,
    classes::{Class, VenomPlan},
    curatives::firstaid::FirstAidPriorities,
    timeline::{AetObservation, AetTimeSlice},
};

pub const HINT_TREE: &str = "HINTS";

pub trait AetDatabaseModule {
    fn get_class(&self, who: &String) -> Option<Class>;

    fn set_class(&self, who: &String, class: Class);

    fn get_venom_plan(&self, stack_name: &String) -> Option<Vec<VenomPlan>>;

    fn get_hypno_plan(&self, stack_name: &String) -> Option<Vec<Hypnosis>>;

    fn get_first_aid_priorities(
        &self,
        who: &String,
        priorities_name: &String,
    ) -> Option<FirstAidPriorities>;

    fn set_first_aid_priorities(
        &self,
        who: &String,
        priorities_name: &String,
        priorities: FirstAidPriorities,
    );

    fn insert_hint(&self, key: &String, value: &String);

    fn get_hint(&self, key: &String) -> Option<String>;

    fn set_defense_order(&self, class_who: &String, order: Vec<String>);
    fn get_defense_order(&self, class_who: &String) -> Option<Vec<String>>;

    fn add_defense_command(&self, defense: &String, command: &String);
    fn get_defense_command(&self, defense: &String) -> Option<String>;
    fn remove_defense_command(&self, defense: &String);

    fn add_defense_trigger(&self, defense: &String, trigger: &String);
    fn get_defense_trigger(&self, trigger: &String) -> Option<String>;
    fn remove_defense_trigger(&self, trigger: &String);

    fn add_defense_missing_trigger(&self, defense: &String, trigger: &String);
    fn get_defense_missing_trigger(&self, trigger: &String) -> Option<String>;
    fn remove_defense_missing_trigger(&self, trigger: &String);

    fn get_db_observation(&self, trigger: &String) -> Option<AetObservation> {
        if let Some(defense) = self.get_defense_trigger(&strip_ansi(trigger)) {
            Some(AetObservation::DefenseUp(defense))
        } else if let Some(defense) = self.get_defense_missing_trigger(&strip_ansi(trigger)) {
            Some(AetObservation::DefenseDown(defense))
        } else {
            None
        }
    }

    fn observe(&self, time_slice: &mut AetTimeSlice) {
        let Some(observations) = &mut time_slice.observations else {
            println!("No observations vector in time slice");
            return;
        };
        for (line, _) in &time_slice.lines {
            if let Some(obs) = self.get_db_observation(line) {
                observations.push(obs);
            }
        }
    }
}

impl<T: DatabaseModule> AetDatabaseModule for T {
    fn get_class(&self, who: &String) -> Option<Class> {
        let class_id = self.get("classes", who);
        if let Some(class_id) = class_id {
            if let [class_id] = class_id.as_ref() {
                Class::try_from(*class_id).ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn set_class(&self, who: &String, class: Class) {
        self.insert("classes", who, (&[class as u8]));
    }

    fn get_venom_plan(&self, stack_name: &String) -> Option<Vec<VenomPlan>> {
        self.get_json::<Vec<VenomPlan>>("stacks", stack_name)
    }

    fn get_hypno_plan(&self, stack_name: &String) -> Option<Vec<Hypnosis>> {
        self.get_json::<Vec<Hypnosis>>("hypnosis", stack_name)
    }

    fn set_first_aid_priorities(
        &self,
        who: &String,
        priorities_name: &String,
        priorities: FirstAidPriorities,
    ) {
        self.insert_json::<FirstAidPriorities>(
            "firstaid",
            &format!("{}_{}", who, priorities_name),
            priorities,
        );
    }

    fn get_first_aid_priorities(
        &self,
        who: &String,
        priorities_name: &String,
    ) -> Option<FirstAidPriorities> {
        self.get_json::<FirstAidPriorities>("firstaid", &format!("{}_{}", who, priorities_name))
    }

    fn insert_hint(&self, key: &String, value: &String) {
        self.insert(HINT_TREE, key, value.as_bytes());
    }
    fn get_hint(&self, key: &String) -> Option<String> {
        self.get(HINT_TREE, key).and_then(|bytes| {
            std::str::from_utf8(&bytes)
                .map(|str_ref| str_ref.to_string())
                .ok()
        })
    }

    fn set_defense_order(&self, class_who: &String, order: Vec<String>) {
        self.insert_json::<Vec<String>>("defense_orders", class_who, order);
    }
    fn get_defense_order(&self, class_who: &String) -> Option<Vec<String>> {
        self.get_json::<Vec<String>>("defense_orders", class_who)
    }

    fn add_defense_command(&self, defense: &String, command: &String) {
        self.insert("defense_commands", defense, command.as_bytes());
    }
    fn get_defense_command(&self, defense: &String) -> Option<String> {
        self.get("defense_commands", defense).and_then(|bytes| {
            std::str::from_utf8(&bytes)
                .map(|str_ref| str_ref.to_string())
                .ok()
        })
    }
    fn remove_defense_command(&self, defense: &String) {
        self.remove("defense_commands", defense);
    }

    fn add_defense_trigger(&self, defense: &String, trigger: &String) {
        self.insert("defense_triggers", trigger, defense.as_bytes());
    }
    fn get_defense_trigger(&self, trigger: &String) -> Option<String> {
        self.get("defense_triggers", trigger).and_then(|bytes| {
            std::str::from_utf8(&bytes)
                .map(|str_ref| str_ref.to_string())
                .ok()
        })
    }
    fn remove_defense_trigger(&self, trigger: &String) {
        self.remove("defense_triggers", trigger);
    }

    fn add_defense_missing_trigger(&self, defense: &String, trigger: &String) {
        self.insert("defense_missing_triggers", trigger, defense.as_bytes());
    }
    fn get_defense_missing_trigger(&self, trigger: &String) -> Option<String> {
        self.get("defense_missing_triggers", trigger)
            .and_then(|bytes| {
                std::str::from_utf8(&bytes)
                    .map(|str_ref| str_ref.to_string())
                    .ok()
            })
    }
    fn remove_defense_missing_trigger(&self, trigger: &String) {
        self.remove("defense_missing_triggers", trigger);
    }
}
