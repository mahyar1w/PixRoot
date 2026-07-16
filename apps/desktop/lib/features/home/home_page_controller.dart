import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/socket_manager.dart';

class HomePageController {
  late final SocketManager manager;

  HomePageController({required this.manager});

  Future<bool> startSession() async {
    final response = await manager.sendRequest(
      StartSessionRequest().toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return true;
    } else {
      return false;
    }
  }

  Future<bool> endSession() async {
    final response = await manager.sendRequest(
      EndSessionRequest().toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return true;
    } else {
      return false;
    }
  }
}
