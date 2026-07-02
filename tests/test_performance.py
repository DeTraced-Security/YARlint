import unittest
from yarlinter.rules.performance import LargeHexJump
from yarlinter.config import PerformanceConfig
from yarlinter.rules.base import Rule

class TestLargeHexJump(unittest.TestCase):
    def test_large_jump_range(self):
        config = PerformanceConfig()
        rule = Rule(config)
        large_hex_jump = LargeHexJump(config)
        hex_string = "0x00-0x10000"
        result = list(large_hex_jump.check(hex_string))
        self.assertEqual(len(result), 1)
        self.assertEqual(result[0]["message"], "Large hex jump range detected: [(0, 10000)]")

    def test_unbounded_jump_range(self):
        config = PerformanceConfig()
        rule = Rule(config)
        large_hex_jump = LargeHexJump(config)
        hex_string = "0x00-[-]"
        result = list(large_hex_jump.check(hex_string))
        self.assertEqual(len(result), 1)
        self.assertEqual(result[0]["message"], "Unbounded hex jump range detected: [(0, -1)]")

    def test_total_jump_range(self):
        config = PerformanceConfig()
        rule = Rule(config)
        large_hex_jump = LargeHexJump(config)
        hex_string = "0x00-0x10000, 0x00-0x10000"
        result = list(large_hex_jump.check(hex_string))
        self.assertEqual(len(result), 1)
        self.assertEqual(result[0]["message"], "Large hex jump range detected: [(0, 10000), (0, 10000)]")

if __name__ == "__main__":
    unittest.main()