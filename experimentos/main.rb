require 'fiddle'
require 'fiddle/import'

class Model
  attr_reader :x, :y
  def initialize(x, y)
    @x = x
    @y = y
  end

  def self.all
    puts "Slowly fetching all records from the database..."
    1000000.times.map do |i|
      Model.new(i, i + 1)
    end
  end

  def self.calculate_from_ruby(target)
    all.select do |transaction|
      transaction.x + transaction.y == target
    end.map { |transaction| transaction.x + transaction.y }
  end

  def self.calculate_from_ruby2(target)
    result = []
    all.select do |transaction|
      sum = transaction.x + transaction.y
      
      if sum == target
        result << sum
      end
    end
  end
end

module Rust
  extend Fiddle::Importer
  dlload './point_extension/target/release/libpoint_extension.dylib'
  extern 'void Init_point_extension()'
end

Rust.Init_point_extension


point1 = Point.new(3, 4)
point2 = Point.new(6, 8)

puts "Point1: (#{point1.x}, #{point1.y})"
puts "Point2: (#{point2.x}, #{point2.y})"
puts "Distance: #{point1.distance(point2)}"

puts Model.calculate_from_rust(3)
puts Model.calculate_from_ruby(3)

require 'benchmark/ips'

Benchmark.ips do |x|
  x.report("Rust implementation") { Model.calculate_from_rust(3) }
  x.report("Ruby implementation") { Model.calculate_from_ruby(3) }
  x.report("Ruby implementation 2") { Model.calculate_from_ruby2(3) }


  x.compare!
end
