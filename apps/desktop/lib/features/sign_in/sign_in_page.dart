import 'package:desktop/features/sign_in/sign_in_controller.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:provider/provider.dart';

enum LogInPageState { good, bad }

class SignInPage extends StatefulWidget {
  const SignInPage({super.key});

  @override
  State<SignInPage> createState() => _SignInPageState();
}

class _SignInPageState extends State<SignInPage> {
  LogInPageState state = LogInPageState.good;
  bool _hidePassword = true;

  void showWrongPasswordWarning() {
    setState(() {
      state = LogInPageState.bad;
    });

    Future.delayed(const Duration(seconds: 3), () {
      if (!mounted) return;

      setState(() {
        state = LogInPageState.good;
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    final manager = context.read<SocketManager>();

    final loginController = LoginController(manager: manager);
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            SizedBox(
              width: 300,
              child: TextField(
                controller: loginController.emailField,
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
                controller: loginController.passwordField,
                obscureText: _hidePassword,
                decoration: InputDecoration(
                  prefixIcon: Icon(Icons.lock),
                  suffixIcon: IconButton(
                    icon: Icon(
                      _hidePassword ? Icons.visibility_off : Icons.visibility,
                    ),
                    onPressed: () {
                      setState(() {
                        _hidePassword = !_hidePassword;
                      });
                    },
                  ),
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
                  final logInOkay = await loginController.logIn(
                    loginController.emailField.text,
                    loginController.passwordField.text,
                  );
                  if (logInOkay.$1 == true) {
                    context.read<AppState>().setUsername(logInOkay.$2);
                    context.go("/home");
                  } else {
                    showWrongPasswordWarning();
                    print(logInOkay.$2);
                  }
                },
                child: Text("Log In"),
              ),
            ),
            SizedBox(height: 10),
            if (state == LogInPageState.bad)
              Text(
                "Wrong password! Try again.",
                style: TextStyle(fontSize: 24.27),
              ),
          ],
        ),
      ),
    );
  }
}
