// JavaScript implementation
// Time Complexity: O(n), where n is the length of the input array (for set creation and max search)
// Space Complexity: O(n), for storing the set and the output array.
function findMissingNumbers(arr) {
  if (arr.length === 0) return [];
  const n = Math.max(...arr);
  const nums = new Set(arr);
  const missing = [];
  for (let i = 1; i <= n; i++) {
    if (!nums.has(i)) missing.push(i);
  }
  return missing;
}

console.log(findMissingNumbers([1, 2, 4, 6, 6, 3, 7, 8])); // [5]
console.log(findMissingNumbers([])); // []
