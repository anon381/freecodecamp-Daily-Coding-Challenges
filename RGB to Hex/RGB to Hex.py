def rgb_to_hex(rgb_str):
    # Remove "rgb(" and ")" then split into integers
    r, g, b = map(int, rgb_str.strip("rgb()").split(","))
    # Format each value as 2-digit hex and concatenate
    return "#{:02x}{:02x}{:02x}".format(r, g, b)

