# 概要
サーバとクライアント間で送受信されるバイト列のルールを記述する。

# Client > Server
計1Byte
- byte 0:
  - ユーザの入力を表す。
  - 第0ビットが、RightArrow入力の有無を表す。入力があれば1。
  - 第1ビットが、LeftArrow入力の有無を表す。入力があれば1。

# Server > Client
計16Byte
- byte 0-4:
  - 自身のx座標を表す。
- byte 5-8:
  - 自身のz座標を表す。
- byte 9-12:
  - 他者のx座標を表す。
- byte 13-16:
  - 他者のz座標を表す。