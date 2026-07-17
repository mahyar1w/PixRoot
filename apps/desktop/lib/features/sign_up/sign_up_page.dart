import 'package:desktop/features/sign_up/sign_up_controller.dart';
import 'package:desktop/services/socket/pixroot_sockets.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:provider/provider.dart';

class SignUpPage extends StatelessWidget {
  const SignUpPage({super.key});

  static final signUpController = SignUpController(
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
                controller: signUpController.usernameField,
                decoration: InputDecoration(
                  prefixIcon: Icon(Icons.people),
                  labelText: "Username",
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
                controller: signUpController.emailField,
                decoration: InputDecoration(
                  prefixIcon: Icon(Icons.email),
                  labelText: "Email",
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
                controller: signUpController.passwordField,
                decoration: InputDecoration(
                  prefixIcon: Icon(Icons.lock),
                  labelText: "Password",
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
                  final signUpOkay = await signUpController.signUp(
                    signUpController.usernameField.text,
                    signUpController.emailField.text,
                    signUpController.passwordField.text,
                  );

                  if (signUpOkay == true) {
                    context.watch<AppState>().setUsername(signUpController.usernameField.text);
                    context.go("/home");
                  } else {}
                },
                child: Text("Sign Up"),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
