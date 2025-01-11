# Space Complexity: O(1)
# Time Complexity: O(log n)
"""
Calculates the total number of candles that can be burned, including those made from leftovers.
Arguments:
    candles (int): Initial number of candles.
    make_new (int): Number of leftovers needed to make a new candle.
Returns:
    int: Total number of candles burned.
"""
def burn_candles(candles: int, make_new: int) -> int:
    total_burned = 0
    leftovers = 0
    while candles > 0:
        total_burned += candles
        leftovers += candles
        candles = leftovers // make_new
        leftovers = leftovers % make_new
    return total_burned