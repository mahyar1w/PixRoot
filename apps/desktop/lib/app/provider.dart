import 'package:provider/provider.dart';
import 'package:provider/single_child_widget.dart';

import '../state/app_state.dart';


class AppProviders {

  static List<SingleChildWidget> providers = [

    ChangeNotifierProvider(
      create: (_) => AppState(),
    ),

  ];

}