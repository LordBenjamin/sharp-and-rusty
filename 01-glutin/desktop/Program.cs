using System;
using System.Diagnostics;
using System.Windows.Forms;

namespace Desktop {
    internal class Program {
        public static void Main(string[] args) {
            Form form = new Form();
            form.Text = "Sharp and Rusty";

            var table = new TableLayoutPanel();
            table.Dock = DockStyle.Fill;
            table.ColumnCount = 1;
            table.RowCount= 2;

            table.ColumnStyles.Add(new ColumnStyle(SizeType.AutoSize));
            table.RowStyles.Add(new RowStyle(SizeType.AutoSize));
            table.RowStyles.Add(new RowStyle(SizeType.AutoSize));

            var renderTarget = new Control();
            renderTarget.Dock = DockStyle.Fill;
            table.Controls.Add(renderTarget, 0, 1);

            var fpsLabel = new Label();
            table.Controls.Add(fpsLabel, 0, 0);

            form.Controls.Add(table);

            using (Renderer renderer = new Renderer(renderTarget.Handle)) {
                renderTarget.Paint += (sender, e) => {
                    Stopwatch sw = Stopwatch.StartNew();
                    renderer.Draw();
                    fpsLabel.Text = Math.Round(1.0 / sw.Elapsed.TotalSeconds, 2) + " FPS";
                };

                renderTarget.Resize += (sender, e) => renderer.Resize(renderTarget.Size);

                Application.Run(form);
            }
        }
    }
}