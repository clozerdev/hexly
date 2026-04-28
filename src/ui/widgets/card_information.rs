use adw::{prelude::ActionRowExt, subclass::prelude::*};
use gtk::{
    glib,
    prelude::{ButtonExt, WidgetExt},
};

mod imp {
    use adw::prelude::ActionRowExt;
    use adw::subclass::prelude::PreferencesGroupImpl;
    use gtk::gdk::prelude::DisplayExt;
    use gtk::prelude::ButtonExt;
    use gtk::subclass::prelude::*;

    use gtk::CompositeTemplate;
    use gtk::glib;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/hexly_card_information.ui")]
    pub struct CardInformation {
        #[template_child(id = "card_uid_row")]
        pub uid_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "card_atr_row")]
        pub atr_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "card_type_row")]
        pub type_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "card_capacity_row")]
        pub capacity_row: TemplateChild<adw::ActionRow>,

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

    impl ObjectImpl for CardInformation {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            self.copy_atr_button.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |btn| {
                    let atr = obj.imp().atr_row.subtitle().unwrap_or_default();

                    if let Some(display) = gtk::gdk::Display::default() {
                        println!("CARD_UI: ATR copied to clipboard");
                        display.clipboard().set_text(&atr);
                    }

                    btn.set_icon_name("object-select-symbolic");
                }
            ));
        }
    }

    impl WidgetImpl for CardInformation {}

    impl PreferencesGroupImpl for CardInformation {}
}

glib::wrapper! {
    pub struct CardInformation(ObjectSubclass<imp::CardInformation>)
        @extends gtk::Widget, adw::PreferencesGroup,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CardInformation {
    pub fn set_uid(&self, uid_text: &str) {
        self.imp().uid_row.set_subtitle(uid_text);
    }

    pub fn set_atr(&self, atr_text: &str) {
        self.imp().atr_row.set_subtitle(atr_text);

        let is_valid = atr_text != "-";
        self.imp().copy_atr_button.set_sensitive(is_valid);
    }

    pub fn set_card_type(&self, type_text: &str) {
        self.imp().type_row.set_subtitle(type_text);
    }

    pub fn set_capacity(&self, capacity_text: &str) {
        self.imp().capacity_row.set_subtitle(capacity_text);
    }

    pub fn set_category_sensitive(&self, sensitive: bool) {
        self.imp().uid_row.set_sensitive(sensitive);
        self.imp().atr_row.set_sensitive(sensitive);
        self.imp().type_row.set_sensitive(sensitive);
        self.imp().capacity_row.set_sensitive(sensitive);
    }

    pub fn clear_ui(&self) {
        self.set_uid("-");
        self.set_atr("-");
        self.set_card_type("-");
        self.set_capacity("-");

        self.imp()
            .copy_atr_button
            .set_icon_name("edit-copy-symbolic");

        self.set_category_sensitive(false);
    }
}
