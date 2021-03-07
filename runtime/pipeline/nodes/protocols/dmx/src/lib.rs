use mizer_node::*;
use mizer_protocol_dmx::DmxConnectionManager;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct DmxOutputNode {
    #[serde(default = "default_universe")]
    universe: u16,
    channel: u8,
}

fn default_universe() -> u16 {
    1
}

impl PipelineNode for DmxOutputNode {
    fn details(&self) -> NodeDetails {
        NodeDetails {
            name: "DmxOutputNode".into(),
        }
    }

    fn introspect_port(&self, port: &PortId, _: &Injector) -> Option<PortMetadata> {
        (port == "value").then(|| PortMetadata {
            port_type: PortType::Single,
            direction: PortDirection::Input,
            ..Default::default()
        })
    }

    fn node_type(&self) -> NodeType {
        NodeType::DmxOutput
    }
}

impl ProcessingNode for DmxOutputNode {
    type State = ();

    fn process(&self, context: &impl NodeContext, _: &mut Self::State) -> anyhow::Result<()> {
        let value = context.read_port::<_, f64>("value");
        let output = context
            .inject::<DmxConnectionManager>()
            .and_then(|connection| connection.get_output("output"));
        if output.is_none() {
            anyhow::bail!("Missing dmx output {}", "output");
        }
        let output = output.unwrap();

        if let Some(value) = value {
            let value = (value * u8::max_value() as f64).min(255.).max(0.).floor() as u8;
            output.write_single(1, 0, value);
        }

        Ok(())
    }

    fn create_state(&self) -> Self::State {
        Default::default()
    }
}
