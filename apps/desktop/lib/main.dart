import 'package:desktop/app/app.dart';
import 'package:desktop/core/assets/aseprite_assets.dart';
import 'package:flutter/material.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await SpriteAssets.preload();
  runApp(const PixRootApp());
}
