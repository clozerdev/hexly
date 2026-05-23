use gtk::glib;

use adw::prelude::*;
use adw::subclass::prelude::*;

glib::wrapper! {
    pub struct Pill(ObjectSubclass<imp::Pill>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

mod imp {
    use super::*;

    use std::cell::RefCell;

    use gtk::CompositeTemplate;
    use gtk::glib::Properties;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(resource = "/dev/clozer/Hexly/ui/settings/pill.ui")]
    #[properties(wrapper_type = super::Pill)]
    pub struct Pill {
        #[property(get, set)]
        icon_name: RefCell<String>,

        #[property(get, set)]
        label: RefCell<String>,

        #[property(get, set)]
        name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Pill {
        const NAME: &'static str = "HexlySettingPill";
        type Type = super::Pill;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for Pill {}

    impl WidgetImpl for Pill {}

    impl BoxImpl for Pill {}
}
