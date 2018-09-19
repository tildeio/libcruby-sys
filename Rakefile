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

  task :check do
    sh 'cargo check'
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

  task :tests => 'cargo:check' do
    require 'toml-rb'

    mkdir_p 'tmp/tests'

    cargo_toml = TomlRB.load_file('Cargo.toml')

    cargo_toml['lib'] = {
      'crate-type' => ['cdylib'],
      'path' => '../../src/lib.rs'
    }

    cargo_toml['dependencies'] = cargo_toml.fetch('dependencies')
      .merge(cargo_toml.delete('dev-dependencies') || {})

    File.write('tmp/tests/Cargo.toml', TomlRB.dump(cargo_toml))

    cp 'Cargo.lock', 'tmp/tests/Cargo.lock'

    if Platform::OS == 'windows'
      libruby_path = RbConfig::CONFIG['libdir']
      libruby_name = RbConfig::CONFIG['RUBY_SO_NAME']

      libcruby_sys_path = File.expand_path('lib')
      libcruby_sys_name = 'cruby_sys'

      cp File.expand_path("lib#{libcruby_sys_name}.so", libcruby_sys_path), File.expand_path("lib#{libcruby_sys_name}.dll", libcruby_sys_path)

      sh "cargo rustc --manifest-path tmp/tests/Cargo.toml --target-dir target/tests -- --cfg test -L #{libruby_path.inspect} -l #{libruby_name} -L #{libcruby_sys_path.inspect} -l #{libcruby_sys_name}"
      cp "target/tests/debug/libcruby_sys.#{Platform::LIBEXT}", "test/tests.#{Platform::DLEXT}"
    else
      sh 'cargo rustc --manifest-path tmp/tests/Cargo.toml --target-dir target/tests -- --cfg test -C link-args="-Wl,-undefined,dynamic_lookup"'
      cp "target/tests/debug/liblibcruby_sys.#{Platform::LIBEXT}", "test/tests.#{Platform::DLEXT}"
    end
  end
end

task :test => ['build:extension', 'build:tests'] do
  sh 'ruby -Ilib -Itest test/runner.rb'
end

task :doc do
  require_relative './docs/generator'
  Docs::Generator.run(in_place: ENV.key?('IN_PLACE'))
end

task :default => :test
