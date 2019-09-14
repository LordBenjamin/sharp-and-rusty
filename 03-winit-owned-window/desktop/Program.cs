using System;
using System.Diagnostics;
using System.Threading;
using System.Windows.Forms;

namespace Desktop {
    internal class Program {
        [STAThread]
        public static void Main(string[] args) {
            WinitWindow window = new WinitWindow();
            CustomApplicationContext context = new CustomApplicationContext(window);
            Application.Run(context); // Run event loop
        }
    }
}