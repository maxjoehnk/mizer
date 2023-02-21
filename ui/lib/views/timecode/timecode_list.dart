import 'package:auto_size_text/auto_size_text.dart';
import 'package:flutter/material.dart';
import 'package:mizer/protos/timecode.pb.dart';
import 'package:mizer/state/timecode_bloc.dart';
import 'package:mizer/widgets/hoverable.dart';
import 'package:mizer/widgets/panel.dart';
import 'package:mizer/widgets/popup/popup_input.dart';
import 'package:mizer/widgets/popup/popup_route.dart';
import 'package:provider/provider.dart';

import 'dialogs/add_timecode_dialog.dart';

class TimecodeList extends StatelessWidget {
  final List<Timecode> timecodes;
  final Function(Timecode) onSelect;
  final Timecode? selectedTimecode;

  TimecodeList({required this.timecodes, required this.onSelect, this.selectedTimecode, Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Panel(
        label: "Timecodes",
        actions: [
          PanelAction(label: "Add Timecode", onClick: () => _addTimecode(context)),
          PanelAction(
              label: "Delete",
              disabled: selectedTimecode == null,
              onClick: () => _deleteTimecode(context))
        ],
        child: GridView(
            padding: const EdgeInsets.all(4),
            gridDelegate: SliverGridDelegateWithMaxCrossAxisExtent(
              maxCrossAxisExtent: 100,
              mainAxisExtent: 100,
              crossAxisSpacing: 4,
              mainAxisSpacing: 4,
              childAspectRatio: 1,
            ),
            children: timecodes
                .map((t) => TimecodePane(
                timecode: t,
                selected: t.id == this.selectedTimecode?.id,
                onSelect: () => onSelect(t)))
                .toList()));
  }

  Future<void> _addTimecode(BuildContext context) async {
    String? name =
    await showDialog(context: context, builder: (context) => new AddTimecodeDialog());
    if (name == null) {
      return;
    }
    TimecodeBloc bloc = context.read();
    bloc.addTimecode(name);
  }

  void _deleteTimecode(BuildContext context) {
    TimecodeBloc bloc = context.read();
    bloc.deleteTimecode(selectedTimecode!.id);
  }
}

class TimecodePane extends StatelessWidget {
  final Timecode timecode;
  final bool selected;
  final Function() onSelect;

  TimecodePane({required this.timecode, required this.selected, required this.onSelect, Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onSecondaryTapDown: (details) => Navigator.of(context).push(MizerPopupRoute(
          position: details.globalPosition,
          child: PopupInput(
            title: "Name",
            value: timecode.name,
            onChange: (name) => _renameTimecode(context, name),
          ))),
      child: Hoverable(
          onTap: onSelect,
          builder: (hovered) => Container(
            width: 100,
            height: 100,
            padding: const EdgeInsets.all(2),
            clipBehavior: Clip.antiAlias,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(2),
              color: hovered ? Colors.white10 : Colors.transparent,
              border:
              Border.all(color: selected ? Colors.deepOrange : Colors.white10, width: 4),
            ),
            child: Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
              Text(timecode.id.toString(), textAlign: TextAlign.start),
              AutoSizeText(timecode.name, textAlign: TextAlign.center, maxLines: 2),
            ]),
          )),
    );
  }

  void _renameTimecode(BuildContext context, String name) {
    TimecodeBloc bloc = context.read();
    bloc.renameTimecode(timecode.id, name);
  }
}

