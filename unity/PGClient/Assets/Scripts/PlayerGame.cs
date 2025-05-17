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

        // Bubble��GameObject�̐����A��M����Bubble�̐��ɍ��킹��B
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

        // Bubbles�̈ʒu�E�F���A��M�������ɍ��킹��B
        for (int i = 0; i < bubbleNum; i++)
        {
            this.bubbles[i].ReflectBubbleState(bubbleStates[i]);
        }

        // Bar�̊p�x���A��M�����p�x�ɂ���B
        foreach (Bar bar in this.bars)
        {
            bar.SetAngle(state.GetAngle());
        }

        // Score��GUI���X�V����B
        this.scoreText.SetText($"{this.playerName}: {state.GetScore()}");
    }
}
