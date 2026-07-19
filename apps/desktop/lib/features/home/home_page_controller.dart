import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/models/response.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';

class HomePageController {
  late final SocketManager manager;
  late final AppState appState;
  late final _subscription;

  HomePageController({required this.manager, required this.appState}) {
    _subscription = manager.events.listen(_handleEvent);
  }

  Future<(bool, String)> startSession() async {
    final response = await manager.sendRequest(
      StartSessionRequest().toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return (true, '');
    } else {
      return (false, response.payload['error'] as String);
    }
  }

  Future<(bool, String)> endSession() async {
    final response = await manager.sendRequest(EndSessionRequest().toRequest());
    if (response.payload['status'] == 'ok') {
      return (true, '');
    } else {
      return (false, response.payload['error'] as String);
    }
  }

  Future<(bool, String)> logOut() async {
    final response = await manager.sendRequest(LogoutRequest().toRequest());
    if (response.payload['status'] == 'ok') {
      return (true, '');
    } else {
      return (false, response.payload['error'] as String);
    }
  }

  Future<(bool, String)> saveSession(bool success, int time) async {
    final response = await manager.sendRequest(
      SaveSessionRequest(time: time, success: success).toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return (true, '');
    } else {
      return (false, response.payload['error'] as String);
    }
  }

  Future<void> dispose() async {
    await _subscription.cancel();
  }

  void _handleEvent(Event event) {
    switch (event.event) {
      case 'total_day_time':
        if (event.payload['status'] == 'ok') {
          appState.updateTotalDayTime(event.payload['total_focus']);
        } else {}
        break;
      case 'interrupt':
        switch (appState.warning) {
          case true:
            if (event.payload['stage'] == 'interrupted') {
              appState.changeNotif();
            } else if (event.payload['stage'] == 'warning_failed') {
              appState.changeFailing();
            } else {
              appState.changeNotif();
            }

            break;
          case false:
            appState.changeFailing();

            break;
        }
        // Handle session ended event
        break;
    }
  }
}
