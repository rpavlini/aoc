class Tokens {
  constructor(n) {
    this.n = n;
    this.i = 0;
  }

  consume() {
    if (this.i === this.n.length) {
      return null;
    }
    return this.n[this.i++];
  }

  peek() {
    return this.n[this.i];
  }
}

let nums = `
[[[[4,0],6],[4,4]],[[6,8],0]]
[[1,[[1,6],0]],[[[8,9],2],[[0,8],[5,5]]]]
[[3,8],7]
[[[8,4],[[4,4],4]],[[3,[0,7]],0]]
[[[[2,0],[4,5]],[7,[2,8]]],5]
[[9,7],[[[8,6],3],[8,[0,2]]]]
[[4,9],[2,[[8,4],2]]]
[9,[[[2,6],[3,2]],[[2,5],[0,0]]]]
[[[9,[8,4]],[7,[1,2]]],[[4,[5,6]],[[5,9],[7,5]]]]
[[[0,[7,5]],[[1,8],1]],[[1,[1,9]],9]]
[[[0,[4,5]],[[1,2],[5,0]]],9]
[[[[7,7],3],1],[[0,[0,7]],[[7,1],[1,9]]]]
[[9,2],[3,[8,[6,1]]]]
[[8,[5,[3,9]]],[1,[3,8]]]
[[1,[[4,4],[4,2]]],4]
[[[5,1],[7,[3,5]]],[[9,[8,4]],9]]
[[5,[0,4]],[[1,[6,5]],9]]
[[[1,0],[4,1]],[[1,[3,2]],2]]
[[[0,[5,9]],[9,[7,2]]],[[4,3],[0,7]]]
[[[[7,9],[0,4]],[[5,6],[0,7]]],[7,[[1,1],[9,5]]]]
[[[[2,6],7],[[8,5],8]],[1,6]]
[[1,[5,5]],[[[3,0],[1,1]],[8,3]]]
[[[[7,4],5],[1,3]],[[6,9],[[3,7],2]]]
[[6,[9,[0,6]]],1]
[8,[4,[2,[2,7]]]]
[[[7,6],[2,8]],[[4,[8,1]],0]]
[4,9]
[[[[6,9],[1,7]],[[4,3],[4,3]]],[[[4,4],[3,6]],[7,[7,0]]]]
[[[7,4],[[9,1],[9,4]]],[6,[[0,4],[4,6]]]]
[[[[3,0],[4,7]],[[8,2],[3,9]]],[4,[0,[5,6]]]]
[[[[1,9],[0,4]],2],[8,[4,[0,9]]]]
[[[[9,6],[3,7]],4],[7,[[0,9],[5,8]]]]
[[5,[[4,0],[0,4]]],[[0,1],[2,[6,0]]]]
[[[2,[9,8]],[[7,9],[6,6]]],[[4,[6,4]],[[2,0],[5,0]]]]
[[[[5,8],8],[[3,1],1]],[[5,7],5]]
[[8,[5,1]],[[[5,5],8],[4,6]]]
[7,[[[3,9],3],[8,6]]]
[[[[8,6],4],8],[[7,[4,0]],[[8,0],4]]]
[[[[7,0],8],[[7,7],1]],[[0,5],[[8,2],5]]]
[4,[3,[3,[6,1]]]]
[[1,[[7,1],[1,2]]],9]
[[[9,[5,7]],[4,[4,7]]],8]
[[[3,[7,2]],[[5,8],6]],[2,0]]
[8,[0,[[7,4],[3,3]]]]
[[[[3,4],[1,1]],3],[[[5,3],0],[[0,7],[6,9]]]]
[3,[[9,1],[3,[0,0]]]]
[[[[8,8],[3,7]],[7,6]],[[[4,7],[9,5]],[5,8]]]
[[[[9,0],[5,6]],[[7,9],5]],0]
[[0,3],[[[9,9],[8,9]],[[7,5],0]]]
[6,[[2,0],3]]
[[[9,3],[[6,9],[8,2]]],[7,[[1,3],[0,5]]]]
[[[[9,5],1],5],[[4,2],[8,[9,5]]]]
[[8,4],[[4,[8,3]],[8,[8,3]]]]
[[[[8,0],[4,4]],[5,2]],[[[0,6],[4,0]],[5,8]]]
[[0,4],[3,[[2,3],7]]]
[[[[6,9],[3,0]],8],[[[4,7],[6,1]],[2,0]]]
[5,[[9,[5,1]],7]]
[[[8,0],[[5,0],0]],[[4,[0,7]],[[6,4],0]]]
[[[1,[0,2]],1],8]
[[[[4,8],[2,0]],[[0,4],9]],[4,[[9,8],[3,8]]]]
[[[1,[6,0]],[6,5]],[3,4]]
[[2,[[4,3],[4,4]]],[[[9,7],8],[5,0]]]
[[[[1,6],2],[[3,5],0]],[[[4,3],[8,1]],[[5,2],[2,1]]]]
[[[[4,8],[1,2]],[9,[3,7]]],[1,[4,4]]]
[[[[2,7],[5,8]],[[2,4],[6,8]]],[9,8]]
[[[1,5],[7,0]],[[8,7],4]]
[[[5,3],[[0,3],[6,2]]],[[8,[7,4]],[5,6]]]
[[[[1,4],1],[8,[2,0]]],[[[0,0],[7,9]],[[1,8],3]]]
[[[[0,0],[4,3]],2],3]
[[[8,[8,9]],[1,[6,1]]],[[6,[5,5]],[5,[9,5]]]]
[[[6,[4,2]],[[1,4],[5,6]]],[0,[[5,9],[2,7]]]]
[3,[[[2,5],2],8]]
[[2,[6,[1,6]]],[0,[4,[9,2]]]]
[[[[7,6],[5,9]],[6,[6,0]]],[2,[3,[1,4]]]]
[[[[1,7],[7,4]],[[6,0],[5,3]]],[2,[[5,2],0]]]
[[[7,[6,1]],[[1,7],[7,2]]],[5,6]]
[[3,2],[6,[9,7]]]
[[[7,[7,5]],[[0,9],5]],[[4,[5,6]],[[8,6],[1,8]]]]
[[[1,[1,6]],7],2]
[[[7,[6,2]],3],[[[5,5],6],9]]
[[[1,[9,8]],[0,5]],[[[2,4],5],[[5,6],7]]]
[[[9,[1,1]],[7,0]],[[5,8],2]]
[[[[8,5],[3,0]],[1,[2,6]]],[[[4,3],[3,2]],0]]
[[[[0,5],7],[7,1]],[4,[[3,4],[9,5]]]]
[[[7,6],[5,1]],[9,3]]
[[[[5,4],6],[2,[0,6]]],[[[6,0],[9,5]],[[8,6],[3,4]]]]
[[0,[6,[9,6]]],[[[1,2],[9,6]],[0,[6,2]]]]
[[[[7,7],6],7],[[8,[0,5]],[0,2]]]
[[[[6,7],[0,7]],[6,[5,0]]],[6,7]]
[[7,[1,8]],[[2,3],[[7,0],3]]]
[[8,[5,7]],[[3,[6,5]],4]]
[[9,9],[[[9,9],9],[2,3]]]
[[[[0,6],[1,4]],5],[1,3]]
[[[9,[8,8]],[[9,9],7]],[2,[[7,1],6]]]
[[[1,8],[1,3]],[[[8,1],8],[[4,2],1]]]
[[4,2],[[[0,7],5],7]]
[[[6,[3,6]],[[0,2],[5,6]]],[[0,1],[[0,9],2]]]
[[[[4,5],[1,4]],1],[[[4,7],[2,3]],6]]
[[[2,2],[0,6]],[[6,[6,4]],1]]
[[[5,[7,7]],[[7,0],1]],2]

`;

const leftmostPair = (root) => {
  if (!root) {
    return null;
  }
  if (
    typeof root.left?.value === "number" &&
    typeof root.right?.value == "number"
  ) {
    return root;
  }

  return leftmostPair(root.left) || leftmostPair(root.right);
};

const inorder = (root, p = []) => {
  if (!root) {
    return [];
  }
  if (typeof root.value === "number") {
    p.push(root);
  } else {
    inorder(root.left, p);
    inorder(root.right, p);
  }

  return p;
};

const magnitude = (root) => {
  if (!root) {
    return 0;
  }
  if (root.value) {
    return root.value;
  } else {
    return 3 * magnitude(root.left) + 2 * magnitude(root.right);
  }
};

const split = (root) => {
  let ord = inorder(root);
  let st = ord.find((o) => o.value > 9);

  if (st) {
    let left = Math.floor(st.value / 2);
    let right = Math.ceil(st.value / 2);

    st.value = null;
    st.left = new Node(left, st);
    st.right = new Node(right, st);

    return true;
  }

  return false;
};

const explode = (root) => {
  return _explode(root, root, 0);
};

const _explode = (root, top, lvl = 0) => {
  if (!root) {
    return false;
  }

  if (lvl === 5) {
    let curr = leftmostPair(root.parent);

    const lv = curr.left.value;
    const rv = curr.right.value;

    const ordered = inorder(top);

    if (ordered[0] !== curr.left) {
      let left = ordered[ordered.indexOf(curr.left) - 1];
      left.value += lv;
    }
    if (ordered[ordered.length - 1] !== curr.right) {
      let right = ordered[ordered.indexOf(curr.right) + 1];
      right.value += rv;
    }

    curr.value = 0;
    curr.left = null;
    curr.right = null;
    return true;
  } else {
    return Boolean(
      (root.left && _explode(root.left, top, lvl + 1)) ||
        (root.right && _explode(root.right, top, lvl + 1))
    );
  }
};

const printTree = (root) => {
  const t = printHelper(root, []);

  console.log(t.join(""));
};

const printHelper = (root, s = []) => {
  if (typeof root?.value === "number") {
    s.push(` ${root.value} `);
  } else {
    s.push("(");
    root?.left && printHelper(root.left, s);
    root?.right && printHelper(root.right, s);
    s.push(")");
  }
  return s;
};

class Node {
  constructor(value, parent, left, right) {
    this.value = value;
    this.parent = parent;
    this.left = left;
    this.right = right;
  }
}

const leftmost = (root) => {
  let curr = root;
  while (curr.left) {
    curr = curr.left;
  }
  return curr;
};

const rightmost = (root) => {
  let curr = root;
  while (curr.right) {
    curr = curr.right;
  }
  return curr;
};

const buildTree = (l, p) => {
  if (typeof l === "number") {
    return new Node(l, p, null, null);
  } else {
    const curr = new Node(null, p);

    const left = buildTree(l[0], curr);
    const right = buildTree(l[1], curr);

    curr.left = left;
    curr.right = right;

    return curr;
  }
};

let pairs = nums.split("\n").filter(Boolean);
let ps = [];

for (let i = 0; i < pairs.length - 1; i++) {
  for (let j = 1; j < pairs.length; j++) {
    let a = pairs[i];
    let b = pairs[j];

    ps.push([a, b]);
    ps.push([b, a]);
  }
}

// const trees = nums
//   .split("\n")
//   .filter(Boolean)
//   .map(tokenizeNum)
//   .map((t) => new Tokens(t))
//   .map(parseNum)
//   .map((p) => buildTree(p, null));

const sumNums = (a, b) => {
  let root = new Node(null, null, a, b);
  root.left.parent = root;
  root.right.parent = root;
  // console.log(printTree(root));
  while (true) {
    if (explode(root)) {
      continue;
    }
    if (split(root)) {
      continue;
    }
    break;
  }
  return root;
};

let max = 0;

for (let p of ps) {
  let [a, b] = p;
  a = buildTree(parseNum(new Tokens(tokenizeNum(a))), null);
  b = buildTree(parseNum(new Tokens(tokenizeNum(b))), null);

  max = Math.max(magnitude(sumNums(a, b)), max);
}

// trees.reverse();

// for (let i = 0; i < trees.length - 2; i++) {
//   for (let j = 1; j < trees.length - 1; j++) {
//     max = Math.max(max, magnitude(sumNums(trees[i], trees[j])));
//   }
// }

console.log(max);

const res = nums
  .split("\n")
  .filter(Boolean)
  .map(tokenizeNum)
  .map((t) => new Tokens(t))
  .map(parseNum)
  .map((p) => buildTree(p, null))
  .reduce((acc, curr) => {
    // console.log(printTree(curr));
    if (!acc) {
      return curr;
    } else {
      return sumNums(acc, curr);
    }
  });
// printTree(res);
// console.log(magnitude(res));

function parseInteger(n) {
  let int = Number.isFinite(Number(n));
  if (int) {
    return Number(n);
  }
}

function tokenizeNum(n) {
  let tokens = [];

  let curr = "";
  let i = 0;

  while (i < n.length) {
    let c = n[i];
    if (c === "[") {
      tokens.push(c);
    } else if (c === "]") {
      if (curr.length) {
        tokens.push(parseInteger(curr));
        curr = "";
        tokens.push(c);
      } else {
        tokens.push(c);
      }
    } else if (c === ",") {
      if (curr.length) {
        tokens.push(parseInteger(curr));
        curr = "";
      }
    } else if (typeof parseInteger(c) === "number") {
      curr += c;
    }

    i++;
  }

  return tokens;
}

function parseNum(n) {
  if (typeof n.peek() == "number") {
    return n.consume();
  } else {
    const pair = [];
    n.consume();
    pair.push(parseNum(n));
    pair.push(parseNum(n));
    n.consume();
    return pair;
  }
}
