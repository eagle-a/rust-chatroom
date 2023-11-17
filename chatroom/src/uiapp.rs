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

pub fn app_sign_in() {}

pub fn app_login() {
    let app = app::App::default();
    app::background(255, 255, 255);
    app::set_visible_focus(false);
    let mut window = window::Window::default()
        .with_size(400, 300)
        .with_label("Login and Registration");

    let username_entry =
        input::IntInput::new(100, 50, 100, 20, "username").set_label_color(Color::Blue);
    let password_entry =
        input::IntInput::new(100, 100, 100, 20, "password").set_label_color(Color::Blue);
    let mut submit_btn =
        button::Button::new(180, 120, 60, 25, "Submit").with_align(Align::Center | Align::Inside);

    // username_label.set_frame(FrameType::FlatBox);
    // username_label.set_label_color(Color::White);
    // username_label.set_color(BLUE);
    // username_label.draw(|b| {
    //     draw::set_draw_rgb_color(211, 211, 211);
    //     draw::draw_rectf(0, b.height(), b.width(), 3);
    // });

    submit_btn.set_callback(move |submit_btn| {
    let app_wrong = app::App::default();
    app::background(255, 255, 255);
    app::set_visible_focus(false);
    let mut window_wrong = window::Window::default()
        .with_size(200, 100)
        .with_label("Login and Registration");
    let mut bar =
    frame::Frame::new(0, 0, WIDTH, 60, " 密码错误").with_align(Align::Left | Align::Inside);
    window_wrong.end();
    window_wrong.make_resizable(true);
    window_wrong.show();
    app_wrong.run().unwrap();
    });

    let mut login_group = group::Group::default()
        .with_size(300, 200)
        .center_of(&window); // Create a group to align widgets vertically

    login_group.add(&submit_btn);
    login_group.end(); // End the group and align the widgets vertically

    window.add(&login_group); // Add the group to the window
    window.show(); // Show the window

    app.run().unwrap(); // Run the application and wait for events to occur
}

pub fn app_main_ui() {
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
    win.end();
    win.make_resizable(true);
    win.show();

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
    app.run().unwrap();
}
