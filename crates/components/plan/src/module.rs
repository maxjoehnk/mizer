use crate::debug_ui_pane::PlanDebugUiPane;
use mizer_module::{module_name, Module, ModuleContext};
use crate::project_handler::PlanProjectHandler;

pub struct PlansModule;

module_name!(PlansModule);

impl Module for PlansModule {
    const IS_REQUIRED: bool = false;

    fn register(self, context: &mut impl ModuleContext) -> anyhow::Result<()> {
        context.add_debug_ui_pane(PlanDebugUiPane);
        context.add_project_handler(PlanProjectHandler);

        Ok(())
    }
}
