# 概要
サーバとクライアント間で送受信されるバイト列のルールを記述する。

# Client > Server
計1Byte
- byte 0:
  - ユーザの入力を表す。
  - 第0ビットが、RightArrow入力の有無を表す。入力があれば1。
  - 第1ビットが、LeftArrow入力の有無を表す。入力があれば1。

# Server > Client
自身のbubble数を$n_1$、相手のbubble数を$n_2$とする。
計$16 + n_1 + n_2$Byte。
- 0-3:
  - 自身のangle: f32
- 4-7:
  - 自身のbubble数$n_1$: i32
- 8-$8 + 8n_1$
  - 自身のbubbleの位置が続く。
  - 各bubbleの位置は8Byteで、
    - 最初4Byteがx座標。
    - 後4Byteがy座標。
- 相手の同じ情報