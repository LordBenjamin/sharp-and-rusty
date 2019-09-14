using System;
using System.Runtime.InteropServices;
using System.Threading;
using System.Windows.Forms;

namespace Desktop {
    // Provides a managed wrapper around the unmanaged Renderer type defined in Rust
    [System.Security.Permissions.PermissionSet(System.Security.Permissions.SecurityAction.Demand, Name = "FullTrust")]
    internal class WinitWindow : NativeWindow {
        public event FormClosedEventHandler Closed;

        // Constant value from "windows.h" header file.
        private const int WM_CLOSE = 0x0010;
        private const int WM_SIZE = 0x0005;

        private IntPtr hwnd;

        public WinitWindow() {
            // Call Rust library to create a window and store the resulting hwnd
            IntPtr pointer = NativeMethods.create_window();
            // Window is now created, assign handle to NativeWindow.
            WindowWrapper wrapper = Marshal.PtrToStructure<WindowWrapper>(pointer);
            AssignHandle(wrapper.hwnd);
        }

        [System.Security.Permissions.PermissionSet(System.Security.Permissions.SecurityAction.Demand, Name = "FullTrust")]
        protected override void WndProc(ref Message m) {
            // Listen for operating system messages
            switch (m.Msg) {
                case WM_CLOSE:
                    Closed?.Invoke(this, new FormClosedEventArgs(CloseReason.FormOwnerClosing));
                    break;

                case WM_SIZE:
                    int lparam = m.LParam.ToInt32();
                    int height = lparam >> 16;
                    int width = lparam & 0xfff;

                    Console.WriteLine("Size: " + width + "x" + height);
                    break;

            }
            base.WndProc(ref m);
        }

        // Define our C ABI exported from Rust as C# extern methods
        private static class NativeMethods {
            [DllImport("winit_owned_window.dll")]
            public static extern IntPtr create_window();
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct WindowWrapper {
            public IntPtr hwnd;
        }
    }
}