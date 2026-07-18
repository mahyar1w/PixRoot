import 'dart:async';

import 'package:desktop/features/home/animation_handling.dart';
import 'package:desktop/features/home/home_page_controller.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:provider/provider.dart';

enum HomePageState { idle, running, finished }

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  late final manager;
  late final homePageController;
  double minuteCounter = 0;
  int fullTime = 0;
  HomePageState state = HomePageState.idle;
  int secondCounter = 0;
  int separator = 0;
  int totalDayTime = 0;
  Timer? timer;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();

    manager = context.read<SocketManager>();
    homePageController = HomePageController(
      manager: manager,
      appState: context.read<AppState>(),
    );
  }

  @override
  void initState() {
    super.initState();
    _loadAnalyz();
  }

  Future<void> _loadAnalyz() async {
    // add the request for getting the total day time for the first time opening app in a day

    if (!mounted) return;

    setState(() {
      totalDayTime = context.read<AppState>().totalDayTime;
    });
  }

  void startTimer() {
    timer?.cancel();
    timer = null;
    fullTime = minuteCounter.toInt();

    timer = Timer.periodic(Duration(seconds: 1), (t) {
      if (secondCounter <= 0) {
        t.cancel();
        timer = null;
        _endIt();
        return;
      }

      setState(() {
        if (secondCounter % 60 == 0) {
          minuteCounter--;
        }
        secondCounter--;
      });
    });
  }

  Future<void> _endIt() async {
    final endOkay = await homePageController.endSession();
    if (endOkay.$1) {
      final saveOkay = await homePageController.saveSession(true, fullTime);
      if (saveOkay.$1 == true) {
        if (mounted) {
          setState(() {
            state = HomePageState.idle;
          });
          showDialog(context: context, builder: (_) => SuccessSession());
        }
        return;
      } else {
        print(saveOkay.$2);
      }
    } else {
      print(endOkay.$2);
    }
  }

  void cancelTimer() {
    timer?.cancel();
    timer = null;
  }

  @override
  void dispose() {
    timer?.cancel();
    super.dispose();
    homePageController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    String username = context.read<AppState>().username ?? "User";
    return Scaffold(
      appBar: AppBar(
        title: Center(child: Text("PixRoot")),
        actions: [
          IconButton(
            onPressed: () {},
            icon: Icon(Icons.dark_mode, size: 24.27),
          ),
        ],
      ),
      drawer: Drawer(
        child: Column(
          children: [
            DrawerHeader(
              decoration: BoxDecoration(color: Colors.grey),
              child: Align(
                alignment: Alignment.bottomLeft,
                child: Text("Hello, $username", style: TextStyle(fontSize: 24)),
              ),
            ),
            Expanded(
              child: ListView(
                children: [
                  ListTile(
                    title: const Text("Settings"),
                    onTap: () {
                      context.push("/settings");
                    },
                  ),
                  ListTile(title: Text("License"), onTap: () {}),
                  ListTile(title: Text("About Us"), onTap: () {}),
                ],
              ),
            ),
            Divider(),
            ListTile(
              title: Row(
                children: [
                  const Text("LogOut"),
                  SizedBox(width: 10),
                  const Icon(Icons.logout),
                ],
              ),
              onTap: () async {
                final logoutOkay = await homePageController.logOut();
                if (logoutOkay.$1 == true) {
                  context.read<AppState>().setUsername("");
                  context.go("/welcome");
                } else {
                  print(logoutOkay.$2);
                }
              },
            ),
          ],
        ),
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          SizedBox(height: 40),
          if (state == HomePageState.idle)
            SizedBox(
              width: 250,
              child: ElevatedButton(
                onPressed: () async {
                  final startOkay = await homePageController.startSession();

                  if (startOkay.$1 == true) {
                    secondCounter = minuteCounter.toInt() * 60;
                    separator = secondCounter ~/ 10;

                    startTimer();
                    setState(() {
                      state = HomePageState.running;
                    });
                  } else {
                    print(startOkay.$2);
                  }
                },
                child: Text("Start", style: TextStyle(fontSize: 32.36)),
              ),
            ),
          if (state == HomePageState.running)
            SizedBox(
              width: 250,
              child: ElevatedButton(
                onPressed: () async {
                  final endOkay = await homePageController.endSession();
                  if (endOkay.$1 == true) {
                    cancelTimer();
                    secondCounter = 0;
                    minuteCounter = 0;
                    setState(() {
                      state = HomePageState.idle;
                    });

                    showDialog(
                      context: context,
                      builder: (_) => FailedSession(),
                    );
                    final saveOkay = await homePageController.saveSession(
                      false,
                      0,
                    );
                    if (saveOkay.$1 == true) {
                    } else {
                      print(saveOkay.$2);
                    }
                  } else {
                    print(endOkay.$2);
                  }
                },
                child: Text("Cancel", style: TextStyle(fontSize: 32.36)),
              ),
            ),
          Text(
            "${(minuteCounter.toInt() ~/ 60).toString().padLeft(2, "0")}:${(minuteCounter.toInt() % 60).toString().padLeft(2, "0")}",
            style: TextStyle(
              fontSize: state == HomePageState.idle ? 80.09 : 97.08,
            ),
          ),
          if (state == HomePageState.idle)
            SizedBox(
              height: 256,
              width: 256,
              child: Image.asset(
                "assets/images/Kharab.png",
                fit: BoxFit.fill,
                filterQuality: FilterQuality.none,
              ),
            ),
          if (state == HomePageState.running)
            whichAnimation(secondCounter, separator),
          if (state == HomePageState.idle)
            SizedBox(
              width: 400,
              child: Slider(
                value: minuteCounter,
                onChanged: (value) {
                  setState(() {
                    minuteCounter = value;
                  });
                },
                min: 0,
                max: 180,
                divisions: 36,
              ),
            ),
          SizedBox(height: 50),

          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              if (state == HomePageState.idle)
                Padding(
                  padding: const EdgeInsets.all(8),
                  child: Row(
                    children: [
                      Icon(Icons.timer, size: 30.742),
                      Text(
                        "$totalDayTime Minutes",
                        style: TextStyle(fontSize: 24.27),
                      ),
                    ],
                  ),
                ),
              if (state == HomePageState.idle)
                Padding(
                  padding: const EdgeInsets.all(8),
                  child: IconButton(
                    onPressed: () {},
                    icon: Icon(Icons.forest, size: 30.742),
                  ),
                ),
            ],
          ),
        ],
      ),
    );
  }
}

class FailedSession extends StatelessWidget {
  const FailedSession({super.key});

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: SizedBox(
        width: 700,
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            mainAxisSize: MainAxisSize.min,
            children: [
              Padding(
                padding: const EdgeInsets.all(30.0),
                child: Text(
                  "You failed :( It's ok, try it next time.",
                  style: TextStyle(fontSize: 24.27),
                ),
              ),
              SizedBox(
                height: 512,
                width: 512,
                child: Image.asset(
                  "assets/images/Kharab.png",
                  fit: BoxFit.fill,
                  filterQuality: FilterQuality.none,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class SuccessSession extends StatelessWidget {
  const SuccessSession({super.key});

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: SizedBox(
        width: 700,
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Padding(
                padding: const EdgeInsets.all(30.0),
                child: Text(
                  "You succeeded! Great job!",
                  style: TextStyle(fontSize: 24.27),
                ),
              ),
              SizedBox(
                height: 512,
                width: 512,
                child: Image.asset(
                  "assets/images/salem_nochain.png",
                  fit: BoxFit.fill,
                  filterQuality: FilterQuality.none,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
