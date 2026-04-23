# Root build configuration
project(
  'hexly',
  'rust',
  version: '0.1',
  meson_version: '>= 1.4'
)

gnome = import('gnome')

base_id = 'dev.clozer.Hexly'

dependency('gtk4', version: '>= 4.18')
dependency('libadwaita-1', version: '>= 1.7')
dependency('libpcsclite', version: '>= 2.3')

glib_compile_resources = find_program('glib-compile-resources', required: true)

version = meson.project_version()
prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')
iconsdir = datadir / 'icons'
pkgdatadir = datadir / meson.project_name()

is_devel = get_option('profile') == 'development'
if is_devel
  profile = 'Devel'
  application_id = '@0@.@1@'.format(base_id, profile)
else
  profile = ''
  application_id = base_id
endif

if is_devel
  message('Setting up git pre-commit hook...')
  run_command('cp', '-f', 'hooks/pre-commit.hook', '.git/hooks/pre-commit', check: false)
endif

subdir('data')
subdir('src')

gnome.post_install(
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
