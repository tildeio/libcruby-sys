require 'mkmf'

if CONFIG['MAJOR'] == '2' && CONFIG['MINOR'] == '4'
  # Ruby 2.4 shipped with some warnings, causing `append_ldflags` to fail
  append_cflags '-Wno-attributes'
end

append_ldflags '-Wl,--export-all-symbols'
create_makefile 'libcruby_sys'
