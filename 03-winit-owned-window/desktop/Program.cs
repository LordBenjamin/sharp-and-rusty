using System;
using System.Diagnostics;
using System.Threading;
using System.Windows.Forms;

namespace Desktop {
    internal class Program {
        [STAThread]
        public static void Main(string[] args) {
            // Create a new window
            WinitWindow window = new WinitWindow();

            // Create a custom ApplicationContext that will exit when the native window is closed
            CustomApplicationContext context = new CustomApplicationContext(window);

            // Run event loop
            // TODO: Trying to figure out how to do this on the Rust side
            Application.Run(context); 
        }
    }
}