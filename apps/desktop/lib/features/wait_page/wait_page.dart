import 'package:desktop/core/assets/aseprite_assets.dart';
import 'package:desktop/shared/widgets/pixel_animation.dart';
import 'package:flutter/material.dart';

class WaitPage extends StatelessWidget {
  const WaitPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Padding(
          padding: const EdgeInsets.only(top:16),
          child: const Center(child: Text("PixRoot", style: TextStyle(fontSize: 48))),
        ),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            PixelAnimation(animation: SpriteAssets.robot_idle, scale: 3),
            Text("Loading...", style: TextStyle(fontSize: 24)),
          ],
        ),
      ),
    );
  }
}
