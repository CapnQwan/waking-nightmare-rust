use crate::World;
use std::thread;

/// A trait object wrapper for "something that can be run as a system".
///
/// Why not store `FnMut(&World)` directly?
/// - We want a single object-safe trait so we can later add metadata:
///   access patterns, names, ordering constraints, profiling hooks, etc.
pub trait SystemFn: Send + 'static {
  /// Run the system once.
  fn run(&mut self, world: &World);
}

/// Blanket implementation so plain closures can be used as systems.
///
/// Requirements:
/// - `Send` because we may run systems on worker threads.
/// - `'static` because systems are stored in a long-lived list.
impl<F> SystemFn for F
where
  F: FnMut(&World) + Send + 'static,
{
  fn run(&mut self, world: &World) {
    (self)(world);
  }
}

/// Container for all systems belonging to an ECS instance.
///
/// Current behavior:
/// - Runs all systems in parallel each update.
///
/// Important note:
/// - Running "everything in parallel" is correct due to locks, but it may be:
///   - slow due to contention
///   - nondeterministic in gameplay behavior (ordering-sensitive logic)
///
/// Next step:
/// - Add per-system access declarations and schedule/batch systems to reduce contention
///   and keep deterministic ordering where required.
pub struct Systems {
  systems: Vec<Box<dyn SystemFn>>,
}

impl Systems {
  /// Create an empty system list.
  pub fn new() -> Self {
    Systems { systems: Vec::new() }
  }

  /// Add a system to the list.
  ///
  /// Systems are stored in insertion order, though current parallel execution
  /// does not enforce order.
  pub fn add_system<F>(&mut self, f: F) -> &mut Self
  where
    F: FnMut(&World) + Send + 'static,
  {
    self.systems.push(Box::new(f));
    self
  }

  /// Run one update tick.
  ///
  /// Implementation:
  /// - Uses `thread::scope` so spawned threads can borrow `&World`.
  /// - Each system runs in its own scoped thread.
  ///
  /// Future:
  /// - Replace "thread per system" with a thread pool or work-stealing executor.
  /// - Add a scheduler to batch compatible systems.
  pub fn update(&mut self, world: &World) {
    thread::scope(|s| {
      for sys in &mut self.systems {
        s.spawn(move || {
          sys.run(world);
        });
      }
    });
  }
}
