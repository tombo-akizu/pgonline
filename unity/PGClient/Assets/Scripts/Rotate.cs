using UnityEngine;

public class Rotate : MonoBehaviour
{
    private int rotation = 0;

    // Update is called once per frame
    private void Update()
    {
        this.transform.Rotate(0, this.rotation, 0);
    }

    public void SetRotation(int rotation)
    {
        this.rotation = rotation;
    }
}
