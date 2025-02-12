
def is_valid_ipv4(s: str) -> bool:
    parts = s.split(".")
    if len(parts) != 4:
        return False
    for p in parts:
        if not p or not p.isdigit() or len(p) > 3:
            return False
        if p[0] == "0" and p != "0":
            return False
        if int(p) > 255:
            return False
    return True
