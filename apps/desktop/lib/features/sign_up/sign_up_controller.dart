import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/socket_manager.dart';

class SignUpController {
  late final SocketManager manager;
  SignUpController({required this.manager});

  Future<bool> signUp(String username, String email, String password) async {
    final response = await manager.sendRequest(
      SignUpRequest(username: username, email: email, password: password).toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return true;
    } else {
      return false;
    }
  }
}