use crate::proto::fixtures::*;
use crate::proto::programmer::*;
use crate::RuntimeApi;
use futures::stream::{Stream, StreamExt};
use itertools::Itertools;
use mizer_command_executor::*;
use mizer_fixtures::definition::FixtureControlValue;
use mizer_fixtures::manager::FixtureManager;
use mizer_fixtures::programmer::ProgrammerView;
use mizer_fixtures::GroupId;
use mizer_sequencer::Sequencer;
use std::ops::Deref;

#[derive(Clone)]
pub struct ProgrammerHandler<R> {
    fixture_manager: FixtureManager,
    runtime: R,
}

impl<R: RuntimeApi> ProgrammerHandler<R> {
    pub fn new(fixture_manager: FixtureManager, _: Sequencer, runtime: R) -> Self {
        Self {
            fixture_manager,
            runtime,
        }
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn state_stream(&self) -> impl Stream<Item = ProgrammerState> + 'static {
        let programmer = self.fixture_manager.get_programmer();
        programmer.bus().map(ProgrammerState::from)
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn write_control(&self, request: WriteControlRequest) {
        let mut programmer = self.fixture_manager.get_programmer();
        let control = request.as_controls();
        programmer.write_control(control);
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn select_fixtures(&self, fixture_ids: Vec<FixtureId>) {
        log::debug!("select_fixtures {:?}", fixture_ids);
        let mut programmer = self.fixture_manager.get_programmer();
        programmer.select_fixtures(fixture_ids.into_iter().map(|id| id.into()).collect());
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn unselect_fixtures(&self, fixture_ids: Vec<FixtureId>) {
        log::debug!("unselect_fixtures {:?}", fixture_ids);
        let mut programmer = self.fixture_manager.get_programmer();
        programmer.unselect_fixtures(fixture_ids.into_iter().map(|id| id.into()).collect());
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn select_group(&self, group_id: u32) {
        log::debug!("select_group {:?}", group_id);
        if let Some(group) = self.fixture_manager.get_group(GroupId(group_id)) {
            let mut programmer = self.fixture_manager.get_programmer();
            programmer.select_group(&group);
        }
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn clear(&self) {
        let mut programmer = self.fixture_manager.get_programmer();
        programmer.clear();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn highlight(&self, highlight: bool) {
        let mut programmer = self.fixture_manager.get_programmer();
        programmer.set_highlight(highlight);
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn store(&self, sequence_id: u32, store_mode: store_request::Mode, cue_id: Option<u32>) {
        let (controls, effects) = {
            let programmer = self.fixture_manager.get_programmer();
            let controls = programmer.get_controls();
            let effects = programmer.active_effects().cloned().collect::<Vec<_>>();

            (controls, effects)
        };
        self.runtime
            .run_command(StoreProgrammerInSequenceCommand {
                sequence_id,
                cue_id,
                controls,
                store_mode: store_mode.into(),
                effects,
            })
            .unwrap();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn get_presets(&self) -> Presets {
        Presets {
            intensities: self
                .fixture_manager
                .presets
                .intensity_presets()
                .into_iter()
                .sorted_by_key(|(_, preset)| preset.id)
                .map(Preset::from)
                .collect(),
            shutters: self
                .fixture_manager
                .presets
                .shutter_presets()
                .into_iter()
                .sorted_by_key(|(_, preset)| preset.id)
                .map(Preset::from)
                .collect(),
            colors: self
                .fixture_manager
                .presets
                .color_presets()
                .into_iter()
                .sorted_by_key(|(_, preset)| preset.id)
                .map(Preset::from)
                .collect(),
            positions: self
                .fixture_manager
                .presets
                .position_presets()
                .into_iter()
                .sorted_by_key(|(_, preset)| preset.id)
                .map(Preset::from)
                .collect(),
            ..Default::default()
        }
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn call_preset(&self, preset_id: PresetId) {
        log::debug!("call_preset {:?}", preset_id);
        let mut programmer = self.fixture_manager.get_programmer();
        programmer.call_preset(&self.fixture_manager.presets, preset_id.into());
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn call_effect(&self, effect_id: u32) {
        log::debug!("call_effect {:?}", effect_id);
        let mut programmer = self.fixture_manager.get_programmer();
        programmer.call_effect(effect_id);
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn get_groups(&self) -> Groups {
        Groups {
            groups: self
                .fixture_manager
                .get_groups()
                .into_iter()
                .map(|group| group.deref().clone().into())
                .collect(),
            ..Default::default()
        }
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn add_group(&self, name: String) -> Group {
        let group = self.runtime.run_command(AddGroupCommand { name }).unwrap();

        group.into()
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn delete_group(&self, id: u32) {
        self.runtime
            .run_command(DeleteGroupCommand { id: id.into() })
            .unwrap();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn rename_group(&self, id: u32, name: String) {
        self.runtime
            .run_command(RenameGroupCommand {
                id: id.into(),
                name,
            })
            .unwrap();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn assign_fixtures_to_group(&self, group_id: u32, fixture_ids: Vec<FixtureId>) {
        let group_id = GroupId(group_id);
        let fixture_ids = fixture_ids
            .into_iter()
            .map(|id| id.into())
            .collect::<Vec<_>>();
        self.runtime
            .run_command(AssignFixturesToGroupCommand {
                group_id,
                fixture_ids,
            })
            .unwrap();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn assign_fixture_selection_to_group(&self, group_id: u32) {
        let group_id = GroupId(group_id);
        let fixture_ids = {
            let programmer = self.fixture_manager.get_programmer();
            let state = programmer.view().read();
            state.all_fixtures()
        };

        self.runtime
            .run_command(AssignFixturesToGroupCommand {
                group_id,
                fixture_ids,
            })
            .unwrap();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn programmer_view(&self) -> ProgrammerView {
        self.fixture_manager.get_programmer().view()
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn update_block_size(&self, block_size: usize) {
        self.fixture_manager
            .get_programmer()
            .set_block_size(block_size);
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn update_groups(&self, groups: usize) {
        self.fixture_manager.get_programmer().set_groups(groups);
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn update_wings(&self, wings: usize) {
        self.fixture_manager.get_programmer().set_wings(wings);
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn next(&self) {
        self.fixture_manager.get_programmer().next();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn prev(&self) {
        self.fixture_manager.get_programmer().prev();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn set(&self) {
        self.fixture_manager.get_programmer().set();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn shuffle(&self) {
        self.fixture_manager.get_programmer().shuffle();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn mark_offline(&self) {
        self.fixture_manager.get_programmer().set_offline(true);
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn mark_online(&self) {
        self.fixture_manager.get_programmer().set_offline(false);
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn write_effect_rate(&self, request: WriteEffectRateRequest) {
        self.fixture_manager
            .get_programmer()
            .write_rate(request.effect_id, request.effect_rate);
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn write_effect_offset(&self, request: WriteEffectOffsetRequest) {
        self.fixture_manager
            .get_programmer()
            .write_offset(request.effect_id, request.effect_offset);
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn add_preset(
        &self,
        preset_type: preset_id::PresetType,
        name: Option<String>,
    ) -> anyhow::Result<()> {
        self.runtime.run_command(AddPresetCommand {
            preset_type: preset_type.into(),
            name,
            values: self.get_preset_values(preset_type.into()),
        })?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn store_programmer_to_preset(&self, preset_id: PresetId) -> anyhow::Result<()> {
        let value = self.get_preset_values(preset_id.r#type().into());

        self.runtime.run_command(StoreInPresetCommand {
            id: preset_id.into(),
            values: value,
        })?;

        Ok(())
    }

    fn get_preset_values(
        &self,
        preset_type: mizer_fixtures::programmer::PresetType,
    ) -> Vec<FixtureControlValue> {
        self.fixture_manager
            .get_programmer()
            .get_channels()
            .into_iter()
            .map(|control| control.value)
            .filter(|value| preset_type.contains_control(value))
            .collect()
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn delete_preset(&self, preset_id: PresetId) {
        self.runtime
            .run_command(DeletePresetCommand {
                id: preset_id.into(),
            })
            .unwrap();
    }

    #[tracing::instrument(skip(self))]
    #[profiling::function]
    pub fn rename_preset(&self, preset_id: PresetId, label: String) {
        self.runtime
            .run_command(RenamePresetCommand {
                id: preset_id.into(),
                label,
            })
            .unwrap();
    }
}
