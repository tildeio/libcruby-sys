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
    sh 'cargo rustc -- --cfg test -C link-args="-Wl,--enable-auto-image-base,--enable-auto-import"'
    cp "target/debug/liblibcruby_sys.#{Platform::LIBEXT}", "target/debug/tests.#{Platform::DLEXT}"
  end
end

task :test => ['build:extension', 'build:tests'] do
  sh 'ruby -Ilib -Itest -Itarget/debug test/runner.rb'
end

task :default => :test
