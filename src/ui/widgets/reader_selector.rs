use adw::{
    prelude::{ActionRowExt, ComboRowExt},
    subclass::prelude::*,
};
use gtk::{
    glib::{self, object::CastNone},
    prelude::WidgetExt,
};

use crate::core::pcsc::types::ReaderInfo;

mod imp {
    use adw::prelude::ComboRowExt;
    use adw::subclass::prelude::PreferencesGroupImpl;
    use gtk::subclass::prelude::*;

    use gtk::CompositeTemplate;
    use gtk::glib;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/hexly_reader_selector.ui")]
    pub struct ReaderSelector {
        #[template_child(id = "active_reader_row")]
        pub active_reader_row: TemplateChild<adw::ComboRow>,

        #[template_child(id = "reader_status_row")]
        pub reader_status_row: TemplateChild<adw::ActionRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ReaderSelector {
        const NAME: &'static str = "HexlyReaderSelector";
        type Type = super::ReaderSelector;
        type ParentType = adw::PreferencesGroup;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ReaderSelector {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            self.active_reader_row.connect_selected_notify(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.update_tooltip();
                }
            ));
        }
    }

    impl WidgetImpl for ReaderSelector {}

    impl PreferencesGroupImpl for ReaderSelector {}
}

glib::wrapper! {
    pub struct ReaderSelector(ObjectSubclass<imp::ReaderSelector>)
        @extends gtk::Widget, adw::PreferencesGroup,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ReaderSelector {
    pub fn set_readers(&self, readers: &[ReaderInfo]) {
        let imp = self.imp();

        if readers.is_empty() {
            return;
        }

        imp.active_reader_row.set_sensitive(true);
        imp.reader_status_row.set_sensitive(true);

        let model = gtk::StringList::new(
            &readers
                .iter()
                .map(|reader| reader.reader_name.as_str())
                .collect::<Vec<&str>>(),
        );

        imp.active_reader_row.set_model(Some(&model));
    }

    pub fn selected_reader(&self) -> Option<String> {
        let imp = self.imp();
        let selected = imp.active_reader_row.selected();

        if selected == gtk::INVALID_LIST_POSITION {
            return None;
        }

        imp.active_reader_row
            .selected_item()
            .and_downcast::<gtk::StringObject>()
            .map(|obj| obj.string().to_string())
    }

    pub fn update_tooltip(&self) {
        let imp = self.imp();

        let reader_name = self.selected_reader();
        imp.active_reader_row
            .set_tooltip_text(reader_name.as_deref());
    }

    pub fn clear_ui(&self) {
        let imp = self.imp();

        let empty = gtk::StringList::new(&[]);
        imp.active_reader_row.set_model(Some(&empty));
        imp.active_reader_row
            .set_selected(gtk::INVALID_LIST_POSITION);
        imp.active_reader_row.set_sensitive(false);
        imp.active_reader_row.set_subtitle("No readers found");

        imp.reader_status_row.set_subtitle("Unknown");
        imp.reader_status_row.set_sensitive(false);
    }

    pub fn set_reader_row_text(&self, reader_status: &str) {
        self.imp().active_reader_row.set_subtitle(reader_status);
    }

    pub fn set_reader_status_row_text(&self, status_text: &str) {
        self.imp().reader_status_row.set_subtitle(status_text);
    }

    pub fn set_reader_row_sensitive(&self, value: bool) {
        self.imp().active_reader_row.set_sensitive(value);
    }

    pub fn set_reader_status_row_sensitive(&self, value: bool) {
        self.imp().reader_status_row.set_sensitive(value);
    }

    pub fn set_active_reader_tooltip_text(&self, text: &str) {
        self.imp().active_reader_row.set_tooltip_text(Some(text));
    }

    // Helpers for connecting events and signals
    pub fn connect_reader_changed<F: Fn(&Self) + 'static>(&self, f: F) {
        let this = self.clone();

        self.imp()
            .active_reader_row
            .connect_selected_notify(move |_| {
                f(&this);
            });
    }
}
