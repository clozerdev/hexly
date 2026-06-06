use async_channel::Receiver;

use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::gio::{self, ActionEntryBuilder};
use gtk::glib;

use glib::Object;

use crate::config::APP_ID;

use crate::core::nfc::messages::{NfcCommand, NfcEvent};
use crate::core::nfc::worker::NfcWorker;

use crate::ui::window::HexlyWindow;
use crate::ui::windows::common::get_window_shell;

mod imp {
    use async_channel::Receiver;

    use super::*;
    use std::cell::RefCell;

    use crate::core::nfc::worker::NfcWorker;

    #[derive(Default)]
    pub struct HexlyApplication {
        pub(super) nfc_worker: RefCell<Option<NfcWorker>>,
        pub(super) nfc_events_receiver: RefCell<Option<Receiver<NfcEvent>>>,
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
            setup_app_style();

            self.obj().setup_nfc_worker();
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
            window.setup_nfc_events(&app);
            window.present();
        }

        fn shutdown(&self) {
            if let Some(mut worker) = self.nfc_worker.take() {
                worker.shutdown();
            }

            self.parent_shutdown();
        }
    }

    impl GtkApplicationImpl for HexlyApplication {}

    impl AdwApplicationImpl for HexlyApplication {}
}

glib::wrapper! {
    pub struct HexlyApplication(ObjectSubclass<imp::HexlyApplication>)
        @extends gio::Application, adw::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Default for HexlyApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl HexlyApplication {
    pub(crate) fn new() -> Self {
        Object::builder().property("application-id", APP_ID).build()
    }

    pub(crate) fn send_nfc_command(&self, command: NfcCommand) {
        if let Some(worker) = self.imp().nfc_worker.borrow().as_ref() {
            let _ = worker.send_command(command);
        }
    }

    pub(crate) fn nfc_events_receiver(&self) -> Option<Receiver<NfcEvent>> {
        self.imp().nfc_events_receiver.take()
    }

    fn setup_app_actions(&self) {
        let settings = ActionEntryBuilder::new("preferences")
            .activate(glib::clone!(
                #[weak(rename_to = app)]
                self,
                move |_, _, page| {
                    let page = page
                        .map(|v| v.get::<String>().expect("Missing parameter"))
                        .unwrap_or("application".to_string());

                    let window = app
                        .windows_by_type::<crate::ui::windows::settings::Shell>()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| {
                            let settings_shell = crate::ui::windows::settings::Shell::new();
                            let window = app.new_window(&settings_shell);

                            window.set_modal(true);
                            window.set_default_size(700, 500);
                            window.set_title(Some("Settings"));

                            window.upcast()
                        });

                    let shell = get_window_shell(&window)
                        .and_downcast::<crate::ui::windows::settings::Shell>()
                        .expect("No settings shell");

                    shell.set_page(&page);
                    window.present()
                }
            ))
            .build();

        let quit = ActionEntryBuilder::new("quit")
            .activate(glib::clone!(
                #[weak(rename_to = app)]
                self,
                move |_, _, _| {
                    for window in app.windows() {
                        window.close();
                    }

                    if app.windows().is_empty() {
                        app.quit();
                    }
                }
            ))
            .build();

        self.add_action_entries([settings, quit]);
    }

    fn setup_nfc_worker(&self) {
        // We create the NFC event channel - Worker -> UI
        let (event_sender, event_receiver) = async_channel::unbounded::<NfcEvent>();

        let worker = NfcWorker::new(event_sender);
        self.imp().nfc_worker.replace(Some(worker));
        self.imp().nfc_events_receiver.replace(Some(event_receiver));
    }

    pub fn windows_by_type<T>(&self) -> Vec<gtk::Window>
    where
        T: IsA<gtk::Widget>,
    {
        self.windows()
            .into_iter()
            .filter(|win| match win.downcast_ref::<adw::ApplicationWindow>() {
                Some(adw_win) => adw_win.content().and_downcast_ref::<T>().is_some(),
                None => win.child().and_downcast_ref::<T>().is_some(),
            })
            .collect::<Vec<gtk::Window>>()
    }

    pub fn new_window<T>(&self, child: &T) -> gtk::ApplicationWindow
    where
        T: IsA<gtk::Widget>,
    {
        let window = gtk::ApplicationWindow::new(self);
        window.set_child(Some(child));
        window.upcast()
    }
}

fn setup_app_style() {
    gtk::Window::set_default_icon_name(APP_ID);

    let style_manager = adw::StyleManager::default();
    style_manager.set_color_scheme(adw::ColorScheme::PreferDark);

    install_css();
}

fn install_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/dev/clozer/Hexly/ui/style.css");

    let Some(display) = gtk::gdk::Display::default() else {
        return;
    };

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
