use crate::{
    ast::{
        nodes::{CodeNode, NodeValue},
        tree_node::TreeNode,
    },
    lexical::tokens::token_type::Type,
};

pub type CollectedVisitorResult = Result<(), Vec<String>>;
pub type VisitorResult = Result<(), String>;

pub trait Visitor {
    fn visit(&mut self, node: &CodeNode) -> CollectedVisitorResult {
        let mut errors: Vec<String> = Vec::new();

        if let Err(e) = match &node.borrow().value {
            NodeValue::Leaf(l) => self.visit_leaf(node, l),
            NodeValue::Tree(t) => self.visit_tree(node, t),
            NodeValue::Marker => panic!("Unexpected marker node!"),
        } {
            errors.push(e);
        };

        for child in node.children() {
            if let Err(e) = self.visit(&child) {
                errors.extend(e);
            };
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn visit_leaf(&mut self, _node: &CodeNode, _type_node: &Type) -> VisitorResult {
        Ok(())
    }

    fn visit_tree(&mut self, node: &CodeNode, tree_node: &TreeNode) -> VisitorResult {
        let mut children = node.children();

        match tree_node {
            TreeNode::ArgumentList() => self.visit_argument_list(node, children.collect()),
            TreeNode::ArithExpr() => self.visit_arith_expr(node, children.next().unwrap()),
            TreeNode::Assignment() => {
                self.visit_assignment(node, children.next().unwrap(), children.next().unwrap())
            }
            TreeNode::Attribute() => self.visit_attribute(
                node,
                children.next().unwrap().try_into()?,
                children.next().unwrap().try_into()?,
                children.next().unwrap().try_into()?,
                children.next().unwrap(),
            ),
            TreeNode::Class() => self.visit_class(
                node,
                children.next().unwrap().try_into()?,
                children.next().unwrap(),
                children.next().unwrap(),
            ),
            TreeNode::ClassMembers() => self.visit_class_members(node, children.collect()),
            TreeNode::ConstructorFunc() => self.visit_constructor_func(
                node,
                children.next().unwrap().try_into()?,
                children.next().unwrap().try_into()?,
                children.next().unwrap(),
            ),
            TreeNode::ElseBlock() => self.visit_else_block(node, children.collect()),
            TreeNode::Expr() => self.visit_expr(node, children.collect()),
            TreeNode::Factor() => self.visit_factor(node, children.next().unwrap().try_into()?),
            TreeNode::Function() => {
                self.visit_function(node, children.next().unwrap(), children.next().unwrap())
            }
            TreeNode::FunctionBody() => self.visit_function_body(node, children.collect()),
            TreeNode::FunctionCall() => {
                self.visit_function_call(node, children.next().unwrap(), children.next().unwrap())
            }
            TreeNode::FunctionHead() => self.visit_function_head(
                node,
                children.next().unwrap(),
                children.next().unwrap(),
                children
                    .next()
                    .map(|c| c.try_into())
                    .map_or(Ok(None), |r| r.map(Some))?,
            ),
            TreeNode::If() => {
                self.visit_if(node, children.next().unwrap(), children.next().unwrap())
            }
            TreeNode::IfBlock() => self.visit_if_block(node, children.collect()),
            TreeNode::IndexedVar() => self.visit_indexed_var(node, children.collect()),
            TreeNode::IndiceList() => self.visit_indice_list(node, children.collect()),
            TreeNode::InheritsList() => self.visit_inherits_list(node, children.collect()),
            TreeNode::LocalVarDecl() => self.visit_local_var_decl(
                node,
                children.next().unwrap().try_into()?,
                children.next().unwrap().try_into()?,
                children.next().unwrap(),
            ),
            TreeNode::MemberFunc() => self.visit_member_func(
                node,
                children.next().unwrap().try_into()?,
                children.next().unwrap().try_into()?,
                children.next().unwrap(),
                children.next().unwrap().try_into()?,
            ),
            TreeNode::NestedVar() => self.visit_nested_var(
                node,
                children.next().unwrap().try_into()?,
                children.next().unwrap(),
            ),
            TreeNode::Parameter() => self.visit_parameter(
                node,
                children.next().unwrap().try_into()?,
                children.next().unwrap().try_into()?,
                children.next().unwrap(),
            ),
            TreeNode::ParameterList() => self.visit_parameter_list(node, children.collect()),
            TreeNode::Program() => self.visit_program(node, children.collect()),
            TreeNode::Read() => self.visit_read(node, children.next().unwrap()),
            TreeNode::RelExpr() => self.visit_rel_expr(
                node,
                children.next().unwrap(),
                children.next().unwrap().try_into()?,
                children.next().unwrap(),
            ),
            TreeNode::Return() => self.visit_return(node, children.next().unwrap()),
            TreeNode::RightRecArithExpr() => self.visit_right_rec_arith_expr(
                node,
                children.next().unwrap().try_into()?,
                children.next().unwrap(),
            ),
            TreeNode::RightRecTerm() => self.visit_right_rec_term(
                node,
                children.next().unwrap().try_into()?,
                children.next().unwrap(),
            ),
            TreeNode::Scope() => self.visit_scope(
                node,
                children.next().unwrap().try_into()?,
                children.next().unwrap().try_into()?,
            ),
            TreeNode::Variable() => self.visit_variable(node, children.collect()),
            TreeNode::While() => {
                self.visit_while(node, children.next().unwrap(), children.next().unwrap())
            }
            TreeNode::WhileBlock() => self.visit_while_block(node, children.collect()),
            TreeNode::Write() => self.visit_write(node, children.next().unwrap()),
        }?;

        Ok(())
    }

    fn visit_argument_list(
        &mut self,
        _node: &CodeNode,
        _argument_list: Vec<CodeNode>,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_arith_expr(&mut self, _node: &CodeNode, _arith_expr: CodeNode) -> VisitorResult {
        Ok(())
    }

    fn visit_assignment(
        &mut self,
        _node: &CodeNode,
        _variable: CodeNode,
        _expr: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_attribute(
        &mut self,
        _node: &CodeNode,
        _visibility: Type,
        _id: Type,
        _type: Type,
        _indices: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_class(
        &mut self,
        _node: &CodeNode,
        _id: Type,
        _inherits: CodeNode,
        _members: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_class_members(&mut self, _node: &CodeNode, _members: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_constructor_func(
        &mut self,
        _node: &CodeNode,
        _visibility: Type,
        _id: Type,
        _params: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_else_block(&mut self, _node: &CodeNode, _body: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_expr(&mut self, _node: &CodeNode, _expr: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_factor(&mut self, _node: &CodeNode, _factor: Type) -> VisitorResult {
        Ok(())
    }

    fn visit_function(
        &mut self,
        _node: &CodeNode,
        _head: CodeNode,
        _body: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_function_body(&mut self, _node: &CodeNode, _body: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_function_call(
        &mut self,
        _node: &CodeNode,
        _id: CodeNode,
        _param_list: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_function_head(
        &mut self,
        _node: &CodeNode,
        _id: CodeNode,
        _param_list: CodeNode,
        _return_type: Option<Type>,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_if(
        &mut self,
        _node: &CodeNode,
        _condition: CodeNode,
        _if_block: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_if_block(&mut self, _node: &CodeNode, _body: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_indexed_var(&mut self, _node: &CodeNode, _expr: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_indice_list(&mut self, _node: &CodeNode, _indices: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_inherits_list(&mut self, _node: &CodeNode, _inherits: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_local_var_decl(
        &mut self,
        _node: &CodeNode,
        _id: Type,
        _type: Type,
        _indice_or_args: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_member_func(
        &mut self,
        _node: &CodeNode,
        _visibility: Type,
        _id: Type,
        _params: CodeNode,
        _return_type: Type,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_nested_var(
        &mut self,
        _node: &CodeNode,
        _id: Type,
        _indices: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_parameter(
        &mut self,
        _node: &CodeNode,
        _id: Type,
        _type: Type,
        _indices: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_parameter_list(&mut self, _node: &CodeNode, _params: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_program(
        &mut self,
        _node: &CodeNode,
        _classes_or_funcs: Vec<CodeNode>,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_read(&mut self, _node: &CodeNode, _variable: CodeNode) -> VisitorResult {
        Ok(())
    }

    fn visit_rel_expr(
        &mut self,
        _node: &CodeNode,
        _left: CodeNode,
        _op: Type,
        _right: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_return(&mut self, _node: &CodeNode, _expr: CodeNode) -> VisitorResult {
        Ok(())
    }

    fn visit_right_rec_arith_expr(
        &mut self,
        _node: &CodeNode,
        _op: Type,
        _expr: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_right_rec_term(
        &mut self,
        _node: &CodeNode,
        _op: Type,
        _expr: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_scope(&mut self, _node: &CodeNode, _super: Type, _id: Type) -> VisitorResult {
        Ok(())
    }

    fn visit_variable(&mut self, _node: &CodeNode, _nodes: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_while(
        &mut self,
        _node: &CodeNode,
        _condition: CodeNode,
        _while_block: CodeNode,
    ) -> VisitorResult {
        Ok(())
    }

    fn visit_while_block(&mut self, _node: &CodeNode, _body: Vec<CodeNode>) -> VisitorResult {
        Ok(())
    }

    fn visit_write(&mut self, _node: &CodeNode, _expr: CodeNode) -> VisitorResult {
        Ok(())
    }
}
