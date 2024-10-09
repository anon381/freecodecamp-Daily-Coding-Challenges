function isBalanced(s) {
  const vowels = "aeiouAEIOU";
  const n = s.length;
  const half = Math.floor(n / 2);

  const first = s.slice(0, half);
  const second = s.slice(n - half);

  const count = str => [...str].filter(c => vowels.includes(c)).length;

  return count(first) === count(second);
}

console.log(isBalanced("racecar")); // true
console.log(isBalanced("Lorem Ipsum")); // true
console.log(isBalanced("Kitty Ipsum")); // false
