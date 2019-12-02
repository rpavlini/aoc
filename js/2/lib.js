function solve(opcodes) {
  const interpreter = makeInterpreter(opcodes);
  return interpreter.interpret();
}

function makeInterpreter(opcodesArr) {
  const opcodes = opcodesArr;
  let pointer = 0;
  let halted = false;

  const next = () => {
    return opcodes[++pointer];
  };

  const handleOpcode = opcode => {
    const op1 = opcodes[next()];
    const op2 = opcodes[next()];
    const reg = next();

    if (opcode === 99) {
      halted = true;
      return;
    }
    switch (opcode) {
      case 1:
        opcodes[reg] = op1 + op2;
        break;
      case 2:
        opcodes[reg] = op1 * op2;
        break;
      default:
        throw opcode;
    }
    next();
  };

  return {
    interpret: () => {
      while (!halted) {
        handleOpcode(opcodes[pointer]);
      }
      return opcodes[0];
    },
    reset: () => {
      pointer = 0;
      halted = false;
    }
  };
}

module.exports = {
  solve
};
