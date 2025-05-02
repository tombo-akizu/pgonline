using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using NativeWebSocket;

public class Connection : MonoBehaviour
{
    private WebSocket websocket;
    private InputObserver observer;
    private Rotate rotate;

    // Start is called once before the first execution of Update after the MonoBehaviour is created
    async void Start()
    {
        this.websocket = new WebSocket("ws://localhost:3000");
        this.observer = FindFirstObjectByType<InputObserver>();
        this.rotate = FindFirstObjectByType<Rotate>();

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
            Debug.Log("OnMessage!");
            switch (bytes[0])
            {
                case 0:
                    this.rotate.SetRotation(0);
                    break;
                case 1:
                    this.rotate.SetRotation(1);
                    break;
                case 2:
                    this.rotate.SetRotation(-1);
                    break;
            }
        };

        // Keep sending messages at every 0.3s
        this.InvokeRepeating(nameof(SendWebSocketMessage), 0.0f, 0.3f);

        // waiting for messages
        await this.websocket.Connect();
    }

    // Update is called once per frame
    void Update()
    {
#if !UNITY_WEBGL || UNITY_EDITOR
        this.websocket.DispatchMessageQueue();
#endif
    }

    async void SendWebSocketMessage()
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