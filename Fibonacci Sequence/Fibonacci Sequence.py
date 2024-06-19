
# Time Complexity: O(n), where n is the length of the sequence to generate.
# Space Complexity: O(n), for storing the output sequence of length n.
from typing import List

def fibonacci_sequence(start: List[int], length: int) -> List[int]:
    if length == 0:
        return []
    if length == 1:
        return [start[0]]
    if length == 2:
        return start[:2]
    
    seq = start[:2]
    while len(seq) < length:
        seq.append(seq[-1] + seq[-2])
    return seq
