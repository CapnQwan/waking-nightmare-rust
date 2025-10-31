# Waking-Nightmare - Editor

Welcome to the editor for Waking Nightmare.
The editor is designed for the user to be able to create games and animations with ease.
It gives the user the ability to create and add custom assets, components and systems to the engine to allow them the flexibility and

### Editor Structure:

- Engine Context
  - Reference to the current instance of the engine and it's logic
- Render Context
  - Holds window context, custom GL context from GLWN and custom Glow wrapper for rendering UI via EGUI
- Asset Context
  - Holds instances of all assets allowing the user to add Assets (textures, meshes, shaders...) to entities
  - Has the ability to watch an Asset folder to detect changes and update what assets are know within the context of the editor
  - Has the ability to load assets into the editor
- World Context
  - Holds references to all the entities and all the components applied to an entity within the world (Just taken from the Engine Context?)
- Inspector / Entity Context
  - Loads details about a selected Entity or Asset allowing the user to modify the focused item

### Notes to self

- Instead of rendering the whole UI as one big function or such maybe creating some kind of Frames similar to some of the demos would be better
- Inspector Frame, Assets Frame, World Frame?

### Tips for scaleability and a clean workspace

- Utilize traits to there maximum potential to allow flexible logic
- Seperation of concerns keep core logic seperate (Renderer, Assets Management, ECS, UI...)
- Embrace “Ports and Adapters” / “Hexagonal” architecture
- Decouple Systems via Message Passing or Events
- Use Trait Objects and Generics Wisely
- Dependency Injection via Traits

```
trait Logger {
    fn log(&self, msg: &str);
}
```

```
fn run_game<L: Logger>(logger: &L) {
    logger.log("Starting game..."); // This way depending on the logger passed to the function it could log to the UI or to a file or to the console
}
```
