
def tribonacci_sequence(signature, n):
    if n == 0:
        return []
    if n <= 3:
        return signature[:n]

    seq = signature[:]
    while len(seq) < n:
        seq.append(seq[-1] + seq[-2] + seq[-3])
    return seq
