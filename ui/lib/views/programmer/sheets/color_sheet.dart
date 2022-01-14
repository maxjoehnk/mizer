import 'package:collection/collection.dart';
import 'package:flutter/widgets.dart';
import 'package:mizer/api/contracts/programmer.dart';
import 'package:mizer/protos/fixtures.extensions.dart';
import 'package:mizer/protos/fixtures.pb.dart';
import 'package:mizer/widgets/inputs/color.dart';

import 'fixture_group_control.dart';

const CONTROLS = [
  FixtureControl.COLOR,
  FixtureControl.FOCUS,
  FixtureControl.PRISM,
  FixtureControl.FROST,
  FixtureControl.IRIS,
];

const NAMES = {
  FixtureControl.ZOOM: "Zoom",
  FixtureControl.FOCUS: "Focus",
  FixtureControl.PRISM: "Prism",
  FixtureControl.FROST: "Frost",
  FixtureControl.IRIS: "Iris",
};

class ColorSheet extends StatelessWidget {
  final List<FixtureInstance> fixtures;
  final List<ProgrammerChannel> channels;

  const ColorSheet({required this.fixtures, required this.channels, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      child: controls.isNotEmpty
          ? ListView(
              scrollDirection: Axis.horizontal,
              children:
                  controls.map((control) => FixtureGroupControl(control)).toList())
          : null,
    );
  }

  Iterable<Control> get controls {
    if (fixtures.isEmpty) {
      return [];
    }
    return fixtures.first.controls
        .where((control) => control.control == FixtureControl.COLOR)
        .map((control) => Control("Color",
            color: control.color,
            channel: channels.firstWhereOrNull((channel) => channel.control == control.control),
            update: (v) {
              ColorValue value = v;
              return WriteControlRequest(
                  control: control.control,
                  color: ColorChannel(red: value.red, green: value.green, blue: value.blue),
                );
            }));
  }
}
