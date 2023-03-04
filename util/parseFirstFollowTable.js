const [, ...rows] = document.querySelector(".stats").children[0].children;

const termTypeMap = {
  $: "Type::EndOfFile",
  eof: "Type::EndOfFile",
  id: 'Type::Id("".to_owned())',
  intlit: "Type::IntNum(0)",
  intnum: "Type::IntNum(0)",
  floatlit: "Type::FloatNum(0f64)",
  floatnum: "Type::FloatNum(0f64)",
  equal: "Type::Assign",
  eq: "Type::Eq",
  neq: "Type::NotEq",
  noteq: "Type::NotEq",
  lt: "Type::Lt",
  gt: "Type::Gt",
  leq: "Type::LEq",
  geq: "Type::GEq",
  plus: "Type::Plus",
  minus: "Type::Minus",
  mult: "Type::Mult",
  div: "Type::Div",
  assign: "Type::Assign",
  lpar: "Type::OpenPar",
  openpar: "Type::OpenPar",
  rpar: "Type::ClosePar",
  closepar: "Type::ClosePar",
  lcurbr: "Type::OpenCubr",
  opencubr: "Type::OpenCubr",
  rcurbr: "Type::CloseCubr",
  closecubr: "Type::CloseCubr",
  lsqbr: "Type::OpenSqbr",
  opensqbr: "Type::OpenSqbr",
  rsqbr: "Type::CloseSqbr",
  closesqbr: "Type::CloseSqbr",
  semi: "Type::Semi",
  comma: "Type::Comma",
  dot: "Type::Dot",
  colon: "Type::Colon",
  arrow: "Type::ReturnType",
  returntype: "Type::ReturnType",
  sr: "Type::ScopeOp",
  scopeop: "Type::ScopeOp",
  or: "Type::Or",
  and: "Type::And",
  not: "Type::Not",
  integer: "Type::Integer",
  float: "Type::Float",
  void: "Type::Void",
  class: "Type::Class",
  self: "Type::SelfT",
  isa: "Type::IsA",
  while: "Type::While",
  if: "Type::If",
  then: "Type::Then",
  else: "Type::Else",
  read: "Type::Read",
  write: "Type::Write",
  return: "Type::Return",
  localvar: "Type::LocalVar",
  constructor: "Type::Constructor",
  constructorkeyword: "Type::Constructor",
  attribute: "Type::Attribute",
  function: "Type::Function",
  public: "Type::Public",
  private: "Type::Private",
  inlinecmt: "Type::InlineCmt",
  blockcmt: "Type::BlockCmt",
};

function parseTerminals(node) {
  return Array.from(node.querySelectorAll("nonterm")).map(
    (n) => termTypeMap[n.innerText]
  );
}

function parseRows(rows) {
  const table = new Map();

  for (const row of rows) {
    const [nontermNode, firstNode, followNode] = row.querySelectorAll("td");
    const nonterm = nontermNode.querySelector("nonterm").innerText;
    const firstSet = parseTerminals(firstNode);
    const followSet = parseTerminals(followNode);

    table.set(nonterm, { firstSet, followSet });
  }

  return table;
}

function formatTable(table) {
  const firstLines = ["HashMap::from(["];
  const followLines = ["HashMap::from(["];

  for (const [k, v] of table.entries()) {
    firstLines.push(`    ("${k}", vec![${v.firstSet.join(", ")}]),`);
    followLines.push(`    ("${k}", vec![${v.followSet.join(", ")}]),`);
  }

  firstLines.push("])");
  followLines.push("])");
  return { firstSet: firstLines.join("\n"), followSet: followLines.join("\n") };
}

function printTables(set) {
  console.log("First set:");
  console.log(set.firstSet);
  console.log("Follow set:");
  console.log(set.followSet);
}

printTables(formatTable(parseRows(rows)));
