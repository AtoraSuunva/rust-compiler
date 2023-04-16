use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
    ast::{
        nodes::{CodeNode, NodeValue, SymbolData},
        tree_node::TreeNode,
    },
    lexical::tokens::token_type::Type,
    semantic::visitor::{Visitor, VisitorResult, FLOAT_SIZE, INT_SIZE},
};

pub struct CodegenVisitor {
    alloc: String,
    code: String,
    label_count: AtomicUsize,
    registers: Vec<String>,
}

impl CodegenVisitor {
    pub fn new() -> Self {
        Self {
            alloc: String::from("strbuf res 32\n"),
            code: String::from(""),
            label_count: AtomicUsize::new(0),
            registers: (1..10).map(|r| format!("r{r}",)).rev().collect(),
        }
    }

    pub fn new_temp_label(&self) -> String {
        format!("t{}", self.label_count.fetch_add(1, Ordering::SeqCst))
    }

    pub fn get_code(&self) -> String {
        format!(
            "align
{}
entry
% set up stack
addi r14, r0, topaddr
% \"i mentioned this last week-ish in this chat, but i found that the topaddr of 16000 is actually the location that data in r0 is stored
% so at the beginning of the program you just have to decrease r14 by 4\" -- mamamia on discord thank you so much
subi r14, r14, 4

% main()
{}% end main()

hlt",
            self.alloc, self.code
        )
    }
}

impl Default for CodegenVisitor {
    fn default() -> Self {
        Self::new()
    }
}

fn get_symbol_data(start: &CodeNode, id: &str) -> Option<Rc<RefCell<SymbolData>>> {
    let mut parent = start.parent();

    while let Some(p) = parent {
        if let Some(table) = p.borrow().symbol_table.borrow().clone() {
            if let Some(info) = table.get(id) {
                return Some(info.clone());
            }
        }

        parent = p.parent();
    }

    None
}

fn get_current_function(start: &CodeNode) -> Option<CodeNode> {
    let mut parent = start.parent();

    while let Some(p) = parent {
        if let NodeValue::Tree(TreeNode::Function()) = p.borrow().value {
            return Some(p.clone());
        }

        parent = p.parent();
    }

    None
}

impl Visitor for CodegenVisitor {
    fn visit_local_var_decl(
        &mut self,
        node: &CodeNode,
        id: Type,
        _type_: Type,
        _indice_or_args: CodeNode,
    ) -> VisitorResult {
        let id = match id {
            Type::Id(ref id) => id,
            _ => return Err(format!("Expected identifier at '{}'!", node.borrow().value)),
        };

        let symbol_data = get_symbol_data(node, id)
            .ok_or_else(|| format!("Found no symbol table entry for '{}'!", id))?;

        let offset = symbol_data.borrow().offset;
        let label = format!("{offset}(r14)");
        let size = symbol_data.borrow().size;

        if size == 0 {
            // Probably a class
            // let class = match type_ {
            //     Type::Id(ref id) => id,
            //     _ => return Err(format!("Expected identifier at '{}'!", node.borrow().value)),
            // };
            // TODO: class
        }

        symbol_data.borrow_mut().label = Some(label.clone());
        node.borrow().label.borrow_mut().replace(label);
        Ok(())
    }

    fn visit_variable(
        &mut self,
        node: &CodeNode,
        id: CodeNode,
        _indices: Option<CodeNode>,
    ) -> VisitorResult {
        let id_ref = id.borrow();
        let id = match id_ref.value {
            NodeValue::Leaf(Type::Id(ref id)) => id,
            _ => return Err(format!("Expected identifier at '{}'!", node.borrow().value)),
        };

        let symbol_data = get_symbol_data(node, id)
            .ok_or_else(|| format!("Found no symbol table entry for '{}'!", id))?;

        let label = symbol_data
            .borrow()
            .label
            .clone()
            .ok_or_else(|| format!("Found no label for '{}', was it never initialized?", id))?;

        node.borrow().label.borrow_mut().replace(label);
        Ok(())
    }

    fn visit_factor(&mut self, node: &CodeNode, factor: Type) -> VisitorResult {
        let (key, size, lit, bytes) = get_literal_label(&factor);

        let mut code = String::new();
        let reg = self.registers.pop().unwrap();

        if size == INT_SIZE {
            code.push_str(&format!("% assign literal {lit}\n"));
            code.push_str(&format!("addi {reg}, r0, {lit}\n"));
        } else {
            let bytes_str = bytes
                .iter()
                .map(|b| format!("{}", b))
                .collect::<Vec<String>>()
                .join(",");

            self.alloc.push_str(&format!("{key} db {}\n", bytes_str));
            code.push_str(&format!("% assign literal {lit}\n"));
            code.push_str(&format!("sw {reg}, {key}\n"));
        }

        node.borrow().label.borrow_mut().replace(reg);
        node.borrow().code.borrow_mut().replace(code);

        Ok(())
    }

    fn visit_rel_expr(
        &mut self,
        node: &CodeNode,
        left: CodeNode,
        op: Type,
        right: CodeNode,
    ) -> VisitorResult {
        let left_label = left
            .borrow()
            .label
            .borrow()
            .clone()
            .ok_or_else(|| format!("Expected a label at {}", left))?;

        let left_code = left.borrow().code.borrow().clone();

        let right_label = right
            .borrow()
            .label
            .borrow()
            .clone()
            .ok_or_else(|| format!("Expected a label at {}", right))?;

        let right_code = right.borrow().code.borrow().clone();

        let operand = match op {
            Type::Not => "not",
            Type::NotEq => "ceq",
            Type::Lt => "clt",
            Type::LEq => "cle",
            Type::Gt => "cgt",
            Type::GEq => "cge",
            _ => return Err(format!("Expected operator at {}!", op)),
        };

        let reg = self.registers.pop().unwrap();
        let mut code = String::new();

        code.push_str("% rel expression\n");

        if let Some(left_code) = left_code {
            code.push_str(&left_code);
        }

        if let Some(right_code) = right_code {
            code.push_str(&right_code);
        }

        let l_reg = if is_reg(&left_label) {
            left_label
        } else {
            let reg = self.registers.pop().unwrap();
            code.push_str(&format!("lw {reg}, {left_label}\n"));
            reg
        };

        let r_reg = if is_reg(&right_label) {
            right_label
        } else {
            let reg = self.registers.pop().unwrap();
            code.push_str(&format!("lw {reg}, {right_label}\n"));
            reg
        };

        code.push_str(&format!(
            "{operand} {reg}, {l_reg}, {r_reg}\n% end rel expression\n"
        ));

        self.registers.push(l_reg);
        self.registers.push(r_reg);

        node.borrow().label.borrow_mut().replace(reg);
        node.borrow().code.borrow_mut().replace(code);
        Ok(())
    }

    fn visit_arith_expr(
        &mut self,
        node: &CodeNode,
        left: CodeNode,
        op: CodeNode,
        right: CodeNode,
    ) -> VisitorResult {
        let left_label = left
            .borrow()
            .label
            .borrow()
            .clone()
            .ok_or_else(|| format!("Expected a label at {}", left))?;

        let left_code = left.borrow().code.borrow().clone();

        let right_label = right
            .borrow()
            .label
            .borrow()
            .clone()
            .ok_or_else(|| format!("Expected a label at {}", right))?;

        let right_code = right.borrow().code.borrow().clone();

        let operand = match op.borrow().value {
            NodeValue::Leaf(ref leaf) => match leaf {
                Type::Plus => "add",
                Type::Minus => "sub",
                Type::Mult => "mul",
                Type::Div => "div",
                _ => return Err(format!("Expected operator at {}!", op)),
            },
            _ => return Err(format!("Expected operator at {}!", op)),
        };

        let reg = self.registers.pop().unwrap();
        let mut code = String::new();

        code.push_str("% arith expression\n");

        if let Some(left_code) = left_code {
            code.push_str(&left_code);
        }

        if let Some(right_code) = right_code {
            code.push_str(&right_code);
        }

        let l_reg = if is_reg(&left_label) {
            left_label
        } else {
            let reg = self.registers.pop().unwrap();
            code.push_str(&format!("lw {reg}, {left_label}\n"));
            reg
        };

        let r_reg = if is_reg(&right_label) {
            right_label
        } else {
            let reg = self.registers.pop().unwrap();
            code.push_str(&format!("lw {reg}, {right_label}\n"));
            reg
        };

        code.push_str(&format!(
            "{operand} {reg}, {l_reg}, {r_reg}\n% end arith expression\n"
        ));

        self.registers.push(l_reg);
        self.registers.push(r_reg);

        node.borrow().label.borrow_mut().replace(reg);
        node.borrow().code.borrow_mut().replace(code);
        Ok(())
    }

    fn visit_expr(&mut self, node: &CodeNode, expr: Vec<CodeNode>) -> VisitorResult {
        let first = expr.first().unwrap();

        let label = first.borrow().label.borrow().clone();
        *node.borrow().label.borrow_mut() = label;

        let code = first.borrow().code.borrow().clone();
        *node.borrow().code.borrow_mut() = code;

        Ok(())
    }

    fn visit_assignment(
        &mut self,
        node: &CodeNode,
        variable: CodeNode,
        expr: CodeNode,
    ) -> VisitorResult {
        // Variables store their labels
        let variable_label = variable
            .borrow()
            .label
            .borrow()
            .clone()
            .ok_or_else(|| format!("Expected label at {}!", node))?;

        // Expressions store their labels
        let expr_label = expr.borrow().label.borrow().clone().unwrap();
        let expr_code = expr.borrow().code.borrow().clone();

        let mut code = String::new();

        code.push_str("% assignment\n");
        if let Some(expr_code) = expr_code {
            code.push_str(&expr_code);
        }
        code.push_str(&format!("sw {variable_label}, {expr_label}\n\n"));

        node.borrow().code.borrow_mut().replace(code);

        if is_reg(&expr_label) {
            // release the register back into the wild
            self.registers.push(expr_label);
        }

        Ok(())
    }

    fn visit_while(
        &mut self,
        node: &CodeNode,
        condition: CodeNode,
        while_block: CodeNode,
    ) -> VisitorResult {
        let condition_label = condition
            .borrow()
            .label
            .borrow()
            .clone()
            .ok_or_else(|| format!("Expected label at {}!", condition))?;

        let condition_code = condition.borrow().code.borrow().clone();

        let while_block_code = while_block
            .children()
            .filter_map(|c| c.borrow().code.borrow().clone())
            .collect::<Vec<_>>()
            .join("\n");

        let while_label = self.new_temp_label();
        let end_label = self.new_temp_label();
        let mut code = String::new();

        code.push_str("% while\n");
        code.push_str(&format!("{while_label} nop\n"));
        if let Some(condition_code) = condition_code {
            code.push_str(&condition_code);
        }

        // Branch if zero to the end
        code.push_str(&format!("bz {condition_label}, {end_label}\n"));
        code.push_str("% while block\n");
        code.push_str(&while_block_code);
        code.push_str(&format!("j {while_label}\n"));
        code.push_str("% end while block\n");
        code.push_str(&format!("{end_label} nop\n"));
        code.push_str("% end while\n");

        node.borrow().code.borrow_mut().replace(code);
        Ok(())
    }

    fn visit_if(
        &mut self,
        node: &CodeNode,
        condition: CodeNode,
        if_block: CodeNode,
        else_block: CodeNode,
    ) -> VisitorResult {
        let condition_label = condition
            .borrow()
            .label
            .borrow()
            .clone()
            .ok_or_else(|| format!("Expected label at {}!", condition))?;

        let condition_code = condition.borrow().code.borrow().clone();

        let if_block_code = if_block
            .children()
            .filter_map(|c| c.borrow().code.borrow().clone())
            .collect::<Vec<_>>()
            .join("\n");

        let else_block_code = else_block
            .children()
            .filter_map(|c| c.borrow().code.borrow().clone())
            .collect::<Vec<_>>()
            .join("\n");

        let else_label = self.new_temp_label();
        let end_label = self.new_temp_label();
        let mut code = String::new();

        code.push_str("% if\n");
        if let Some(condition_code) = condition_code {
            code.push_str(&condition_code);
        }

        // Branch if zero to the else block
        code.push_str(&format!("bz {condition_label}, {else_label}\n"));
        code.push_str("% if block\n");
        code.push_str(&if_block_code);
        code.push_str(&format!("j {end_label}\n"));
        code.push_str("% else block\n");
        code.push_str(&format!("{else_label} nop\n"));
        code.push_str(&else_block_code);
        code.push_str(&format!("{end_label} nop\n"));

        node.borrow().code.borrow_mut().replace(code);
        Ok(())
    }

    fn visit_read(&mut self, node: &CodeNode, variable: CodeNode) -> VisitorResult {
        // get func size
        // incr stack pointer (r14)
        // store buffer pointer at -8(r14) (at least 8 bytes)
        // call getstr
        // same -8(r14) used for pointer to string buffer (so dont do anything)
        // call strint
        // decr stack pointer r14
        // store r13 in variable
        // continue

        let cur_func = get_current_function(node)
            .ok_or_else(|| format!("Expected parent function at {}!", node))?;

        let func_size = cur_func
            .borrow()
            .symbol_table
            .borrow()
            .clone()
            .unwrap()
            .values()
            .fold(0, |acc, x| acc + x.borrow().size);

        let var_label = variable
            .borrow()
            .label
            .borrow()
            .clone()
            .expect("Expected label!");

        let var_code = variable.borrow().code.borrow().clone().unwrap_or_default();

        let mut code = String::new();
        code.push_str("% read expr\n");
        // run any var init code
        code.push_str(&var_code);
        // inc stack pointer
        code.push_str(&format!("addi r14, r14, -{}\n", func_size));
        // store buffer pointer at -8(r14)
        let buf_reg = self.registers.pop().unwrap();
        code.push_str(&format!("addi {buf_reg}, r0, strbuf\n"));
        code.push_str(&format!("sw -8(r14), {buf_reg}\n"));
        self.registers.push(buf_reg);
        // call getstr
        code.push_str("jl r15, getstr\n");
        // call strint
        code.push_str("jl r15, strint\n");
        // decr stack pointer
        code.push_str(&format!("addi r14, r14, {}\n", func_size));
        // store r13 in variable
        code.push_str(&format!("sw {var_label}, r13\n"));

        node.borrow().code.borrow_mut().replace(code);
        Ok(())
    }

    fn visit_write(&mut self, node: &CodeNode, expr: CodeNode) -> VisitorResult {
        // get func size
        // incr stack pointer (r14)
        // store int (from expr) at -8(r14)
        // store buffer pointer at -12(r14) (at least 12 bytes)
        // call intstr
        // get return value from r13 (first char of string)

        // store return value in -8(r14) (pointer to string arg)
        // call putstr
        // decr stack pointer r14
        // continue

        let cur_func = get_current_function(node)
            .ok_or_else(|| format!("Expected parent function at {}!", node))?;

        let func_size = cur_func
            .borrow()
            .symbol_table
            .borrow()
            .clone()
            .unwrap()
            .values()
            .fold(0, |acc, x| acc + x.borrow().size);

        let expr_label = expr
            .borrow()
            .label
            .borrow()
            .clone()
            .expect("Expected label!");

        let expr_code = expr.borrow().code.borrow().clone().unwrap_or_default();

        let mut code = String::new();

        code.push_str("% write expr\n");
        code.push_str(&expr_code);
        let expr_reg = self.registers.pop().unwrap();

        // store expr int
        code.push_str("% store expr int\n");
        if is_reg(&expr_label) {
            code.push_str(&format!("add {expr_reg}, r0, {expr_label}\n"));
            self.registers.push(expr_label);
        } else {
            code.push_str(&format!("lw {expr_reg}, {expr_label}\n"));
        }
        code.push_str("% write call\n");
        // inc stack pointer
        code.push_str(&format!("addi r14, r14, -{func_size}\n"));
        code.push_str(&format!("sw -8(r14), {expr_reg}\n"));
        self.registers.push(expr_reg);
        // store buffer pointer
        let buf_reg = self.registers.pop().unwrap();
        code.push_str(&format!("addi {buf_reg}, r0, strbuf\n"));
        code.push_str(&format!("sw -12(r14), {buf_reg}\n"));
        self.registers.push(buf_reg);
        // call intstr
        code.push_str("jl r15, intstr\n");
        // store return value
        code.push_str("sw -8(r14), r13\n");
        // call putstr
        code.push_str("jl r15, putstr\n");

        // print newline
        code.push_str("% write newline\n");
        code.push_str("addi r13, r0, 13\n");
        code.push_str("putc r13\n");
        code.push_str("addi r13, r0, 10\n");
        code.push_str("putc r13\n");

        // decr stack pointer
        code.push_str("% write end, return stack pointer\n");
        code.push_str(&format!("addi r14, r14, {func_size}\n"));

        node.borrow().code.borrow_mut().replace(code);

        Ok(())
    }

    fn visit_program(&mut self, node: &CodeNode, classes_or_funcs: Vec<CodeNode>) -> VisitorResult {
        let mut code = String::new();

        classes_or_funcs.iter().for_each(|child| {
            if let Some(child_code) = child.borrow().code.borrow().clone() {
                code.push_str(&child_code);
            };
        });

        node.borrow().code.borrow_mut().replace(code.clone());
        self.code.push_str(&code);
        Ok(())
    }

    fn visit_function(
        &mut self,
        node: &CodeNode,
        _head: CodeNode,
        body: CodeNode,
    ) -> VisitorResult {
        let mut code = String::new();

        body.children().for_each(|child| {
            if let Some(child_code) = child.borrow().code.borrow().clone() {
                code.push_str(&child_code);
            };
        });

        node.borrow().code.borrow_mut().replace(code);
        Ok(())
    }
}

fn get_literal_label(lit_type: &Type) -> (String, usize, String, Vec<u8>) {
    let (key, size, bytes) = match lit_type {
        Type::IntNum(ref key) => (key.to_string(), INT_SIZE, key.to_le_bytes()),
        Type::FloatNum(ref key) => (key.to_string(), FLOAT_SIZE, key.to_le_bytes()),
        _ => panic!("Expected literal!"),
    };

    // Prefix with L to avoid conflicts with labels
    (
        format!("l{key}").replace('.', "_"),
        size,
        key,
        Vec::from(bytes),
    )
}

/// Returns true if the string is a register
fn is_reg(string: &str) -> bool {
    string.starts_with('r')
}
