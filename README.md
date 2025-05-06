# About
PGOnlineは、Webブラウザで動作するオンラインゲームである。
ゲーム内容は[スーパーマリオパーティ ジャンボリー](https://www.nintendo.com/jp/switch/a7hla/index.html)の「プクプクとゲッソー」というミニゲームのコピーである。

2名のユーザがWebクライアントを起動すると、ゲームが開始する。
ゲームの処理はサーバで行われ、Webクライアントはユーザ入力の送信と、ゲーム状態の受信・描画のみを行う。

# Boot
本システムは次のコマンドで起動できる。

```
git clone https://github.com/tombo-akizu/pgonline.git
cd pgonline
docker-compose up --build -d
```

コマンドを実行すると、ゲームサーバ(server)とクライアント用のWebサーバ(client)がローカルホストに立ち上がる。
ポート番号は、serverが8080、clientが3000にバインディングされる。
