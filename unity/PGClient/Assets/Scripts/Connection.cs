using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using NativeWebSocket;

public class Connection : MonoBehaviour
{
    private const float FPS = 1f / 30;

    private WebSocket websocket;
    private InputObserver observer;

    [SerializeField] PlayerGame playerGame;
    [SerializeField] PlayerGame opponentGame;

    // Start is called once before the first execution of Update after the MonoBehaviour is created
    private async void Start()
    {
        this.websocket = new WebSocket("ws://54.150.123.87:8080");
        //this.websocket = new WebSocket("ws://localhost:8080");
        this.observer = FindFirstObjectByType<InputObserver>();

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
            DecodedState state = new(bytes);
            this.playerGame.ReflectGameState(state.GetPlayerState());
            this.opponentGame.ReflectGameState(state.GetOpponentState());
        };

        this.InvokeRepeating(nameof(SendWebSocketMessage), 0.0f, FPS);

        // waiting for messages
        await this.websocket.Connect();
    }

    // Update is called once per frame
    private void Update()
    {
#if !UNITY_WEBGL || UNITY_EDITOR
        this.websocket.DispatchMessageQueue();
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
        await this.websocket.Close();
    }

}