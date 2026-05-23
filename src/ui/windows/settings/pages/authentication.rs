use gtk::glib;

mod imp {
    use adw::subclass::preferences_page::PreferencesPageImpl;
    use gtk::gio::SimpleActionGroup;
    use gtk::gio::prelude::{ActionMapExt, SettingsExt};
    use gtk::prelude::WidgetExt;
    use gtk::subclass::prelude::*;
    use gtk::{CompositeTemplate, glib};

    use crate::settings::Settings;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/settings/page_card_authentication.ui")]
    pub struct CardAuthentication {
        settings: Settings,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CardAuthentication {
        const NAME: &'static str = "HexlySettingsPageAuthentication";
        type Type = super::CardAuthentication;
        type ParentType = adw::PreferencesPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CardAuthentication {
        fn constructed(&self) {
            self.parent_constructed();

            let action_group = SimpleActionGroup::new();
            let actions = ["default-auth-keys"];

            for action in actions {
                let action = self.settings.create_action(action);
                action_group.add_action(&action);
            }

            self.obj()
                .insert_action_group("widget", Some(&action_group));
        }
    }

    impl WidgetImpl for CardAuthentication {}

    impl PreferencesPageImpl for CardAuthentication {}
}

glib::wrapper! {
    pub struct CardAuthentication(ObjectSubclass<imp::CardAuthentication>)
        @extends gtk::Widget, adw::PreferencesPage,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for CardAuthentication {
    fn default() -> Self {
        Self::new()
    }
}

impl CardAuthentication {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
