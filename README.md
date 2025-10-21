# Waking-Nightmare

## What is Waking-Nightmare

Waking-Nightmare is a modular, light weight game / render engine build in Rust. this engine is an ECS (entity, component, system) at it's core as I believe this fits the nature of rust.

## Why make Waking-Nightmare

I started making Waking-Nightmare as a proof of concept and a learning experience. Originally it was built in Typescript but with the natural restricitions of ts / js I wanted to compare how a WASM based engine would compare to a typescript based engine.

#### Items I want to explore

- I want to compare bundle sizes of typescript version with a WASM version
- I want to compare compile times and total blocking time (TBT)
- I want to see how much performance improves when leveraging Rust’s parallelism and low-level control

## Current Status

**_NOTE: this project is currently highly experimential and under heavy development._**
**_Many systems are still being reorganized or rewritten. Expect breaking changes, incomplete features, and temporary code._**

In its current state, Waking-Nightmare serves as a sandbox for experimentation — major subsystems will likely be refactored and decoupled as development continues.

### Current goal

The immediate goal is to get a basic working engine with:

- A simple editor interface
- Working desktop and WASM builds
- Minimal core functionality (rendering, input, ECS, materials, etc.)
- A clean, modular architecture that’s easy to extend later

### Future Goals / Roadmap

- Add core editor functionallity
  - Add / Edit entities
  - Add and modify components
  - Register and Manage systems
- Decouple core engine systems (renderer, ECS...)
- Expand shader support and reflection
- Add documentation
- Add build support for building a project to either WASM or Desktop
- Add some kind of serialization for saving and loading projects
- Add some kind of serialization for reading and displaying component data to in editor
- Add some examples and demos to help explain architecture
- Add Meta data / better intergration with the editor so the editor can deconstruct
- Add better support for shaders

## Quick Start

First clone the repo to your local machine.

You can run the editor via
`cargo run -p editor`

You can run the desktop version via
`cargo run -p desktop`

**Wasm support is yet to be properly implemented**

Once implemented you can run the wasm version via
`cargo run -p desktop`

## Contributing

Right now, Waking-Nightmare is a personal learning project — but feedback, discussions, and ideas are very welcome. If you’d like to contribute or test, feel free to open an issue or PR.

## License

Licensed under the MIT License — free to use, modify, and distribute.

## Final notes

This project is still finding its shape — think of it as a growing skeleton for something more ambitious.
The goal isn’t to compete with engines like Unity or Unreal, but to understand what makes them tick and to experiment with a smaller, hackable foundation built in Rust.
