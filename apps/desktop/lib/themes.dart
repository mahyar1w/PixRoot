import 'package:flutter/material.dart';

/// A named, ready-to-use [ThemeData] pair for one color family.
class AppTheme {
  const AppTheme(this.name, this.light, this.dark);

  final String name;
  final ThemeData light;
  final ThemeData dark;
}

ThemeData _build(Color seed, Brightness brightness) => ThemeData(
      useMaterial3: true,
      colorScheme: ColorScheme.fromSeed(seedColor: seed, brightness: brightness),
    );

/// 16 popular themes: 8 color families x light/dark.
/// Drop into MaterialApp via `theme: t.light, darkTheme: t.dark`.
final List<AppTheme> appThemes = [
  AppTheme('Blue', _build(Colors.blue, Brightness.light), _build(Colors.blue, Brightness.dark)),
  AppTheme('Purple', _build(Colors.purple, Brightness.light), _build(Colors.purple, Brightness.dark)),
  AppTheme('Teal', _build(Colors.teal, Brightness.light), _build(Colors.teal, Brightness.dark)),
  AppTheme('Pink', _build(Colors.pink, Brightness.light), _build(Colors.pink, Brightness.dark)),
  AppTheme('Indigo', _build(Colors.indigo, Brightness.light), _build(Colors.indigo, Brightness.dark)),
  AppTheme('Deep Purple', _build(Colors.deepPurple, Brightness.light), _build(Colors.deepPurple, Brightness.dark)),
  AppTheme('Cyan', _build(Colors.cyan, Brightness.light), _build(Colors.cyan, Brightness.dark)),
  AppTheme('Orange', _build(Colors.orange, Brightness.light), _build(Colors.orange, Brightness.dark)),
  AppTheme('Amber', _build(Colors.amber, Brightness.light), _build(Colors.amber, Brightness.dark)),
  AppTheme('Brown', _build(Colors.brown, Brightness.light), _build(Colors.brown, Brightness.dark)),
  AppTheme('Blue Grey', _build(Colors.blueGrey, Brightness.light), _build(Colors.blueGrey, Brightness.dark)),
  AppTheme('Lime', _build(Colors.lime, Brightness.light), _build(Colors.lime, Brightness.dark)),
  AppTheme('Green', _build(Colors.green, Brightness.light), _build(Colors.green, Brightness.dark)),
  AppTheme('Red', _build(Colors.red, Brightness.light), _build(Colors.red, Brightness.dark)),
  AppTheme('Deep Orange', _build(Colors.deepOrange, Brightness.light), _build(Colors.deepOrange, Brightness.dark)),
  AppTheme('Light Blue', _build(Colors.lightBlue, Brightness.light), _build(Colors.lightBlue, Brightness.dark)),
];
