import 'dart:async';

import 'package:flutter/material.dart';

enum HomePageState { idle, running, finished }

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  double minuteCounter = 0;
  HomePageState state = HomePageState.idle;
  int secondCounter = 0;
  Timer? timer;

  void startTimer() {
    timer?.cancel();
    timer = null;

    timer = Timer.periodic(Duration(seconds: 1), (t) {
      if (secondCounter <= 0) {
        t.cancel();
        timer = null;

        setState(() {
          state = HomePageState.idle;
        });

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

  void cancelTimer() {
    timer?.cancel();
    timer = null;
  }

  @override
  void dispose() {
    timer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
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
        child: ListView(
          children: [
            const DrawerHeader(
              decoration: BoxDecoration(color: Colors.blue),
              child: Text("Header"),
            ),
            ListTile(title: const Text("Settings"), onTap: () {}),
            ListTile(title: Text("Themes"), onTap: () {}),
            ListTile(title: Text("License"), onTap: () {}),
            ListTile(title: Text("About Us"), onTap: () {}),
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
                onPressed: () {
                  secondCounter = minuteCounter.toInt() * 60;
                  startTimer();
                  setState(() {
                    state = HomePageState.running;
                  });
                },
                child: Text("Start", style: TextStyle(fontSize: 32.36)),
              ),
            ),
          if (state == HomePageState.running)
            SizedBox(
              width: 250,
              child: ElevatedButton(
                onPressed: () {
                  cancelTimer();
                  secondCounter = 0;
                  minuteCounter = 0;
                  setState(() {
                    state = HomePageState.idle;
                  });
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
          if (state == HomePageState.running)
            Text(
              (secondCounter % 60).toString().padLeft(2, "0"),
              style: TextStyle(fontSize: 16.18),
            ),
          SizedBox(height: 300),
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
              Padding(
                padding: const EdgeInsets.all(8),
                child: Row(
                  children: [
                    Icon(Icons.timer, size: 30.742),
                    Text("250 Minutes", style: TextStyle(fontSize: 24.27)),
                  ],
                ),
              ),
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
