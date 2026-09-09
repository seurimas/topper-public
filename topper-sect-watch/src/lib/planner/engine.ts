export type ActionDescriptor = {
    id: string;
    label: string;
    targeted: boolean;
};

export type PassiveDescriptor = {
    id: string;
    label: string;
    period_ms: number;
    targeted: boolean;
};

export type EngineState = {
    active_class: string;
    battle_stats: unknown;
};

export type ActionExecution = {
    action_id: string;
    command: string;
    applied_time: number;
    observations: string[];
    battle_stats: unknown;
};

let initialized = false;
let engine: any;

async function loadWasm() {
    const wasmModule = await import('topper');
    if (!initialized) {
        await wasmModule.default();
        initialized = true;
    }
    return wasmModule;
}

async function getEngine() {
    const wasm: any = await loadWasm();
    if (!engine) {
        engine = new wasm.LearnerEngine();
    }
    return engine;
}

export async function getSupportedClasses(): Promise<string[]> {
    const learner = await getEngine();
    return learner.get_supported_classes() as string[];
}

export async function setActiveClass(className: string): Promise<EngineState> {
    const learner = await getEngine();
    return learner.set_active_class(className) as EngineState;
}

export async function getActions(): Promise<ActionDescriptor[]> {
    const learner = await getEngine();
    return learner.get_actions() as ActionDescriptor[];
}

export async function getPassives(): Promise<PassiveDescriptor[]> {
    const learner = await getEngine();
    return learner.get_passives() as PassiveDescriptor[];
}

export async function getState(): Promise<EngineState> {
    const learner = await getEngine();
    return learner.get_state() as EngineState;
}

export async function simulateAction(actionId: string): Promise<ActionExecution> {
    const learner = await getEngine();
    return learner.simulate_action(actionId) as ActionExecution;
}

export async function advanceTimelineBy(deltaMs: number): Promise<EngineState> {
    const learner = await getEngine();
    return learner.advance_by(deltaMs) as EngineState;
}

export async function advanceTimelineToQeb(): Promise<EngineState> {
    const learner = await getEngine();
    return learner.advance_to_qeb() as EngineState;
}
