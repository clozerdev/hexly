use gtk::glib;

mod imp {
    use gtk::subclass::prelude::*;

    use gtk::CompositeTemplate;
    use gtk::glib;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/hexly_reader_selector.ui")]
    pub struct ReaderSelector {
        #[template_child(id = "active_reader_row")]
        pub active_reader: TemplateChild<adw::ComboRow>,

        #[template_child(id = "reader_status_row")]
        pub reader_status: TemplateChild<adw::ActionRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ReaderSelector {
        const NAME: &'static str = "HexlyReaderSelector";
        type Type = super::ReaderSelector;
        type ParentType = gtk::Box;

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
        }
    }

    impl WidgetImpl for ReaderSelector {}

    impl BoxImpl for ReaderSelector {}
}

glib::wrapper! {
    pub struct ReaderSelector(ObjectSubclass<imp::ReaderSelector>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ReaderSelector {}
