using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using NativeWebSocket;

public class Connection : MonoBehaviour
{
    private const float FPS = 1f / 30;

    private WebSocket websocket = null;
    private InputObserver observer;

    [SerializeField] PlayerGame playerGame;
    [SerializeField] PlayerGame opponentGame;

    private StagingController stagingController;

    private enum ControlByte
    {
        GameState,
        GameStart,
        GameEnd
    }

    private void Awake()
    {
        this.stagingController = FindFirstObjectByType<StagingController>();
        this.observer = FindFirstObjectByType<InputObserver>();
    }

    // Reactから呼び出される。
    public async void Initialize(string ipAddress)
    {
        // サーバのIPアドレスをReactから受け取る。
        // Unity AppのビルドはReactと比べてかなり長い時間を要するため、ビルド頻度を減らしたい。
        // 開発時にIPアドレスを変更する際、Unity Appを再ビルドしなくてよいよう、IPアドレスをReactから受け取っている。
        this.websocket = new WebSocket("ws://" + ipAddress);

        List<GameObject> bubbleObjects0 = new();
        List<GameObject> bubbleObjects1 = new();

        this.websocket.OnOpen += () =>
        {
            Debug.Log("Connection open!");
        };

        this.websocket.OnError += (e) =>
        {
            Debug.Log("Error! " + e);
        };

        this.websocket.OnClose += (e) =>
        {
            Debug.Log("Connection closed!");
        };

        this.websocket.OnMessage += (bytes) =>
        {
            switch (bytes[0])   // 第0バイトは制御バイト
            {
                case (byte)ControlByte.GameState:
                    DecodedState state = new(bytes[1..bytes.Length]);
                    this.playerGame.ReflectGameState(state.GetPlayerState());
                    this.opponentGame.ReflectGameState(state.GetOpponentState());
                    break;
                case (byte)ControlByte.GameStart:
                    this.stagingController.StartStaging();
                    break;
                case (byte)ControlByte.GameEnd:
                    this.stagingController.EndStaging();
                    break;
            }
        };

        this.InvokeRepeating(nameof(SendWebSocketMessage), 0.0f, FPS);

        // メッセージを待機する。
        await this.websocket.Connect();
    }

    private void Update()
    {
#if !UNITY_WEBGL || UNITY_EDITOR
        if (this.websocket != null)
        {
            this.websocket.DispatchMessageQueue();
        }
#endif
    }

    private async void SendWebSocketMessage()
    {
        if (this.websocket.State == WebSocketState.Open)
        {
            await this.websocket.Send(this.observer.PopInputByte());
        }
    }

    private async void OnApplicationQuit()
    {
        if (this.websocket != null)
        {
            await this.websocket.Close();
        }
    }

}