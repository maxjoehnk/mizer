use std::collections::HashMap;
use std::path::PathBuf;

use mizer_clock::Clock;
use mizer_command_executor::CommandHistory;
use mizer_connections::{
    midi_device_profile::DeviceProfile, Connection, DmxConfig, DmxView, MidiView,
};
use mizer_devices::DeviceManager;
use mizer_fixtures::library::FixtureLibrary;
use mizer_message_bus::Subscriber;
use mizer_module::Runtime;
use mizer_protocol_dmx::{DmxConnectionManager, DmxOutput};
use mizer_protocol_midi::{MidiConnectionManager, MidiEvent};

use crate::fixture_libraries_loader::FixtureLibrariesLoader;
use crate::{ApiCommand, Mizer};

pub struct ApiHandler {
    pub(super) recv: flume::Receiver<ApiCommand>,
}

impl ApiHandler {
    #[profiling::function]
    pub fn handle(&self, mizer: &mut Mizer) {
        loop {
            match self.recv.try_recv() {
                Ok(cmd) => self.handle_command(cmd, mizer),
                Err(flume::TryRecvError::Empty) => break,
                Err(flume::TryRecvError::Disconnected) => {
                    panic!("api command receiver disconnected")
                }
            }
        }
    }

    fn handle_command(&self, command: ApiCommand, mizer: &mut Mizer) {
        match command {
            ApiCommand::WritePort(path, port, value, sender) => {
                mizer.runtime.pipeline.write_port(path, port, value);

                sender
                    .send(Ok(()))
                    .expect("api command sender disconnected");
            }
            ApiCommand::ReadFaderValue(path, sender) => {
                let value = mizer
                    .runtime
                    .pipeline
                    .get_state::<f64>(&path)
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("No fader node with given path found"));

                sender.send(value).expect("api command sender disconnected");
            }
            ApiCommand::GetNodePreviewRef(path, sender) => sender
                .send(mizer.runtime.get_history_ref(&path))
                .expect("api command sender disconnected"),
            ApiCommand::SetClockState(state) => {
                mizer.runtime.clock.set_state(state);
            }
            ApiCommand::SetBpm(bpm) => {
                let speed = mizer.runtime.clock.speed_mut();
                *speed = bpm;
            }
            ApiCommand::SaveProject(sender) => {
                let result = mizer.save_project();
                sender
                    .send(result)
                    .expect("api command sender disconnected");
            }
            ApiCommand::SaveProjectAs(path, sender) => {
                let result = mizer.save_project_as(PathBuf::from(&path));
                sender
                    .send(result)
                    .expect("api command sender disconnected");
            }
            ApiCommand::NewProject(sender) => {
                mizer.new_project();
                sender
                    .send(Ok(()))
                    .expect("api command sender disconnected");
            }
            ApiCommand::LoadProject(path, sender) => {
                let result = mizer.load_project_from(PathBuf::from(&path));
                sender
                    .send(result)
                    .expect("api command sender disconnected");
            }
            ApiCommand::GetConnections(sender) => {
                let connections = self.get_connections(mizer);
                sender
                    .send(connections)
                    .expect("api command sender disconnected");
            }
            ApiCommand::GetMidiDeviceProfiles(sender) => {
                let device_profiles = self.get_midi_device_profiles(mizer);
                sender
                    .send(device_profiles)
                    .expect("api command sender disconnected");
            }
            ApiCommand::GetDmxMonitor(output_id, sender) => {
                let result = self.monitor_dmx(mizer, output_id);
                sender
                    .send(result)
                    .expect("api command sender disconnected");
            }
            ApiCommand::GetMidiMonitor(name, sender) => {
                let result = self.monitor_midi(mizer, name);
                sender
                    .send(result)
                    .expect("api command sender disconnected");
            }
            ApiCommand::ObserveSession(sender) => {
                sender
                    .send(mizer.session_events.subscribe())
                    .expect("api command sender disconnected");
            }
            ApiCommand::ReloadFixtureLibraries(paths, sender) => {
                let injector = mizer.runtime.injector();
                let library = injector.get::<FixtureLibrary>().unwrap();
                let result = FixtureLibrariesLoader(library.clone()).reload(paths);

                sender
                    .send(result)
                    .expect("api command sender disconnected");
            }
            ApiCommand::GetHistory(sender) => {
                let injector = mizer.runtime.injector();
                let history = injector.get::<CommandHistory>().unwrap();
                let result = history.items();
                let cursor = history.index();

                sender
                    .send((result, cursor))
                    .expect("api command sender disconnected");
            }
        }
    }

    fn get_connections(&self, mizer: &mut Mizer) -> Vec<Connection> {
        let manager = mizer
            .runtime
            .injector()
            .get::<MidiConnectionManager>()
            .unwrap();
        let midi_connections = manager
            .list_available_devices()
            .into_iter()
            .map(|device| MidiView {
                name: device.name,
                device_profile: device.profile.as_ref().map(|profile| profile.id.clone()),
            })
            .map(Connection::from);
        let dmx_manager = mizer
            .runtime
            .injector()
            .get::<DmxConnectionManager>()
            .unwrap();
        let dmx_connections = dmx_manager
            .list_outputs()
            .into_iter()
            .map(|(id, output)| DmxView {
                output_id: id.clone(),
                name: output.name(),
                config: match output {
                    mizer_protocol_dmx::DmxConnection::Artnet(config) => DmxConfig::Artnet {
                        host: config.host.clone(),
                        port: config.port,
                    },
                    mizer_protocol_dmx::DmxConnection::Sacn(_) => DmxConfig::Sacn,
                },
            })
            .map(Connection::from);
        let device_manager = mizer.runtime.injector().get::<DeviceManager>().unwrap();
        let devices = device_manager
            .current_devices()
            .into_iter()
            .map(Connection::from);

        let mut connections = Vec::new();
        connections.extend(midi_connections);
        connections.extend(dmx_connections);
        connections.extend(devices);

        connections
    }

    fn get_midi_device_profiles(&self, mizer: &mut Mizer) -> Vec<DeviceProfile> {
        let manager = mizer
            .runtime
            .injector()
            .get::<MidiConnectionManager>()
            .unwrap();

        manager.list_available_device_profiles()
    }

    fn monitor_dmx(
        &self,
        mizer: &mut Mizer,
        output_id: String,
    ) -> anyhow::Result<HashMap<u16, [u8; 512]>> {
        let dmx_manager = mizer
            .runtime
            .injector()
            .get::<DmxConnectionManager>()
            .unwrap();
        let dmx_connection = dmx_manager.get_output(&output_id).unwrap();
        let buffer = dmx_connection.read_buffer();

        Ok(buffer)
    }

    fn monitor_midi(
        &self,
        mizer: &mut Mizer,
        name: String,
    ) -> anyhow::Result<Subscriber<MidiEvent>> {
        let midi_manager = mizer
            .runtime
            .injector()
            .get::<MidiConnectionManager>()
            .unwrap();
        let device = midi_manager
            .request_device(&name)?
            .ok_or_else(|| anyhow::anyhow!("Unknown Midi Device"))?;
        let subscriber = device.events();

        Ok(subscriber)
    }
}
