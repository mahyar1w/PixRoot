import 'dart:convert';
import 'dart:io';

import 'package:desktop/services/socket/models/request.dart';

class PixrootSockets {
  Socket? _socket;

  Future<void> connect() async {
    _socket = await Socket.connect(
      InternetAddress("/tmp/focus_app.sock", type: InternetAddressType.unix),
      0,
    );
  }

  void sendMessage(Request message) {
    if (_socket == null) {
      throw StateError("Socket is not connected.");
    }
    if (_socket != null) {
      _socket!.writeln(jsonEncode(message.toJson()));
    }
  }

  void close() {
    _socket?.close();
  }

  void listen(void Function(String) onData) {
    if (_socket == null) {
      throw StateError("Socket is not connected.");
    }

    _socket!
        .cast<List<int>>()
        .transform(utf8.decoder)
        .transform(const LineSplitter())
        .listen(onData);
  }
}
