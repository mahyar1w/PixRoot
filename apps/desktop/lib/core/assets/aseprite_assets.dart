import 'package:flame/cache.dart';
import 'package:flame/components.dart';

class SpriteAssets {
  SpriteAssets._();

  static late final SpriteAnimation robot_idle;
  static late final SpriteAnimation part1;
  static late final SpriteAnimation part2;
  static late final SpriteAnimation part3;
  static late final SpriteAnimation part4;
  static late final SpriteAnimation part5;
  static late final SpriteAnimation part6;
  static late final SpriteAnimation part7;
  static late final SpriteAnimation part8;
  static late final SpriteAnimation part9;
  static late final SpriteAnimation part10;

  static final Images _images = Images(prefix: 'assets/animations/');

  static final Map<String, SpriteAnimation> _cache = {};

  static Future<SpriteAnimation> loadAnimation({
    required String asset,
    required int frameCount,
    required double fps,
    required Vector2 frameSize,
    bool loop = true,
  }) async {
    if (_cache.containsKey(asset)) {
      return _cache[asset]!;
    }

    final image = await _images.load(asset);

    final animation = SpriteAnimation.fromFrameData(
      image,
      SpriteAnimationData.sequenced(
        amount: frameCount,
        stepTime: 1 / fps,
        textureSize: frameSize,
        loop: loop,
      ),
    );

    _cache[asset] = animation;

    return animation;
  }

  static Future<void> preload() async {
    robot_idle = await loadAnimation(
      asset: 'FinalMovement.png',
      frameCount: 32,
      fps: 3,
      frameSize: Vector2(128, 128),
      loop: true,
    );
    part1 = await loadAnimation(
      asset: 'part1.png',
      frameCount: 72,
      fps: 12,
      frameSize: Vector2(128, 128),
      loop: true,
    );
    part2 = await loadAnimation(
      asset: 'part2.png',
      frameCount: 72,
      fps: 12,
      frameSize: Vector2(128, 128),
      loop: true,
    );
    part3 = await loadAnimation(
      asset: 'part3.png',
      frameCount: 72,
      fps: 12,
      frameSize: Vector2(128, 128),
      loop: true,
    );
    part4 = await loadAnimation(
      asset: 'part4.png',
      frameCount: 72,
      fps: 12,
      frameSize: Vector2(128, 128),
      loop: true,
    );
    part5 = await loadAnimation(
      asset: 'part5.png',
      frameCount: 72,
      fps: 12,
      frameSize: Vector2(128, 128),
      loop: true,
    );
    part6 = await loadAnimation(
      asset: 'part6.png',
      frameCount: 72,
      fps: 12,
      frameSize: Vector2(128, 128),
      loop: true,
    );
    part7 = await loadAnimation(
      asset: 'part7.png',
      frameCount: 72,
      fps: 12,
      frameSize: Vector2(128, 128),
      loop: true,
    );
    part8 = await loadAnimation(
      asset: 'part8.png',
      frameCount: 72,
      fps: 12,
      frameSize: Vector2(128, 128),
      loop: true,
    );
    part9 = await loadAnimation(
      asset: 'part9.png',
      frameCount: 72,
      fps: 12,
      frameSize: Vector2(128, 128),
      loop: true,
    );
    part10 = await loadAnimation(
      asset: 'part10.png',
      frameCount: 72,
      fps: 12,
      frameSize: Vector2(128, 128),
      loop: true,
    );
  }
}
