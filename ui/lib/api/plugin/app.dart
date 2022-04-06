import 'package:flutter/services.dart';

import '../contracts/settings.dart';

class ApplicationPluginApi implements SettingsApi {
  final MethodChannel channel = const MethodChannel("mizer.live/application");

  Future<void> exit() async {
    await channel.invokeMethod("exit");
  }

  @override
  Future<Settings> loadSettings() async {
    var settings = await channel.invokeMethod("loadSettings");

    return Settings.fromBuffer(_convertBuffer(settings));
  }

  @override
  Future<void> saveSettings(Settings settings) async {
    await channel.invokeMethod("saveSettings", settings.writeToBuffer());
  }

  static List<int> _convertBuffer(List<Object> response) {
    return response.map((dynamic e) => e as int).toList();
  }
}
