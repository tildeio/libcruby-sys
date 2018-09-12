Gem::Specification.new do |s|
  s.name        = 'libcruby_sys'
  s.version     = '0.1.0'
  s.licenses    = ['MIT']
  s.summary     = "This is an example!"
  s.description = "Much longer explanation of the example!"
  s.authors     = ["Godfrey Chan", "Yehuda Katz", "Peter Wagenet"],
  s.email       = ["gofreykfc@gmail.com", "wycats@gmail.com", "peter.wagenet@gmail.com"]
  s.homepage    = 'https://github.com/tildeio/libcruby-sys'
  s.metadata    = { "source_code_uri" => "https://github.com/tildeio/libcruby-sys.git" }

  s.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(\.|docs|test|spec|src|Cargo)/}) }
  s.extensions    = ["ext/extconf.rb"]
  s.require_paths = ["lib"]
end