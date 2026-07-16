import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/socket_manager.dart';

class WaitPageController {
  late final SocketManager manager;
  WaitPageController({required this.manager});

  Future<(bool, String)> checkLogin() async {
    final response = await manager.sendRequest(
      CheckLoginRequest().toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return (true, response.payload['username'] as String);
    } else {
      return (false, '');
    }
  }
}