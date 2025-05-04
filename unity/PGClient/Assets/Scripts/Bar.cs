using UnityEngine;

public class Bar : MonoBehaviour
{
    private float angle = 0;

    // Update is called once per frame
    private void Update()
    {
        this.transform.rotation = Quaternion.Euler(new Vector3(0, 0, angle));
    }

    public void SetAngle(float angle)
    {
        this.angle = angle;
    }
}
