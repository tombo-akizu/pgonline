using UnityEngine;
using System.Collections.Generic;
using System;

// データ型。サーバから受け取ったゲームの状態を扱う。
public class DecodedState
{
    private readonly GameState[] states = new GameState[2];

    private enum PlayerIndex
    {
        Player,
        Opponent
    }

    public DecodedState(byte[] data)
    {
        int currentByte = 0;

        for (int i = 0; i < states.Length; i++)
        {
            float angle = BitConverter.ToSingle(data, currentByte);
            currentByte += 4;

            byte score = data[currentByte];
            currentByte += 1;

            byte bubbleNum = data[currentByte];
            currentByte += 1;

            List<BubbleState> bubbleStates = new();

            for (int j = 0; j < bubbleNum; j++)
            {
                float positionX = BitConverter.ToSingle(data, currentByte);
                currentByte += 4;

                float positionY = BitConverter.ToSingle(data, currentByte);
                currentByte += 4;

                byte color = data[currentByte];
                currentByte += 1;

                bubbleStates.Add(
                    new BubbleState(positionX, positionY, color)
                );
            }

            this.states[i] = new GameState(angle, score, bubbleStates);
        }
    }

    public GameState GetPlayerState()
    {
        return this.states[(int)PlayerIndex.Player];
    }

    public GameState GetOpponentState()
    {
        return this.states[(int)PlayerIndex.Opponent];
    }
}

public class GameState
{
    private readonly float angle;
    private readonly byte score;
    private readonly List<BubbleState> bubbleStates;

    public GameState(float angle, byte score, List<BubbleState> bubbleStates)
    {
        this.angle = angle;
        this.score = score;
        this.bubbleStates = bubbleStates;
    }

    public byte GetBubbleNum()
    {
        return (byte)this.bubbleStates.Count;
    }

    public List<BubbleState> GetBubbleStates()
    {
        return this.bubbleStates;
    }

    public float GetAngle()
    {
        return this.angle;
    }

    public byte GetScore()
    {
        return this.score;
    }
}

public class BubbleState
{
    private readonly Vector3 position;
    private readonly Color color;

    public BubbleState(float positionX, float positionY, byte color)
    {
        this.position = new Vector3(positionX, positionY, 0);
        if (color == 0x00)
        {
            this.color = Color.red;
        }
        else
        {
            this.color = Color.white;
        }
    }

    public Vector3 GetPosition()
    {
        return this.position;
    }

    public Color GetColor()
    {
        return this.color;
    }
}
