from yarlinter.rules.base import Rule
from yarlinter.utils import extract_jump_range

class LargeHexJump(Rule):
    """
    Flag hex strings with very large upper bounds (e.g. [0-10000]) that force YARA to attempt matching across an enormous byte range, severely impacting performance.
    """
    name = "Performance/LargeHexJump"
    description = "Flag large hex jump ranges"
    config_options = {
        "threshold": {"type": "int", "default": 500}
    }

    def __init__(self, config):
        super().__init__(config)
        self.threshold = config.get("threshold")

    def check(self, rule):
        for hex_string in rule.strings:
            jump_ranges = extract_jump_range(hex_string)
            total_jump = sum(jump_range[1] for jump_range in jump_ranges)
            if total_jump > self.threshold:
                yield {
                    "message": f"Large hex jump range detected: {jump_ranges}",
                    "location": hex_string.location
                }
            for jump_range in jump_ranges:
                if jump_range[1] > self.threshold:
                    yield {
                        "message": f"Large hex jump range detected: {jump_range}",
                        "location": hex_string.location
                    }
                elif jump_range[1] == -1:  # unbounded jump
                    yield {
                        "message": f"Unbounded hex jump range detected: {jump_range}",
                        "location": hex_string.location
                    }