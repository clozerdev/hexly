subdir('icons')

blueprint_files = [
  'ui/main_window.blp',
  'ui/reader_selector.blp',
  'ui/card_information.blp',
  'ui/settings/pill.blp',
  'ui/settings/shell.blp',
  'ui/settings/page_card_authentication.blp',
]

blueprints = custom_target(
  input: files(blueprint_files),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

resources = gnome.compile_resources(
  'hexly',
  'hexly.gresource.xml',
  dependencies: blueprints,
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
)

settings_path = '/@0@/'.format(application_id.replace('.', '/'))
gschema_file = configure_file(
  input: 'dev.clozer.Hexly.gschema.xml.in',
  output: '@0@.gschema.xml'.format(application_id),
  configuration: {
    'app_id': application_id,
    'settings_path': settings_path,
  }
)

install_data(
  gschema_file,
  install_dir: datadir / 'glib-2.0' / 'schemas',
)

desktop_file = configure_file(
  input: 'hexly.desktop.in',
  output: '@0@.desktop'.format(application_id),
  configuration: { 'app_id': application_id },
  install_dir: join_paths(get_option('datadir'), 'applications')
)
desktop_utils = find_program('desktop-file-validate', required: true)
test('Validate desktop file', desktop_utils, args: [desktop_file])
