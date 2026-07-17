import 'package:desktop/features/sign_in/sign_in_controller.dart';
import 'package:desktop/services/socket/pixroot_sockets.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class SignInPage extends StatelessWidget {
  const SignInPage({super.key});
  static final logInController = LoginController(
    manager: SocketManager(client: PixrootSockets()),
  );

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            SizedBox(
              width: 300,
              child: TextField(
                controller: logInController.emailField,
                decoration: InputDecoration(
                  prefixIcon: Icon(Icons.email),
                  labelText: "Enter your gmail",
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(20),
                  ),
                ),
              ),
            ),
            SizedBox(height: 10),
            SizedBox(
              width: 300,
              child: TextField(
                controller: logInController.passwordField,
                decoration: InputDecoration(
                  prefixIcon: Icon(Icons.lock),
                  labelText: "Enter your password",
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(20),
                  ),
                ),
              ),
            ),
            SizedBox(height: 10),
            SizedBox(
              width: 300,
              child: ElevatedButton(
                onPressed: () async {
                  final (logInOkay, username) = await logInController.logIn(
                    logInController.emailField.text,
                    logInController.passwordField.text,
                  );
                  if (logInOkay == true) {
                    context.watch<AppState>().setUsername(username);
                  } else {}
                },
                child: Text("Log In"),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
