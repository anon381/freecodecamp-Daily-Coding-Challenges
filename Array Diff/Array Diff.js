

// Time Complexity: O(n + m), where n = arr1.length, m = arr2.length
function arrayDiff(arr1, arr2) {
    const set1 = new Set(arr1);
    const set2 = new Set(arr2);
    const diff = new Set([...set1].filter(x => !set2.has(x)).concat([...set2].filter(x => !set1.has(x))));
    return Array.from(diff).sort((a, b) => a - b);
}

// Input validation example
console.log(arrayDiff([1, 2, 3], [3, 4, 5]));
