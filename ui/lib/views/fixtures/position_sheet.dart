import 'package:flutter/widgets.dart';
import 'package:mizer/protos/fixtures.pb.dart';

import 'fixture_group_control.dart';

class PositionSheet extends StatelessWidget {
  final List<Fixture> fixtures;

  const PositionSheet({this.fixtures, Key key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      child: groups.isNotEmpty
          ? ListView(
              scrollDirection: Axis.horizontal,
              children: groups
                  .map((group) => FixtureGroupControl(group, fixtures: fixtures))
                  .toList())
          : null,
    );
  }

  Iterable<FixtureChannelGroup> get groups {
    return fixtures.first.channels.where((element) => element.hasPan() || element.hasTilt());
  }
}
