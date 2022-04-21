import 'dart:developer';

import 'package:fixnum/fixnum.dart';
import 'package:flutter/material.dart' hide MenuItem;
import 'package:flutter/scheduler.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mizer/api/contracts/sequencer.dart';
import 'package:mizer/api/plugin/ffi/sequencer.dart';
import 'package:mizer/platform/platform.dart';
import 'package:mizer/protos/layouts.pb.dart';
import 'package:mizer/protos/nodes.pb.dart';
import 'package:mizer/settings/hotkeys/hotkey_provider.dart';
import 'package:mizer/state/layouts_bloc.dart';
import 'package:mizer/state/nodes_bloc.dart';
import 'package:mizer/widgets/panel.dart';
import 'package:mizer/widgets/platform/context_menu.dart';
import 'package:mizer/widgets/popup/popup_route.dart';
import 'package:mizer/widgets/tabs.dart' as tabs;

import 'add_control_popup.dart';
import 'control.dart';

const double MULTIPLIER = 75;

class LayoutView extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    var layoutsBloc = context.read<LayoutsBloc>();
    layoutsBloc.add(FetchLayouts());
    return BlocBuilder<LayoutsBloc, LayoutState>(builder: (context, state) {
      log("${state.layouts}", name: "LayoutView");
      context.read<NodesBloc>().add(FetchNodes());
      return HotkeyProvider(
        hotkeySelector: (hotkeys) => hotkeys.layouts,
        hotkeyMap: {
          "add_layout": () => _addLayout(context, layoutsBloc),
        },
        child: Panel.tabs(
          label: "Layout",
          tabIndex: state.tabIndex,
          onSelectTab: (index) => layoutsBloc.add(SelectLayoutTab(tabIndex: index)),
          padding: false,
          tabs: state.layouts
              .map((layout) => tabs.Tab(
                  header: (active, setActive) => ContextMenu(
                      menu: Menu(items: [
                        MenuItem(
                            label: "Rename", action: () => _onRename(context, layout, layoutsBloc)),
                        MenuItem(
                            label: "Delete", action: () => _onDelete(context, layout, layoutsBloc)),
                      ]),
                      child: tabs.TabHeader(layout.id, selected: active, onSelect: setActive)),
                  child: SequencerStateFetcher(builder: (context, sequencerStates) {
                    return ControlLayout(
                      layout: layout,
                      sequencerState: sequencerStates,
                    );
                  })))
              .toList(),
          onAdd: () => _addLayout(context, layoutsBloc),
        ),
      );
    });
  }

  Future<void> _addLayout(BuildContext context, LayoutsBloc layoutsBloc) {
    return showDialog(
      context: context,
      useRootNavigator: false,
      builder: (_) => NameLayoutDialog(),
    ).then((name) => layoutsBloc.add(AddLayout(name: name)));
  }

  void _onDelete(BuildContext context, Layout layout, LayoutsBloc bloc) async {
    bool result = await showDialog(
        context: context,
        builder: (BuildContext context) => AlertDialog(
              title: Text("Delete Layout"),
              content: SingleChildScrollView(
                child: Text("Delete Layout ${layout.id}?"),
              ),
              actions: [
                TextButton(
                  child: Text("Cancel"),
                  onPressed: () => Navigator.of(context).pop(false),
                ),
                TextButton(
                  autofocus: true,
                  child: Text("Delete"),
                  onPressed: () => Navigator.of(context).pop(true),
                ),
              ],
            ));
    if (result) {
      bloc.add(RemoveLayout(id: layout.id));
    }
  }

  void _onRename(BuildContext context, Layout layout, LayoutsBloc bloc) async {
    String? result =
        await showDialog(context: context, builder: (context) => NameLayoutDialog(name: layout.id));
    if (result != null) {
      bloc.add(RenameLayout(id: layout.id, name: result));
    }
  }
}

class NameLayoutDialog extends StatelessWidget {
  final String? name;
  final TextEditingController nameController;

  NameLayoutDialog({this.name, Key? key})
      : nameController = TextEditingController(text: name),
        super(key: key);

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
        title: Text(name != null ? "Rename Layout" : "Add Layout"),
        actions: [
          ElevatedButton(
              onPressed: () => Navigator.of(context).pop(nameController.text),
              child: Text(name != null ? "Rename" : "Add"))
        ],
        content: TextField(
          controller: nameController,
          autofocus: true,
          decoration: InputDecoration(labelText: "Name"),
          onSubmitted: (text) => Navigator.of(context).pop(text),
        ));
  }
}

class ControlLayout extends StatelessWidget {
  final Layout layout;
  final Map<int, SequenceState> sequencerState;

  ControlLayout({required this.layout, required this.sequencerState});

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<NodesBloc, Nodes>(
        builder: (context, nodes) => Container(
              width: 20 * MULTIPLIER,
              height: 10 * MULTIPLIER,
              child: Stack(
                children: [
                  GestureDetector(
                    onSecondaryTapDown: (details) {
                      LayoutsBloc bloc = context.read();
                      int x = (details.localPosition.dx / MULTIPLIER).floor();
                      int y = (details.localPosition.dy / MULTIPLIER).floor();
                      var position = ControlPosition(x: Int64(x), y: Int64(y));
                      Navigator.of(context).push(MizerPopupRoute(
                          position: details.globalPosition,
                          child: AddControlPopup(
                              nodes: nodes,
                              onCreateControl: (nodeType) => bloc.add(AddControl(
                                  layoutId: layout.id, nodeType: nodeType, position: position)),
                              onAddControlForExisting: (node) => bloc.add(AddExistingControl(
                                  layoutId: layout.id, node: node, position: position)))));
                    },
                  ),
                  CustomMultiChildLayout(
                      delegate: ControlsLayoutDelegate(layout),
                      children: layout.controls
                          .map((e) => LayoutId(
                              id: e.node, child: LayoutControlView(layout.id, e, sequencerState)))
                          .toList()),
                ],
              ),
            ));
  }
}

class ControlsLayoutDelegate extends MultiChildLayoutDelegate {
  final Layout layout;

  ControlsLayoutDelegate(this.layout);

  @override
  void performLayout(Size size) {
    for (var control in layout.controls) {
      var controlSize =
          Size(control.size.width.toDouble(), control.size.height.toDouble()) * MULTIPLIER;
      layoutChild(control.node, BoxConstraints.tight(controlSize));
      var controlOffset =
          Offset(control.position.x.toDouble(), control.position.y.toDouble()) * MULTIPLIER;
      positionChild(control.node, controlOffset);
    }
  }

  @override
  bool shouldRelayout(covariant MultiChildLayoutDelegate oldDelegate) {
    return false;
  }
}

class SequencerStateFetcher extends StatefulWidget {
  final Widget Function(BuildContext, Map<int, SequenceState>) builder;

  const SequencerStateFetcher({required this.builder, Key? key}) : super(key: key);

  @override
  _SequencerStateFetcherState createState() => _SequencerStateFetcherState();
}

class _SequencerStateFetcherState extends State<SequencerStateFetcher>
    with SingleTickerProviderStateMixin {
  SequencerPointer? _pointer;
  Map<int, SequenceState> sequenceStates = {};
  Ticker? ticker;

  @override
  void initState() {
    super.initState();
    var sequencerApi = context.read<SequencerApi>();
    sequencerApi.getSequencerPointer().then((pointer) => setState(() {
          _pointer = pointer;
          ticker = this.createTicker((elapsed) {
            setState(() {
              sequenceStates = _pointer!.readState();
            });
          });
          ticker!.start();
        }));
  }

  @override
  void dispose() {
    _pointer?.dispose();
    ticker?.stop(canceled: true);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return widget.builder(context, sequenceStates);
  }
}
