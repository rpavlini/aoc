function cartesian(a, b) {
  let final = [];
  for (let i of a) {
    for (let j of b) {
      final.push([i, j]);
    }
  }
  return final;
}

function makeArr({ from = 0, to = 100 } = {}) {
  return Array.from({ length: to - from }).map((_, i) => i + from);
}

module.exports = {
  cartesian,
  makeArr
};
