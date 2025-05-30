using UnityEngine;
using UnityEngine.InputSystem;

public class InputObserver : MonoBehaviour
{
    private bool rightIsInputed = false;
    private bool leftIsInputed = false;

    private InputAction move;

    private void Awake()
    {
        this.move = FindFirstObjectByType<PlayerInput>().actions["Move"];
    }

    private void Update()
    {
        Vector2 input = this.move.ReadValue<Vector2>();
        this.rightIsInputed = input.x > 0;
        this.leftIsInputed = input.x < 0;
    }

    public byte[] PopInputByte()
    {
        byte output = 0;    // Output of this method.
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
