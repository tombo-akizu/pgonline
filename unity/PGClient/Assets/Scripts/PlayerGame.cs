using System.Collections.Generic;
using UnityEngine;
using TMPro;

public class PlayerGame : MonoBehaviour
{
    [SerializeField] Bar[] bars;
    [SerializeField] GameObject bubblePrefab;
    [SerializeField] TextMeshProUGUI scoreText;
    [SerializeField] string playerName;

    private readonly List<Bubble> bubbles = new();

    public void ReflectGameState(GameState state)
    {
        int bubbleNum = state.GetBubbleNum();
        List<BubbleState> bubbleStates = state.GetBubbleStates();

        // BubbleのGameObjectの数を、受信したBubbleの数に合わせる。
        for (int i = 0; i < bubbleNum - this.bubbles.Count; i++)
        {
            this.bubbles.Add(
                Instantiate(this.bubblePrefab, this.transform)
                    .GetComponent<Bubble>()
            );
        }
        for (int i = 0; i < this.bubbles.Count - bubbleNum; i++)
        {
            Destroy(this.bubbles[0].gameObject);
            this.bubbles.RemoveAt(0);
        }

        // Bubblesの位置・色を、受信した情報に合わせる。
        for (int i = 0; i < bubbleNum; i++)
        {
            this.bubbles[i].ReflectBubbleState(bubbleStates[i]);
        }

        // Barの角度を、受信した角度にする。
        foreach (Bar bar in this.bars)
        {
            bar.SetAngle(state.GetAngle());
        }

        // ScoreのGUIを更新する。
        this.scoreText.SetText($"{this.playerName}: {state.GetScore()}");
    }
}
