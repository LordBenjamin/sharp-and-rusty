using System;
using System.Windows.Forms;
using Core;

namespace desktop
{
    class Program
    {
        static void Main(string[] args)
        {
            Form form = new Form();
            form.Show();

            IntPtr renderer = NativeMethods.create_renderer(form.Handle);

            form.Paint += (sender, e) => NativeMethods.renderer_draw(renderer);

            Application.Run(form);
        }
    }
}
