require 'rake/extensiontask'

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
    sh 'cargo rustc -- --cfg test -C link-args="-Wl,-undefined,dynamic_lookup"'
    cp 'target/debug/liblibcruby_sys.dylib', 'target/debug/tests.bundle'
  end
end

task :test => ['build:extension', 'build:tests'] do
  sh 'ruby -Ilib -Itest -Itarget/debug test/runner.rb'
end

task :default => :test
