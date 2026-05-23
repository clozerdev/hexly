use adw::prelude::AdwApplicationWindowExt;

use gtk::glib::object::{CastNone, MayDowncastTo};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

/// A trait to be used by shell widgets.
pub trait CommonShell: WidgetImpl
where
    <Self as ObjectSubclass>::Type: IsA<gtk::Widget>,
{
    /// Returns the root() object, already casted to gtk::ApplicationWindow.
    fn get_application_window(&self) -> Option<gtk::ApplicationWindow> {
        self.obj().root().and_downcast()
    }

    fn is_client_side(&self) -> bool {
        self.get_application_window()
            .and_downcast::<adw::ApplicationWindow>()
            .is_some()
    }
}

pub fn get_window_shell<T>(win: &T) -> Option<gtk::Widget>
where
    T: IsA<gtk::Window> + MayDowncastTo<adw::ApplicationWindow>,
{
    match win.downcast_ref::<adw::ApplicationWindow>() {
        Some(adw_win) => adw_win.content(),
        None => win.child(),
    }
}
