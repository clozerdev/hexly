use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

use crate::config::APP_ID;
use crate::core::pcsc::service::PcscService;
use crate::ui::window::HexlyWindow;

mod imp {
    use super::APP_ID;

    use std::cell::OnceCell;

    use adw::subclass::prelude::*;

    use gtk::glib;
    use gtk::prelude::*;

    use crate::core::pcsc::service::PcscService;
    use crate::ui::window::HexlyWindow;

    #[derive(Default)]
    pub struct HexlyApplication {
        pub pcsc_service: OnceCell<PcscService>,
    }

    impl HexlyApplication {
        fn init_pcsc_service(&self) {
            let service = PcscService::new().expect("Failed to initialize PC/SC service");

            if self.pcsc_service.set(service).is_err() {
                panic!("PC/SC error initializing");
            }
        }
    }

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

            self.init_pcsc_service();
        }

        fn activate(&self) {
            self.parent_activate();

            let app = self.obj();
            if let Some(window) = app.active_window() {
                window.present();

                if let Ok(window) = window.downcast::<HexlyWindow>() {
                    window.init_window();
                }

                return;
            }

            let window = super::HexlyWindow::new(&app);
            window.present();
            window.init_window();
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

    pub fn pcsc_service(&self) -> &PcscService {
        self.imp()
            .pcsc_service
            .get()
            .expect("PC/SC service not initialized")
    }
}
