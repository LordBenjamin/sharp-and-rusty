using System;
using System.Runtime.InteropServices;

namespace Core {
    public static class NativeMethods {
        [DllImport("rust_gl.dll")]
        public static extern IntPtr create_renderer(IntPtr hwnd);

        [DllImport("rust_gl.dll")]
        public static extern void renderer_draw(IntPtr renderer);
    }
}