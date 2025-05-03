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
- byte 0-3:
  - 自身のx座標を表す。
- byte 4-7:
  - 自身のz座標を表す。
- byte 8-11:
  - 他者のx座標を表す。
- byte 12-15:
  - 他者のz座標を表す。