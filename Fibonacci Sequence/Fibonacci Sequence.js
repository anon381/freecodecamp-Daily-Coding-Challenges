
// Time Complexity: O(n), where n is the length of the sequence to generate.
// Space Complexity: O(n), for storing the output sequence of length n.
function fibonacciSequence(start, length) {
  if (length === 0) return [];
  if (length === 1) return [start[0]];
  if (length === 2) return [start[0], start[1]];

  let seq = [start[0], start[1]];
  while (seq.length < length) {
    let n = seq.length;
    seq.push(seq[n - 1] + seq[n - 2]);
  }
  return seq;
}

console.log(fibonacciSequence([0, 1], 10));
console.log(fibonacciSequence([123456789, 987654321], 5));
