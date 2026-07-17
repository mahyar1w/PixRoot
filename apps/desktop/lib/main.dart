import 'dart:io';

import 'package:desktop/app/app.dart';
import 'package:desktop/core/assets/aseprite_assets.dart';
import 'package:desktop/services/socket/pixroot_sockets.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await SpriteAssets.preload();
  await Process.start(
    "/home/hashira/Computer-science/PixRoot/services/daemon/target/debug/database",
  );

  final socket = PixrootSockets();
  await socket.connect();
  runApp(
    ChangeNotifierProvider(
      create: (_) => AppState(),
      child: const PixRootApp(),
    ),
  );
}
