use adw::{prelude::ActionRowExt, subclass::prelude::*};
use gtk::glib;

mod imp {
    use gtk::subclass::prelude::*;

    use gtk::CompositeTemplate;
    use gtk::glib;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/hexly_card_information.ui")]
    pub struct CardInformation {
        #[template_child(id = "card_uid_row")]
        pub uid_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "card_capacity_row")]
        pub capacity_row: TemplateChild<adw::ActionRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CardInformation {
        const NAME: &'static str = "HexlyCardInformation";
        type Type = super::CardInformation;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CardInformation {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for CardInformation {}

    impl BoxImpl for CardInformation {}
}

glib::wrapper! {
    pub struct CardInformation(ObjectSubclass<imp::CardInformation>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CardInformation {
    pub fn set_uid(&self, uid_text: &str) {
        self.imp().uid_row.set_subtitle(uid_text);
    }

    pub fn clear(&self) {
        self.imp().uid_row.set_subtitle("-");
        self.imp().capacity_row.set_subtitle("-");
    }
}
