use mizer_fixtures::library::FixtureLibrary;
use mizer_fixtures::manager::FixtureManager;
use mizer_media::MediaServer;
use mizer_sequencer::{EffectEngine, Sequencer};
use mizer_status_bus::StatusBus;
use mizer_surfaces::SurfaceRegistryApi;
use mizer_timecode::TimecodeManager;

use crate::RuntimeApi;

pub use self::connections::*;
pub use self::effects::*;
pub use self::fixtures::*;
pub use self::layouts::*;
pub use self::mappings::*;
pub use self::media::*;
pub use self::nodes::*;
pub use self::plans::*;
pub use self::programmer::*;
pub use self::sequencer::*;
pub use self::session::*;
pub use self::settings::*;
pub use self::status::*;
pub use self::surfaces::*;
pub use self::timecode::*;
pub use self::transport::*;

mod connections;
mod effects;
mod fixtures;
mod layouts;
mod mappings;
mod media;
mod nodes;
mod plans;
mod programmer;
mod sequencer;
mod session;
mod settings;
mod status;
mod surfaces;
mod timecode;
mod transport;

#[derive(Clone)]
pub struct Handlers<R: RuntimeApi> {
    pub connections: ConnectionsHandler<R>,
    pub fixtures: FixturesHandler<R>,
    pub layouts: LayoutsHandler<R>,
    pub media: MediaHandler,
    pub nodes: NodesHandler<R>,
    pub session: SessionHandler<R>,
    pub transport: TransportHandler<R>,
    pub sequencer: SequencerHandler<R>,
    pub programmer: ProgrammerHandler<R>,
    pub settings: SettingsHandler<R>,
    pub effects: EffectsHandler<R>,
    pub plans: PlansHandler<R>,
    pub mappings: MappingsHandler<R>,
    pub timecode: TimecodeHandler<R>,
    pub status: StatusHandler,
    pub surfaces: SurfacesHandler<R>,
}

impl<R: RuntimeApi> Handlers<R> {
    pub fn new(
        runtime: R,
        fixture_manager: FixtureManager,
        fixture_library: FixtureLibrary,
        media_server: MediaServer,
        sequencer: Sequencer,
        effect_engine: EffectEngine,
        timecode_manager: TimecodeManager,
        status_bus: StatusBus,
        surface_registry_api: SurfaceRegistryApi,
    ) -> Self {
        Handlers {
            connections: ConnectionsHandler::new(runtime.clone()),
            fixtures: FixturesHandler::new(
                fixture_manager.clone(),
                fixture_library,
                runtime.clone(),
            ),
            layouts: LayoutsHandler::new(runtime.clone()),
            media: MediaHandler::new(media_server),
            nodes: NodesHandler::new(runtime.clone()),
            session: SessionHandler::new(runtime.clone()),
            transport: TransportHandler::new(runtime.clone()),
            sequencer: SequencerHandler::new(sequencer.clone(), runtime.clone()),
            effects: EffectsHandler::new(effect_engine, runtime.clone()),
            programmer: ProgrammerHandler::new(fixture_manager.clone(), sequencer, runtime.clone()),
            settings: SettingsHandler::new(runtime.clone()),
            plans: PlansHandler::new(fixture_manager, runtime.clone()),
            mappings: MappingsHandler::new(runtime.clone()),
            timecode: TimecodeHandler::new(timecode_manager, runtime.clone()),
            status: StatusHandler::new(status_bus),
            surfaces: SurfacesHandler::new(runtime, surface_registry_api),
        }
    }
}
