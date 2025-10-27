use crate::untargetted_action;
#[macro_use(affliction_stacker, affliction_plan_stacker)]
use crate::{affliction_stacker, affliction_plan_stacker};
use super::actions::*;
use super::constants::*;
use crate::alpha_beta::ActionPlanner;
use crate::classes::*;
use crate::defense::*;
use crate::timeline::*;
use crate::types::*;

targetted_action!(PierceActionLeft, "dhuriv pierce {} left");
targetted_action!(PierceActionRight, "dhuriv pierce {} right");
targetted_action!(SeverActionLeft, "dhuriv sever {} left");
targetted_action!(SeverActionRight, "dhuriv sever {} right");

untargetted_action!(MightAction, "might");

targetted_action!(DualrazeAction, "dhuriv dualraze {}");
targetted_action!(SpinecutAction, "dhuriv spinecut {}");

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum FirstStrike {
    Slash(&'static str),
    Ambush(&'static str),
    Blind,
    Twirl,
    Strike,
    Crosscut,
    WeakenArms,
    WeakenLegs,
    Reave,
    Trip,
    Slam,
    Daunt(&'static str),
    Icebreath,
}

impl FirstStrike {
    pub fn combo_str(&self) -> String {
        match self {
            FirstStrike::Slash(venom) => "slash".to_string(),
            FirstStrike::Ambush(venom) => "ambush".to_string(),
            FirstStrike::Blind => "blind".to_string(),
            FirstStrike::Twirl => "twirl".to_string(),
            FirstStrike::Strike => "strike".to_string(),
            FirstStrike::Crosscut => "crosscut".to_string(),
            FirstStrike::WeakenArms => "weaken arms".to_string(),
            FirstStrike::WeakenLegs => "weaken legs".to_string(),
            FirstStrike::Reave => "reave".to_string(),
            FirstStrike::Trip => "trip".to_string(),
            FirstStrike::Slam => "slam".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn full_str(&self, target: &String) -> String {
        match self {
            FirstStrike::Daunt(animal) => format!("order {} daunt {}", animal, target),
            FirstStrike::Icebreath => format!("order icewyrm icebreath {}", target),
            _ => "".to_string(), // TODO
        }
    }

    pub fn venom(&self) -> &'static str {
        match self {
            FirstStrike::Slash(venom) | FirstStrike::Ambush(venom) => venom,
            _ => "",
        }
    }

    pub fn flourish(&self) -> bool {
        match self {
            FirstStrike::Daunt(_) | FirstStrike::Icebreath => true,
            _ => false,
        }
    }

    pub fn ignores_rebounding(&self) -> bool {
        match self {
            FirstStrike::Twirl => false, // TODO: We need to handle for second strike rebounding if we try this.
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum SecondStrike {
    Stab(&'static str),
    Slice(&'static str),
    Thrust(&'static str),
    Flourish(&'static str),
    Disarm,
    Gouge,
    Heartbreaker,
    Slit,
}

impl SecondStrike {
    pub fn combo_str(&self) -> String {
        match self {
            SecondStrike::Stab(venom) => "stab".to_string(),
            SecondStrike::Slice(venom) => "slice".to_string(),
            SecondStrike::Thrust(venom) => "thrust".to_string(),
            SecondStrike::Disarm => "disarm".to_string(),
            SecondStrike::Gouge => "gouge".to_string(),
            SecondStrike::Heartbreaker => "heartbreaker".to_string(),
            SecondStrike::Slit => "slit".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn full_str(&self, target: &String) -> String {
        match self {
            SecondStrike::Flourish(venom) => format!("dhuriv flourish {} {}", target, venom),
            _ => "".to_string(), // TODO
        }
    }

    pub fn venom(&self) -> &'static str {
        match self {
            SecondStrike::Stab(venom)
            | SecondStrike::Slice(venom)
            | SecondStrike::Thrust(venom) => venom,
            _ => "",
        }
    }
}

lazy_static! {
    static ref FIRST_STRIKES: HashMap<FType, FirstStrike> = {
        let mut val = HashMap::new();
        for (aff, venom) in AFFLICT_VENOMS.iter() {
            val.insert(*aff, FirstStrike::Slash(venom));
        }
        val.insert(FType::Frozen, FirstStrike::Icebreath);
        val.insert(FType::Shivering, FirstStrike::Icebreath);
        val.insert(FType::Confusion, FirstStrike::Twirl);
        val.insert(FType::Impairment, FirstStrike::Crosscut);
        val.insert(FType::Addiction, FirstStrike::Crosscut);
        val.insert(FType::Lethargy, FirstStrike::WeakenLegs);
        val.insert(FType::Epilepsy, FirstStrike::Slam);
        val.insert(FType::Laxity, FirstStrike::Slam);
        val
    };
}

lazy_static! {
    static ref FIRST_STRIKE_AFFS: HashMap<FirstStrike, Vec<FType>> = {
        let mut val = HashMap::new();
        for (aff, venom) in AFFLICT_VENOMS.iter() {
            val.insert(FirstStrike::Slash(venom), vec![*aff]);
        }
        val.insert(FirstStrike::Slash("epseth"), vec![FType::LeftLegCrippled, FType::RightLegCrippled]);
        val.insert(FirstStrike::Slash("epteth"), vec![FType::LeftArmCrippled, FType::RightArmCrippled]);
        val.insert(FirstStrike::Twirl, vec![FType::Confusion]);
        // Wrong, only one actually applies
        val.insert(FirstStrike::Crosscut, vec![FType::Impairment, FType::Addiction]);
        val.insert(FirstStrike::WeakenLegs, vec![FType::Lethargy]);
        val.insert(FirstStrike::Slam, vec![FType::Epilepsy, FType::Laxity]);
        val
    };
}

affliction_plan_stacker!(
    add_first_strike_from_plan,
    get_first_strike_from_plan,
    FIRST_STRIKES,
    FirstStrike
);

fn assume_hit(who: &mut AgentState, strike: &FirstStrike) {
    if let Some(affs) = FIRST_STRIKE_AFFS.get(strike) {
        for aff in affs {
            println!("Hit {:?}", aff);
            who.set_flag(*aff, true);
        }
    }
}

lazy_static! {
    static ref SECOND_STRIKES: HashMap<FType, SecondStrike> = {
        let mut val = HashMap::new();
        for (aff, venom) in AFFLICT_VENOMS.iter() {
            val.insert(*aff, SecondStrike::Stab(venom));
        }
        val.insert(FType::Impatience, SecondStrike::Gouge);
        val.insert(FType::Arrhythmia, SecondStrike::Heartbreaker);
        val.insert(FType::CrippledThroat, SecondStrike::Slit);
        val
    };
}

affliction_plan_stacker!(
    add_second_strike_from_plan,
    get_second_strike_from_plan,
    SECOND_STRIKES,
    SecondStrike
);

/**
 * AetObservations
 **/

lazy_static! {
    static ref DUALRAZE_ORDER: Vec<FType> = vec![FType::Shielded, FType::Rebounding, FType::Speed,];
}

lazy_static! {
    static ref REAVE_ORDER: Vec<FType> = vec![FType::Shielded, FType::Rebounding];
}
/**
 *
 * ActiveTransitions!
 *
**/

pub struct ComboAction {
    pub caster: String,
    pub target: String,
    pub first_strike: FirstStrike,
    pub second_strike: SecondStrike,
}

impl ComboAction {
    pub fn new(
        caster: String,
        target: String,
        first_strike: FirstStrike,
        second_strike: SecondStrike,
    ) -> Self {
        ComboAction {
            caster,
            target,
            first_strike,
            second_strike,
        }
    }
}

impl ActiveTransition for ComboAction {
    fn simulate(&self, _timeline: &AetTimeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(get_combo_action(
            &timeline,
            &self.target,
            &self.first_strike,
            &self.second_strike,
        ))
    }
}

fn get_combo_action(
    timeline: &AetTimeline,
    target: &String,
    first_strike: &FirstStrike,
    second_strike: &SecondStrike,
) -> String {
    let attack = if first_strike.flourish() {
        format!(
            "{};;{}",
            first_strike.full_str(target),
            second_strike.full_str(target)
        )
    } else {
        format!(
            "dhuriv combo {} {} {} {} {}",
            target,
            first_strike.combo_str(),
            second_strike.combo_str(),
            first_strike.venom(),
            second_strike.venom(),
        )
    };
    format!("order loyal attack {};;stand;;stand;;{}", target, attack)
}

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

pub fn get_balance_attack<'s>(
    timeline: &AetTimeline,
    who_am_i: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Box<dyn ActiveTransition> {
    if strategy.eq("damage") {
        return Box::new(Inactivity);
    } else {
        let me = timeline.state.borrow_agent(who_am_i);
        let mut you = timeline.state.borrow_agent(target);
        let mut stack = get_stack(timeline, &you, strategy, db);
        if want_spinecut(&you) {
            return Box::new(SpinecutAction::new(who_am_i.to_string(), target.clone()));
        } else if want_fitness(&me) {
            return Box::new(FitnessAction::new(who_am_i.to_string()));
        } else if want_might(&me) {
            return Box::new(MightAction::new(who_am_i.to_string()));
        } else if you.is(FType::Shielded) && you.is(FType::Rebounding) {
            return Box::new(DualrazeAction::new(who_am_i.to_string(), target.clone()));
        } else if let Some(side) = want_pierce(&you) {
            match side.as_str() {
                "left" => {
                    return Box::new(PierceActionLeft::new(who_am_i.to_string(), target.clone()))
                }
                "right" => {
                    return Box::new(PierceActionRight::new(who_am_i.to_string(), target.clone()))
                }
                _ => (),
            }
        } else if let Some(side) = want_sever(&you) {
            match side.as_str() {
                "left" => {
                    return Box::new(SeverActionLeft::new(who_am_i.to_string(), target.clone()))
                }
                "right" => {
                    return Box::new(SeverActionRight::new(who_am_i.to_string(), target.clone()))
                }
                _ => (),
            }
        } else {
            let first_strike = get_first_strike_from_plan(
                &stack,
                1,
                &you,
                &vec![
                    FType::LeftLegCrippled,
                    FType::RightLegCrippled,
                    FType::LeftArmCrippled,
                    FType::RightArmCrippled,
                ],
            )
            .pop();
            if let Some(mut first_strike) = first_strike {
                if you.is(FType::Rebounding) && !first_strike.ignores_rebounding() {
                    first_strike = FirstStrike::Reave;
                }
                assume_hit(&mut you, &first_strike);
                stack = get_stack(timeline, &you, strategy, db);
                let second_strike = if first_strike.flourish() {
                    get_venoms_from_plan(
                        &stack,
                        1,
                        &you,
                        &vec![
                            FType::LeftLegCrippled,
                            FType::RightLegCrippled,
                            FType::LeftArmCrippled,
                            FType::RightArmCrippled,
                        ],
                    )
                    .pop()
                    .map(|venom| SecondStrike::Flourish(venom))
                } else {
                    get_second_strike_from_plan(
                        &stack,
                        1,
                        &you,
                        &vec![
                            FType::LeftLegCrippled,
                            FType::RightLegCrippled,
                            FType::LeftArmCrippled,
                            FType::RightArmCrippled,
                        ],
                    )
                    .pop()
                };
                if let Some(second_strike) = second_strike {
                    return Box::new(ComboAction::new(
                        who_am_i.to_string(),
                        target.clone(),
                        first_strike,
                        second_strike,
                    ));
                }
            }
        }
        return Box::new(Inactivity);
    }
}

pub fn get_action_plan(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> ActionPlan {
    let mut action_plan = ActionPlan::new(me);
    let mut balance = get_balance_attack(timeline, me, target, strategy, db);
    if let Some(parry) = get_needed_parry(timeline, me, target, db) {
        balance = Box::new(SeparatorAction::pair(
            Box::new(ParryAction::by_class(
                me.to_string(),
                parry,
                Class::Sentinel,
            )),
            balance,
        ));
    }
    if let Ok(_activation) = balance.act(&timeline) {
        action_plan.add_to_qeb(balance);
    }
    action_plan
}

pub fn get_attack(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> String {
    let action_plan = get_action_plan(&timeline, &timeline.who_am_i(), &target, &strategy, db);
    action_plan.get_inputs(&timeline)
}
