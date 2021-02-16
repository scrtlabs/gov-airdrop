const fs = require("fs");
const parse = require("csv-parse/lib/sync");
// const stringify = require("csv-stringify/lib/sync");
const { keccakFromString } = require("ethereumjs-util");
const { MerkleTree } = require("./merkleTree");

function calcLeafHashes(records) {
  let merkleElements = [];
  for (let i = 0; i < records.length; i++) {
    const entryHash = keccakFromString(
      /*i.toString() +*/ records[i].address + records[i].amount // Index is added for cases where addresses appear multiple times
    ).toString("hex");

    merkleElements.push(entryHash);
    records[i].hash = entryHash;
  }

  return merkleElements;
}

// function recordsToMap(records) {
//   map = new Map();

//   for (entry of records) {
//     map.set(entry.address, {
//       amount: entry.amount,
//       hash: entry.amount,
//     });
//   }

//   return map;
// }

function main() {
  // csv to json
  const data = fs.readFileSync("airdrop.csv", "utf8");
  const records = parse(data, {
    columns: true,
  });

  const merkleElements = calcLeafHashes(records);

  console.log(records);
  fs.writeFileSync("airdrop.json", JSON.stringify(records));

  const tree = new MerkleTree(merkleElements);
  console.log(tree);
  fs.writeFileSync("airdropMerkle.json", JSON.stringify(tree));

  // let a = JSON.stringify(tree);
  // console.log(JSON.parse(a));
  // let b = JSON.parse(a, (k, v) => {
  //   if (
  //     v !== null &&
  //     typeof v === "object" &&
  //     "type" in v &&
  //     v.type === "Buffer" &&
  //     "data" in v &&
  //     Array.isArray(v.data)
  //   ) {
  //     return Buffer.from(v.data, "utf-8");
  //   }
  //   return v;
  // });
  // let c = Object.setPrototypeOf(b, MerkleTree.prototype);
  // console.log(c);
}

main();
