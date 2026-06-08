use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::glib;

mod builders;

mod imp {
    use super::*;

    use gtk::CompositeTemplate;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/card_contents_panel.ui")]
    pub struct CardContentsPanel {
        #[template_child(id = "card_contents_state_stack")]
        pub(super) state_stack: TemplateChild<adw::ViewStack>,

        #[template_child(id = "read_card_button")]
        pub(super) read_card_button: TemplateChild<gtk::Button>,

        #[template_child(id = "no_card_reader_row")]
        pub(super) no_card_reader_row: TemplateChild<adw::ActionRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CardContentsPanel {
        const NAME: &'static str = "HexlyCardContentsPanel";
        type Type = super::CardContentsPanel;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CardContentsPanel {}

    impl WidgetImpl for CardContentsPanel {}

    impl BinImpl for CardContentsPanel {}
}

glib::wrapper! {
    pub struct CardContentsPanel(ObjectSubclass<imp::CardContentsPanel>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CardContentsPanel {
    pub(crate) fn show_no_reader(&self) {
        self.imp().state_stack.set_visible_child_name("no-reader");
    }

    pub(crate) fn show_no_card(&self, reader_label: &str) {
        self.imp().no_card_reader_row.set_subtitle(reader_label);
        self.imp().state_stack.set_visible_child_name("no-card");
    }

    pub(crate) fn show_reading(&self) {
        self.imp().state_stack.set_visible_child_name("reading");
    }

    pub(crate) fn connect_read_card_requested<F>(&self, f: F)
    where
        F: Fn() + 'static,
    {
        self.imp().read_card_button.connect_clicked(move |_| {
            f();
        });
    }
}
