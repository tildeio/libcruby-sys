require 'rake/extensiontask'

module Platform
  OS = case os = RbConfig::CONFIG['host_os'].downcase
  when /linux/
    # The official ruby-alpine Docker containers pre-build Ruby. As a result,
    #   Ruby doesn't know that it's on a musl-based platform. `ldd` is the
    #   only reliable way to detect musl that we've found.
    # See https://github.com/skylightio/skylight-ruby/issues/92
    if `ldd --version 2>&1` =~ /musl/
      "linux-musl"
    else
      "linux"
    end
  when /darwin/
    "darwin"
  when /freebsd/
    "freebsd"
  when /netbsd/
    "netbsd"
  when /openbsd/
    "openbsd"
  when /sunos|solaris/
    "solaris"
  when /mingw|mswin/
    "windows"
  else
    os
  end

  LIBEXT = case OS
  when /darwin/
    'dylib'
  when /linux|bsd|solaris/
    'so'
  when /windows|cygwin/
    'dll'
  else
    'so'
  end

  DLEXT = RbConfig::CONFIG['DLEXT']
end

namespace :cargo do
  task :clean do
    sh 'cargo clean'
  end
end

task :clean => 'cargo:clean'
task :clobber => 'cargo:clean'

namespace :extension do
  Rake::ExtensionTask.new do |ext|
    ext.name = 'libcruby_sys'
  end
end

namespace :build do
  task :extension => 'extension:compile'

  task :tests do
    # sh 'cargo rustc -- --cfg test -C link-args="-Wl,-flat_namespace,-undefined,dynamic_lookup"'
    require 'rbconfig'

    libruby_path = RbConfig::CONFIG['libdir']
    libruby_name = RbConfig::CONFIG['RUBY_SO_NAME']

    libcruby_sys_path = File.expand_path('lib')
    libcruby_sys_name = 'libcruby_sys'

    cp File.expand_path("#{libcruby_sys_name}.so", libcruby_sys_path), File.expand_path("#{libcruby_sys_name}.dll", libcruby_sys_path)

    link_args = '-Wl,--enable-auto-image-base,--enable-auto-import'

    sh "cargo rustc --verbose -- --cfg test -L #{libruby_path.inspect} -l #{libruby_name} -L #{libcruby_sys_path.inspect} -l #{libcruby_sys_name} -C link-args=#{link_args.inspect}"
    cp "target/debug/libcruby_sys.#{Platform::LIBEXT}", "target/debug/tests.#{Platform::DLEXT}"
  end
end

task :test => ['build:extension', 'build:tests'] do
  sh 'ruby -Ilib -Itest -Itarget/debug test/runner.rb'
end

task :default => :test
