# Daku v1.0.0-pre.0 (draft v13)

> Asynchronous host interface abstraction API for WebAssembly plugins, drivers,
> applications, and more! 

The `daku` crate is designed to be used for applications that run within a host,
such as a plugin in an application, or a driver in an operating system, and even
a program in an operation system (similar to WASI), and more!

## Goals
 
 - Simple
 - Efficient
 - Modular
 - Minimal (in API surface, and memory footprint)
 - Asynchronous
 - Backwards Compatible
 - Reduced context switching
 - Security-first
 - First-class multimedia portals (WASI compatible)

## License
Copyright © 2022-2024 The Daku Contributors.

Licensed under any of
 - Apache License, Version 2.0, ([LICENSE\_APACHE] or
   <https://www.apache.org/licenses/LICENSE-2.0>)
 - Boost Software License, Version 1.0, ([LICENSE\_BOOST] or
   <https://www.boost.org/LICENSE_1_0.txt>)
 - MIT License, ([LICENSE\_MIT] or <https://mit-license.org/>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.

## Help

If you want help using or contributing to this library or specification, feel
free to send me an email at <aldaronlau@gmail.com>.

[LICENSE\_APACHE]: https://github.com/ardaku/daku/blob/v1/LICENSE_APACHE
[LICENSE\_BOOST]: https://github.com/ardaku/daku/blob/v1/LICENSE_BOOST
[LICENSE\_MIT]: https://github.com/ardaku/daku/blob/v1/LICENSE_MIT
