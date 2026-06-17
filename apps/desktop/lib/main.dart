import 'package:desktop/features/sign_up/sign_up_page.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'PixRoot Demo',
      theme: ThemeData(
        colorScheme: .fromSeed(seedColor: Colors.grey),
      ),
      home: const SignUpPage(),
    );
  }
}
