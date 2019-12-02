const fs = require("fs");
const { solve } = require("./");
const { cartesian, makeArr } = require("../util");

const target = 19690720;

fs.readFile("./input", (err, data) => {
  const opcodes = data
    .toString()
    .split(",")
    .map(n => parseInt(n));

  let workingOpcodes = [...opcodes];

  const cart = cartesian(makeArr(), makeArr());

  let noun = 0; //1
  let verb = 0; //2

  for (let c of cart) {
    const [n, v] = c;
    workingOpcodes = [...opcodes];
    try {
      workingOpcodes[1] = n;
      workingOpcodes[2] = v;

      solve(workingOpcodes);
      if (workingOpcodes[0] === target) {
        noun = n;
        verb = v;
        break;
      }
    } catch (e) {
      console.log("retrying", e);
      continue;
    }
  }
  console.log(`noun: ${noun}, verb: ${verb}, solution: ${100 * noun + verb}`);
});
