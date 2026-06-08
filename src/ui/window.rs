use gtk::gio;
use gtk::glib;

use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::Object;

use crate::app::HexlyApplication;

use crate::core::nfc::NfcWorkerState;
use crate::core::nfc::messages::NfcCommand;
use crate::core::nfc::messages::NfcEvent;
use crate::core::state::AppState;
use crate::core::state::ReaderStatus;
use crate::core::state::{Reader, ReaderId};

use crate::ui::formatters::format_card_type;
use crate::ui::formatters::format_reader_status;

mod imp {
    use super::*;

    use std::cell::RefCell;

    use adw::subclass::prelude::*;

    use glib::types::StaticTypeExt;
    use gtk::CompositeTemplate;

    use crate::ui::widgets::card_contents_panel::CardContentsPanel;
    use crate::ui::widgets::card_information::CardInformation;
    use crate::ui::widgets::reader_selector::ReaderSelector;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/clozer/Hexly/ui/main_window.ui")]
    pub struct HexlyWindow {
        #[template_child(id = "reader_selector")]
        pub(super) reader_selector: TemplateChild<ReaderSelector>,

        #[template_child(id = "card_information")]
        pub(super) card_information: TemplateChild<CardInformation>,

        #[template_child(id = "card_contents_panel")]
        pub(super) card_contents_panel: TemplateChild<CardContentsPanel>,

        #[template_child(id = "event_log_scrolled_window")]
        pub(super) event_log_scrolled_window: TemplateChild<gtk::ScrolledWindow>,

        #[template_child(id = "event_log_text_view")]
        pub(super) event_log_text_view: TemplateChild<gtk::TextView>,

        pub(super) state: RefCell<AppState>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HexlyWindow {
        const NAME: &'static str = "HexlyWindow";
        type Type = super::HexlyWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            ReaderSelector::ensure_type();
            CardInformation::ensure_type();
            CardContentsPanel::ensure_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for HexlyWindow {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().init_state();
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
    pub(crate) fn new(app: &HexlyApplication) -> Self {
        Object::builder().property("application", Some(app)).build()
    }

    pub(crate) fn setup_nfc_events(&self, app: &HexlyApplication) {
        let Some(events_receiver) = app.nfc_events_receiver() else {
            return;
        };

        let window_weak = self.downgrade();
        glib::MainContext::default().spawn_local(async move {
            while let Ok(event) = events_receiver.recv().await {
                let Some(window) = window_weak.upgrade() else {
                    break;
                };

                window.handle_nfc_event(event);
            }
        });

        app.send_nfc_command(NfcCommand::Start);
    }

    fn app(&self) -> Option<HexlyApplication> {
        self.application()?.downcast::<HexlyApplication>().ok()
    }

    fn init_state(&self) {
        self.setup_ui_commands();

        let state = AppState::default();

        self.imp().state.replace(state.clone());
        self.render(&state);
    }

    fn handle_nfc_event(&self, event: NfcEvent) {
        match event {
            NfcEvent::StateChanged(state) => {
                self.append_event_log(&format!("NFC state: {:?}", state))
            }
            NfcEvent::ReadersChanged(readers) => self.apply_readers(readers),
            NfcEvent::ReadStarted { reader_id } => self.apply_reading(reader_id),
            NfcEvent::Log(message) => self.append_event_log(&message),
        }
    }

    fn apply_readers(&self, readers: Vec<Reader>) {
        let state = if !readers.is_empty() {
            let selected_reader = readers.first().map(|r| r.id.clone());

            AppState::ReadersAvailable {
                readers,
                selected_reader,
                reader_status: ReaderStatus::Ready,
            }
        } else {
            AppState::NoReaders
        };

        self.imp().state.replace(state.clone());
        self.render(&state);
    }

    fn apply_reading(&self, reader_id: ReaderId) {
        let current = self.imp().state.borrow().clone();
        let new = match current {
            AppState::ReadersAvailable {
                readers,
                selected_reader: Some(selected_reader),
                ..
            } if selected_reader == reader_id => AppState::ReadersAvailable {
                readers,
                selected_reader: Some(selected_reader),
                reader_status: ReaderStatus::Working,
            },
            other => other,
        };

        self.imp().state.replace(new.clone());
        self.render(&new);
    }

    fn on_reader_selected(&self, selected: Option<u32>) {
        let Some(index) = selected.map(|v| v as usize) else {
            return;
        };

        let mut state = self.imp().state.borrow().clone();
        let readers = match &state {
            AppState::ReadersAvailable { readers, .. } | AppState::CardPresent { readers, .. } => {
                readers.clone()
            }
            _ => return,
        };

        let Some(reader) = readers.get(index) else {
            return;
        };

        match &state {
            AppState::ReadersAvailable {
                selected_reader: Some(selected_reader),
                ..
            } if selected_reader == &reader.id => return,
            AppState::CardPresent {
                selected_reader, ..
            } if selected_reader == &reader.id => return,
            _ => {}
        }

        state = match state {
            AppState::ReadersAvailable {
                readers,
                reader_status,
                ..
            } => AppState::ReadersAvailable {
                readers,
                selected_reader: Some(reader.id.clone()),
                reader_status,
            },
            AppState::CardPresent {
                readers,
                reader_status,
                card,
                ..
            } => AppState::CardPresent {
                readers,
                selected_reader: reader.id.clone(),
                reader_status,
                card,
            },
            other => other,
        };

        self.imp().state.replace(state.clone());
        self.render(&state);
        self.append_event_log(&format!("Selected reader: {}", reader.label));
    }

    fn on_read_card_requested(&self) {
        let reader_id = {
            let state = self.imp().state.borrow();

            match &*state {
                AppState::ReadersAvailable {
                    selected_reader: Some(reader_id),
                    ..
                } => reader_id.clone(),
                _ => return,
            }
        };

        let Some(app) = self.app() else {
            return;
        };

        app.send_nfc_command(NfcCommand::ReadCard { reader_id });
    }

    fn append_event_log(&self, line: &str) {
        let buffer = self.imp().event_log_text_view.buffer();
        let existing = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);

        let line = format!("[{}] {line}", now_hms());

        if existing.is_empty() {
            buffer.set_text(&line);
        } else {
            buffer.set_text(&format!("{existing}\n{line}"));
        }
    }

    fn setup_ui_commands(&self) {
        self.imp()
            .reader_selector
            .connect_reader_selected(glib::clone!(
                #[weak(rename_to = window)]
                self,
                move |selected| {
                    window.on_reader_selected(selected);
                }
            ));

        self.imp()
            .card_contents_panel
            .connect_read_card_requested(glib::clone!(
                #[weak(rename_to = window)]
                self,
                move || {
                    window.on_read_card_requested();
                }
            ));
    }

    fn render(&self, state: &AppState) {
        let imp = self.imp();

        match state {
            AppState::NoReaders { .. } => {
                imp.reader_selector.clear();
                imp.card_information.clear();
                imp.card_contents_panel.show_no_reader();
            }
            AppState::ReadersAvailable {
                readers,
                selected_reader,
                reader_status,
                ..
            } => {
                let labels: Vec<String> = readers.iter().map(|r| r.label.clone()).collect();
                let selected = selected_index(readers, selected_reader.as_ref()).map(|v| v as u32);
                let formatted_status = format_reader_status(reader_status);

                let reader_label = selected_reader
                    .as_ref()
                    .and_then(|id| readers.iter().find(|r| &r.id == id))
                    .map(|r| r.label.as_str())
                    .unwrap_or_else(|| "");

                imp.reader_selector.set_readers(&labels, selected);
                imp.reader_selector.set_reader_status(formatted_status);
                imp.card_information.clear();

                match reader_status {
                    ReaderStatus::Ready => imp.card_contents_panel.show_no_card(reader_label),
                    ReaderStatus::Working => imp.card_contents_panel.show_reading(),
                    ReaderStatus::Disconnected => imp.card_contents_panel.show_no_reader(),
                }
            }
            AppState::CardPresent {
                readers,
                selected_reader,
                reader_status,
                card,
                ..
            } => {
                let labels: Vec<String> = readers.iter().map(|r| r.label.clone()).collect();
                let selected = selected_index(readers, Some(selected_reader)).map(|v| v as u32);
                let formatted_status = format_reader_status(reader_status);

                imp.reader_selector.set_readers(&labels, selected);
                imp.reader_selector.set_reader_status(formatted_status);

                imp.card_information.set_card_info(
                    card.uid.as_deref(),
                    card.atr.as_deref(),
                    format_card_type(card.card_type.as_ref()),
                    card.capacity_bytes,
                );
            }
        }
    }
}

fn selected_index(readers: &[Reader], selected: Option<&ReaderId>) -> Option<usize> {
    let selected = selected?;
    readers.iter().position(|reader| &reader.id == selected)
}

fn now_hms() -> String {
    glib::DateTime::now_local()
        .ok()
        .and_then(|dt| dt.format("%H:%M:%S").ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "--:--:--".to_string())
}
