use beep::beep;
use std::{thread, time::Duration};

fn main() {
    beep_play();
}

fn beep_play() {
    beep(440);
    thread::sleep(Duration::from_millis(500));
    beep(880);
    thread::sleep(Duration::from_millis(500));
    beep(0);
}
