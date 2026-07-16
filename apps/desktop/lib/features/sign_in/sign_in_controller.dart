import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/socket_manager.dart';

class LoginController {
  late final SocketManager manager;

  LoginController({required this.manager});

  Future<(bool, String)> logIn(String email, String password) async {
    final response = await manager.sendRequest(
      LoginRequest(email: email, password: password).toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return (true, response.payload['username'] as String);
    } else {
      return (false, '');
    }
  }
}