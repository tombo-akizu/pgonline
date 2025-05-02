using UnityEngine;

public class InputObserver : MonoBehaviour
{
    private bool rightIsInputed = false;
    private bool leftIsInputed = false;

    // Update is called once per frame
    private void Update()
    {
        if (Input.GetKey(KeyCode.RightArrow))
        {
            this.rightIsInputed = true;
        }
        if (Input.GetKey(KeyCode.LeftArrow))
        {
            this.leftIsInputed = true;
        }
    }

    public byte[] PopInputByte()
    {
        byte output = 0;
        if (this.rightIsInputed)
        {
            output |= 1 << 0;
        }
        if (this.leftIsInputed)
        {
            output |= 1 << 1;
        }
        this.rightIsInputed = false;
        this.leftIsInputed = false;

        return new byte[] { output };
    }
}
