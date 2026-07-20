import 'package:desktop/features/sign_up/sign_up_controller.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:provider/provider.dart';

class SignUpPage extends StatefulWidget {
  const SignUpPage({super.key});

  @override
  State<SignUpPage> createState() => _SignUpPageState();
}

class _SignUpPageState extends State<SignUpPage> {
  bool _hidePassword = true;
  late final SignUpController signUpController;
  late final manager;
  bool _initialized = false;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();

    if (!_initialized) {
      manager = context.read<SocketManager>();
      signUpController = SignUpController(manager: manager);

      signUpController.emailField.text = context.read<AppState>().email;
      _initialized = true;
    }
  }

  @override
  void dispose() {
    signUpController.emailField.dispose();
    signUpController.passwordField.dispose();
    signUpController.usernameField.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Center(child: Text("SignUp", style: TextStyle(fontSize: 32.36))),
      ),
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
                readOnly: true,
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
              height: 45,
              width: 300,
              child: ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.blueGrey,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(20),
                  ),
                ),
                onPressed: () async {
                  final signUpOkay = await signUpController.signUp(
                    signUpController.usernameField.text,
                    signUpController.emailField.text,
                    signUpController.passwordField.text,
                  );

                  if (signUpOkay.$1 == true) {
                    context.read<AppState>().setUsername(
                      signUpController.usernameField.text,
                    );
                    context.go("/home");
                  } else {
                    print('signup failed: ${signUpOkay.$2}');
                  }
                },
                child: Text(
                  "Sign Up",
                  style: TextStyle(fontSize: 16.18, color: Colors.black),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
