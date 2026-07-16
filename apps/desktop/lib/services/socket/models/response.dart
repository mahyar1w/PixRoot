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