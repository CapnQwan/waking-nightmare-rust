use events::event::Event;

fn main() {
  let mut event: Event<i32> = Event::new();

  event.subscribe(|index| print!("\n\nThis was emited {} \n", index));
  let event_id = event.subscribe(|index| print!("This was two {} \n", index));

  event.emit(&11);

  event.subscribe(|index| print!("This was emited later {} \n", index));

  event.emit(&23);

  event.unsubscribe(event_id);

  event.emit(&35);

  event.clear();

  event.emit(&42);

  event.subscribe(|index| print!("New sub {} \n", index));

  event.emit(&66);

  print!("test complete");
}
