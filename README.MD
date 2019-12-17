# Current Size:
**4,096 bytes**

# Current Features:
- [x] Renders two words

# How?
- Rust
- `#[no-std]`
- Using winuser rendering that's already built into Windows

# Credits
This was all set in motion by [this blog post](https://www.codeslow.com/2019/12/tiny-windows-executable-in-rust.html).
If it weren't for this blog post, I wouldn't have even attempted this craziness.

Thanks to [Ralith](https://github.com/Ralith) for making the `hecs` ECS library and helping me write a `#[no_std]` PR for it.

Thanks to [Lokathor](https://github.com/Lokathor) for explaining to me how broken some `#[no_std]` stuff is.

Thanks to [Fenrir](https://github.com/FenrirWolf) for the inspired idea to copy rust's libstd's windows allocator code directly.

Last but not least, thanks to [mr. bun bun](https://github.com/retep998) for writing the `winapi` crate
and helping me port the libstd's allocator to use its bindings instead of the internal Rust ones.
