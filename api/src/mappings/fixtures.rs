use protobuf::SingularPtrField;

use mizer_fixtures::definition::{self, ChannelResolution, PhysicalFixtureData};
use mizer_fixtures::fixture::IFixture;

use crate::models;
use crate::models::fixtures::*;

impl From<mizer_fixtures::definition::FixtureDefinition> for FixtureDefinition {
    fn from(definition: mizer_fixtures::definition::FixtureDefinition) -> Self {
        let physical: FixturePhysicalData = definition.physical.into();
        FixtureDefinition {
            id: definition.id,
            name: definition.name,
            manufacturer: definition.manufacturer,
            modes: definition
                .modes
                .into_iter()
                .map(FixtureMode::from)
                .collect::<Vec<_>>()
                .into(),
            tags: definition.tags.into(),
            physical: SingularPtrField::some(physical),
            provider: definition.provider.to_string(),
            ..Default::default()
        }
    }
}

impl From<mizer_fixtures::definition::PhysicalFixtureData> for FixturePhysicalData {
    fn from(physical_data: PhysicalFixtureData) -> Self {
        let mut data = FixturePhysicalData::new();
        if let Some(dimensions) = physical_data.dimensions {
            data.width = dimensions.width;
            data.height = dimensions.height;
            data.depth = dimensions.depth;
        }
        if let Some(weight) = physical_data.weight {
            data.weight = weight;
        }

        data
    }
}

impl From<mizer_fixtures::definition::FixtureMode> for FixtureMode {
    fn from(mode: mizer_fixtures::definition::FixtureMode) -> Self {
        FixtureMode {
            name: mode.name,
            channels: mode
                .channels
                .into_iter()
                .map(FixtureChannel::from)
                .collect::<Vec<_>>()
                .into(),
            ..Default::default()
        }
    }
}

impl From<mizer_fixtures::definition::FixtureChannelDefinition> for FixtureChannel {
    fn from(channel: mizer_fixtures::definition::FixtureChannelDefinition) -> Self {
        FixtureChannel {
            name: channel.name,
            resolution: Some(channel.resolution.into()),
            ..Default::default()
        }
    }
}

impl From<mizer_fixtures::definition::ChannelResolution> for FixtureChannel_oneof_resolution {
    fn from(resolution: mizer_fixtures::definition::ChannelResolution) -> Self {
        match resolution {
            ChannelResolution::Coarse(coarse) => {
                FixtureChannel_oneof_resolution::coarse(FixtureChannel_CoarseResolution {
                    channel: coarse.into(),
                    ..Default::default()
                })
            }
            ChannelResolution::Fine(fine, coarse) => {
                FixtureChannel_oneof_resolution::fine(FixtureChannel_FineResolution {
                    fineChannel: fine.into(),
                    coarseChannel: coarse.into(),
                    ..Default::default()
                })
            }
            ChannelResolution::Finest(finest, fine, coarse) => {
                FixtureChannel_oneof_resolution::finest(FixtureChannel_FinestResolution {
                    finestChannel: finest.into(),
                    fineChannel: fine.into(),
                    coarseChannel: coarse.into(),
                    ..Default::default()
                })
            }
        }
    }
}

impl FixtureControls {
    fn fader(control: FixtureControl, value: Option<f64>) -> Self {
        FixtureControls {
            control,
            value: value.map(|value| {
                FixtureControls_oneof_value::fader(FaderChannel {
                    value,
                    ..Default::default()
                })
            }),
            ..Default::default()
        }
    }

    pub fn with_values<TChannel>(
        fixture: &impl IFixture,
        fixture_controls: mizer_fixtures::definition::FixtureControls<TChannel>,
    ) -> Vec<Self> {
        let mut controls = Vec::new();
        if let Some(_) = fixture_controls.intensity {
            let value =
                fixture.read_control(mizer_fixtures::definition::FixtureFaderControl::Intensity);
            controls.push(FixtureControls::fader(FixtureControl::INTENSITY, value));
        }
        if let Some(_) = fixture_controls.shutter {
            let value =
                fixture.read_control(mizer_fixtures::definition::FixtureFaderControl::Shutter);
            controls.push(FixtureControls::fader(FixtureControl::SHUTTER, value));
        }
        if let Some(_) = fixture_controls.iris {
            let value = fixture.read_control(mizer_fixtures::definition::FixtureFaderControl::Iris);
            controls.push(FixtureControls::fader(FixtureControl::IRIS, value));
        }
        if let Some(_) = fixture_controls.zoom {
            let value = fixture.read_control(mizer_fixtures::definition::FixtureFaderControl::Zoom);
            controls.push(FixtureControls::fader(FixtureControl::ZOOM, value));
        }
        if let Some(_) = fixture_controls.frost {
            let value =
                fixture.read_control(mizer_fixtures::definition::FixtureFaderControl::Frost);
            controls.push(FixtureControls::fader(FixtureControl::FROST, value));
        }
        if let Some(_) = fixture_controls.prism {
            let value =
                fixture.read_control(mizer_fixtures::definition::FixtureFaderControl::Prism);
            controls.push(FixtureControls::fader(FixtureControl::PRISM, value));
        }
        if let Some(_) = fixture_controls.focus {
            let value =
                fixture.read_control(mizer_fixtures::definition::FixtureFaderControl::Focus);
            controls.push(FixtureControls::fader(FixtureControl::FOCUS, value));
        }
        if let Some(channel) = fixture_controls.pan {
            let value = fixture.read_control(mizer_fixtures::definition::FixtureFaderControl::Pan);
            controls.push(FixtureControls {
                control: FixtureControl::PAN,
                value: Some(FixtureControls_oneof_value::axis(AxisChannel::with_value(
                    &channel, value,
                ))),
                ..Default::default()
            });
        }
        if let Some(channel) = fixture_controls.tilt {
            let value = fixture.read_control(mizer_fixtures::definition::FixtureFaderControl::Pan);
            controls.push(FixtureControls {
                control: FixtureControl::TILT,
                value: Some(FixtureControls_oneof_value::axis(AxisChannel::with_value(
                    &channel, value,
                ))),
                ..Default::default()
            });
        }
        if let Some(_) = fixture_controls.color_mixer {
            controls.push(FixtureControls {
                control: FixtureControl::COLOR_MIXER,
                value: Some(FixtureControls_oneof_value::color_mixer(
                    ColorMixerChannel {
                        red: fixture
                            .read_control(
                                mizer_fixtures::definition::FixtureFaderControl::ColorMixer(
                                    mizer_fixtures::definition::ColorChannel::Red,
                                ),
                            )
                            .unwrap_or_default(),
                        green: fixture
                            .read_control(
                                mizer_fixtures::definition::FixtureFaderControl::ColorMixer(
                                    mizer_fixtures::definition::ColorChannel::Red,
                                ),
                            )
                            .unwrap_or_default(),
                        blue: fixture
                            .read_control(
                                mizer_fixtures::definition::FixtureFaderControl::ColorMixer(
                                    mizer_fixtures::definition::ColorChannel::Red,
                                ),
                            )
                            .unwrap_or_default(),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            })
        }
        if let Some(color_wheel) = fixture_controls.color_wheel {
            controls.push(FixtureControls {
                control: FixtureControl::COLOR_WHEEL,
                value: Some(FixtureControls_oneof_value::color_wheel(
                    ColorWheelChannel {
                        colors: color_wheel
                            .colors
                            .into_iter()
                            .map(ColorWheelSlot::from)
                            .collect(),
                        value: fixture
                            .read_control(
                                mizer_fixtures::definition::FixtureFaderControl::ColorWheel,
                            )
                            .unwrap_or_default(),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            })
        }
        if let Some(gobo) = fixture_controls.gobo {
            controls.push(FixtureControls {
                control: FixtureControl::GOBO,
                value: Some(FixtureControls_oneof_value::gobo(GoboChannel {
                    gobos: gobo.gobos.into_iter().map(Gobo::from).collect(),
                    value: fixture
                        .read_control(mizer_fixtures::definition::FixtureFaderControl::Gobo)
                        .unwrap_or_default(),
                    ..Default::default()
                })),
                ..Default::default()
            })
        }
        for channel in fixture_controls.generic {
            let value = fixture.read_control(
                mizer_fixtures::definition::FixtureFaderControl::Generic(channel.label.clone()),
            );
            controls.push(FixtureControls {
                control: FixtureControl::GENERIC,
                value: value.map(|value| {
                    FixtureControls_oneof_value::generic(GenericChannel {
                        name: channel.label,
                        value,
                        ..Default::default()
                    })
                }),
                ..Default::default()
            });
        }

        controls
    }
}

impl From<mizer_fixtures::definition::ColorWheelSlot> for ColorWheelSlot {
    fn from(color: mizer_fixtures::definition::ColorWheelSlot) -> Self {
        Self {
            name: color.name,
            value: color.value,
            colors: color.color.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl From<mizer_fixtures::definition::Gobo> for Gobo {
    fn from(gobo: mizer_fixtures::definition::Gobo) -> Self {
        Self {
            name: gobo.name,
            value: gobo.value,
            image: gobo.image.map(Gobo_oneof_image::from),
            ..Default::default()
        }
    }
}

impl From<mizer_fixtures::definition::GoboImage> for Gobo_oneof_image {
    fn from(image: mizer_fixtures::definition::GoboImage) -> Self {
        use mizer_fixtures::definition::GoboImage::*;
        match image {
            Raster(bytes) => Self::raster(*bytes),
            Svg(svg) => Self::svg(svg),
        }
    }
}

impl From<f64> for GenericChannel {
    fn from(value: f64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl AxisChannel {
    fn with_value<TChannel>(
        axis: &mizer_fixtures::definition::AxisGroup<TChannel>,
        value: Option<f64>,
    ) -> Self {
        AxisChannel {
            value: value.unwrap_or_default(),
            angle_from: axis
                .angle
                .map(|angle| angle.from as f64)
                .unwrap_or_default(),
            angle_to: axis.angle.map(|angle| angle.to as f64).unwrap_or_default(),
            ..Default::default()
        }
    }
}

impl From<mizer_fixtures::FixtureId> for FixtureId {
    fn from(id: mizer_fixtures::FixtureId) -> Self {
        match id {
            mizer_fixtures::FixtureId::Fixture(fixture_id) => Self {
                id: Some(FixtureId_oneof_id::fixture(fixture_id)),
                ..Default::default()
            },
            mizer_fixtures::FixtureId::SubFixture(fixture_id, child_id) => Self {
                id: Some(FixtureId_oneof_id::sub_fixture(SubFixtureId {
                    fixture_id,
                    child_id,
                    ..Default::default()
                })),
                ..Default::default()
            },
        }
    }
}

impl From<FixtureId> for mizer_fixtures::FixtureId {
    fn from(id: FixtureId) -> Self {
        match id.id {
            Some(FixtureId_oneof_id::fixture(fixture_id)) => Self::Fixture(fixture_id),
            Some(FixtureId_oneof_id::sub_fixture(SubFixtureId {
                fixture_id,
                child_id,
                ..
            })) => Self::SubFixture(fixture_id, child_id),
            _ => unreachable!(),
        }
    }
}

impl From<definition::FixtureFaderControl> for models::FixtureControl {
    fn from(control: definition::FixtureFaderControl) -> Self {
        use definition::FixtureFaderControl::*;

        match control {
            Intensity => Self::INTENSITY,
            Shutter => Self::SHUTTER,
            Pan => Self::PAN,
            Tilt => Self::TILT,
            Prism => Self::PRISM,
            Iris => Self::IRIS,
            Frost => Self::FROST,
            ColorMixer(_) => Self::COLOR_MIXER,
            ColorWheel => Self::COLOR_WHEEL,
            Generic(_) => Self::GENERIC,
            Zoom => Self::ZOOM,
            Focus => Self::FOCUS,
            Gobo => Self::GOBO,
        }
    }
}
