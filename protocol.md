# 概要
サーバとクライアント間で送受信されるバイト列のルールを記述する。

# Client > Server
計1Byte
- byte 0:
  - ユーザの入力を表す。
  - 第0ビットが、RightArrow入力の有無を表す。入力があれば1。
  - 第1ビットが、LeftArrow入力の有無を表す。入力があれば1。

# Server > Client
計1Byte
- byte 0:
  - `Rotate`による回転の向きを表す。
  - 0x00なら停止。
  - 0x01なら正方向。(y軸角速度1/frame)
  - 0x02なら負方向。(y軸角速度-1/frame)