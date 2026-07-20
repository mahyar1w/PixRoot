import 'package:desktop/app/router.dart';
import 'package:flutter/material.dart';

class PixRootApp extends StatelessWidget {
  const PixRootApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      debugShowCheckedModeBanner: false,
      routerConfig: router,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.grey),
        fontFamily: "MyPixelFont"
      ),
    );
  }
}
