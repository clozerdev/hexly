use gtk::glib;

use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::ui::windows::settings::pages::CardAuthentication;
use crate::ui::windows::settings::pill::Pill;

glib::wrapper! {
    pub struct Shell(ObjectSubclass<imp::Shell>)
        @extends gtk::Widget, adw::BreakpointBin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Shell {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_page(&self, page: &str) {
        let mut index = 0;

        while let Some(row) = self.imp().sidebar_box.row_at_index(index) {
            let child = row.child().and_downcast::<Pill>().expect("Not a pill?");

            if child.name() == page {
                self.imp().sidebar_box.select_row(Some(&row));
                self.imp().select_stack_page(&row);
                return;
            }

            index += 1;
        }
    }
}

mod imp {
    use super::*;

    use glib::subclass::InitializingObject;
    use gtk::CompositeTemplate;

    use crate::ui::windows::common::CommonShell;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/dev/clozer/Hexly/ui/settings/shell.ui")]
    pub struct Shell {
        #[template_child]
        split_view: TemplateChild<adw::NavigationSplitView>,

        #[template_child]
        settings_page: TemplateChild<adw::NavigationPage>,

        #[template_child]
        sidebar: TemplateChild<adw::NavigationPage>,

        #[template_child]
        settings_header_bar: TemplateChild<adw::HeaderBar>,

        #[template_child]
        sidebar_header_bar: TemplateChild<adw::HeaderBar>,

        #[template_child]
        pub(super) settings_stack: TemplateChild<adw::ViewStack>,

        #[template_child]
        pub(super) sidebar_box: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Shell {
        const NAME: &'static str = "HexlySettingsShell";
        type Type = super::Shell;
        type ParentType = adw::BreakpointBin;

        fn class_init(klass: &mut Self::Class) {
            Pill::ensure_type();
            CardAuthentication::ensure_type();

            klass.bind_template();
            klass.bind_template_callbacks();

            klass.add_binding_action(
                gtk::gdk::Key::Escape,
                gtk::gdk::ModifierType::empty(),
                "window.close",
            );
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Shell {
        fn constructed(&self) {
            self.parent_constructed();

            self.add_page(
                "card-authentication",
                "Authentication",
                "dialog-password-symbolic",
                &CardAuthentication::new(),
            );

            if let Some(row) = self.sidebar_box.row_at_index(0) {
                self.sidebar_box.select_row(Some(&row));
                self.select_stack_page(&row);
            }
        }
    }

    impl WidgetImpl for Shell {}

    impl BreakpointBinImpl for Shell {}

    impl CommonShell for Shell {}

    #[gtk::template_callbacks]
    impl Shell {
        #[template_callback]
        fn on_row_selected(&self, row: &gtk::ListBoxRow) {
            self.select_stack_page(row);
        }

        pub(super) fn select_stack_page(&self, row: &gtk::ListBoxRow) {
            let index = row.index() as u32;
            let pages = self.settings_stack.pages();

            if let Some(item) = pages.item(index)
                && let Ok(page) = item.downcast::<adw::ViewStackPage>()
            {
                self.settings_stack.set_visible_child(&page.child());
            }

            self.split_view.set_show_content(true);
        }

        fn add_page(&self, name: &str, title: &str, icon_name: &str, page: &impl IsA<gtk::Widget>) {
            self.settings_stack.add_named(page, Some(name));

            let pill = glib::Object::builder::<Pill>()
                .property("name", name)
                .property("label", title)
                .property("icon-name", icon_name)
                .build();

            let row = gtk::ListBoxRow::new();
            row.set_child(Some(&pill));
            self.sidebar_box.append(&row);
        }
    }
}
