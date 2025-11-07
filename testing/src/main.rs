use events::event::Event;

fn main() {
  let mut event: Event<i32> = Event::new();

  event.subscribe(|index| print!("This was emited {} \n", index));
  event.subscribe(|index| print!("This was two {} \n", index));

  event.emit(&11);

  event.subscribe(|index| print!("This was emited later {} \n", index));

  event.emit(&23);

  print!("test");
}
