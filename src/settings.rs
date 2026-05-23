use std::ops::Deref;
use std::path::Path;

use gtk::gio;

use crate::config::{APP_ID, RESOURCES_FILE};

pub struct Settings(gio::Settings);

impl Default for Settings {
    fn default() -> Self {
        if let Some(default_source) = gio::SettingsSchemaSource::default() {
            if let Some(schema) = default_source.lookup(APP_ID, false) {
                return Self(gio::Settings::new_full(
                    &schema,
                    None::<&gio::SettingsBackend>,
                    None,
                ));
            }
        }

        let resource_path = Path::new(RESOURCES_FILE);
        let local_schema_dir = resource_path
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("glib-2.0").join("schemas"));

        if let Some(schema_dir) = local_schema_dir
            && let Ok(source) = gio::SettingsSchemaSource::from_directory(
                schema_dir,
                gio::SettingsSchemaSource::default().as_ref(),
                false,
            )
            && let Some(schema) = source.lookup(APP_ID, false)
        {
            return Self(gio::Settings::new_full(
                &schema,
                None::<&gio::SettingsBackend>,
                None,
            ));
        }

        panic!(
            "GSettings schema '{APP_ID}' not found in default source or local install directory"
        );
    }
}

impl Deref for Settings {
    type Target = gio::Settings;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
