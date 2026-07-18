import 'package:desktop/features/wait/wait_page.dart';
import 'package:desktop/features/setting/setting_page.dart';
import 'package:desktop/features/sign_up/sign_up_page.dart';
import 'package:desktop/features/welcome/welcome_page.dart';
import 'package:desktop/features/home/home_page.dart';
import 'package:desktop/features/sign_in/sign_in_page.dart';
import 'package:go_router/go_router.dart';

final router = GoRouter(
  routes: [
    GoRoute(path: "/", builder: (_, __) => const WaitPage()),
    GoRoute(path: "/settings", builder: (_, __) => const SettingPage()),
    GoRoute(path: "/login", builder: (_, __) => const SignInPage()),
    GoRoute(path: "/register", builder: (_, __) => const SignUpPage()),
    GoRoute(path: "/welcome", builder: (_, __) => const WelcomePage()),
    GoRoute(path: "/home", builder: (_, __) => const HomePage()),
  ],
);
