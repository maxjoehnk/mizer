use serde::{Deserialize, Serialize};

use crate::path::NodePath;
use mizer_ports::{PortId, PortType};

#[derive(Hash, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct NodeLink {
    pub source: NodePath,
    pub source_port: PortId,
    pub target: NodePath,
    pub target_port: PortId,
    pub port_type: PortType,
    pub local: bool,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct PortMetadata {
    pub port_type: PortType,
    pub direction: PortDirection,
    pub dimensions: Option<(u64, u64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum PortDirection {
    Input,
    Output,
}

impl Default for PortDirection {
    fn default() -> Self {
        Self::Input
    }
}
