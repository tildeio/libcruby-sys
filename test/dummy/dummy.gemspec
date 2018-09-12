# coding: utf-8

Gem::Specification.new do |spec|
  spec.name          = "dummy"
  spec.version       = "1.0.0"
  spec.authors       = ["Yehuda Katz", "Godfrey Chan"]
  spec.email         = ["wycats@gmail.com", "godfreykfc@gmail.com"]

  spec.summary       = "A dummy gem that re-export the C variables and functions to Ruby-land."

  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  spec.extensions    = ["ext/dummy/extconf.rb"]
  spec.require_paths = ["lib"]

  spec.add_development_dependency "libcruby_sys"
  spec.add_development_dependency "minitest", "~> 5.11"
  spec.add_development_dependency "rake", "~> 12.3"
  spec.add_development_dependency "rake-compiler", "~> 1.0"
end
