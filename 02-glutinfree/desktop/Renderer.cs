using System;
using System.Drawing;
using System.Runtime.InteropServices;
using Microsoft.Win32.SafeHandles;

namespace Desktop {
    // Provides a managed wrapper around the unmanaged Renderer type defined in Rust
    internal class Renderer : IDisposable {
        private RendererHandle handle;

        public Renderer(IntPtr hwnd, Size size) {
            handle = NativeMethods.create_renderer(hwnd, size.Width, size.Height);
        }

        public void Draw() => NativeMethods.renderer_draw(handle);

        public void Resize(int width, int height) => NativeMethods.renderer_resize(handle, width, height);

        public void Resize(Size size) => Resize(size.Width, size.Height);

        public void Dispose() => Dispose(true);

        protected virtual void Dispose(bool disposing) {
            if (!handle.IsClosed && !handle.IsInvalid) {
                handle.Dispose();
            }
        }

        // Strong typing and memory safety for our unmanaged pointer to the Renderer struct defined in Rust
        private class RendererHandle : SafeHandleZeroOrMinusOneIsInvalid {
            public RendererHandle() : base(true) {
            }

            protected override bool ReleaseHandle() {
                NativeMethods.destroy_renderer(handle);
                return true;
            }
        }

        // Define our C ABI exported from Rust as C# extern methods
        private static class NativeMethods {
            [DllImport("rust_gl.dll")]
            public static extern RendererHandle create_renderer(IntPtr hwnd, double width, double height);

            [DllImport("rust_gl.dll")]
            public static extern void renderer_draw(RendererHandle renderer);

            [DllImport("rust_gl.dll")]
            public static extern void renderer_resize(RendererHandle renderer, double width, double height);

            [DllImport("rust_gl.dll")]
            public static extern RendererHandle destroy_renderer(IntPtr hwnd);
        }
    }
}