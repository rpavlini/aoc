import { readFileSync } from "fs";

const input = readFileSync("./input").toString();

type Option<T> = T | null;

type Token = number | "[" | "]";

class Tokens {
  private cursor = 0;
  private tokens: Token[];

  constructor(input: string) {
    this.tokens = this.tokenize(input);
  }

  private tokenize(input: string) {
    let tokens: Token[] = [];

    let curr = "";
    let i = 0;

    while (i < input.length) {
      let c = input[i];
      if (c === "[") {
        tokens.push(c);
      } else if (c === "]") {
        if (curr.length) {
          tokens.push(this.tokenizeInt(curr)!);
          curr = "";
          tokens.push(c);
        } else {
          tokens.push(c);
        }
      } else if (c === ",") {
        if (curr.length) {
          tokens.push(this.tokenizeInt(curr)!);
          curr = "";
        }
      } else if (typeof this.tokenizeInt(c) === "number") {
        curr += c;
      }

      i++;
    }

    return tokens;
  }

  private tokenizeInt(n: string): Option<number> {
    let int = Number(n);
    if (Number.isFinite(int)) {
      return int;
    }
    return null;
  }

  consume(): Option<Token> {
    if (this.cursor === this.tokens.length) {
      return null;
    }
    return this.tokens[this.cursor++];
  }

  peek(): Option<Token> {
    return this.tokens[this.cursor];
  }
}

type AST = AST[] | number | null;

class Parser {
  constructor(private tokens: Tokens) {}

  parse(): Option<AST> {
    if (typeof this.tokens.peek() == "number") {
      return this.tokens.consume() as number;
    } else {
      const pair = [];

      this.tokens.consume(); // consume "["

      pair.push(this.parse());
      pair.push(this.parse());

      this.tokens.consume(); // consume "]"

      return pair;
    }
  }
}

class Node {
  constructor(
    public value: Option<number> = null,
    public parent: Option<Node> = null,
    public left: Option<Node> = null,
    public right: Option<Node> = null
  ) {}

  get hasLeftValue() {
    return typeof this.left?.value === "number";
  }

  get hasRightValue() {
    return typeof this.right?.value === "number";
  }

  get hasValue() {
    return typeof this.value === "number";
  }

  add(n: number) {
    this.value = (this.value || 0) + n;
  }

  static buildTree(ast: AST, parent: Option<Node> = null): Option<Node> {
    if (typeof ast === "number") {
      return new Node(ast, parent);
    } else {
      const root = new Node(null, parent);

      const left = Node.buildTree(ast![0], root);
      const right = Node.buildTree(ast![1], root);

      root.left = left;
      root.right = right;

      return root;
    }
  }
}

class TreeWalker {
  constructor(private root: Option<Node>) {}

  print(): void {
    let parts = this.printHelper(this.root);

    console.log(parts.join(""));
  }

  private printHelper(root: Option<Node>, acc: string[] = []): string[] {
    if (root?.hasValue) {
      acc.push(` ${root.value} `);
    } else {
      acc.push("[");
      root?.left && this.printHelper(root.left!, acc);
      root?.right && this.printHelper(root.right!, acc);
      acc.push("]");
    }

    return acc;
  }

  leftmostPair(root: Option<Node> = this.root): Option<Node> {
    if (!root) {
      return null;
    }

    if (root.hasLeftValue && root.hasRightValue) {
      return root;
    }

    return this.leftmostPair(root.left) || this.leftmostPair(root.right);
  }

  inorder(root: Option<Node> = this.root, acc: Node[] = []): Node[] {
    if (!root) {
      return [];
    }

    if (root.hasValue) {
      acc.push(root);
    } else {
      this.inorder(root.left, acc);
      this.inorder(root.right, acc);
    }

    return acc;
  }

  magnitude(root: Option<Node> = this.root): number {
    if (!root) {
      return 0;
    }

    if (root.hasValue) {
      return root.value!;
    } else {
      return 3 * this.magnitude(root.left) + 2 * this.magnitude(root.right);
    }
  }
}

class TreeReducer {
  private root: Node;

  constructor(a: Option<Node>, b: Option<Node>) {
    this.root = new Node(null, null, a, b);
    this.root.left!.parent = this.root;
    this.root.right!.parent = this.root;
  }

  private split(root = this.root): boolean {
    let sorted = new TreeWalker(root).inorder();
    let target = sorted.find((n) => n.hasValue && n.value! > 9);

    if (target) {
      const left = Math.floor(target.value! / 2);
      const right = Math.ceil(target.value! / 2);

      target.value = null;
      target.left = new Node(left, target);
      target.right = new Node(right, target);

      return true;
    }

    return false;
  }

  private explode(root = this.root) {
    return this.explodeHelper(root, 0);
  }

  private explodeHelper(root: Option<Node>, lvl = 0): boolean {
    if (!root) {
      return false;
    }

    if (lvl === 5) {
      const curr = new TreeWalker(root.parent).leftmostPair();

      const leftVal = curr!.left!.value;
      const rightVal = curr!.right!.value;

      const sorted = new TreeWalker(this.root).inorder();

      // if we're not the leftmost node
      if (sorted[0] !== curr!.left) {
        const left = sorted[sorted.indexOf(curr!.left!) - 1];
        left.add(leftVal!);
      }

      // if we're not the rightmost node
      if (sorted[sorted.length - 1] !== curr!.right) {
        const right = sorted[sorted.indexOf(curr!.right!) + 1];
        right.add(rightVal!);
      }

      curr!.value = 0;
      curr!.left = null;
      curr!.right = null;

      return true;
    } else {
      return Boolean(
        (root.left && this.explodeHelper(root.left, lvl + 1)) ||
          (root.right && this.explodeHelper(root.right, lvl + 1))
      );
    }
  }

  reduce() {
    while (true) {
      if (this.explode()) {
        continue;
      }

      if (this.split()) {
        continue;
      }
      break;
    }

    return this.root;
  }
}

function first(nums: string): number {
  const reduced = nums
    .split("\n")
    .map((n) => n.trim())
    .filter(Boolean)
    .map((n) => new Tokens(n))
    .map((t) => new Parser(t).parse())
    .map((ast) => Node.buildTree(ast))
    .reduce((acc, curr) => {
      if (!acc) {
        return curr;
      } else {
        return new TreeReducer(acc, curr).reduce();
      }
    });

  const magnitude = new TreeWalker(reduced).magnitude();
  console.log(magnitude);
  return magnitude;
}

function second(nums: string): number {
  const split = nums.split("\n").filter(Boolean);
  const pairs = [];

  for (let i = 0; i < split.length - 1; i++) {
    for (let j = 1; j < split.length; j++) {
      let a = split[i];
      let b = split[j];

      pairs.push([a, b]);
      pairs.push([b, a]);
    }
  }

  let max = -Infinity;

  for (const p of pairs) {
    const [a, b] = p;

    const nodeA = Node.buildTree(new Parser(new Tokens(a)).parse());
    const nodeB = Node.buildTree(new Parser(new Tokens(b)).parse());

    const reduced = new TreeReducer(nodeA, nodeB).reduce();

    max = Math.max(new TreeWalker(reduced).magnitude(), max);
  }

  console.log(max);
  return max;
}

first(input);
second(input);
