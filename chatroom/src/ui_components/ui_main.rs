use fltk::{
    enums::{Align, Color, Font, FrameType},
    prelude::*,
    *,
};
const BLUE: Color = Color::from_hex(0x42A5F5);
const SEL_BLUE: Color = Color::from_hex(0x2196F3);
const GRAY: Color = Color::from_hex(0x757575);
const WIDTH: i32 = 600;
const HEIGHT: i32 = 400;

pub struct UiMain {
    app: app::App,
    main_win: window::Window,
    pub sign_status: bool,
}
impl UiMain {
    pub fn new() -> Self {
        let app = app::App::default();
        let mut win = window::Window::default()
            .with_size(WIDTH, HEIGHT)
            .with_label("Welcome to chatroom!");
        let mut bar =
            frame::Frame::new(0, 0, WIDTH, 60, " Chatroom").with_align(Align::Left | Align::Inside);
        let mut text = frame::Frame::default()
            .with_size(100, 40)
            .center_of(&win)
            .with_label("Choose your friends!");
        let mut count = frame::Frame::default()
            .size_of(&text)
            .below_of(&text, 0)
            .with_label("0");
        let mut button_plus = button::Button::new(WIDTH - 100, HEIGHT - 100, 60, 60, "@+6plus");

        // Theming
        app::background(255, 255, 255);
        app::set_visible_focus(false);

        bar.set_frame(FrameType::FlatBox);
        bar.set_label_size(22);
        bar.set_label_color(Color::White);
        bar.set_color(BLUE);
        bar.draw(|b| {
            draw::set_draw_rgb_color(211, 211, 211);
            draw::draw_rectf(0, b.height(), b.width(), 3);
        });

        text.set_label_size(18);
        text.set_label_font(Font::Times);

        count.set_label_size(36);
        count.set_label_color(GRAY);

        button_plus.set_color(BLUE);
        button_plus.set_selection_color(SEL_BLUE);
        button_plus.set_label_color(Color::White);
        button_plus.set_frame(FrameType::OFlatFrame);
        // End theming

        button_plus.set_callback(move |_| {
            let label = (count.label().parse::<i32>().unwrap() + 1).to_string();
            count.set_label(&label);
        });
        Self {
            app,
            main_win: win,
            sign_status: false,
        }
    }

    pub fn ui_main_lanuch(&mut self) {
        &self.main_win.end();
        &self.main_win.make_resizable(true);
        &self.main_win.show();
        &self.app.run().unwrap();
    }
}
