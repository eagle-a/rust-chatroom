use fltk::{
    enums::{Align, Color, Font, FrameType},
    prelude::*,
    *,
};

pub struct LoginStruct {
    app: app::App,
    main_win: window::Window,
  pub sign_status: bool,
    //username_entry,
    //  password_entry,
    //receiver: app::Receiver<Message>,
}

impl LoginStruct {
    pub fn new() -> Self {
     //   let count = 0;
        let app = app::App::default();
        app::background(255, 255, 255);
        let mut main_win = window::Window::default()
            .with_size(400, 300)
            .with_label("Login and Registration");

        let username_entry = input::IntInput::default()
            .with_pos(100, 50)
            .with_size(100, 20)
            .with_label("username:");

        let password_entry = input::IntInput::default()
            .with_pos(100, 100)
            .with_size(100, 20)
            .with_label("password:");

        let mut submit_btn = button::Button::new(180, 120, 60, 25, "Submit")
            .with_align(Align::Center | Align::Inside);

        let  sign_status: bool =
            (password_entry.value() == "1") && (username_entry.value() == "1");

        submit_btn.set_callback(move |submit_btn| {
            let app_wrong = app::App::default();
            app::background(255, 255, 255);
            app::set_visible_focus(false);
            let mut window_wrong = window::Window::default()
                .with_size(200, 100)
                .with_label("Login and Registration");

            let bar = frame::Frame::new(0, 0, 0, 60, "密码错误")
                .center_of(&window_wrong)
                .with_align(Align::Left | Align::Inside);
            if !sign_status
            {
                window_wrong.show();
                app_wrong.run().unwrap();
            }
        });
        let mut login_group = group::Group::default()
            .with_size(300, 200)
            .center_of(&main_win); // Create a group to align widgets vertically

        login_group.add(&username_entry);
        login_group.add(&password_entry);
        login_group.add(&submit_btn);
        login_group.end(); // End the group and align the widgets vertically

        main_win.add(&login_group); // Add the group to the window
        Self {
            app,
            main_win,
            sign_status,
        }
    }

    pub fn login_start(&mut self) -> bool {
        // self.main_win.login_group.password
        self.main_win.show();
        self.app.run().unwrap();
        self.sign_status
    }
    pub fn login_update(&self) -> bool {
        self.app.run().unwrap();
        self.sign_status
    }

    pub fn login_end(&mut self) {
        self.main_win.hide();
        self.app.quit();
    }
}
