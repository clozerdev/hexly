use glib::Object;
use gtk::{
    gio,
    glib::{self, object::Cast},
};

use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::{
    app::HexlyApplication,
    core::pcsc::{
        format_bytes,
        types::{CardInfo, PcscCommand, PcscEvent, ReaderInfo},
        utils::ByteFormat,
    },
    utils::format::{format_capacity, format_card_kind},
};

mod imp {
    use adw::subclass::prelude::*;

    use gtk::CompositeTemplate;
    use gtk::glib;
    use gtk::glib::types::StaticTypeExt;

    use crate::ui::widgets::card_information::CardInformation;
    use crate::ui::widgets::reader_selector::ReaderSelector;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/hexly_window.ui")]
    pub struct HexlyWindow {
        #[template_child(id = "reader_selector")]
        pub reader_selector: TemplateChild<ReaderSelector>,

        #[template_child(id = "card_information")]
        pub card_information: TemplateChild<CardInformation>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HexlyWindow {
        const NAME: &'static str = "HexlyWindow";
        type Type = super::HexlyWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            ReaderSelector::ensure_type();
            CardInformation::ensure_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for HexlyWindow {}

    impl WidgetImpl for HexlyWindow {}

    impl WindowImpl for HexlyWindow {}

    impl ApplicationWindowImpl for HexlyWindow {}

    impl AdwApplicationWindowImpl for HexlyWindow {}
}

glib::wrapper! {
    pub struct HexlyWindow(ObjectSubclass<imp::HexlyWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl HexlyWindow {
    pub fn new(app: &HexlyApplication) -> Self {
        Object::builder().property("application", Some(app)).build()
    }

    fn app(&self) -> HexlyApplication {
        self.application()
            .expect("Window has no application")
            .downcast::<HexlyApplication>()
            .expect("Expected HexlyApplication")
    }

    pub fn init_window(&self) {
        self.setup_pcsc_events();
        self.refresh_readers();
    }

    // #region PC/SC commands & monitoring
    pub fn refresh_readers(&self) {
        let app = self.app();
        let worker = app.pcsc_worker();

        if let Err(err) = worker.sender().try_send(PcscCommand::RefreshReaders) {
            eprintln!("Failed to send RefreshReaders command: {err}");

            self.imp().reader_selector.clear_ui();
            self.imp().card_information.clear_ui();
        }
    }

    fn setup_pcsc_events(&self) {
        println!("UI: Setting PCSC events");

        let app = self.app();
        let receiver = app.pcsc_receiver();

        let weak_window = self.downgrade();
        glib::MainContext::default().spawn_local(async move {
            while let Ok(event) = receiver.recv().await {
                let Some(window) = weak_window.upgrade() else {
                    break;
                };

                window.handle_pcsc_event(event);
            }
        });
    }

    fn handle_pcsc_event(&self, event: PcscEvent) {
        println!("UI: Handling event: {:?}", event);

        match event {
            PcscEvent::ReadersUpdated { readers } => {
                self.apply_readers(readers);
            }
            PcscEvent::ReaderStatusUpdated {
                reader_name,
                card_present,
            } => {
                if card_present {
                    self.imp()
                        .reader_selector
                        .set_reader_status_row_text("Card present");

                    self.read_card_info(reader_name);
                } else {
                    self.imp()
                        .reader_selector
                        .set_reader_status_row_text("Card not present");
                    self.imp().card_information.clear_ui();
                }
            }
            PcscEvent::CardInfoUpdated { card } => match card {
                Some(card) => self.apply_card_info(card),
                None => self.imp().card_information.clear_ui(),
            },
            PcscEvent::Error { message } => {
                eprintln!("PC/SC error: {message}");

                self.imp().reader_selector.clear_ui();
                self.imp().card_information.clear_ui();
            }
        }
    }

    fn watch_selected_reader(&self) {
        println!("UI: watch_selected_readers");

        let Some(reader_name) = self.imp().reader_selector.selected_reader() else {
            self.stop_watching_reader();

            self.imp().reader_selector.clear_ui();
            self.imp().card_information.clear_ui();
            return;
        };

        let app = self.app();
        let worker = app.pcsc_worker();

        if let Err(err) = worker
            .sender()
            .try_send(PcscCommand::WatchReader { reader_name })
        {
            eprintln!("Failed to send WatchReader command: {err}");
        }
    }

    fn stop_watching_reader(&self) {
        let app = self.app();
        let worker = app.pcsc_worker();

        let _ = worker.sender().try_send(PcscCommand::StopWatchingReader);
    }

    fn apply_readers(&self, readers: Vec<ReaderInfo>) {
        println!("UI: apply_readers");
        self.imp().reader_selector.set_readers(&readers);

        if readers.is_empty() {
            self.stop_watching_reader();

            self.imp().reader_selector.clear_ui();
            self.imp().card_information.clear_ui();

            return;
        }

        self.watch_selected_reader();
    }

    fn read_card_info(&self, reader_name: String) {
        let app = self.app();
        let worker = app.pcsc_worker();

        if let Err(err) = worker
            .sender()
            .try_send(PcscCommand::ReadCardInfo { reader_name })
        {
            eprintln!("Failed to send ReadCardInfo command: {err}");
        }
    }

    pub fn apply_card_info(&self, card: CardInfo) {
        let uid = format_bytes(&card.uid, ByteFormat::Hex);
        let atr = format_bytes(&card.atr, ByteFormat::Hex);
        let kind = format_card_kind(&card.kind);
        let capacity = format_capacity(card.capacity.as_ref());

        let card_info = &self.imp().card_information;

        card_info.set_uid(&uid);
        card_info.set_atr(&atr);
        card_info.set_card_type(&kind);
        card_info.set_capacity(&capacity);
        card_info.set_category_sensitive(true);
    }
    // #endregion
}
