# sharp-and-rusty
C# / .NET Core / WinForms application hosting an OpenGL triangle renderer written in Rust / Glutin.

There is also a full Rust implementation in `main.rs`.

* Build the `rust` crate using `cargo build`
* Build / run the `desktop` program using `dotnet run`

Rust / Glutin code inspired by https://gist.github.com/matthewjberger/9da00592b472b50ec1e6b3238719264b, although it required a lot of TLC to get it to compile against the latest Glutin crate.
