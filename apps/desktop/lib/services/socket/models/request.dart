class Request {
  final String action;
  final Map<String, dynamic> payload;

  Request({required this.action, required this.payload});

  Map<String, dynamic> toJson() {
    return {'action': action, 'payload': payload};
  }
}

class LoginRequest {
  final String email;
  final String password;

  LoginRequest({required this.email, required this.password});

  Request toRequest() {
    return Request(
      action: 'login',
      payload: {'email': email, 'password': password},
    );
  }
}

class SignUpRequest {
  final String email;
  final String password;
  final String username;

  SignUpRequest({
    required this.email,
    required this.password,
    required this.username,
  });

  Request toRequest() {
    return Request(
      action: 'signup',
      payload: {'email': email, 'password': password, 'username': username},
    );
  }
}

class CheckEmailRequest {
  final String email;

  CheckEmailRequest({required this.email});

  Request toRequest() {
    return Request(action: 'check_email', payload: {'email': email});
  }
}

class CheckLoginRequest {
  Request toRequest() {
    return Request(action: 'check_login', payload: {});
  }
}

class StartSessionRequest {
  Request toRequest() {
    return Request(action: 'start_session', payload: {});
  }
}

class EndSessionRequest {
  Request toRequest() {
    return Request(action: 'end_session', payload: {});
  }
}

class WarningSetRequest {
  final bool warning;

  WarningSetRequest({required this.warning});

  Request toRequest() {
    return Request(action: 'set_warning', payload: {'warning': warning});
  }
}

class BlockedAppsSetRequest {
  final List<String> blockedApps;

  BlockedAppsSetRequest({required this.blockedApps});

  Request toRequest() {
    return Request(
      action: 'set_blocked_apps',
      payload: {'blocked_apps': blockedApps.join(',')},
    );
  }
}

class WarningGetRequest {
  Request toRequest() {
    return Request(action: 'get_warning', payload: {});
  }
}

class BlockedAppsGetRequest {
  Request toRequest() {
    return Request(action: 'get_blocked_apps', payload: {});
  }
}
