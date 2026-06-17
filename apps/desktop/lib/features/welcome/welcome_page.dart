import 'package:flutter/material.dart';

class WelcomePage extends StatelessWidget {
  const WelcomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Padding(
          padding: EdgeInsets.all(8),
          child: SizedBox(
            width: 300,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                TextField(
                  decoration: InputDecoration(
                    prefixIcon: Icon(Icons.email),
                    labelText: "Enter your gmail",
                    border: OutlineInputBorder(borderRadius: BorderRadius.circular(20))
                  ),
                ),
                SizedBox(height: 5),
                SizedBox(width: 300,
                height: 45,
                child: ElevatedButton(onPressed: () {}, child: Text("Start")))
              ],
            ),
          ),
        ),
      ),
    );
  }
}