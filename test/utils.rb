require 'minitest'

class LibcrubySysTests < Minitest::Test
end

module Testing
  module Assertions
    class Ok
      def initialize(predicate, message)
        @predicate = predicate
        @message = message
      end

      def run!(context)
        context.send(:assert, @predicate, @message)
      end

      def inspect
        "assert #{@predicate.inspect} #{@message.inspect}"
      end

      alias_method :to_s, :inspect
    end

    class Equal
      def initialize(expected, actual)
        @expected = expected
        @actual = actual
      end

      def run!(context)
        context.send(:assert_equal, @expected, @actual)
      end

      def inspect
        "assert_equal #{@expected.inspect} #{@actual.inspect}"
      end

      alias_method :to_s, :inspect
    end

    class NotEqual
      def initialize(expected, actual)
        @expected = expected
        @actual = actual
      end

      def run!(context)
        context.send(:refute_equal, @expected, @actual)
      end

      def inspect
        "assert_not_equal #{@expected.inspect} #{@actual.inspect}"
      end

      alias_method :to_s, :inspect
    end

    class Nil
      def initialize(value)
        @value = value
      end

      def run!(context)
        context.send(:assert_nil, @value)
      end

      def inspect
        "assert_nil #{@value.inspect}"
      end

      alias_method :to_s, :inspect
    end
  end

  class LazyValue
    def initialize(code)
      @code = code
    end

    def value
      Module.new.instance_eval(@code)
    end

    def to_s
      value.to_s
    end

    def inspect
      value.inspect
    end

    def ==(other)
      value == other
    end
  end

  module Tests
    def self.build!
      mod = self

      test_methods = methods.find_all do |name|
        name.to_s.start_with?('test_')
      end

      LibcrubySysTests.class_eval do
        test_methods.each do |name|
          define_method name do
            mod.send(name).map do |assertion|
              assertion.run!(self)
            end
          end
        end
      end
    end
  end
end
