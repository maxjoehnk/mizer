use crate::pipeline_access::PipelineAccess;
use mizer_commander::{Command, RefMut};
use mizer_execution_planner::ExecutionPlanner;
use mizer_node::NodeLink;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct AddLinkCommand {
    pub link: NodeLink,
}

impl<'a> Command<'a> for AddLinkCommand {
    type Dependencies = (RefMut<PipelineAccess>, RefMut<ExecutionPlanner>);
    type State = ();
    type Result = ();

    fn label(&self) -> String {
        format!(
            "Add Link from '{}:{}' to '{}:{}'",
            self.link.source, self.link.source_port, self.link.target, self.link.target_port
        )
    }

    fn apply(
        &self,
        (pipeline, planner): (&mut PipelineAccess, &mut ExecutionPlanner),
    ) -> anyhow::Result<(Self::Result, Self::State)> {
        pipeline.add_link(self.link.clone())?;
        planner.add_link(self.link.clone());

        Ok(((), ()))
    }

    fn revert(
        &self,
        (pipeline, planner): (&mut PipelineAccess, &mut ExecutionPlanner),
        _: Self::State,
    ) -> anyhow::Result<()> {
        planner.remove_link(&self.link);
        pipeline.remove_link(&self.link);

        Ok(())
    }
}
