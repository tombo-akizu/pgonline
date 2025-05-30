using UnityEngine;

public class Bubble : MonoBehaviour
{
    private MeshRenderer meshRenderer;

    void Awake()
    {
        this.meshRenderer = GetComponentInChildren<MeshRenderer>();
    }

    public void ReflectBubbleState(BubbleState state)
    {
        this.transform.localPosition = state.GetPosition();
        this.meshRenderer.material.color = state.GetColor();
    }
}
