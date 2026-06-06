use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::glib;

mod imp {
    use super::*;

    use gtk::CompositeTemplate;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/card_information.ui")]
    pub struct CardInformation {
        #[template_child(id = "card_uid_row")]
        pub uid_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "card_atr_row")]
        pub atr_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "card_type_row")]
        pub type_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "card_capacity_row")]
        pub capacity_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "btn_copy_uid")]
        pub copy_uid_button: TemplateChild<gtk::Button>,

        #[template_child(id = "btn_copy_atr")]
        pub copy_atr_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CardInformation {
        const NAME: &'static str = "HexlyCardInformation";
        type Type = super::CardInformation;
        type ParentType = adw::PreferencesGroup;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CardInformation {}

    impl WidgetImpl for CardInformation {}

    impl PreferencesGroupImpl for CardInformation {}
}

glib::wrapper! {
    pub struct CardInformation(ObjectSubclass<imp::CardInformation>)
        @extends gtk::Widget, adw::PreferencesGroup,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CardInformation {
    pub(crate) fn set_card_info(
        &self,
        uid: Option<&str>,
        atr: Option<&str>,
        card_type: Option<&str>,
        capacity_bytes: Option<u32>,
    ) {
        let imp = self.imp();

        imp.uid_row.set_subtitle(uid.unwrap_or("-"));
        imp.atr_row.set_subtitle(atr.unwrap_or("-"));
        imp.type_row.set_subtitle(card_type.unwrap_or("-"));

        imp.copy_uid_button.set_sensitive(uid.is_some());
        imp.copy_atr_button.set_sensitive(atr.is_some());

        let capacity = capacity_bytes.map(|b| format!("{b} bytes"));
        let unwrapped = capacity.unwrap_or_else(|| "-".to_string());
        imp.capacity_row.set_subtitle(&unwrapped);
    }

    pub(crate) fn clear(&self) {
        let imp = self.imp();

        imp.uid_row.set_subtitle("-");
        imp.atr_row.set_subtitle("-");
        imp.type_row.set_subtitle("-");
        imp.capacity_row.set_subtitle("-");
        imp.copy_uid_button.set_sensitive(false);
        imp.copy_atr_button.set_sensitive(false);
    }
}
