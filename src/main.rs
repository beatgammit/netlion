#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate netlion;

use std::thread;
use std::sync::{Arc, Mutex};

use conrod::{Labelable, Positionable, Sizeable, Theme, Widget, Canvas, Text, TextBox, DropDownList, Button};
use conrod::color::{Color, Colorable};
use piston_window::{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings};
use netlion::*;

/// Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

fn main() {
    // Construct the window.
    let mut window: PistonWindow = WindowSettings::new("netlion", [800, 600])
        .exit_on_esc(true).build().unwrap();

    // construct our `Ui`.
    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("conrod").unwrap();
        let font_path = assets.join("assets/fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    let port = &mut String::from("8080");
    let mut net_mode = String::from("tcp");
    let options = &mut vec![String::from("udp"), String::from("tcp")];
    let mut sel_option = Some(1);
    let text = Arc::new(Mutex::new(String::from("Welcome to netlion:\n")));

    window.set_ups(60);

    // Poll events from the window.
    while let Some(event) = window.next() {
        ui.handle_event(&event);
        event.update(|_| ui.set_widgets(|mut ui| {
            let ui = &mut ui;

            // Generate the ID for the Button COUNTER.
            widget_ids!(CANVAS, TEXT_BOX, START, PROTO_LIST, RESULT);

            // Create a background canvas upon which we'll place the button.
            Canvas::new().pad(40.0).set(CANVAS, ui);

            TextBox::new(port)
                .top_left_of(CANVAS)
                .w_h(200.0, 40.0)
                .react(|s: &mut String|{println!("react: {}", s)})
                .set(TEXT_BOX, ui);

            // Draw the button and increment `count` if pressed.
            DropDownList::new(options, &mut sel_option)
                .right_from(TEXT_BOX, 10.0)
                .w_h(150.0, 40.0)
                .react(|selected_idx: &mut Option<usize>, new_idx: usize, s: &str| {
                    *selected_idx = Some(new_idx);
                    net_mode = String::from(s);
                })
                .set(PROTO_LIST, ui);

            Button::new()
                .right_from(PROTO_LIST, 10.0)
                .w_h(80.0, 40.0)
                .label(&String::from("Start"))
                .react(|| {
                    let text = text.clone();
                    start(net_mode.as_str(), port, text);
                })
                .set(START, ui);

            Text::new(text.lock().unwrap().as_str())
                .down_from(TEXT_BOX, 10.0)
                .color(Color::Rgba(0.5, 0.5, 0.5, 1.0))
                .align_text_left()
                .line_spacing(10.0)
                .set(RESULT, ui);
        }));

        window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
    }
}

fn start(mode: &str, port: &String, text: Arc<Mutex<String>>) {
    let text = text.clone();
    let host = "127.0.0.1";
    let port = port.parse::<u16>().unwrap();

    println!("Starting {} listener: {}:{}", mode, host, port);
    match mode {
        "tcp" => {thread::spawn(move || listen_tcp(host, port, text));},
        "udp" => {thread::spawn(move || listen_udp(host, port, text));},
        _ => println!("Listener type not recognized: {}", mode),
    };
}
