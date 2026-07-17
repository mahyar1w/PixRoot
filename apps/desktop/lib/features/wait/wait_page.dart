import 'package:desktop/core/assets/aseprite_assets.dart';
import 'package:desktop/features/wait/wait_page_controller.dart';
import 'package:desktop/services/socket/pixroot_sockets.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/shared/widgets/pixel_animation.dart';
import 'package:desktop/state/app_state.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:provider/provider.dart';

class WaitPage extends StatefulWidget {
  const WaitPage({super.key});

  @override
  State<WaitPage> createState() => _WaitPageState();
}

class _WaitPageState extends State<WaitPage> {

  @override
  void initState() {
    super.initState();

    checkLogin();
  }

  Future<void> checkLogin() async {
    // Implement your login check logic here
    late final bool isLoggedIn;
    late final String username;

    (isLoggedIn, username) = await WaitPageController(manager: SocketManager(client: PixrootSockets())).checkLogin();
    
    if (isLoggedIn) {
      context.watch<AppState>().setUsername(username);
      context.go("/home");
      return;
    } else {
      context.go("/welcome");
      return;
    }
  }

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
