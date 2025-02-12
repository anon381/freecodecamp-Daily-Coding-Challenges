function findDuplicates(arr) {
    const counts = {};
        arr.forEach(num => {
        counts[num] = (counts[num] || 0) + 1;
    });
    return Object.keys(counts)
                 .filter(num => counts[num] > 1)
                 .map(Number)
                 .sort((a, b) => a - b);
}


