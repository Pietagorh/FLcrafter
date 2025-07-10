use xcap::Window;
use enigo::{
    Enigo, Key, Keyboard, Settings,
    Direction::{Press, Release},
};
use std::{thread, time::Duration};

fn get_fantasy_life_window() -> Result<Window, String> {
    let windows = Window::all().unwrap();

    for window in windows {
        if window.title().unwrap().contains("Fantasy Life") {
            return Ok(window);
        }
    }
    Err("Could not find window".to_string())
}

fn press_key_for(key: Key, ms: u64, mut enigo: Enigo) {
    // Not using `Click` because Citra is a baby that wants inputs held for longer than that
    // (i think) (anyway Click doesn't work with Citra)
    enigo.key(key, Press).unwrap();
    thread::sleep(Duration::from_millis(ms));
    enigo.key(key, Release).unwrap();
}

fn main() {
    let window = get_fantasy_life_window().unwrap();

    let enigo = Enigo::new(&Settings::default()).unwrap();

    press_key_for(Key::Unicode('h'), 100, enigo);

    window.capture_image();
    // wip
}
