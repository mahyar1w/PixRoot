// import 'package:desktop/features/setting/setting_controller.dart';
// import 'package:desktop/services/socket/pixroot_sockets.dart';
// import 'package:desktop/services/socket/socket_manager.dart';
// import 'package:desktop/state/app_state.dart';
// import 'package:flutter/material.dart';
// import 'package:provider/provider.dart';

// class SettingPage extends StatefulWidget {
//   const SettingPage({super.key});

//   @override
//   State<SettingPage> createState() => _SettingPageState();
// }

// class _SettingPageState extends State<SettingPage> {
//   final settingPageController = SettingController(
//     manager: SocketManager(client: PixrootSockets()),
//   );
//   bool? warning;

//   @override
//   void initState() async {
//     super.initState();

//     warning = await settingPageController.getWarning();
//     final block_apps = await settingPageController.getBlockedApps();

//     context.watch<AppState>().changeWarning(warning!);
//     context.watch<AppState>().setBlockedApps(block_apps);
//   }

//   @override
//   Widget build(BuildContext context) {
//     return Scaffold(
//       appBar: AppBar(
//         title: Center(
//           child: Text("Settings", style: TextStyle(fontSize: 32.36)),
//         ),
//         bottom: PreferredSize(
//           preferredSize: Size.fromHeight(7),
//           child: SizedBox(
//             width: 700,
//             child: Divider(height: 1, thickness: 2, color: Colors.black),
//           ),
//         ),
//       ),
//       body: Padding(
//         padding: const EdgeInsets.only(
//           top: 10,
//           bottom: 10,
//           left: 100,
//           right: 100,
//         ),
//         child: Column(
//           children: [
//             Expanded(
//               child: Column(
//                 children: [
//                   Align(
//                     alignment: Alignment.centerLeft,
//                     child: Text("General", style: TextStyle(fontSize: 24.27)),
//                   ),
//                   SizedBox(height: 50),
//                   Padding(
//                     padding: const EdgeInsets.only(
//                       left: 50,
//                       right: 50,
//                       top: 10,
//                       bottom: 10,
//                     ),
//                     child: Row(
//                       mainAxisAlignment: MainAxisAlignment.spaceBetween,
//                       children: [
//                         Text(
//                           "Apps to be filtered:",
//                           style: TextStyle(fontSize: 16.18),
//                         ),
//                         ElevatedButton(
//                           onPressed: () => showDialog(
//                             context: context,
//                             builder: (BuildContext context) {
//                               return AlertDialog(
//                                 title: Text("Select Apps"),
//                                 content: Checkbox(
//                                   value: filter,
//                                   onChanged: (bool? value) {
//                                     setState(() {
//                                       filter = value;
//                                     });
//                                   },
//                                 ),
//                                 actions: [
//                                   TextButton(
//                                     onPressed: () => Navigator.pop(
//                                       context,
//                                     ), // closes the dialog
//                                     child: Text("Done"),
//                                   ),
//                                 ],
//                               );
//                             },
//                           ),
//                           child: Text("Launch apps"),
//                         ),
//                       ],
//                     ),
//                   ),
//                   Padding(
//                     padding: const EdgeInsets.only(
//                       left: 50,
//                       right: 50,
//                       top: 10,
//                       bottom: 10,
//                     ),
//                     child: Row(
//                       mainAxisAlignment: MainAxisAlignment.spaceBetween,
//                       children: [
//                         Text(
//                           "5 Second Warning for getting distracted",
//                           style: TextStyle(fontSize: 16.18),
//                         ),
//                         Switch(
//                           value: context.read<AppState>().warning,
//                           onChanged: (bool value) async {
//                             final sendOkay = await settingPageController
//                                 .setWarning(value);
//                             if (sendOkay == true) {
//                               setState(() {
//                                 warning = value;
//                               });
//                               context.watch<AppState>().changeWarning();
//                             } else {}
//                           },
//                         ),
//                       ],
//                     ),
//                   ),
//                 ],
//               ),
//             ),
//             Align(
//               alignment: Alignment.centerLeft,
//               child: Text("Theme", style: TextStyle(fontSize: 24.27)),
//             ),
//             Text("Adding feature soon...", style: TextStyle(fontSize: 16.18)),
//             SizedBox(height: 200),
//           ],
//         ),
//       ),
//     );
//   }
// }

// class SelectedApps extends StatefulWidget {
//   const SelectedApps({super.key});

//   @override
//   State<SelectedApps> createState() => _SelectedAppsState();
// }

// class _SelectedAppsState extends State<SelectedApps> {


//   @override
//   Widget build(BuildContext context) {
//     return Dialog(
//       child: Column(
//         mainAxisSize: MainAxisSize.min,
//         children: [
//           const Text("Select apps"),

//           Expanded(
//             child: ListView(
//               children: blocked_apps.map((app) {
//                 return CheckboxListTile(
//                   title: Text(app),
//                   value: selected.contains(app),
//                   onChanged: (value) {
//                     setState(() {
//                       value! ? selected.add(app) : selected.remove(app);
//                     });
//                   },
//                 );
//               }).toList(),
//             ),
//           ),

//           ElevatedButton(
//             onPressed: () => Navigator.pop(context, selected),
//             child: const Text("Done"),
//           ),
//         ],
//       ),
//     );
//   }
// }
