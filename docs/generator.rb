require 'fileutils'
require 'toml'

module Docs
  class CDef

    attr_reader :generator, :header_path, :c_path, :signature, :type, :name,
                :ruby_module, :ruby_name

    def initialize(generator:, header_path:, c_path:, signature:, type:, name:)
      @generator = generator
      @header_path = header_path
      @c_path = c_path
      @signature = signature
      @type = type
      @name = name
      @links = {}
    end

    # This stuff happens after we've loaded all definitions
    def process_ruby_source(version)
      original_ruby_module, original_ruby_name = @ruby_module, @ruby_name
      @ruby_module, @ruby_name = get_ruby_module_and_name

      if @ruby_processed
        if @ruby_module != original_ruby_module
          raise "Ruby module changed for #{self} between versions"
        end

        if @ruby_name != original_ruby_name
          raise "Ruby name changed for #{self} between versions"
        end
      end

      @ruby_processed = true

      if expects_ruby_name?
        warn "Can't find ruby name for `#{name}` (`#{signature}`)" unless @ruby_name
      end
    end

    def namespaced_ruby_name
      if ruby_name && ruby_module
        parent_name = ruby_module.namespaced_ruby_name
        warn "can't get module's name: #{ruby_module}" unless parent_name
        "#{parent_name}::#{ruby_name}"
      else
        ruby_name
      end
    end

    def add_links(**args)
      add_docs_link(**args)
      add_header_link(**args)
      add_def_link(**args)
    end

    def build_docs(indent: '')
      docs = ""
      docs << "#{indent}/// # Defined In\n"
      docs << "#{indent}///\n"
      @links.each do |ver, links|
        docs << "#{indent}/// * **#{ver}:**\n"
        num_links = links.length
        links.each_with_index do |(text, url), index|
          last = (index == num_links - 1)
          docs << "#{indent}///     [#{text}](#{url})#{last ? '' : ' |'}\n"
        end
      end
      docs
    end

    private

    def expects_ruby_name?
      false
    end

    def get_ruby_module_and_name; end

    def add_docs_link(version:); end

    def add_header_link(version:)
      h_file = generator.find_ruby_header_file(header_path)

      if (index = h_file.find_index { |l| l.include?(signature) })
        url = generator.ruby_header_github_url(version: version, file: header_path, line: index+1)
        add_link(version, header_path, url)
      else
        raise "`#{name}` (`#{signature}`) not found in #{version[:short]} #{header_path}"
      end
    end

    def add_def_link(version:)
      c_file = generator.find_ruby_file(c_path)

      # TODO: This is a bit confusing
      index = c_file.find_index { |l| l =~ /^#{name}\(/ }
      if index && c_file[index - 1] == "#{type}\n"
        end_index = c_file[index..-1].find_index { |l| l[0] == '}' }
        raise "couldn't find end_index for #{name}" unless end_index
        url = generator.ruby_file_github_url(version: version, file: c_path, start_line: index, end_line: index+end_index+1)
        add_link(version, c_path, url)
      else
        raise "`#{name}` (`#{signature}`) not found in #{version[:short]} #{c_path}"
      end
    end

    def add_link(version, filename, url)
      @links[version[:short]] ||= {}
      @links[version[:short]][filename] = url
    end
  end

  class CStatic < CDef

    private

    def add_def_link(version:)
      c_file = generator.find_ruby_file(c_path)

      # Don't need to check full signature, since it was defined in the .h
      if (index = c_file.find_index { |l| l =~ /\b#{name}\s*=/ })
        url = generator.ruby_file_github_url(version: version, file: c_path, line: index+1)
        add_link(version, c_path, url)
      else
        raise "`#{name}` not found in #{version[:short]} #{c_path}"
      end
    end

  end

  class CClass < CStatic

    # //+ c-class: object.c `VALUE rb_cObject`
    MATCHER = %r{^(?<c_path>\S+\.c) `(?<signature>(?<type>.+\S)\s*(?<name>rb_[a-zA-Z0-9_]+))`\s*$}

    private

    def expects_ruby_name?
      true
    end

    # TODO: Clean this up
    def get_ruby_module_and_name
      ruby_module = nil
      ruby_name = nil

      c_file = generator.find_ruby_file(c_path)
      c_file.each_with_index do |line, index|
        if (match = line.match(/\b#{name}\s*=\s*(?:rb_define_class|boot_defclass|rb_struct_define_without_accessor)\((?:"([^"]+)"|\n)/))
          if !match[1]
            # It's on the next line
            # See https://github.com/ruby/ruby/blob/v2_5_1/range.c#L1314
            match = c_file[index + 1].match(/\s*"([^"]+)"/)
          end
          ruby_name = match[1]
          break
        elsif (match = line.match(/\b#{name}\s*=\s*rb_define_class_under\(([^,]+),[\s\n]*"([^"]+)"/))
          ruby_module = generator.c_defs.find { |i| i.name == match[1] }
          raise "missing parent: #{match[1]}" unless ruby_module
          ruby_name = match[2]
          break
        end
      end

      [ruby_module, ruby_name]
    end

    def add_docs_link(version:)
      if ruby_name && version[:doc]
        url = generator.ruby_doc_url(version: version, name: namespaced_ruby_name)
        add_link(version, "documentation", url)
      end
    end

  end

  class CModule < CClass

    # //+ c-module: object.c `VALUE rb_mKernel`
    MATCHER = %r{^(?<c_path>\S+\.c) `(?<signature>(?<type>.+\S)\s*(?<name>rb_[a-zA-Z0-9_]+))`\s*$}

    private

    def expects_ruby_name?
      true
    end

    # TODO: Clean this up
    def get_ruby_module_and_name
      ruby_module = nil
      ruby_name = nil

      c_file = generator.find_ruby_file(c_path)
      c_file.each_with_index do |line, index|
        if (match = line.match(/\b#{name}\s*=\s*rb_define_module\((?:"([^"]+)"|\n)/))
          if !match[1]
            # It's on the next line
            # See https://github.com/ruby/ruby/blob/v2_5_1/range.c#L1314
            match = c_file[index + 1].match(/\s*"([^"]+)"/)
          end
          ruby_name = match[1]
          break
        elsif (match = line.match(/\b#{name}\s*=\s*rb_define_module_under\(([^,]+),[\s\n]*"([^"]+)"/))
          ruby_module = generator.c_defs.find { |i| i.name == match[1] }
          raise "missing parent: #{match[1]}" unless ruby_module
          ruby_name = match[2]
          break
        end
      end

      [ruby_module, ruby_name]
    end

  end

  class CMethod < CDef

    # //+ c-func: symbol.c `ID rb_intern(const char*)`
    MATCHER = %r{^(?<c_path>\S+\.c) `(?<signature>(?<type>.+\S)\s*(?<name>rb_[a-zA-Z0-9_]+)\s*\((?<arguments>.*)\))`\s*$}

    attr_reader :arguments

    def initialize(arguments:, **args)
      super(**args)
      @arguments = arguments
    end
  end

  class Generator

    DIRECTIVE_MATCHER = %r{^\s*//\+ (?<cmd>[a-z\-]+):\s*(?<args>.+)}

    FILE_MAP = {
      'ruby.rs' => 'ruby.h',
      'intern.rs' => 'intern.h'
    }.freeze

    RUBY_VERSIONS = [
      { short: '2.3', tag: 'v2_3_7', doc: '2.3.7' }.freeze,
      { short: '2.4', tag: 'v2_4_4', doc: '2.4.4' }.freeze,
      { short: '2.5', tag: 'v2_5_1', doc: '2.5.1' }.freeze,
      { short: '2.6', tag: 'v2_6_0_preview2' }.freeze
    ].freeze

    def self.run(**args)
      new(**args).run
    end

    attr_reader :c_defs

    def initialize(root: File.expand_path('..', __dir__),
                    in_place: false,
                    file_map: FILE_MAP,
                    ruby_versions: RUBY_VERSIONS,
                    ruby_repo_url: "https://github.com/ruby/ruby.git")
      @root = root

      @in_place = in_place
      @target = @root

      @file_map = file_map

      @ruby_versions = ruby_versions
      @ruby_repo_url = ruby_repo_url

      @rust_files = { }
      @ruby_files = { }
    end

    def run
      clone unless @in_place
      load_definitions
      process_ruby
      update_rust_source
      build_cargo_docs
    end

    def find_ruby_file(path)
      @ruby_files[path] ||=
        File.readlines(File.expand_path(path, ruby_source_path))
    end

    def find_ruby_header_file(path)
      find_ruby_file("include/ruby/#{path}")
    end

    def ruby_header_github_url(file:, **args)
      ruby_file_github_url(file: "include/ruby/#{file}", **args)
    end

    def ruby_file_github_url(version:, file:, line: nil, start_line: nil, end_line: nil)
      start_line ||= line
      if end_line && !start_line
        raise ArgumentError, "must provide `start_line` if `end_line` is provided"
      end

      # TODO: Don't share
      url = "https://github.com/ruby/ruby/blob/#{version[:tag]}/#{file}"
      url << "#L#{start_line}" if start_line
      url << "-L#{end_line}" if end_line
      url
    end

    def ruby_doc_url(version:, name:)
      path = name.gsub('::', '/')
      # TODO: Maybe verify that this file exists?
      "https://ruby-doc.org/core-#{version[:doc]}/#{path}.html"
    end

    private

    def run_cmd(cmd)
      puts cmd
      system(cmd)
    end

    def cargo_pkg
      @cargo_pkg ||= TOML.load_file(File.expand_path('Cargo.toml', @root))['package']
    end

    def clone
      run_cmd('cargo package --allow-dirty')
      @target = File.expand_path("target/package/#{cargo_pkg['name']}-#{cargo_pkg['version']}", @root)
      raise "clone failed" unless Dir.exist?(@target)
    end

    def find_rust_file(path)
      @rust_files[path] ||=
        File.readlines(File.expand_path("src/#{path}", @target))
    end

    def load_definitions
      @c_defs = []

      @file_map.map do |(rs_path, h_path)|
        find_rust_file(rs_path).each do |line|
          if (match = line.match(DIRECTIVE_MATCHER))
            # TODO: Clean this up
            type = case match['cmd']
                  when 'c-module' then CModule
                  when 'c-class'  then CClass
                  when 'c-func'   then CMethod
                  end

            if type
              if (arg_match = match['args'].match(type.const_get(:MATCHER)))
                args = Hash[*arg_match.names.map { |n| [n.to_sym, arg_match[n]] }.flatten]
                c_defs << type.new(generator: self, header_path: h_path, **args)
              else
                raise "Unrecognized args `#{match['args']}` for `#{match['cmd']}`"
              end
            else
              warn "Unknown directive: #{line}"
            end
          end
        end
      end

      raise "no c_defs found" if @c_defs.empty?
    end

    def ruby_source_path
      File.expand_path('tmp/ruby-source', @root)
    end

    def with_ruby_source(&block)
      if Dir.exist?(ruby_source_path)
        # Start clean
        Dir.chdir(ruby_source_path) do
          run_cmd('git clean -f -d')
          run_cmd('git checkout .')
        end
      else
        run_cmd("git clone #{@ruby_repo_url} #{ruby_source_path}")
      end

      Dir.chdir(ruby_source_path, &block)
    end

    def each_ruby_version
      with_ruby_source do
        @ruby_versions.each do |version|
          @ruby_files.clear
          run_cmd("git checkout #{version[:tag]}")
          yield version
        end
      end
    end

    def process_ruby
      each_ruby_version do |version|
        c_defs.each do |c_def|
          c_def.process_ruby_source(version)
        end

        c_defs.each do |c_def|
          c_def.add_links(version: version)
        end
      end
    end

    def update_rust_source
      @rust_files.each do |(name, lines)|
        File.open(File.expand_path("src/#{name}", @target), 'w') do |file|
          strip_comments = false

          lines.each do |line|
            if strip_comments
              next if line =~ %r{^\s*///}
              strip_comments = false
            end

            file << line

            c_def = nil
            # FIXME: Don't duplicate
            if (match = line.match(DIRECTIVE_MATCHER))
              type = case match['cmd']
                    when 'c-module' then CModule
                    when 'c-class'  then CClass
                    when 'c-func'   then CMethod
                    end

              if type
                arg_match = match['args'].match(type.const_get(:MATCHER))
                c_def = c_defs.find { |d| d.class == type && d.signature == arg_match['signature'] }
                raise "couldn't find match: `#{type}`, `#{arg_match[:signature]}`" unless c_def
              end
            end

            if c_def
              strip_comments = true
              indent = (indent_match = line.match(%r{^(\s*)//\+})) ? indent_match[1] : ''
              file << c_def.build_docs(indent: indent)
            end
          end
        end
      end
    end

    def build_cargo_docs
      toml_path = File.expand_path('Cargo.toml', @target)
      run_cmd("cargo doc --no-deps --manifest-path #{toml_path.inspect}")

      puts "\n\nView Docs at:"
      puts File.expand_path("target/doc/#{cargo_pkg['name'].tr('-', '_')}/index.html", @target)
    end

  end

end
