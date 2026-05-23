use adw::prelude::*;
use gtk::gio;
use gtk::glib;
use gtk::glib::clone;

use glib::Object;

use gtk::subclass::prelude::*;

use crate::config::APP_ID;
use crate::core::pcsc::types::PcscEvent;
use crate::core::pcsc::worker::PcscWorker;
use crate::ui::window::HexlyWindow;
use crate::ui::windows::settings::Shell;

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

    pub fn setup_app_actions(&self) {
        let settings_action = gio::SimpleAction::new("preferences", None);
        settings_action.connect_activate(clone!(
            #[weak(rename_to = app)]
            self,
            move |_, _| {
                let parent_window = app.active_window();
                let settings_window = adw::Window::builder()
                    .application(&app)
                    .title("Settings")
                    .default_width(980)
                    .default_height(680)
                    .build();

                if let Some(parent) = parent_window.as_ref() {
                    settings_window.set_transient_for(Some(parent));
                }

                let shell = Shell::new();
                shell.set_page("card-authentication");

                settings_window.set_content(Some(&shell));
                settings_window.present();
            }
        ));

        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(
            #[weak(rename_to = app)]
            self,
            move |_, _| {
                for window in app.windows() {
                    window.close();
                }

                if app.windows().is_empty() {
                    app.quit();
                }
            }
        ));

        self.add_action(&settings_action);
        self.add_action(&quit_action);
    }
}

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

            self.obj().setup_app_actions();
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
