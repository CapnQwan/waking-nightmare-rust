# waking-nightmare - glwn

**GLWN** is a lightweight crate that provides all the **OpenGL ES 2.0 bindings** for the [Waking Nightmare](https://github.com/capn_qwan/waking-nightmare-rust) engine (or any other project that needs them).

This crate was extracted from the core engine crate so it can be reused across multiple targets — desktop, mobile, and web — without pulling in the rest of the engine dependencies.

---

## Overview

`glwn` uses [`gl_generator`](https://crates.io/crates/gl_generator) at build time to generate the full set of OpenGL ES 2.0 bindings into `OUT_DIR/gl_bindings.rs`.  
This gives you a consistent, platform-agnostic way to work with OpenGL functions, types, and constants across environments.

The bindings are exposed through the `gl` module and can be accessed via the `Gl` struct alias for convenience.

---

## Example

```rust
use glwn::gl::Gl;

fn draw(gl: &Gl) {
    unsafe {
        gl.ClearColor(0.1, 0.1, 0.1, 1.0);
        gl.Clear(gl::COLOR_BUFFER_BIT);
    }
}
```
