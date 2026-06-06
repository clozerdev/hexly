use gtk::glib;

use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::ui::windows::settings::pages;
use crate::ui::windows::settings::pill::Pill;

glib::wrapper! {
    pub struct Shell(ObjectSubclass<imp::Shell>)
        @extends gtk::Widget, adw::BreakpointBin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Shell {
    pub(crate) fn new() -> Self {
        glib::Object::new()
    }

    pub(crate) fn set_page(&self, page: &str) {
        let mut index = 0;

        while let Some(row) = self.imp().sidebar_box.row_at_index(index) {
            let child = row.child().and_downcast::<Pill>().expect("Not a pill");
            if child.name() == page {
                row.activate();
                return;
            }

            index += 1;
        }
    }
}

mod imp {
    use super::*;

    use gtk::CompositeTemplate;

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

            pages::CardAuthentication::ensure_type();

            klass.bind_template();
            klass.bind_template_callbacks();

            klass.add_binding_action(
                gtk::gdk::Key::Escape,
                gtk::gdk::ModifierType::empty(),
                "window.close",
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Shell {
        fn constructed(&self) {
            self.parent_constructed();

            self.init_content_view();
            self.init_sidebar();
            self.update_settings_page_title();
        }
    }

    impl WidgetImpl for Shell {}

    impl BreakpointBinImpl for Shell {}

    #[gtk::template_callbacks]
    impl Shell {
        #[template_callback]
        fn on_row_selected(&self, row: Option<&gtk::ListBoxRow>) {
            if let Some(pill) = row.and_then(|row| row.child()).and_downcast::<Pill>() {
                let target = pill.name();
                self.settings_stack.set_visible_child_name(&target);
                self.split_view.set_show_content(true);
            }
        }

        fn init_content_view(&self) {
            let pages: Vec<adw::PreferencesPage> = vec![pages::CardAuthentication::new().upcast()];

            for page in pages {
                let name = page.name().map(|s| s.to_string());
                let title = page.title();
                let icon_name = page.icon_name().map(|s| s.to_string()).unwrap_or_default();

                self.settings_stack.add_titled_with_icon(
                    &page,
                    name.as_deref(),
                    title.as_str(),
                    icon_name.as_str(),
                );
            }
        }

        fn init_sidebar(&self) {
            for page in self
                .settings_stack
                .pages()
                .iter::<adw::ViewStackPage>()
                .flatten()
            {
                let pill: Pill = glib::Object::builder()
                    .property("icon-name", page.icon_name())
                    .property("label", page.title())
                    .property("name", page.name())
                    .build();

                let child = gtk::ListBoxRow::builder().child(&pill).build();
                self.sidebar_box.append(&child);
            }
        }

        fn update_settings_page_title(&self) {
            let title = self
                .settings_stack
                .visible_child()
                .map(|page| self.settings_stack.page(&page))
                .and_then(|page| page.title())
                .unwrap_or_default();

            self.settings_page.set_title(title.as_str());
        }
    }
}
