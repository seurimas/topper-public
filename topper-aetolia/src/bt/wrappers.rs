use topper_bt::unpowered::*;

use super::*;

pub struct WithoutAffsInStack {
    pub affs: Vec<FType>,
    pub tree: Box<
        dyn UnpoweredFunction<Model = BehaviorModel, Controller = BehaviorController> + Send + Sync,
    >,
}

impl WithoutAffsInStack {
    pub fn new(
        tree: Box<
            dyn UnpoweredFunction<Model = BehaviorModel, Controller = BehaviorController>
                + Send
                + Sync,
        >,
        affs: Vec<FType>,
    ) -> Self {
        Self { affs, tree }
    }
}

impl UnpoweredFunction for WithoutAffsInStack {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        if let Some(original_stack) = controller.aff_priorities.clone() {
            let mut stack = original_stack.clone();
            stack.retain(|aff| !self.affs.contains(&aff.affliction()));
            controller.aff_priorities = Some(stack);
            let result = self.tree.resume_with(model, controller);
            controller.aff_priorities = Some(original_stack);
            result
        } else {
            self.tree.resume_with(model, controller)
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        self.tree.reset(model);
    }
}
