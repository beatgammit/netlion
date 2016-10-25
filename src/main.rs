#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate netlion;

use std::thread;
use std::sync::{Arc, Mutex};

use piston_window::{EventLoop, PistonWindow, UpdateEvent, WindowSettings};
use netlion::*;

// Generate the ID for the Button COUNTER.
widget_ids! {
    struct Ids {
        canvas, text_box, start, proto_list, result,
    }
}

struct Netlion {
    port: String,
    sel_option: Option<usize>,
    net_mode: String,
    text: Arc<Mutex<String>>,
}

impl Netlion {
    fn new() -> Netlion {
        Netlion{
            port: String::from("8080"),
            sel_option: Some(1),
            net_mode: String::from("tcp"),
            text: Arc::new(Mutex::new(String::from("Welcome to netlion:\n"))),
        }
    }
}

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // Construct the window.
    let mut window: PistonWindow = WindowSettings::new("netlion", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();

    // Identifiers used for instantiating our widgets.
    let mut ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("conrod").unwrap();
    let font_path = assets.join("assets/fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_texture_cache =
conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);


    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    // let mut ui = {
    //     let theme = Theme::default();
    //     let glyph_cache = Glyphs::new(&font_path, window.factory.clone());
    //     Ui::new(glyph_cache.unwrap(), theme)
    // };

    let mut app = Netlion::new();

    window.set_ups(60);

    // Poll events from the window.
    while let Some(event) = window.next() {
        // Convert the piston event to a conrod event.
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            let mut ui = ui.set_widgets();

            set_widgets(&mut ui, &mut app, &mut ids);
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }
}

fn set_widgets(ui: &mut conrod::UiCell, app: &mut Netlion, ids: &mut Ids) {
     use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    // Create a background canvas upon which we'll place the button.
    widget::Canvas::new().pad(40.0).set(ids.canvas, ui);

    widget::TextBox::new(app.port.as_str())
        .top_left_of(ids.canvas)
        .w_h(200.0, 40.0)
        // .react(|s: &mut String|{println!("react: {}", s)})
        .set(ids.text_box, ui);

    let options = &mut vec![String::from("udp"), String::from("tcp")];

    // Draw the button and increment `count` if pressed.
    for selected_idx in widget::DropDownList::new(options, app.sel_option)
        .right_from(ids.text_box, 10.0)
        .w_h(150.0, 40.0)
        .set(ids.proto_list, ui)
    {
        app.sel_option = Some(selected_idx);
        app.net_mode = options[selected_idx].clone();
    }

    if widget::Button::new()
        .right_from(ids.proto_list, 10.0)
        .w_h(80.0, 40.0)
        .label(&String::from("Start"))
        .set(ids.start, ui).was_clicked()
    {
        let text = app.text.clone();
        start(app.net_mode.as_str(), &app.port, text);
    }

    widget::Text::new(app.text.lock().unwrap().as_str())
        .down_from(ids.text_box, 10.0)
        .color(color::Color::Rgba(0.5, 0.5, 0.5, 1.0))
        .align_text_left()
        .line_spacing(10.0)
        .set(ids.result, ui);
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
