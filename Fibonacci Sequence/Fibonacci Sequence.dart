
// Time Complexity: O(n), where n is the length of the sequence to generate.
// Space Complexity: O(n), for storing the output sequence of length n.
List<BigInt> fibonacciSequence(List<BigInt> start, int length) {
  if (length == 0) return [];
  if (length == 1) return [start[0]];
  if (length == 2) return [start[0], start[1]];

  List<BigInt> seq = [start[0], start[1]];
  while (seq.length < length) {
    int n = seq.length;
    seq.add(seq[n - 1] + seq[n - 2]);
  }
  return seq;
}

void main() {
  print(fibonacciSequence([BigInt.from(0), BigInt.from(1)], 10));
  print(fibonacciSequence([BigInt.from(123456789), BigInt.from(987654321)], 5));
}
