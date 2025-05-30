using UnityEngine;

public class Bar : MonoBehaviour
{
    public void SetAngle(float angle)
    {
        this.transform.rotation = Quaternion.Euler(new Vector3(0, 0, angle));
    }
}
