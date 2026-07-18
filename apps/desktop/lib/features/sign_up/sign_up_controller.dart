import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:flutter/material.dart';

class SignUpController {
  late final SocketManager manager;
  
  late final usernameField = TextEditingController();
  late final emailField = TextEditingController();
  late final passwordField = TextEditingController();

  SignUpController({required this.manager});

  Future<(bool, String)> signUp(String username, String email, String password) async {
    final response = await manager.sendRequest(
      SignUpRequest(username: username, email: email, password: password).toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return (true, '');
    } else {
      return (false, response.payload['error'] as String);
    }
  }
}