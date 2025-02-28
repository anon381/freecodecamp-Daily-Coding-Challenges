def char_strength(c: str) -> int:
    if 'a' <= c <= 'z':
        return ord(c) - ord('a') + 1
    elif 'A' <= c <= 'Z':
        return ord(c) - ord('A') + 27
    elif '0' <= c <= '9':
        return int(c)
    else:
        return 0

def battle(your_army: str, opponent_army: str) -> str:
    # Retreat conditions
    if len(your_army) > len(opponent_army):
        return "Opponent retreated"
    elif len(your_army) < len(opponent_army):
        return "We retreated"

    # If same length, battle
    our_wins, their_wins = 0, 0
    for y, o in zip(your_army, opponent_army):
        y_val = char_strength(y)
        o_val = char_strength(o)
        if y_val > o_val:
            our_wins += 1
        elif o_val > y_val:
            their_wins += 1

    if our_wins > their_wins:
        return "We won"
    elif our_wins < their_wins:
        return "We lost"
    else:
        return "It was a tie"
