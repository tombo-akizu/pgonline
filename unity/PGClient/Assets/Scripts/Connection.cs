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

    [SerializeField] Transform[] cubes;

    // Start is called once before the first execution of Update after the MonoBehaviour is created
    private async void Start()
    {
        this.websocket = new WebSocket("ws://localhost:3000");
        this.observer = FindFirstObjectByType<InputObserver>();

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

            float x0 = BitConverter.ToSingle(bytes, 0);
            float z0 = BitConverter.ToSingle(bytes, 4);
            float x1 = BitConverter.ToSingle(bytes, 8);
            float z1 = BitConverter.ToSingle(bytes, 12);

            this.cubes[0].position = new Vector3(x0, 0, z0);
            this.cubes[1].position = new Vector3(x1, 0, z1);
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