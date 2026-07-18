import 'package:flutter/material.dart';

class AppState extends ChangeNotifier {
  bool isLoggedIn = false;
  String? username;
  bool warning = true;
  bool intrrupt = false;
  int totalDayTime = 0;
  List<dynamic> blockedApps = [];

  bool showNotif = false;

  void changeWarning(bool value) {
    warning = value;
    notifyListeners();
  }

  void setBlockedApps(List<dynamic> apps) {
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

  void updateTotalDayTime(int time) {
    totalDayTime += time;
    notifyListeners();
  }

  void changeNotif() {
    showNotif = !showNotif;
    notifyListeners();
  }
}
