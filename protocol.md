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
計$18 + 9n_1 + 9n_2$Byte。
- 0-3:
  - 自身のangle: f32
- 4:
  - 自身のスコア: i8
- 5-8:
  - 自身のbubble数$n_1$: i32
- 9-$9 + 9n_1$
  - 自身のbubbleの情報が$n_1$個続く。
  - 各bubbleの情報は9Byteで、
    - 最初4Byteがx座標: f32
    - 次の4Byteがy座標: f32
    - 最後の1Byteが色: u8。赤なら0x00、白なら0x01。
- 相手の同じ情報