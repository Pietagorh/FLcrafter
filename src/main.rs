use xcap::Window;
use enigo::{
    Enigo, Key, Keyboard, Settings,
    Direction::{Press, Release},
};
use std::{thread, time::Duration};
use iced::{
    Element,
    widget::image,
    advanced::image::Handle,
};

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

#[derive(Debug, Clone)]
enum Message {
}

fn update(_counter: &mut u64, _message: Message) {
    ()
}

fn view(_counter: &u64) -> Element<Message> {
    let window = get_fantasy_life_window().unwrap();
    let ss = window.capture_image().unwrap();
    image(Handle::from_rgba(ss.width(), ss.height(), ss.into_vec())).into()
}

fn main() {
    let enigo = Enigo::new(&Settings::default()).unwrap();

    press_key_for(Key::Unicode('h'), 100, enigo);

    iced::run("Crafter", update, view).expect("GUI exited with error");
}
