using System.Windows.Forms;

namespace Desktop
{
    internal class Program
    {
        public static void Main(string[] args)
        {
            Form form = new Form();
            form.Text = "Sharp and Rusty";

            using(Renderer renderer = new Renderer(form.Handle)) {
                form.Paint += (sender, e) => renderer.Draw();
                form.Resize += (sender, e) =>
                renderer.Resize(form.Size);

                Application.Run(form);
            }
        }
    }
}
