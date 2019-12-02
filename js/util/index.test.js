const { cartesian, makeArr } = require("./");

test("cartesian", () => {
  const testCases = [
    {
      input: [
        [0, 1],
        [4, 5]
      ],
      output: [
        [0, 4],
        [0, 5],
        [1, 4],
        [1, 5]
      ]
    }
  ];

  testCases.forEach(tc => {
    expect(cartesian(...tc.input)).toEqual(tc.output);
  });
});

test("makeArr", () => {
  const testCases = [
    {
      input: {
        from: 1,
        to: 9
      },
      output: [1, 2, 3, 4, 5, 6, 7, 8]
    }
  ];

  testCases.forEach(tc => {
    expect(makeArr(tc.input)).toEqual(tc.output);
  });

  expect(makeArr().length).toEqual(100);
});
