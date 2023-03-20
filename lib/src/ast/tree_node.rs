use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum TreeNode {
    ArgumentList(),
    ArithExpr(),
    Assignment(),
    Attribute(),
    Class(),
    ClassMembers(),
    ConstructorFunc(),
    ElseBlock(),
    Expr(),
    Factor(),
    Function(),
    FunctionBody(),
    FunctionCall(),
    FunctionHead(),
    If(),
    IfBlock(),
    IndexedVar(),
    IndiceList(),
    InheritsList(),
    LocalVarDecl(),
    MemberFunc(),
    NestedVar(),
    Parameter(),
    ParameterList(),
    Program(),
    Read(),
    RelExpr(),
    Return(),
    RightRecArithExpr(),
    RightRecTerm(),
    Scope(),
    Variable(),
    While(),
    WhileBlock(),
    Write(),
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeNode::ArgumentList() => write!(f, "ArgumentList"),
            TreeNode::ArithExpr() => write!(f, "ArithExpr"),
            TreeNode::Assignment() => write!(f, "Assignment"),
            TreeNode::Attribute() => write!(f, "Attribute"),
            TreeNode::Class() => write!(f, "Class"),
            TreeNode::ClassMembers() => write!(f, "ClassMembers"),
            TreeNode::ConstructorFunc() => write!(f, "ConstructorFunc"),
            TreeNode::ElseBlock() => write!(f, "ElseBlock"),
            TreeNode::Expr() => write!(f, "Expr"),
            TreeNode::Factor() => write!(f, "Factor"),
            TreeNode::Function() => write!(f, "Function"),
            TreeNode::FunctionBody() => write!(f, "FunctionBody"),
            TreeNode::FunctionCall() => write!(f, "FunctionCall"),
            TreeNode::FunctionHead() => write!(f, "FunctionHead"),
            TreeNode::If() => write!(f, "If"),
            TreeNode::IfBlock() => write!(f, "IfBlock"),
            TreeNode::IndexedVar() => write!(f, "IndexedVar"),
            TreeNode::IndiceList() => write!(f, "IndiceList"),
            TreeNode::InheritsList() => write!(f, "InheritsList"),
            TreeNode::LocalVarDecl() => write!(f, "LocalVarDecl"),
            TreeNode::MemberFunc() => write!(f, "MemberFunc"),
            TreeNode::NestedVar() => write!(f, "NestedVar"),
            TreeNode::Parameter() => write!(f, "Parameter"),
            TreeNode::ParameterList() => write!(f, "ParameterList"),
            TreeNode::Program() => write!(f, "Program"),
            TreeNode::Read() => write!(f, "Read"),
            TreeNode::RelExpr() => write!(f, "RelExpr"),
            TreeNode::Return() => write!(f, "Return"),
            TreeNode::RightRecArithExpr() => write!(f, "RightRecArithExpr"),
            TreeNode::RightRecTerm() => write!(f, "RightRecTerm"),
            TreeNode::Scope() => write!(f, "Scope"),
            TreeNode::Variable() => write!(f, "Variable"),
            TreeNode::While() => write!(f, "While"),
            TreeNode::WhileBlock() => write!(f, "WhileBlock"),
            TreeNode::Write() => write!(f, "Write"),
        }
    }
}