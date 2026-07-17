import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class WelcomeController {
  late final SocketManager manager;

  WelcomeController({required this.manager});

  Future<bool> checkEmail(String email) async {
    final response = await manager.sendRequest(
      CheckEmailRequest(email: email).toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return true;
    } else {
      return false;
    }
  }
}