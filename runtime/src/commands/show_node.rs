use crate::pipeline_access::PipelineAccess;
use mizer_commander::{Command, RefMut};
use mizer_node::{NodePath, NodePosition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct ShowNodeCommand {
    pub path: NodePath,
    pub position: NodePosition,
}

impl<'a> Command<'a> for ShowNodeCommand {
    type Dependencies = RefMut<PipelineAccess>;
    type State = (bool, NodePosition);
    type Result = ();

    fn label(&self) -> String {
        format!("Showing Node '{}'", self.path)
    }

    fn apply(&self, pipeline: &mut PipelineAccess) -> anyhow::Result<(Self::Result, Self::State)> {
        let mut nodes = pipeline.designer.read();
        let node = nodes
            .get_mut(&self.path)
            .ok_or_else(|| anyhow::anyhow!("Unknown node {}", self.path))?;
        let mut hidden = false;
        let mut position = self.position;
        std::mem::swap(&mut node.hidden, &mut hidden);
        std::mem::swap(&mut node.position, &mut position);
        pipeline.designer.set(nodes);

        Ok(((), (hidden, position)))
    }

    fn revert(
        &self,
        pipeline: &mut PipelineAccess,
        (hidden, position): Self::State,
    ) -> anyhow::Result<()> {
        let mut nodes = pipeline.designer.read();
        let node = nodes
            .get_mut(&self.path)
            .ok_or_else(|| anyhow::anyhow!("Unknown node {}", self.path))?;
        node.hidden = hidden;
        node.position = position;
        pipeline.designer.set(nodes);

        Ok(())
    }
}
