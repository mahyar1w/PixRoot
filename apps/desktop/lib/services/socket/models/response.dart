class Response {
  final String action;
  final Map<String, dynamic> payload;

  Response({required this.action, required this.payload});

  factory Response.fromJson(Map<String, dynamic> json) {
    return Response(
      action: json['action'],
      payload: json['payload'],
    );
  }
}

class Event {
    late final String event;
    late final Map<String, dynamic> payload;

    Event({required this.event, required this.payload});

    factory Event.fromJson(Map<String, dynamic> json) {
        return Event(
            event: json['event'],
            payload: json['payload'],
        );
    }
}