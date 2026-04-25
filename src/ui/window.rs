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
        types::ReaderInfo,
        utils::{ByteFormat, format_bytes},
    },
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
        #[template_child(id = "refresh_button")]
        pub refresh_button: TemplateChild<gtk::Button>,

        #[template_child(id = "reader_selector")]
        pub reader_selector: TemplateChild<ReaderSelector>,

        #[template_child(id = "card_information")]
        pub card_information: TemplateChild<CardInformation>,
    }

    #[gtk::template_callbacks]
    impl HexlyWindow {
        #[template_callback]
        fn on_refresh_readers(&self, _: &gtk::Button) {
            self.obj().update_readers();
        }

        fn setup_callbacks(&self) {
            let obj = self.obj();
            self.reader_selector.connect_reader_changed(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.load_selected_reader_status();
                }
            ));
        }
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
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for HexlyWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.setup_callbacks();
        }
    }

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

    pub fn init_window(&self) {
        self.update_readers();
    }

    fn app(&self) -> HexlyApplication {
        self.application()
            .expect("Window has no application")
            .downcast::<HexlyApplication>()
            .expect("Expected HexlyApplication")
    }

    pub fn update_readers(&self) {
        let app = self.app();
        let service = app.pcsc_service();

        match service.list_readers() {
            Ok(readers) => {
                self.imp().reader_selector.set_readers(&readers);

                if !readers.is_empty() {
                    self.load_selected_reader_status();
                }
            }
            Err(err) => {
                eprintln!("REFRESH_READER ERROR: {err}");
                self.imp().reader_selector.clear_reader_info();
            }
        }
    }

    pub fn update_card_information(&self, reader_name: &str) {
        let app = self.app();
        let service = app.pcsc_service();

        match service.read_card_info(reader_name) {
            Ok(Some(card)) => {
                let uid = format_bytes(&card.uid, ByteFormat::Hex);
                self.imp().card_information.set_uid(&uid);
            }
            Ok(None) => {
                self.imp().card_information.clear();
            }
            Err(err) => {
                eprintln!("UPDATE CARD INFORMATION ERROR: {err}");
                self.imp().card_information.clear();
            }
        }
    }

    pub fn load_selected_reader_status(&self) {
        let Some(reader_name) = self.imp().reader_selector.selected_reader() else {
            self.imp()
                .reader_selector
                .set_reader_row_text("No reader selected");
            return;
        };

        let app = self.app();
        let service = app.pcsc_service();

        match service.reader_status(&reader_name) {
            Ok(reader_info_status) => {
                self.show_reader_status(reader_info_status);
                self.update_card_information(&reader_name);
            }
            Err(err) => {
                eprintln!("READER STATUS ERROR: {err}");
                self.imp().reader_selector.clear_reader_info();
            }
        }
    }

    pub fn show_reader_status(&self, reader_info: ReaderInfo) {
        if reader_info.card_present {
            self.imp()
                .reader_selector
                .set_reader_status_row_text("Card present");
        } else {
            self.imp()
                .reader_selector
                .set_reader_status_row_text("No card present");
        }
    }
}
