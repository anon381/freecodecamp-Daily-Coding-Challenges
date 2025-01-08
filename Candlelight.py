# Space Complexity: O(1)
# Time Complexity: O(log n)
def burn_candles(candles: int, make_new: int) -> int:
    total_burned = 0
    leftovers = 0
    while candles > 0:
        total_burned += candles
        leftovers += candles
        candles = leftovers // make_new
        leftovers = leftovers % make_new
    return total_burned