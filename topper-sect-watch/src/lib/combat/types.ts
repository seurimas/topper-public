/**
 * LimbState {
    pub damage: f32,
    pub crippled: bool,
    pub broken: bool,
    pub mangled: bool,
    pub amputated: bool,
    pub is_restoring: bool,
    pub is_parried: bool,
    pub is_dislocated: bool,
    pub welt: bool,
    pub bruise_level: usize,
    pub fleshbaned_count: usize,
}
 */

export type LimbState = {
    damage: number;
    crippled: boolean;
    broken: boolean;
    mangled: boolean;
    amputated: boolean;
    is_restoring: boolean;
    is_parried: boolean;
    is_dislocated: boolean;
    welt: boolean;
    bruise_level: number;
    fleshbaned_count: number;
}

export type LimbsState = {
    head: LimbState;
    torso: LimbState;
    left_arm: LimbState;
    right_arm: LimbState;
    left_leg: LimbState;
    right_leg: LimbState;
}

export type Balances = Map<string, number | undefined>;

export type Afflictions = string[];

export type TimelineControl = {
    type: 'scrollLock',
} | {
    type: 'timeStep',
    speed: number,
} | {
    type: 'manual',
};