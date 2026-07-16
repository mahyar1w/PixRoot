import 'package:flutter/material.dart';

class AppState extends ChangeNotifier {
  bool isLoggedIn = false;
  String? username;
  bool warning = true;
  bool intrrupt = false;
  double totalDayTime = 0.0;
  List<String> blockedApps = [];

  void changeWarning() {
    warning = !warning;
    notifyListeners();
  }

  void setBlockedApps(List<String> apps) {
    blockedApps = apps;
    notifyListeners();
  }

  void setUsername(String name) {
    username = name;
    notifyListeners();
  }

  void changeIntrrupt() {
    intrrupt = !intrrupt;
    notifyListeners();
  }

  void updateTotalDayTime(double time) {
    totalDayTime += time;
    notifyListeners();
  }
}
