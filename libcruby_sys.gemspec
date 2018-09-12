Gem::Specification.new do |s|
  s.name        = 'libcruby_sys'
  s.version     = '0.1.0'
  # s.licenses    = ['...']
  s.summary     = "Ruby bindings for Rust"
  s.authors     = ["Godfrey Chan", "Yehuda Katz", "Peter Wagenet"],
  s.email       = ["gofreykfc@gmail.com", "wycats@gmail.com", "peter.wagenet@gmail.com"]
  s.homepage    = 'https://github.com/tildeio/libcruby-sys'

  s.metadata      = {
    "bug_tracker_uri" => "https://github.com/tildeio/libcruby-sys/issues",
    # "changelog_uri"   => "https://github.com/tildeio/libcruby-sys/blob/master/CHANGELOG.md",
    "source_code_uri" => "https://github.com/tildeio/libcruby-sys",
  }

  s.files         = Dir["lib/**/*.rb"] | Dir["ext/**/*.{rb,c,h}"]
  s.extensions    = ["ext/extconf.rb"]
  s.require_paths = ["lib"]

  s.add_development_dependency "rake", "~> 12.3"
  s.add_development_dependency "rake-compiler", "~> 1.0"
  s.add_development_dependency "minitest", "~> 5.11"
  s.add_development_dependency "toml-rb"
end
