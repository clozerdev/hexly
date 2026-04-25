subdir('icons')

blueprint_files = [
  'ui/hexly_window.blp',
  'ui/hexly_reader_selector.blp',
  'ui/hexly_card_information.blp',
]

blueprints = custom_target(
  input: files(blueprint_files),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

resources = gnome.compile_resources(
  'hexly',
  'resources.gresource.xml',
  dependencies: blueprints,
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
)

desktop_file = configure_file(
  input: 'hexly.desktop.in',
  output: '@0@.desktop'.format(application_id),
  configuration: { 'app_id': application_id },
  install_dir: join_paths(get_option('datadir'), 'applications')
)
desktop_utils = find_program('desktop-file-validate', required: true)
test('Validate desktop file', desktop_utils, args: [desktop_file])
