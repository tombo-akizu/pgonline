using UnityEngine;

// UI演出を管理する。
public class StagingController : MonoBehaviour
{
    [SerializeField] GameObject startPrefab;
    [SerializeField] GameObject endPrefab;

    [SerializeField] Transform canvasTransform;

    // 開始演出
    public void StartStaging()
    {
        Instantiate(this.startPrefab, this.canvasTransform);
    }

    // 終了演出
    public void EndStaging()
    {
        Instantiate(this.endPrefab, this.canvasTransform);
    }
}
