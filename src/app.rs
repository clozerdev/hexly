use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

use crate::config::APP_ID;
use crate::core::pcsc::types::PcscEvent;
use crate::core::pcsc::worker::PcscWorker;
use crate::ui::window::HexlyWindow;

mod imp {
    use std::cell::OnceCell;

    use super::APP_ID;

    use adw::subclass::prelude::*;

    use gtk::glib;
    use gtk::prelude::*;

    use crate::core::pcsc::types::PcscEvent;
    use crate::core::pcsc::worker::PcscWorker;

    #[derive(Default)]
    pub struct HexlyApplication {
        pub pcsc_worker: OnceCell<PcscWorker>,
        pub pcsc_event_receiver: OnceCell<async_channel::Receiver<PcscEvent>>,
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

            let style_manager = adw::StyleManager::default();
            style_manager.set_color_scheme(adw::ColorScheme::Default);

            let (sender, receiver) = async_channel::unbounded();
            let worker = PcscWorker::start(sender);

            self.pcsc_worker
                .set(worker)
                .expect("Worker already initialized");

            self.pcsc_event_receiver
                .set(receiver)
                .expect("Receiver already initialized");
        }

        fn activate(&self) {
            self.parent_activate();

            let app = self.obj();
            if let Some(window) = app.active_window() {
                window.present();
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

    pub fn pcsc_worker(&self) -> &PcscWorker {
        self.imp()
            .pcsc_worker
            .get()
            .expect("PcscWorker not initialized")
    }

    pub fn pcsc_receiver(&self) -> async_channel::Receiver<PcscEvent> {
        self.imp()
            .pcsc_event_receiver
            .get()
            .expect("PcscReceiver not initialized")
            .clone()
    }
}
