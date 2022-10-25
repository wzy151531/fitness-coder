use question::menu;

mod data_calculation;
mod i18n;
mod question;

pub fn run() {
    menu::init_before_select_menu();
    menu::select_menu();
}
