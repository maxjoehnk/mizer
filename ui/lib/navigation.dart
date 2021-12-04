import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';
import 'package:mizer/menu.dart';
import 'package:mizer/settings/hotkeys/hotkey_provider.dart';
import 'package:mizer/state/settings_bloc.dart';
import 'package:mizer/views/connections/connections_view.dart';
import 'package:mizer/views/programmer/programmer_view.dart';
import 'package:mizer/views/layout/layout_view.dart';
import 'package:mizer/views/media/media_view.dart';
import 'package:mizer/views/nodes/nodes_view.dart';
import 'package:mizer/views/patch/fixture_patch.dart';
import 'package:mizer/widgets/transport/transport_controls.dart';
import 'package:mizer/views/sequencer/sequencer_view.dart';
import 'package:mizer/views/session/session_view.dart';

import 'actions/actions.dart';
import 'api/contracts/settings.dart';

List<Route> routes = [
  Route(() => LayoutView(), Icons.view_quilt_outlined, 'Layout', LogicalKeyboardKey.f1,
      View.Layout),
  Route(
      () => Container(), Icons.view_comfortable, '2D Plan', LogicalKeyboardKey.f2, View.Plan),
  Route(() => Container(), MdiIcons.video3D, 'PreViz', LogicalKeyboardKey.f3, View.PreViz),
  Route(() => FetchNodesView(), Icons.account_tree_outlined, 'Nodes', LogicalKeyboardKey.f4,
      View.Nodes),
  Route(() => SequencerView(), MdiIcons.animationPlayOutline, 'Sequencer', LogicalKeyboardKey.f5,
      View.Sequences),
  Route(() => ProgrammerView(), MdiIcons.tuneVertical, 'Programmer', LogicalKeyboardKey.f6,
      View.Programmer),
  Route(
      () => MediaView(), Icons.perm_media_outlined, 'Media', LogicalKeyboardKey.f7, View.Media),
  Route(() => ConnectionsView(), Icons.device_hub, 'Connections', LogicalKeyboardKey.f8,
      View.Connections),
  Route(() => FixturePatchView(), MdiIcons.spotlight, 'Patch', LogicalKeyboardKey.f9,
      View.FixturePatch),
  Route(() => SessionView(), Icons.mediation, 'Session', LogicalKeyboardKey.f10, View.Session),
];

Map<String, OpenViewIntent> shortcuts = getShortcuts(routes);

Map<String, OpenViewIntent> getShortcuts(List<Route> routes) {
  Map<String, OpenViewIntent> shortcuts = {};
  for (var route in routes) {
    shortcuts[route.viewKey.toHotkeyString()] = OpenViewIntent(route.viewKey);
  }
  return shortcuts;
}

class Home extends StatefulWidget {
  @override
  _HomeState createState() => _HomeState();
}

class _HomeState extends State<Home> {
  int _selectedIndex = 0;
  Widget? _currentWidget;

  _HomeState() {
    _updateWidget();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: BlocBuilder<SettingsBloc, Settings>(
          builder: (context, settings) => HotkeyProvider(
            hotkeySelector: (hotkeys) => hotkeys.global,
            global: true,
            onHotkey: (hotkey) {
              var intent = shortcuts[hotkey]!;
              this._selectView(intent.view.index);
            },
            child: ApplicationMenu(
                child: Actions(
                  actions: <Type, CallbackAction>{
                    OpenViewIntent: CallbackAction<OpenViewIntent>(
                      onInvoke: (intent) => this._selectView(intent.view.index),
                    ),
                  },
                  child: Focus(
                    autofocus: true,
                    child: Row(
                      children: [
                        NavigationBar(
                          selectedIndex: _selectedIndex,
                          onSelect: this._selectView,
                          routes: routes,
                        ),
                        Expanded(
                            child: Column(
                              children: [
                                Expanded(
                                  child: Container(
                                      child: _currentWidget,
                                      clipBehavior: Clip.antiAlias,
                                      decoration: BoxDecoration()),
                                ),
                                TransportControls()
                              ],
                            ))
                      ],
                      crossAxisAlignment: CrossAxisAlignment.start,
                    ),
                  ),
                ),
            ),
          ),
        ));
  }

  void _updateWidget() {
    _currentWidget = routes[_selectedIndex].view();
  }

  void _selectView(int index) {
    setState(() {
      _selectedIndex = index;
      _updateWidget();
    });
  }
}

class Route {
  final WidgetFunction view;
  final IconData icon;
  final String label;
  final LogicalKeyboardKey key;
  final View viewKey;

  Route(this.view, this.icon, this.label, this.key, this.viewKey);
}

typedef WidgetFunction = Widget Function();

class NavigationBar extends StatelessWidget {
  final List<Route> routes;
  final int selectedIndex;
  final Function(int) onSelect;

  NavigationBar({required this.routes, required this.selectedIndex, required this.onSelect});

  @override
  Widget build(BuildContext context) {
    return Container(
        color: Colors.grey.shade800,
        width: 64,
        child: ListView(
            children: this
                .routes
                .mapEnumerated((route, i) =>
                    NavigationItem(route, this.selectedIndex == i, () => this.onSelect(i)))
                .toList()));
  }
}

extension MapWithIndex<T> on List<T> {
  List<R> mapEnumerated<R>(R Function(T, int i) callback) {
    return this.asMap().map((key, value) => MapEntry(key, callback(value, key))).values.toList();
  }
}

class NavigationItem extends StatefulWidget {
  final Route route;
  final bool selected;
  final void Function() onSelect;

  NavigationItem(this.route, this.selected, this.onSelect);

  @override
  _NavigationItemState createState() => _NavigationItemState();
}

class _NavigationItemState extends State<NavigationItem> {
  bool hovering = false;

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context);
    var textTheme = theme.textTheme;
    var color = this.widget.selected ? theme.accentColor : theme.hintColor;

    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onExit: (e) => setState(() => hovering = false),
      onHover: (e) => setState(() => hovering = true),
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        onTap: this.widget.onSelect,
        child: Container(
          height: 64,
          color: backgroundColor,
          padding: const EdgeInsets.symmetric(vertical: 8),
          child: Stack(
            children: [
              Center(
                child: Column(
                  children: [
                    Icon(
                      this.widget.route.icon,
                      color: color,
                      size: 24,
                    ),
                    Text(this.widget.route.label,
                        style: textTheme.subtitle2!.copyWith(color: color, fontSize: 10)),
                  ],
                  crossAxisAlignment: CrossAxisAlignment.center,
                  mainAxisAlignment: MainAxisAlignment.center,
                ),
              ),
              Align(alignment: Alignment.topRight, child: Text(widget.route.key.keyLabel, style: textTheme.caption!.copyWith(fontSize: 9))),
            ],
          ),
        ),
      ),
    );
  }

  Color? get backgroundColor {
    if (widget.selected) {
      return Colors.black26;
    }
    if (hovering) {
      return Colors.black12;
    }
    return null;
  }
}
