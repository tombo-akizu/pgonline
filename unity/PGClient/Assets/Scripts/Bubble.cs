using UnityEngine;

public class Bubble : MonoBehaviour
{
    private MeshRenderer meshRenderer;

    // Start is called once before the first execution of Update after the MonoBehaviour is created
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
