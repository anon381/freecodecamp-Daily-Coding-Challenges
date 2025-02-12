function rotate(matrix) {
  const n = matrix.length;
  const res = Array.from({ length: n }, () => Array(n).fill(0));

  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      res[j][n - 1 - i] = matrix[i][j];
    }
  }
  return res;
}

