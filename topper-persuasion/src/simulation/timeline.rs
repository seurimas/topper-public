use serde::{Deserialize, Serialize};

use crate::{AppealType, Appeals, Personality, PersuasionAff, PersuasionState, PersuasionStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampedEvent {
    pub time: String,
    pub event: PersuasionEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompellingType {
    Compelling,
    DecisivelyCompelling,
    IrrefutablyCompelling,
    ResoundinglyCompelling,
    IncontrovertiblyCompelling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersuasionEvent {
    Stats(i32, i32, i32), // Strength, Wisdom, Intelligence
    Scrutinised(String, Personality, i32, i32),
    ResolveAffect(i32, AppealType),
    Compelling(CompellingType),
    Cyclic(Appeals),
    Evidence(AppealType), // Your evidence weakens a dishevelled hedge mage's resistance to ethos appeals!
    AcumenLost(i32),
    AcumenGain(i32),
    StartPersuasion(String),
    SipPrudence,
    Retort(Retorts),
    Appeal(Appeals),
    Draw(Vec<Appeals>),
    Gained(PersuasionAff),
    Lost(PersuasionAff),
    CasualityReveals(Appeals), // Your logical foresight reveals your next argument will appeal to inspiration.
    RhetoricStart, // You focus your mind on the art of rhetoric, preparing to chain your arguments together.
    RhetoricLost,  // You lose your focus on the art of rhetoric, breaking your chain of arguments.
    SuccessfulPersuasion, // You have successfully persuaded a gruff, menacing drifter!
    FailedPersuasion, // You have failed to persuade a gruff, menacing drifter.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Retorts {
    SimpleCounterpoint, // A dishevelled hedge mage raises a simple counterpoint, challenging the foundation of your argument.
    SimpleDismiss, // With a wave of his hand, a gruff, menacing drifter dismisses your points as irrelevant and unworthy of consideration.
    SimpleRefusal, // A gruff, menacing drifter crosses his arms and shakes his head, steadfastly refusing to accept your perspective.
    SimpleResistance, // A gruff, menacing drifter plants his feet firmly and meets your gaze with unwavering resistance to your persuasion
    SimpleSkepticism, // A weatherworn vagrant narrows her eyes and questions the validity of your claims with pointed skepticism.
    // Gives non-persuasion affs.
    SimpleDisruption, // A gruff, menacing drifter unleashes a disorienting barrage of conflicting arguments that shatter your cognitive focus, leaving your thoughts fragmented and chaotic.
    SimpleParanoia, // A gruff, menacing drifter weaves subtle insinuations into the debate, planting seeds of doubt until you begin to question every word you say.
    SimpleRecklessness, // A gruff, menacing drifter bellows a wild, impassioned challenge that provokes you into rash, unthinking actions, driving you toward reckless arguments.
    SimpleConfusion, // A gruff, menacing drifter launches into a cascade of perplexing metaphors and contradictory claims, leaving your thoughts in a tangled mess.
    SimpleIndifference, // A weatherworn vagrant dismisses your arguments in a flat, monotonous tone that saps your enthusiasm, leaving you feeling strangely indifferent.
    SimpleGnawing, // A weatherworn vagrant methodically chips away at your confidence with cutting remarks, leaving you with a persistent, gnawing unease that undermines your resolve.
    SimpleWeariness, // A weatherworn vagrant drones on in a long, monotonous monologue, the relentless minutiae sapping your mental energy until you feel both physically and mentally wearied.
    SimpleDizziness, // A weatherworn vagrant bombards you with an avalanche of statistics and convoluted figures so rapidly that your head spins, and you feel utterly dizzy.
    SimpleStupidity, // A dishevelled hedge mage sneers condescendingly, his words slicing through your confidence until you feel unnaturally stupid.
    // Gives persuasion affs.
    Disrupted, // A weatherworn vagrant unleashes a torrent of disjointed assertions that rattle your mental equilibrium, leaving you struggling to regain composure.
    Pressure, // A dishevelled hedge mage zeroes in on a weak point in your argument, pressuring you to defend it properly.
    Discard, // A gruff, menacing drifter interrupts your argument with a sudden, unexpected question that throws you off balance, making you lose your train of thought on one of your arguments.
    Fatigued, // A gruff, menacing drifter methodically challenges each point you make, wearing down your mental stamina.
    LimitedAppeals, // A weatherworn vagrant unleashes a dizzying barrage of counterpoints that scrambles your thoughts, forcing you to limit your arguments.
    Slandered, // A weatherworn vagrant spreads subtle doubts about your credibility, undermining the weight of your arguments.
    Conflicted, // A weatherworn vagrant presents a challenging dilemma that demands a multi-faceted response.
    Confounded, // A Nazetu halberdier launches into a bewildering tangent that leaves you struggling to follow the thread of the debate.

    // Appeal-based
    HitProvocation, // A moss-veiled noblewoman's scathing retort strikes right to the heart of your argument!
    MissedProvocation, // A weatherworn vagrant's hasty retort falls flat and unconvincing.
    Intimidated, // A gruff, menacing drifter presents a hastily constructed argument, his usual rhetorical flair notably diminished in the face of your recent intimidation.
}

impl PersuasionEvent {
    pub fn apply(&self, my_state: &mut PersuasionState, their_status: &mut PersuasionStatus) {
        match self {
            PersuasionEvent::Stats(strength, wisdom, intelligence) => {
                my_state.str = Some(*strength);
                my_state.wis = Some(*wisdom);
                my_state.int = Some(*intelligence);
            }
            PersuasionEvent::Scrutinised(name, personality, resolve, max_resolve) => {
                *their_status = PersuasionStatus::Scrutinised {
                    resolve: *resolve,
                    max_resolve: *max_resolve,
                    personality: *personality,
                    weakened: vec![],
                    unique: false,
                };
            }
            PersuasionEvent::ResolveAffect(amount, appeal) => {
                their_status.resolve_affect(*amount);
            }
            PersuasionEvent::AcumenGain(amount) => {
                my_state.acumen += amount;
            }
            PersuasionEvent::AcumenLost(amount) => {
                my_state.acumen -= amount;
            }
            PersuasionEvent::StartPersuasion(name) => {
                my_state.start_persuasion(0);
                their_status.start_persuasion();
            }
            PersuasionEvent::Appeal(appeal) => {
                my_state.appeal(appeal.clone());
            }
            PersuasionEvent::Retort(retort) => match retort {
                Retorts::HitProvocation | Retorts::MissedProvocation => {}
                _ => {
                    their_status.retorted();
                }
            },
            PersuasionEvent::Cyclic(appeals) => {
                my_state.cyclic = Some(appeals.clone());
            }
            PersuasionEvent::Draw(appeals) => {
                for appeal in appeals {
                    my_state.drawn(appeal.clone());
                }
            }
            PersuasionEvent::Gained(aff) => {
                my_state.set(aff.clone(), true);
            }
            PersuasionEvent::Lost(aff) => {
                my_state.set(aff.clone(), false);
            }
            PersuasionEvent::RhetoricStart => {
                my_state.rhetoric_start();
            }
            PersuasionEvent::SipPrudence => {
                my_state.sip_prudence();
            }
            PersuasionEvent::Evidence(appeal) => {
                their_status.evidence(appeal.clone());
            }
            PersuasionEvent::Compelling(_)
            | PersuasionEvent::CasualityReveals(_)
            | PersuasionEvent::RhetoricLost
            | PersuasionEvent::SuccessfulPersuasion
            | PersuasionEvent::FailedPersuasion => {}
        }
    }
}
