using UnityEngine;

public class StartUI : MonoBehaviour
{
    // アニメーションから呼び出される。
    public void EndAnimation()
    {
        Destroy(this.gameObject);
    }
}
