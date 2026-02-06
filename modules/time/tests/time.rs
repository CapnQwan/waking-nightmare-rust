use std::thread;
use std::time::Duration;

use time::Time;

#[test]
fn new_starts_with_zero_delta_time() {
    let t = Time::new();
    assert_eq!(t.delta_time(), 0.0);
}

#[test]
fn tick_sets_non_negative_delta_time() {
    let mut t = Time::new();
    t.tick();
    assert!(t.delta_time().is_finite());
    assert!(t.delta_time() >= 0.0);
}

#[test]
fn tick_after_sleep_produces_positive_delta_time() {
    let mut t = Time::new();

    // Ensure measurable time passes on most systems/CI environments.
    thread::sleep(Duration::from_millis(5));

    t.tick();
    let dt = t.delta_time();

    assert!(dt > 0.0, "expected dt > 0 after sleeping; got {dt}");
    assert!(dt < 1.0, "dt unexpectedly large (test likely stalled); got {dt}");
}

#[test]
fn consecutive_ticks_do_not_go_backwards() {
    let mut t = Time::new();

    thread::sleep(Duration::from_millis(1));
    t.tick();
    let dt1 = t.delta_time();

    t.tick();
    let dt2 = t.delta_time();

    assert!(dt1 >= 0.0);
    assert!(dt2 >= 0.0);
}