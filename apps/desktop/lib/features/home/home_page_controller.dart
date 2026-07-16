import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/models/response.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';

class HomePageController {
  late final SocketManager manager;

  HomePageController({required this.manager}) {
    manager.events.listen(_handleEvent);
  }

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
    final response = await manager.sendRequest(EndSessionRequest().toRequest());
    if (response.payload['status'] == 'ok') {
      return true;
    } else {
      return false;
    }
  }

  Future<bool> logOut() async {
    final response = await manager.sendRequest(LogoutRequest().toRequest());
    if (response.payload['status'] == 'ok') {
      return true;
    } else {
      return false;
    }
  }

  Future<bool> saveSession(bool success, double time) async {
    final response = await manager.sendRequest(
      SaveSessionRequest(time: time, success: success).toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return true;
    } else {
      return false;
    }
  }

  void _handleEvent(Event event) {
    switch (event.event) {
      case 'total_day_time':
        if (event.payload['status'] == 'ok') {
          AppState().updateTotalDayTime(event.payload['total_fucos_time']);
        } else {
          // Handle error
        }
        break;
      case 'interrupt':

        // Handle session ended event
        break;
    }
  }
}
