require 'mkmf'

if CONFIG['MAJOR'] == '2' && CONFIG['MINOR'] == '4'
  # Ruby 2.4 shipped with some warnings, causing `append_ldflags` to fail
  append_cflags '-Wno-attributes'
end

if convertible_int('long long') != 'long long'
  abort <<~MSG
    Cannot build libcruby-sys native extension without compiler support
    for `long long`.

    This should only happen if your C compiler is older than C99. If you
    need assistance, please open a Github issue:

    https://github.com/tildeio/libcruby-sys/issues/new
  MSG
end

append_ldflags '-Wl,--export-all-symbols'
create_makefile 'libcruby_sys'
