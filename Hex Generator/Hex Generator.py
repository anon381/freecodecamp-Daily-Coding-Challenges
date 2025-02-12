import random

def generate_hex(color: str) -> str:
    color = color.lower()

    def rand_channel(limit: int) -> int:
        """Return a random channel value from 0 up to (but not including) limit."""
        return random.randint(0, limit - 1) if limit > 0 else 0

    if color == "red":
        r = random.randint(128, 255)   # dominant red
        g = rand_channel(r)            # must be < r
        b = rand_channel(r)
    elif color == "green":
        g = random.randint(128, 255)   # dominant green
        r = rand_channel(g)
        b = rand_channel(g)
    elif color == "blue":
        b = random.randint(128, 255)   # dominant blue
        r = rand_channel(b)
        g = rand_channel(b)
    else:
        return "Invalid color"

    # Format into 6-character uppercase hex string
    return f"{r:02X}{g:02X}{b:02X}"

