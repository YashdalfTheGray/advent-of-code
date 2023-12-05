# frozen_string_literal: true

# This file is generated by the setup script located in the utils folder in the repository root.
# Run the tests using ruby solution_test.rb

require 'test/unit'

require_relative './solution'

# SanityTest just sanity tests the input
class SanityTest < Test::Unit::TestCase
  def test_input_not_nil
    assert_not_nil(Solution::INPUT)
  end

  def test_line_calibration_value_return_value_at_the_end
    result = Solution.find_line_calibration_value('1abc2')
    assert_equal(12, result)
  end

  def test_line_calibration_value_return_value_in_the_middle
    result = Solution.find_line_calibration_value('pqr3stu8vwx')
    assert_equal(38, result)
  end

  def test_line_calibration_value_return_value_multiple_numbers
    result = Solution.find_line_calibration_value('a1b2c3d4e5f')
    assert_equal(15, result)
  end

  def test_line_calibration_value_return_value_overlapping
    result = Solution.find_line_calibration_value('treb7uchet')
    assert_equal(77, result)
  end

  def test_file_calibration_value_return_value_sample
    input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet".split("\n").reject(&:empty?)

    result = Solution.find_file_calibration_value(input)
    assert_equal(142, result)
  end

  def test_file_calibration_value_return_value_input
    result = Solution.find_file_calibration_value(Solution::INPUT)
    assert_equal(54697, result)
  end

  def test_convert_to_integer_a_word
    result = Solution.convert_to_integer!('two')
    assert_equal('2', result)
  end

  def test_convert_to_integer_a_number
    result = Solution.convert_to_integer!('3')
    assert_equal('3', result)
  end

  def test_line_calibration_value_fixed_return_value_at_the_end
    result = Solution.find_line_calibration_value_fixed('two1nine')
    assert_equal(29, result)
  end

  def test_line_calibration_value_fixed_return_value_at_the_end_again
    result = Solution.find_line_calibration_value_fixed('eightwothree')
    assert_equal(83, result)
  end

  def test_line_calibration_value_fixed_return_value_in_the_middle
    result = Solution.find_line_calibration_value_fixed('abcone2threexyz')
    assert_equal(13, result)
  end

  def test_line_calibration_value_fixed_return_value_overlapping
    result = Solution.find_line_calibration_value_fixed('xtwone3four')
    assert_equal(24, result)
  end

  def test_line_calibration_value_fixed_return_value_at_the_end_numbers
    result = Solution.find_line_calibration_value_fixed('4nineeightseven2')
    assert_equal(42, result)
  end

  def test_line_calibration_value_fixed_return_value_two_places
    result = Solution.find_line_calibration_value_fixed('zoneight234')
    assert_equal(14, result)
  end

  def test_line_calibration_value_fixed_return_value_tricky_tricky
    result = Solution.find_line_calibration_value_fixed('7pqrstsixteen')
    assert_equal(76, result)
  end

  def test_line_calibration_value_fixed_return_value_no_last_number
    result = Solution.find_line_calibration_value_fixed('rlrdkzgnk6mnsbxfkhh')
    assert_equal(66, result)
  end

  def test_line_calibration_value_fixed_return_value_no_last_word
    result = Solution.find_line_calibration_value_fixed('rlrdkzgnkeightmnsbxfkhh')
    assert_equal(88, result)
  end

  def test_file_calibration_value_fixed_return_value_sample
    input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen".split("\n").reject(&:empty?)

    result = Solution.find_file_calibration_value_fixed(input)
    assert_equal(281, result)
  end

  # def test_file_calibration_value_fixed_return_value_input
  #   result = Solution.find_file_calibration_value_fixed(Solution::INPUT)
  #   assert_equal(54878, result)
  # end
end
