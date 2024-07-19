// Time Complexity: O(n), where n is the length of the input string
function isValidNumber(s, base) {
  s = s.toUpperCase();
  for (let i = 0; i < s.length; i++) {
    let c = s[i];
    let val;
    if (/[0-9]/.test(c)) {
      val = parseInt(c, 10);
    } else if (/[A-Z]/.test(c)) {
      val = 10 + (c.charCodeAt(0) - "A".charCodeAt(0));
    } else {
      return false;
    }
    if (val >= base) return false;
  }
  return true;
}

console.log(isValidNumber("10101", 2));   // true
console.log(isValidNumber("10201", 2));   // false
console.log(isValidNumber("ABC", 16));    // true
console.log(isValidNumber("5G3F8F", 16)); // false
console.log(isValidNumber("5G3F8F", 17)); // true
