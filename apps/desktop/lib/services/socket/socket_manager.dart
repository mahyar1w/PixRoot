import 'dart:async';
import 'dart:convert';

import 'package:desktop/services/socket/models/request.dart';
import 'package:desktop/services/socket/models/response.dart';
import 'package:desktop/services/socket/pixroot_sockets.dart';

class SocketManager {
  PixrootSockets client;
  final _pendingRequests = <String, Completer<Response>>{};

  final _eventController = StreamController<Event>.broadcast();

  Stream<Event> get events => _eventController.stream;

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
    try {
      final json = jsonDecode(message);
      if (json.containsKey('event')) {
        final event = Event.fromJson(json);
        _eventController.add(event);
        return;
      } else if (json.containsKey('action')) {
        final response = Response.fromJson(json);

        final completer = _pendingRequests[response.action];

        if (completer != null) {
          completer.complete(response);
          _pendingRequests.remove(response.action);
        }
      }
    } catch (e) {
      print(e);
    }
  }
}
