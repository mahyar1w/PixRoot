import 'package:flutter/material.dart';

class SettingPage extends StatefulWidget {
  const SettingPage({super.key});

  @override
  State<SettingPage> createState() => _SettingPageState();
}

class _SettingPageState extends State<SettingPage> {
  bool warning = true;
  bool? filter;

  @override
  Widget build(BuildContext context) {
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
        padding: const EdgeInsets.all(100.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: [
            Align(
              alignment: Alignment.centerLeft,
              child: Text("General", style: TextStyle(fontSize: 24.27)),
            ),
            Padding(
              padding: const EdgeInsets.all(50.0),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    "Apps to be filtered:",
                    style: TextStyle(fontSize: 16.18),
                  ),
                  ElevatedButton(
                    onPressed: () => showDialog(
                      context: context,
                      builder: (BuildContext context) {
                        return AlertDialog(
                          title: Text("Select Apps"),
                          content: Checkbox(
                            value: filter,
                            onChanged: (bool? value) {
                              setState(() {
                                filter = value;
                              });
                            },
                          ),
                          actions: [
                            TextButton(
                              onPressed: () =>
                                  Navigator.pop(context), // closes the dialog
                              child: Text("Done"),
                            ),
                          ],
                        );
                      },
                    ),
                    child: Text("Launch apps"),
                  ),
                ],
              ),
            ),
            Padding(
              padding: const EdgeInsets.all(50.0),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    "5 Second Warning for getting distracted",
                    style: TextStyle(fontSize: 16.18),
                  ),
                  Switch(
                    value: warning,
                    onChanged: (bool value) {
                      setState(() {
                        warning = value;
                      });
                    },
                  ),
                ],
              ),
            ),
            Align(
              alignment: Alignment.centerLeft,
              child: Text("Theme", style: TextStyle(fontSize: 24.27)),
            ),
          ],
        ),
      ),
    );
  }
}
