import 'dart:async';
import 'dart:convert';

import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/models/response.dart';
import 'package:desktop/services/socket/pixroot_sockets.dart';

class SocketManager {
  PixrootSockets client;
  final _pendingRequests = <String, Completer<Response>>{};

  SocketManager({required this.client}) {
    client.listen(_handleMessage);
  }

  Future<Response> sendRequest(Request request) async {
    final completer = Completer<Response>();

    _pendingRequests[request.action] = completer;

    client.sendMessage(request);

    return completer.future;
  }

  void _handleMessage(String message) {
  final json = jsonDecode(message);

  final response = Response.fromJson(json);

  final completer = _pendingRequests[response.action];

  if (completer != null) {
    completer.complete(response);
    _pendingRequests.remove(response.action);
  }
}
}
