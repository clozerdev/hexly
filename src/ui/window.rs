use glib::Object;
use gtk::{gio, glib};

use crate::app::HexlyApplication;

mod imp {
    use adw::subclass::prelude::*;

    use gtk::CompositeTemplate;
    use gtk::glib;
    use gtk::glib::types::StaticTypeExt;

    use crate::ui::widgets::card_session::CardSession;
    use crate::ui::widgets::reader_selector::ReaderSelector;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/hexly_window.ui")]
    pub struct HexlyWindow {
        #[template_child(id = "refresh_button")]
        pub refresh_button: TemplateChild<gtk::Button>,

        #[template_child(id = "reader_selector")]
        pub reader_selector: TemplateChild<ReaderSelector>,

        #[template_child(id = "card_session")]
        pub card_session: TemplateChild<CardSession>,
    }

    #[gtk::template_callbacks]
    impl HexlyWindow {
        #[template_callback]
        fn on_refresh_readers(&self, _: &gtk::Button) {
            println!("Refresh readers clicked");
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HexlyWindow {
        const NAME: &'static str = "HexlyWindow";
        type Type = super::HexlyWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            ReaderSelector::ensure_type();
            CardSession::ensure_type();

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
}
