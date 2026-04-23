use gtk::glib;

mod imp {
    use gtk::subclass::prelude::*;

    use gtk::CompositeTemplate;
    use gtk::glib;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/hexly_card_session.ui")]
    pub struct CardSession {
        #[template_child(id = "card_uid_row")]
        pub uid_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "card_capacity_row")]
        pub capacity_row: TemplateChild<adw::ActionRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CardSession {
        const NAME: &'static str = "HexlyCardSession";
        type Type = super::CardSession;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CardSession {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for CardSession {}

    impl BoxImpl for CardSession {}
}

glib::wrapper! {
    pub struct CardSession(ObjectSubclass<imp::CardSession>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CardSession {}
