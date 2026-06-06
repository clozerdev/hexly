use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::glib;

use builders::populate_sectors;

use crate::core::state::{Card, CardData};

use crate::ui::widgets::card_contents_panel::builders::all_sectors_expanded;
use crate::ui::widgets::card_contents_panel::builders::set_all_sectors_expanded;

mod builders;

mod imp {
    use super::*;

    use std::cell::{Cell, RefCell};

    use gtk::CompositeTemplate;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/card_contents_panel.ui")]
    pub struct CardContentsPanel {
        #[template_child(id = "card_contents_state_stack")]
        pub(super) state_stack: TemplateChild<adw::ViewStack>,

        #[template_child(id = "card_contents_view_stack")]
        pub(super) view_stack: TemplateChild<adw::ViewStack>,

        #[template_child(id = "detected_uid_label")]
        pub(super) detected_uid_label: TemplateChild<gtk::Label>,

        #[template_child(id = "read_card_button")]
        pub(super) read_card_button: TemplateChild<gtk::Button>,

        #[template_child(id = "read_progress_bar")]
        pub(super) read_progress_bar: TemplateChild<gtk::ProgressBar>,

        #[template_child(id = "read_progress_detail_label")]
        pub(super) read_progress_detail_label: TemplateChild<gtk::Label>,

        #[template_child(id = "partial_read_summary_label")]
        pub(super) partial_read_summary_label: TemplateChild<gtk::Label>,

        #[template_child(id = "read_error_label")]
        pub(super) read_error_label: TemplateChild<gtk::Label>,

        #[template_child(id = "overview_memory_row")]
        pub(super) overview_memory_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "overview_readable_row")]
        pub(super) overview_readable_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "overview_protected_row")]
        pub(super) overview_protected_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "overview_keya_row")]
        pub(super) overview_keya_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "overview_keyb_row")]
        pub(super) overview_keyb_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "hex_dump_text_view")]
        pub(super) hex_dump_text_view: TemplateChild<gtk::TextView>,

        #[template_child(id = "sectors_groups_box")]
        pub(super) sectors_groups_box: TemplateChild<gtk::Box>,

        #[template_child(id = "sectors_expand_switch_box")]
        pub(super) sectors_expand_switch_box: TemplateChild<gtk::Box>,

        #[template_child(id = "sectors_expand_switch")]
        pub(super) sectors_expand_switch: TemplateChild<gtk::Switch>,

        #[template_child(id = "editor_actions_box")]
        pub(super) editor_actions_box: TemplateChild<gtk::Box>,

        #[template_child(id = "editor_status_row")]
        pub(super) editor_status_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "editor_changes_row")]
        pub(super) editor_changes_row: TemplateChild<adw::ActionRow>,

        #[template_child(id = "editor_reset_button")]
        pub(super) editor_reset_button: TemplateChild<gtk::Button>,

        #[template_child(id = "editor_write_button")]
        pub(super) editor_write_button: TemplateChild<gtk::Button>,

        #[template_child(id = "editor_blocks_box")]
        pub(super) editor_blocks_box: TemplateChild<gtk::Box>,

        pub(super) sector_expanders: RefCell<Vec<adw::ExpanderRow>>,
        pub(super) syncing_expand_switch: Cell<bool>,
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

    impl ObjectImpl for CardContentsPanel {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup_callbacks();
        }
    }

    impl WidgetImpl for CardContentsPanel {}

    impl BinImpl for CardContentsPanel {}
}

glib::wrapper! {
    pub struct CardContentsPanel(ObjectSubclass<imp::CardContentsPanel>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CardContentsPanel {
    pub(crate) fn setup_callbacks(&self) {
        let imp = self.imp();

        imp.view_stack
            .connect_visible_child_name_notify(glib::clone!(
                #[weak(rename_to = panel)]
                self,
                move |_| {
                    panel.update_header_controls();
                }
            ));

        imp.sectors_expand_switch
            .connect_active_notify(glib::clone!(
                #[weak(rename_to = panel)]
                self,
                move |switch| {
                    panel.on_sectors_expand_changed(switch.is_active());
                }
            ));
    }

    pub(crate) fn show_no_reader(&self) {
        let imp = self.imp();

        imp.read_card_button.set_sensitive(false);
        imp.sectors_expand_switch.set_sensitive(false);
        imp.editor_reset_button.set_sensitive(false);
        imp.editor_write_button.set_sensitive(false);
        imp.state_stack.set_visible_child_name("no-reader");
    }

    pub(crate) fn show_no_card(&self) {
        let imp = self.imp();

        imp.read_card_button.set_sensitive(false);
        imp.sectors_expand_switch.set_sensitive(false);
        imp.editor_reset_button.set_sensitive(false);
        imp.editor_write_button.set_sensitive(false);
        imp.state_stack.set_visible_child_name("no-card");
    }

    pub(crate) fn show_card_data(&self, card: &Card) {
        let imp = self.imp();
        let Some(read) = card.card_data.as_ref() else {
            self.show_no_card();
            return;
        };

        imp.state_stack.set_visible_child_name("data");
        imp.view_stack.set_visible_child_name("overview");
        imp.read_card_button.set_sensitive(false);
        imp.sectors_expand_switch.set_sensitive(true);
        imp.editor_reset_button.set_sensitive(false);
        imp.editor_write_button.set_sensitive(false);
        imp.editor_status_row
            .set_subtitle("Card connected. No pending local changes.");

        let capacity = card
            .capacity_bytes
            .map(|b| b.to_string())
            .unwrap_or_else(|| "-".to_string());

        self.set_overview_memory(read, &capacity);

        *imp.sector_expanders.borrow_mut() = populate_sectors(&imp.sectors_groups_box, read);
        self.connect_sector_expanders_notify();
        self.sync_sectors_expand_switch();
        self.update_header_controls();
    }

    fn set_overview_memory(&self, data: &CardData, capacity: &String) {
        let imp = self.imp();

        let formatted_sectors = format!(
            "{} sectors · {} blocks · {capacity} bytes",
            data.total_sectors, data.total_blocks
        );

        let fmt_readable = format!("{} / {}", data.readable_sectors, data.total_sectors);
        let fmt_protected = format!("{} / {}", data.protected_sectors, data.total_sectors);
        let fmt_keya = data.key_a_known.to_string();
        let fmt_keyb = data.key_b_known.to_string();
        let fmt_sectors = format!(
            "{} of {} sectors were readable.",
            data.readable_sectors, data.total_sectors
        );

        imp.overview_memory_row.set_subtitle(&formatted_sectors);
        imp.overview_readable_row.set_subtitle(&fmt_readable);
        imp.overview_protected_row.set_subtitle(&fmt_protected);
        imp.overview_keya_row.set_subtitle(&fmt_keya);
        imp.overview_keyb_row.set_subtitle(&fmt_keyb);
        imp.partial_read_summary_label.set_label(&fmt_sectors);
    }

    fn update_header_controls(&self) {
        let imp = self.imp();
        let visible_tab = imp.view_stack.visible_child_name();
        let is_sectors_tab = visible_tab.as_deref() == Some("sectors");

        imp.sectors_expand_switch_box.set_visible(is_sectors_tab);
    }

    fn on_sectors_expand_changed(&self, expanded: bool) {
        let imp = self.imp();
        if imp.syncing_expand_switch.get() {
            return;
        }

        set_all_sectors_expanded(&imp.sector_expanders.borrow(), expanded);
    }

    fn sync_sectors_expand_switch(&self) {
        let imp = self.imp();
        let active = all_sectors_expanded(&imp.sector_expanders.borrow());

        imp.syncing_expand_switch.set(true);
        imp.sectors_expand_switch.set_active(active);
        imp.syncing_expand_switch.set(false);
    }

    fn connect_sector_expanders_notify(&self) {
        let expanders = self.imp().sector_expanders.borrow().clone();

        for expander in expanders {
            expander.connect_expanded_notify(glib::clone!(
                #[weak(rename_to = panel)]
                self,
                move |_| {
                    panel.sync_sectors_expand_switch();
                }
            ));
        }
    }
}
