import 'package:desktop/core/assets/aseprite_assets.dart';
import 'package:desktop/shared/widgets/pixel_animation.dart';
import 'package:flutter/material.dart';

PixelAnimation whichAnimation(int time, int separation) {
  if (time ~/ separation == 0) {
    return PixelAnimation(animation: SpriteAssets.part10, scale: 3, key: const ValueKey(0));
  } else if (time ~/ separation == 1) {
    return PixelAnimation(animation: SpriteAssets.part9, scale: 3, key: const ValueKey(1));
  } else if (time ~/ separation == 2) {
    return PixelAnimation(animation: SpriteAssets.part8, scale: 3, key: const ValueKey(2));
  } else if (time ~/ separation == 3) {
    return PixelAnimation(animation: SpriteAssets.part7, scale: 3, key: const ValueKey(3));
  } else if (time ~/ separation == 4) {
    return PixelAnimation(animation: SpriteAssets.part6, scale: 3, key: const ValueKey(4));
  } else if (time ~/ separation == 5) {
    return PixelAnimation(animation: SpriteAssets.part5, scale: 3, key: const ValueKey(5));
  } else if (time ~/ separation == 6) {
    return PixelAnimation(animation: SpriteAssets.part4, scale: 3, key: const ValueKey(6));
  } else if (time ~/ separation == 7) {
    return PixelAnimation(animation: SpriteAssets.part3, scale: 3, key: const ValueKey(7));
  } else if (time ~/ separation == 8) {
    return PixelAnimation(animation: SpriteAssets.part2, scale: 3, key: const ValueKey(8));
  } else {
    return PixelAnimation(animation: SpriteAssets.part1, scale: 3, key: const ValueKey(9));
  }
}
