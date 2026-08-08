import 'dart:io';

import 'package:desktop/app/app.dart';
import 'package:desktop/core/assets/aseprite_assets.dart';
import 'package:desktop/services/socket/pixroot_sockets.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:path/path.dart' as p;

Directory? findPixRoot(Directory start) {
  Directory current = start;

  while (true) {
    if (p.basename(current.path) == 'PixRoot') {
      return current;
    }

    final parent = current.parent;

    if (parent.path == current.path) {
      return null;
    }

    current = parent;
  }
}

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await SpriteAssets.preload();

  final String rustBinaryName = 'database';
  final String rustProfile = kReleaseMode ? 'release' : 'debug';

  // In an AppImage both executables sit next to each other (usr/bin).
  // In development the daemon lives in the repo instead.
  final executableDir = Directory(p.dirname(Platform.resolvedExecutable));
  var daemonPath = p.join(executableDir.path, rustBinaryName);
  if (!File(daemonPath).existsSync()) {
    final repoRoot = findPixRoot(executableDir)!;
    daemonPath = p.join(
      repoRoot.path,
      'services',
      'daemon',
      'target',
      rustProfile,
      rustBinaryName,
    );
  }

  // The daemon never exits on its own, so an instance left over from an
  // earlier run (debug restarts, closed window) keeps holding the socket and
  // stops answering requests. Kill it so the fresh spawn binds the socket.
  await Process.run('pkill', ['-f', daemonPath]);

  await Process.start(daemonPath, []);

  final socket = PixrootSockets();
  // The daemon binds its Unix socket shortly after spawn; retry briefly so
  // we don't race it.
  for (var attempt = 0; ; attempt++) {
    try {
      await socket.connect();
      break;
    } catch (_) {
      if (attempt >= 24) rethrow;
      await Future<void>.delayed(const Duration(milliseconds: 200));
    }
  }

  final socketManager = SocketManager(client: socket);

  runApp(
    MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) => AppState()),
        Provider(create: (_) => socketManager),
      ],
      child: const PixRootApp(),
    ),
  );
}
