use gtk::glib;

use adw::prelude::*;

use glib::object::MayDowncastTo;

pub(crate) fn get_window_shell<T>(win: &T) -> Option<gtk::Widget>
where
    T: IsA<gtk::Window> + MayDowncastTo<adw::ApplicationWindow>,
{
    match win.downcast_ref::<adw::ApplicationWindow>() {
        Some(adw_win) => adw_win.content(),
        None => win.child(),
    }
}
