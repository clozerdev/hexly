use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::glib;

mod imp {
    use super::*;

    use gtk::CompositeTemplate;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/reader_selector.ui")]
    pub struct ReaderSelector {
        #[template_child(id = "active_reader_row")]
        pub active_row: TemplateChild<adw::ComboRow>,

        #[template_child(id = "reader_status_row")]
        pub status_row: TemplateChild<adw::ActionRow>,
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

    impl ObjectImpl for ReaderSelector {}

    impl WidgetImpl for ReaderSelector {}

    impl PreferencesGroupImpl for ReaderSelector {}
}

glib::wrapper! {
    pub struct ReaderSelector(ObjectSubclass<imp::ReaderSelector>)
        @extends gtk::Widget, adw::PreferencesGroup,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ReaderSelector {
    pub(crate) fn set_readers(&self, readers: &[String], selected: Option<u32>) {
        let imp = self.imp();
        let selected = selected.unwrap_or(gtk::INVALID_LIST_POSITION);

        if imp.active_row.selected() == selected
            && active_row_labels_match(&imp.active_row, readers)
        {
            return;
        }

        let labels: Vec<&str> = readers.iter().map(String::as_str).collect();
        let model = gtk::StringList::new(&labels);
        imp.active_row.set_model(Some(&model));
        imp.active_row.set_selected(selected);
    }

    pub(crate) fn set_reader_status(&self, status: &str) {
        self.imp().status_row.set_subtitle(status);
    }

    pub(crate) fn clear(&self) {
        let imp = self.imp();

        let empty_model = gtk::StringList::new(&[]);
        imp.active_row.set_model(Some(&empty_model));
        imp.active_row.set_selected(gtk::INVALID_LIST_POSITION);
        imp.active_row.set_subtitle("No reader detected");
        imp.status_row.set_subtitle("No reader");
    }

    pub(crate) fn connect_reader_selected<F>(&self, f: F)
    where
        F: Fn(Option<u32>) + 'static,
    {
        self.imp().active_row.connect_selected_notify(move |row| {
            let selected = row.selected();
            if selected == gtk::INVALID_LIST_POSITION {
                f(None)
            } else {
                f(Some(selected))
            }
        });
    }
}

fn active_row_labels_match(active_row: &adw::ComboRow, readers: &[String]) -> bool {
    let Some(model) = active_row.model() else {
        return readers.is_empty();
    };

    if model.n_items() as usize != readers.len() {
        return false;
    }

    for (index, expected) in readers.iter().enumerate() {
        let Some(item) = model.item(index as u32) else {
            return false;
        };

        let Ok(item) = item.downcast::<gtk::StringObject>() else {
            return false;
        };

        if item.string().as_str() != expected {
            return false;
        }
    }

    true
}
