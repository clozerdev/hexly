use glib::Object;
use gtk::glib;

use crate::config::APP_ID;
use crate::ui::window::HexlyWindow;

mod imp {
    use adw::subclass::prelude::*;

    use gtk::glib;
    use gtk::prelude::*;

    use super::APP_ID;

    #[derive(Default)]
    pub struct HexlyApplication;

    #[glib::object_subclass]
    impl ObjectSubclass for HexlyApplication {
        const NAME: &'static str = "HexlyApplication";
        type Type = super::HexlyApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for HexlyApplication {}

    impl ApplicationImpl for HexlyApplication {
        fn startup(&self) {
            self.parent_startup();
            gtk::Window::set_default_icon_name(APP_ID);
        }

        fn activate(&self) {
            self.parent_activate();

            let app = self.obj();
            if let Some(window) = app.active_window() {
                window.present();
                return;
            }

            let window = super::HexlyWindow::new(app.upcast_ref());
            window.present();
        }
    }

    impl GtkApplicationImpl for HexlyApplication {}

    impl AdwApplicationImpl for HexlyApplication {}
}

glib::wrapper! {
    pub struct HexlyApplication(ObjectSubclass<imp::HexlyApplication>)
        @extends adw::gio::Application, adw::Application, gtk::Application,
        @implements gtk::gio::ActionMap, gtk::gio::ActionGroup;
}

impl HexlyApplication {
    pub fn new() -> Self {
        Object::builder().property("application-id", APP_ID).build()
    }
}
