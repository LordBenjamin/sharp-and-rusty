
using System;
using System.Windows.Forms;

namespace Desktop {
        internal class CustomApplicationContext : ApplicationContext
        {
            public CustomApplicationContext(WinitWindow window)
            {
                window.Closed += (sender, e) => {
                    Console.WriteLine("Window closed");
                     ExitThread();
                };
            }
        }
}