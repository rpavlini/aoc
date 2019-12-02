const fs = require("fs");

fs.readFile("./input", (err, data) => {
  const lines = data.toString().split("\n");

  const result = lines.reduce((acc, curr) => {
    const lineValue = calcFuelForModule(curr);
    return lineValue + acc;
  }, 0);

  console.log(result);
});

function calcFuelForValue(value) {
  return Math.floor(parseInt(value) / 3) - 2;
}

function calcFuelForModule(modul) {
  let totalFuel = 0;
  let tempValue = modul;

  while (tempValue > 0) {
    const fuel = calcFuelForValue(tempValue);
    if (fuel < 0) break;
    totalFuel += fuel;
    tempValue = fuel;
  }

  return totalFuel;
}
