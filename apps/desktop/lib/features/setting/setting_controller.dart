import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/socket_manager.dart';

class SettingController {
  late final SocketManager manager;
  SettingController({required this.manager});

  Future<bool> setWarning(bool warning) async {
    final response = await manager.sendRequest(
      WarningSetRequest(warning: warning).toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return true;
    } else {
      return false;
    }
  }

  Future<bool> setBlockedApps(List<String> blockedApps) async {
    final response = await manager.sendRequest(
      BlockedAppsSetRequest(blockedApps: blockedApps).toRequest(),
    );
    if (response.payload['status'] == 'ok') {
      return true;
    } else {
      return false;
    }
  }

  Future<bool> getWarning() async {
    final response = await manager.sendRequest(
      WarningGetRequest().toRequest(),
    );
    return response.payload['warning'];
  }

  Future<List<String>> getBlockedApps() async {
    final response = await manager.sendRequest(
      BlockedAppsGetRequest().toRequest(),
    );
    return response.payload['blocked_apps'].toString().split(',');
  }

  Future<List<String>> allApps() async {
    final response = await manager.sendRequest(
      AllAppsRequest().toRequest(),
    );
    return response.payload['all_apps'] as List<String>;
  }
}