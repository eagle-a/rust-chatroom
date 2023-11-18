
use crate::ui_components;


pub fn ui_run() -> bool {
    let mut login_app = crate::ui_components::login::LoginStruct::new();
    login_app.login_start();
    let mut login_status = login_app.sign_status;
  //  loop {
        //let sign_status:bool = false;
        login_status = login_app.login_update();
      //  login_status = login_app.sign_status;
        if login_status {
            login_app.login_end();
          //  break;
     //  }
    }
    let mut  uimain = crate::ui_components::ui_main::UiMain::new();
    uimain.ui_main_lanuch();
    //ui_main::ui_main_lanuch();
    login_status
}
