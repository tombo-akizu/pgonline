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

    [SerializeField] Bar[] bars1;
    [SerializeField] Bar[] bars2;

    [SerializeField] GameObject bubblePrefab;
    [SerializeField] Transform p0Base;
    [SerializeField] Transform p1Base;

    // Start is called once before the first execution of Update after the MonoBehaviour is created
    private async void Start()
    {
        this.websocket = new WebSocket("ws://localhost:8080");
        this.observer = FindFirstObjectByType<InputObserver>();

        List<GameObject> bubbleObjects0 = new List<GameObject>();
        List<GameObject> bubbleObjects1 = new List<GameObject>();

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

            Debug.Log("length: " + bytes.Length.ToString());

            float angle0 = BitConverter.ToSingle(bytes, 0);
            uint bubbles0 = BitConverter.ToUInt32(bytes, 4);
            List<Vector3> bubblePositions0 = new List<Vector3>();
            for (int i = 0; i < bubbles0; i++)
            {
                bubblePositions0.Add(new Vector3(BitConverter.ToSingle(bytes, 8 + 8 * i), BitConverter.ToSingle(bytes, 8 + 8 * i + 4), 0));
            }
            int baseIndex = 8 + 8 * (int)bubbles0;
            float angle1 = BitConverter.ToSingle(bytes, baseIndex);
            uint bubbles1 = BitConverter.ToUInt32(bytes, baseIndex + 4);
            List<Vector3> bubblePositions1 = new List<Vector3>();
            for (int i = 0; i < bubbles1; i++)
            {
                bubblePositions1.Add(new Vector3(BitConverter.ToSingle(bytes, baseIndex + 8 + 8 * i), BitConverter.ToSingle(bytes, baseIndex + 12 + 8 * i), 0));
            }

            // BubbleのGameObjectの数を、受信したBubbleの数に合わせる。
            for (int i = 0; i < bubbles0 - bubbleObjects0.Count; i++)
            {
                bubbleObjects0.Add(Instantiate(this.bubblePrefab, this.p0Base));
            }
            for (int i = 0; i < bubbles1 - bubbleObjects1.Count; i++)
            {
                bubbleObjects1.Add(Instantiate(this.bubblePrefab, this.p1Base));
            }
            for (int i = 0; i < bubbleObjects0.Count - bubbles0; i++)
            {
                Destroy(bubbleObjects0[0]);
                bubbleObjects0.RemoveAt(0);
            }
            for (int i = 0; i < bubbleObjects1.Count - bubbles1; i++)
            {
                Destroy(bubbleObjects1[0]);
                bubbleObjects1.RemoveAt(0);
            }

            // Bubblesの位置を、受信した位置にする。
            for (int i = 0; i < bubbles0; i++)
            {
                bubbleObjects0[i].transform.localPosition = bubblePositions0[i];
            }
            for (int i = 0; i < bubbles1; i++)
            {
                bubbleObjects1[i].transform.localPosition = bubblePositions1[i];
            }

            // Barの角度を、受信した角度にする。
            foreach (Bar bar in this.bars1)
            {
                bar.SetAngle(angle0);
            }
            foreach (Bar bar in this.bars2)
            {
                bar.SetAngle(angle1);
            }

            Debug.Log("angle0: " + angle0.ToString());
            Debug.Log("angle1: " + angle1.ToString());
            Debug.Log("bubbles0: " + bubbles0.ToString());
            Debug.Log("bubbles1: " + bubbles1.ToString());
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