import 'package:desktop/features/setting/setting_controller.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class SettingPage extends StatefulWidget {
  const SettingPage({super.key});

  @override
  State<SettingPage> createState() => _SettingPageState();
}

class _SettingPageState extends State<SettingPage> {
  late final settingPageController;
  late final manager;
  late bool warning;
  late List<dynamic> block_apps;
  late List<dynamic> all_apps;
  bool loading = true;

  @override
  void initState() {
    super.initState();
    _loadSetting();
  }

  Future<void> _loadSetting() async {
    manager = context.read<SocketManager>();
    settingPageController = SettingController(manager: manager);

    warning = await settingPageController.getWarning();
    block_apps = await settingPageController.getBlockedApps();
    all_apps = await settingPageController.allApps();

    if (!mounted) return;

    setState(() {
      context.read<AppState>().changeWarning(warning);
      context.read<AppState>().setBlockedApps(block_apps);
      loading = false;
    });
  }

  @override
  void dispose() {
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (loading) {
      return const Scaffold(body: Center(child: CircularProgressIndicator()));
    }

    return Scaffold(
      appBar: AppBar(
        title: Center(
          child: Text("Settings", style: TextStyle(fontSize: 32.36)),
        ),
        bottom: PreferredSize(
          preferredSize: Size.fromHeight(7),
          child: SizedBox(
            width: 700,
            child: Divider(height: 1, thickness: 2, color: Colors.black),
          ),
        ),
      ),
      body: Padding(
        padding: const EdgeInsets.only(
          top: 10,
          bottom: 10,
          left: 100,
          right: 100,
        ),
        child: Column(
          children: [
            Expanded(
              child: Column(
                children: [
                  Align(
                    alignment: Alignment.centerLeft,
                    child: Text("General", style: TextStyle(fontSize: 24.27)),
                  ),
                  SizedBox(height: 50),
                  Padding(
                    padding: const EdgeInsets.only(
                      left: 50,
                      right: 50,
                      top: 10,
                      bottom: 10,
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                          "Apps to be filtered:",
                          style: TextStyle(fontSize: 16.18),
                        ),
                        ElevatedButton(
                          onPressed: () async {
                            final result = await showDialog(
                              context: context,
                              builder: (_) => SelectedApps(
                                selected_apps: List.from(block_apps),
                                all_apps: all_apps,
                              ),
                            );
                            if (result != null) {
                              context.read<AppState>().setBlockedApps(result);
                              block_apps = context.read<AppState>().blockedApps;
                              final changeListOkay = await settingPageController
                                  .setBlockedApps(block_apps);
                              if (changeListOkay == true) {
                              } else {}
                            }
                          },
                          child: Text("Launch apps"),
                        ),
                      ],
                    ),
                  ),
                  Padding(
                    padding: const EdgeInsets.only(
                      left: 50,
                      right: 50,
                      top: 10,
                      bottom: 10,
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                          "5 Second Warning for getting distracted",
                          style: TextStyle(fontSize: 16.18),
                        ),
                        Switch(
                          value: context.read<AppState>().warning,
                          onChanged: (bool value) async {
                            final sendOkay = await settingPageController
                                .setWarning(value);
                            if (sendOkay == true) {
                              setState(() {
                                warning = value;
                              });
                              context.read<AppState>().changeWarning(value);
                            } else {}
                          },
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
            Align(
              alignment: Alignment.centerLeft,
              child: Text("Theme", style: TextStyle(fontSize: 24.27)),
            ),
            Text("Adding feature soon...", style: TextStyle(fontSize: 16.18)),
            SizedBox(height: 200),
          ],
        ),
      ),
    );
  }
}

class SelectedApps extends StatefulWidget {
  final List<dynamic> selected_apps;
  final List<dynamic> all_apps;

  const SelectedApps({
    super.key,
    required this.selected_apps,
    required this.all_apps,
  });

  @override
  State<SelectedApps> createState() => _SelectedAppsState();
}

class _SelectedAppsState extends State<SelectedApps> {
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          const Text("Select apps"),

          Expanded(
            child: ListView(
              children: widget.all_apps.map((app) {
                return CheckboxListTile(
                  title: Text(app),
                  value: widget.selected_apps.contains(app),
                  onChanged: (value) {
                    setState(() {
                      value!
                          ? widget.selected_apps.add(app)
                          : widget.selected_apps.remove(app);
                    });
                  },
                );
              }).toList(),
            ),
          ),

          ElevatedButton(
            onPressed: () {
              Navigator.pop(context, widget.selected_apps);
            },
            child: Padding(
              padding: const EdgeInsets.all(8.0),
              child: const Text("Done"),
            ),
          ),
        ],
      ),
    );
  }
}
