pub use crate::module::RuntimeModule;
use mizer_clock::SystemClock;
pub use mizer_pipeline::NodePreviewRef;
pub use node_metadata::NodeMetadataRef;

pub use self::api::*;
pub use self::coordinator::CoordinatorRuntime;
pub use self::pipeline::*;
pub use self::views::LayoutsView;

pub type DefaultRuntime = CoordinatorRuntime<SystemClock>;

mod api;
pub mod commands;
mod context;
mod coordinator;
mod debug_ui_pane;
mod module;
mod node_metadata;
mod pipeline;
pub mod queries;
mod views;
