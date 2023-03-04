const [terminalRows, ...productionRows] =
  document.querySelector(".parse_table").children[0].children;

const terminals = Array.from(terminalRows.querySelectorAll("terminal")).map(
  (e) => e.innerText
);

function parseProductionRow(tr) {
  const production = tr.querySelector("th > nonterm").innerText;

  const rules = Array.from(tr.querySelectorAll("td")).map((e) => {
    const res = Array.from(e.querySelectorAll("nonterm, term"));
    if (res.length === 0) return null;

    return res.slice(1).map((e) => ({ type: e.tagName, value: e.innerText }));
  });

  return { production, rules };
}

function makeTable(terms, prodRows) {
  const table = new Map();

  for (const pr of prodRows) {
    let parsed = parseProductionRow(pr);

    for (let i = 0; i < terms.length; i++) {
      if (parsed.rules[i]) {
        table.set([parsed.production, terms[i]], parsed.rules[i]);
      }
    }
  }

  return table;
}

function parseProductionRow(tr) {
  const production = tr.querySelector("th > nonterm").innerText;

  const rules = Array.from(tr.querySelectorAll("td")).map((e) => {
    const res = Array.from(e.querySelectorAll("nonterm, term"));
    if (res.length === 0) return null;

    return res.slice(1).map((e) => ({ type: e.tagName, value: e.innerText }));
  });

  return { production, rules };
}

function makeTable(terms, prodRows) {
  const table = new Map();

  for (const pr of prodRows) {
    let parsed = parseProductionRow(pr);

    for (let i = 0; i < terms.length; i++) {
      if (parsed.rules[i]) {
        table.set([parsed.production, terms[i]], parsed.rules[i]);
      }
    }
  }

  return table;
}

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

function formatEntry(e) {
  if (e.type === "TERM") {
    return `Production::Term(${termTypeMap[e.value]})`;
  } else if (e.type === "NONTERM") {
    return `Production::NonTerm("${e.value}")`;
  } else {
    return `Production::${e.type}("${e.value}")`;
  }
}

function formatToCode(table) {
  const lines = [];
  const typeMap = {
    TERM: "Term",
    NONTERM: "NonTerm",
  };

  for (const [k, v] of table.entries()) {
    const values = v.map(formatEntry).join(", ");
    lines.push(`(("${k[0]}", ${termTypeMap[k[1]]}), vec![${values}]),`);
  }

  return lines.join("\n");
}

console.log(formatToCode(makeTable(terminals, productionRows)));
