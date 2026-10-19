import 'package:desktop/features/welcome/welcome_page.dart';
import 'package:desktop/services/socket/pixroot_sockets.dart';
import 'package:desktop/services/socket/socket_manager.dart';
import 'package:desktop/state/app_state.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';

// Stands in for the daemon socket so the widget can build without a running daemon.
class FakeSockets extends PixrootSockets {
  @override
  void listen(void Function(String) onData) {}
}

void main() {
  testWidgets('welcome page shows the email field and logo', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      MultiProvider(
        providers: [
          ChangeNotifierProvider(create: (_) => AppState()),
          Provider(create: (_) => SocketManager(client: FakeSockets())),
        ],
        child: const MaterialApp(home: WelcomePage()),
      ),
    );

    expect(find.text('Email'), findsOneWidget);
    expect(find.byType(TextField), findsOneWidget);
    expect(find.byType(Image), findsOneWidget);
  });
}
