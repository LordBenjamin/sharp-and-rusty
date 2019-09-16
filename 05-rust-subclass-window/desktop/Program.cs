using System;
using System.Runtime.InteropServices;
using System.Windows.Forms;

namespace Desktop {
    internal class Program {
        [STAThread]
        public static void Main(string[] args) {
            // Create a new window
            Form window = new Form();

            // Subclass the window using a Rust library
            NativeMethods.subclass(window.Handle); 

            // Run the event loop for the host application as normal
            Application.Run(window);
        }

        private static class NativeMethods
        {
            [DllImport("net_owned_window.dll")]
            public static extern void subclass(IntPtr handle);
        }
    }
}