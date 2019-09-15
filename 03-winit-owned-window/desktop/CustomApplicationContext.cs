
using System;
using System.Windows.Forms;

namespace Desktop {
    // Use a custom ApplicationContext implementation to set the conditions for the event loop exiting.
    // We're listening for the native window raising the Closed event, and then notifying the event loop that it's
    // time to exit/
    internal class CustomApplicationContext : ApplicationContext {
        public CustomApplicationContext(WinitWindow window) {
            window.Closed += (sender, e) => {
                Console.WriteLine("Window closed");
                ExitThread();
            };
        }
    }
}