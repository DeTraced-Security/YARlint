def extract_jump_range(hex_string):
    """
    Extract jump ranges from a hex string.
    """
    import re
    pattern = r'\[(\d+)-(\d+)\]'
    matches = re.findall(pattern, hex_string)
    return matches