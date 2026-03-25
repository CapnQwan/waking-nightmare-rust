use ecs::{Components, Entity, Resources, Systems, World};
use std::any::Any;
use std::sync::{
  atomic::{AtomicUsize, Ordering},
  Arc,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Position(i32, i32);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Velocity(i32, i32);

#[test]
fn world_spawns_and_recycles_entity_ids_lifo() {
  let mut world = World::new();

  let e1 = world.spawn_entity();
  let e2 = world.spawn_entity();
  assert_ne!(e1.id(), e2.id());

  // Recycle e2 first; the free-list is LIFO so the next spawn should reuse e2.
  world.delete_entity(e2);
  let e2_reuse = world.spawn_entity();
  assert_eq!(e2_reuse.id(), e2.id());

  // Recycling e1 should allow it to be reused as well.
  world.delete_entity(e1);
  let e1_reuse = world.spawn_entity();
  assert_eq!(e1_reuse.id(), e1.id());
}

#[test]
fn components_add_get_has_and_remove_work() {
  let mut components = Components::new();
  let e = Entity(123);

  assert!(!components.has_component::<Position>(&e));
  assert_eq!(components.get_component::<Position>(&e), None);

  components.add_component(&e, Position(10, 20));
  assert!(components.has_component::<Position>(&e));
  assert_eq!(components.get_component::<Position>(&e), Some(Position(10, 20)));

  // Different component type is independent.
  assert!(!components.has_component::<Velocity>(&e));
  components.add_component(&e, Velocity(1, -2));
  assert!(components.has_component::<Velocity>(&e));
  assert_eq!(components.get_component::<Velocity>(&e), Some(Velocity(1, -2)));

  components.remove_component::<Position>(&e);
  assert!(!components.has_component::<Position>(&e));
  assert_eq!(components.get_component::<Position>(&e), None);
  assert!(components.has_component::<Velocity>(&e));
}

#[test]
fn components_remove_entity_components_removes_from_all_storages() {
  let mut components = Components::new();
  let e = Entity(7);

  components.add_component(&e, Position(1, 2));
  components.add_component(&e, Velocity(3, 4));

  assert!(components.has_component::<Position>(&e));
  assert!(components.has_component::<Velocity>(&e));

  components.remove_entity_components(&e);

  assert!(!components.has_component::<Position>(&e));
  assert!(!components.has_component::<Velocity>(&e));
}

#[test]
fn resources_add_and_get_round_trips_via_any_box() {
  let mut resources = Resources::new();
  resources.add_resource::<usize>(123);

  let handle = resources.get_resource::<usize>().expect("resource must exist");
  let guard = handle.read().unwrap();

  // Resources are stored as `Box<dyn Any + Send + Sync>`, so we downcast to read.
  let value = guard.downcast_ref::<usize>().copied();
  assert_eq!(value, Some(123));
}

#[test]
fn systems_update_runs_all_systems_and_can_run_in_parallel() {
  let mut world = World::new();
  let mut systems = Systems::new();

  let counter = Arc::new(AtomicUsize::new(0));

  // Add a bunch of systems that all touch shared atomic state.
  // If `Systems::update` fails to run them, the final count will be wrong.
  let n = 64usize;
  for _ in 0..n {
    let c = Arc::clone(&counter);
    systems.add_system(move |_world: &World| {
      c.fetch_add(1, Ordering::Relaxed);
    });
  }

  systems.update(&mut world);

  assert_eq!(counter.load(Ordering::Relaxed), n);
}

#[test]
fn world_add_component_is_thread_safe_under_contention() {
  // This test is intentionally simple: it just tries to do concurrent writes
  // through the public `World` API and expects no panic/deadlock.
  let world = Arc::new(World::new());
  let entity = world.spawn_entity();

  let mut systems = Systems::new();

  let n = 32usize;
  for i in 0..n {
    let w = Arc::clone(&world);
    systems.add_system(move |_world: &World| {
      // Each system writes a component. Using a unique value helps debugging.
      w.add_component(&entity, Position(i as i32, -(i as i32)));
    });
  }

  // Run and assert we didn't deadlock/panic.
  // (We don't assert the final stored value because concurrent writes are last-writer-wins.)
  systems.update(&mut world.clone());
}