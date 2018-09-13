require "mkmf"

root_dir = File.expand_path("../../../../../..", __FILE__)

dir_config "dummy"

# if RUBY_PLATFORM =~ /mingw/
#   raise "not implemented"
#   append_ldflags("-L#{root_dir}/windows_build -lhelix-runtime-#{HelixRuntime::VERSION.gsub('.', '-')}")
# end

create_makefile "dummy"
