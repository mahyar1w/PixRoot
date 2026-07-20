import 'package:desktop/features/welcome/welcome_controller.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:provider/provider.dart';

class WelcomePage extends StatelessWidget {
  const WelcomePage({super.key});
  static final emailController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    final manager = context.read<SocketManager>();

    final welcomeController = WelcomeController(manager: manager);
    return Scaffold(
      body: Center(
        child: Padding(
          padding: EdgeInsets.all(8),
          child: SizedBox(
            width: 300,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                SizedBox(
                  width: 256,
                  height: 256,
                  child: Image.asset(
                    'assets/images/salem_nochain.png',
                    fit: BoxFit.fill,
                    filterQuality: FilterQuality.none,
                  ),
                ),
                TextField(
                  controller: emailController,
                  decoration: InputDecoration(
                    prefixIcon: Icon(Icons.email),
                    labelText: "Email",
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(20),
                    ),
                  ),
                ),
                SizedBox(height: 5),
                SizedBox(
                  width: 300,
                  height: 45,
                  child: ElevatedButton(
                    style: ElevatedButton.styleFrom(
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(20),
                      ),
                    ),
                    onPressed: () async {
                      final hasAccount = await welcomeController.checkEmail(
                        emailController.text,
                      );
                      if (hasAccount.$1 == true) {
                        context.push("/login");
                      } else {
                        print(hasAccount.$2);
                        context.push("/register");
                      }
                    },
                    child: Text("Start", style: TextStyle(fontSize: 16.18),),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
