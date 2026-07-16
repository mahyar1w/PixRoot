import 'package:flame/sprite.dart';
import 'package:flame/widgets.dart';
import 'package:flutter/material.dart';

class PixelAnimation extends StatefulWidget {
  final SpriteAnimation animation;
  final double scale;

  const PixelAnimation({
    super.key,
    required this.animation,
    this.scale = 1,
  });

  @override
  State<PixelAnimation> createState() => _PixelAnimationState();
}

class _PixelAnimationState extends State<PixelAnimation> {
  late final SpriteAnimationTicker _ticker;

  @override
  void initState() {
    super.initState();

    _ticker = widget.animation.createTicker();
  }

  @override
  Widget build(BuildContext context) {
    final frameSize = widget.animation.frames.first.sprite.srcSize;

    return SizedBox(
      width: frameSize.x * widget.scale,
      height: frameSize.y * widget.scale,
      child: SpriteAnimationWidget(
        animation: widget.animation,
        animationTicker: _ticker,
      ),
    );
  }
}