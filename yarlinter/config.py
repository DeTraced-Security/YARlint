from yarlinter.config.base import Config

class PerformanceConfig(Config):
    def __init__(self):
        super().__init__()
        self.performance = {
            "large_hex_jump": {
                "threshold": 500
            }
        }