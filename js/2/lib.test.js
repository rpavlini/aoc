const { solve } = require("./lib");

test("interpreter works", () => {
  const testCases = [
    { input: [1, 0, 0, 0, 99], output: 2 },
    { input: [2, 40, 0, 3, 99], output: 2 },
    { input: [2, 4, 4, 5, 99, 0], output: 2 },
    { input: [1, 1, 1, 4, 99, 5, 6, 0, 99], output: 30 }
  ];

  testCases.forEach(testCase => {
    const { input, output } = testCase;
    expect(solve(input)).toEqual(output);
  });
});
