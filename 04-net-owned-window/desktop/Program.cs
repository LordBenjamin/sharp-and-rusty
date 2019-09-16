using System;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Threading;
using System.Windows.Forms;

namespace Desktop {
    internal class Program {
        [STAThread]
        public static void Main(string[] args) {
            // Create a new window
            Form window = new Form();
            window.Show();
            NativeMethods.run_event_loop(window.Handle);
        }

        private static class NativeMethods
        {
            [DllImport("net_owned_window.dll")]
            public static extern void run_event_loop(IntPtr handle);
        }
    }
}