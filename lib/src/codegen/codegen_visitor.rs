use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{
    ast::nodes::{CodeNode, NodeValue, VarType},
    compiler_error::CompilerError,
    lexical::tokens::token_type::Type,
    semantic::{
        visitor::{Visitor, VisitorResult, FLOAT_SIZE, INT_SIZE},
        visitor_utils::{get_current_function, get_global_table, get_symbol_data},
    },
};

pub struct CodegenVisitor {
    alloc: String,
    code: String,
    label_count: AtomicUsize,
    registers: Vec<String>,
}

const SAVE_REGISTERS: &str = "% Save registers
addi r12, r0, regbuf
sw 0(r12), r1
sw -4(r12), r2
sw -8(r12), r3
sw -12(r12), r4
";

const RESTORE_REGISTERS: &str = "% Restore registers
addi r12, r0, regbuf
lw r1, 0(r12)
lw r2, -4(r12)
lw r3, -8(r12)
lw r4, -12(r12)
";

impl CodegenVisitor {
    pub fn new() -> Self {
        Self {
            alloc: String::from("strbuf res 32\nregbuf res 16\n"),
            code: String::from(""),
            label_count: AtomicUsize::new(0),
            registers: (1..12).map(|r| format!("r{r}",)).rev().collect(),
        }
    }

    #[track_caller]
    fn get_register(&mut self) -> String {
        self.registers.pop().unwrap()
        // let reg = self.registers.pop().unwrap();
        // eprintln!(
        //     " Taking register: {:<3} ({:>2} left) {}",
        //     reg,
        //     self.registers.len(),
        //     Location::caller(),
        // );
        // reg
    }

    fn free_register(&mut self, reg: String) {
        if reg == "r0" || reg == "r13" {
            return;
        }
        // eprintln!(
        //     "Freeing register: {:<3} ({:>2} left)",
        //     reg,
        //     self.registers.len() + 1
        // );
        self.registers.push(reg);
    }

    pub fn new_temp_label(&self) -> String {
        format!("t{}", self.label_count.fetch_add(1, Ordering::SeqCst))
    }

    // % \"i mentioned this last week-ish in this chat, but i found that the topaddr of 16000 is actually the location that data in r0 is stored
    // % so at the beginning of the program you just have to decrease r14 by 4\" -- mamamia on discord thank you so much
    pub fn get_code(&self) -> String {
        format!(
            "align
{}
entry
% set up stack
addi r14, r0, topaddr
% required to avoid overwriting r0
subi r14, r14, 4
jl r15, main
hlt

{}",
            self.alloc, self.code
        )
    }
}

impl Default for CodegenVisitor {
    fn default() -> Self {
        Self::new()
    }
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
            _ => {
                return Err(CompilerError::new(
                    format!("Expected identifier at '{}'!", node.borrow().value),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let symbol_data = get_symbol_data(node, id).ok_or_else(|| {
            CompilerError::new(
                format!("Found no symbol table entry for '{}'!", id),
                node.borrow().token.clone(),
            )
        })?;

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
        id: Type,
        _indices: Option<CodeNode>,
    ) -> VisitorResult {
        let id_str = match id {
            Type::Id(id) => id,
            _ => {
                return Err(CompilerError::new(
                    format!("Expected identifier at '{}'!", node.borrow().value),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let symbol_data = get_symbol_data(node, &id_str).ok_or_else(|| {
            CompilerError::new(
                format!("Found no symbol table entry for '{}'!", id_str),
                node.borrow().token.clone(),
            )
        })?;

        let label = symbol_data.borrow().label.clone().ok_or_else(|| {
            CompilerError::new(
                format!("Found no label for '{}', was it never initialized?", id_str),
                node.borrow().token.clone(),
            )
        })?;

        node.borrow().label.borrow_mut().replace(label);
        Ok(())
    }

    fn visit_factor(&mut self, node: &CodeNode, factor: Type) -> VisitorResult {
        let (key, size, lit, bytes) = get_literal_label(&factor);

        let mut code = String::new();
        let reg = self.get_register();

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
        let left_label = left.borrow().label.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                format!("Expected a label at {}", left),
                left.borrow().token.clone(),
            )
        })?;

        let left_code = left.borrow().code.borrow().clone();

        let right_label = right.borrow().label.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                format!("Expected a label at {}", right),
                right.borrow().token.clone(),
            )
        })?;

        let right_code = right.borrow().code.borrow().clone();

        let operand = match op {
            Type::Eq => "ceq",
            Type::NotEq => "cne",
            Type::Lt => "clt",
            Type::LEq => "cle",
            Type::Gt => "cgt",
            Type::GEq => "cge",
            _ => {
                return Err(CompilerError::new(
                    format!("Expected operator at {}!", op),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let reg = self.get_register();
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
            let reg = self.get_register();
            code.push_str(&format!("lw {reg}, {left_label}\n"));
            reg
        };

        let r_reg = if is_reg(&right_label) {
            right_label
        } else {
            let reg = self.get_register();
            code.push_str(&format!("lw {reg}, {right_label}\n"));
            reg
        };

        code.push_str(&format!(
            "{operand} {reg}, {l_reg}, {r_reg}\n% end rel expression\n"
        ));

        self.free_register(l_reg);
        self.free_register(r_reg);

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
        let left_label = left.borrow().label.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                format!("Expected a label at {}", left),
                left.borrow().token.clone(),
            )
        })?;

        let left_code = left.borrow().code.borrow().clone();

        let right_label = right.borrow().label.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                format!("Expected a label at {}", right),
                right.borrow().token.clone(),
            )
        })?;

        let right_code = right.borrow().code.borrow().clone();

        let operand = match op.borrow().value {
            NodeValue::Leaf(ref leaf) => match leaf {
                Type::Plus => "add",
                Type::Minus => "sub",
                Type::Mult => "mul",
                Type::Div => "div",
                _ => {
                    return Err(CompilerError::new(
                        format!("Expected operator at {}!", op),
                        node.borrow().token.clone(),
                    )
                    .into())
                }
            },
            _ => {
                return Err(CompilerError::new(
                    format!("Expected operator at {}!", op),
                    op.borrow().token.clone(),
                )
                .into())
            }
        };

        let reg = self.get_register();
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
            let reg = self.get_register();
            code.push_str(&format!("lw {reg}, {left_label}\n"));
            reg
        };

        let r_reg = if is_reg(&right_label) {
            right_label
        } else {
            let reg = self.get_register();
            code.push_str(&format!("lw {reg}, {right_label}\n"));
            reg
        };

        code.push_str(&format!(
            "{operand} {reg}, {l_reg}, {r_reg}\n% end arith expression\n"
        ));

        self.free_register(l_reg);
        self.free_register(r_reg);

        node.borrow().label.borrow_mut().replace(reg);
        node.borrow().code.borrow_mut().replace(code);
        Ok(())
    }

    fn visit_expr(&mut self, node: &CodeNode, expr: Vec<CodeNode>) -> VisitorResult {
        let node_ref = node.borrow();
        let first = expr.first().unwrap().borrow();

        let label = first.label.borrow().clone();
        *node_ref.label.borrow_mut() = label;

        let code = first.code.borrow().clone();
        *node_ref.code.borrow_mut() = code;

        let var_type = first.var_type.borrow().clone();
        *node_ref.var_type.borrow_mut() = var_type;

        Ok(())
    }

    fn visit_assignment(
        &mut self,
        node: &CodeNode,
        variable: CodeNode,
        expr: CodeNode,
    ) -> VisitorResult {
        // Variables store their labels
        let variable_label = variable.borrow().label.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                format!("Expected label at {}!", node),
                node.borrow().token.clone(),
            )
        })?;

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
            self.free_register(expr_label);
        }

        Ok(())
    }

    fn visit_while(
        &mut self,
        node: &CodeNode,
        condition: CodeNode,
        while_block: CodeNode,
    ) -> VisitorResult {
        let condition_label = condition.borrow().label.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                format!("Expected label at {}!", condition),
                condition.borrow().token.clone(),
            )
        })?;

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

        if is_reg(&condition_label) {
            self.free_register(condition_label);
        }

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
        let condition_label = condition.borrow().label.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                format!("Expected label at {}!", condition),
                condition.borrow().token.clone(),
            )
        })?;

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

        let cur_func = get_current_function(node).ok_or_else(|| {
            CompilerError::new(
                format!("Expected parent function at {}!", node),
                node.borrow().token.clone(),
            )
        })?;

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
        code.push_str("% Read()\n");
        code.push_str(SAVE_REGISTERS);
        // run any var init code
        code.push_str(&var_code);
        // inc stack pointer
        code.push_str(&format!("addi r14, r14, -{func_size}\n"));
        // store buffer pointer at -8(r14)
        let buf_reg = self.get_register();
        code.push_str(&format!("addi {buf_reg}, r0, strbuf\n"));
        code.push_str(&format!("sw -8(r14), {buf_reg}\n"));
        self.free_register(buf_reg);
        // call getstr
        code.push_str("jl r15, getstr\n");
        // call strint
        code.push_str("jl r15, strint\n");
        // decr stack pointer
        code.push_str(&format!("addi r14, r14, {func_size}\n"));
        // store r13 in variable
        code.push_str(&format!("sw {var_label}, r13\n"));
        code.push_str(RESTORE_REGISTERS);

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

        let cur_func = get_current_function(node).ok_or_else(|| {
            CompilerError::new(
                format!("Expected parent function at {}!", node),
                node.borrow().token.clone(),
            )
        })?;

        let func_size = cur_func
            .borrow()
            .symbol_table
            .borrow()
            .clone()
            .unwrap()
            .values()
            .fold(0, |acc, x| acc + x.borrow().size)
            + 4;

        let expr_label = expr
            .borrow()
            .label
            .borrow()
            .clone()
            .expect("Expected label!");

        let expr_code = expr.borrow().code.borrow().clone().unwrap_or_default();

        let mut code = String::new();

        code.push_str("% Write()\n");
        code.push_str(SAVE_REGISTERS);
        code.push_str(&expr_code);
        let expr_reg = self.get_register();

        // store expr int
        code.push_str("% store expr int\n");
        if is_reg(&expr_label) {
            code.push_str(&format!("add {expr_reg}, r0, {expr_label}\n"));
            self.free_register(expr_label);
        } else {
            code.push_str(&format!("lw {expr_reg}, {expr_label}\n"));
        }
        code.push_str("% write call\n");
        // inc stack pointer
        code.push_str(&format!("addi r14, r14, -{func_size}\n"));
        code.push_str(&format!("sw -8(r14), {expr_reg}\n"));
        self.free_register(expr_reg);
        // store buffer pointer
        let buf_reg = self.get_register();
        code.push_str(&format!("addi {buf_reg}, r0, strbuf\n"));
        code.push_str(&format!("sw -12(r14), {buf_reg}\n"));
        self.free_register(buf_reg);
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
        code.push_str(RESTORE_REGISTERS);
        code.push_str("% write end, return stack pointer\n");
        code.push_str(&format!("addi r14, r14, {func_size}\n\n"));

        node.borrow().code.borrow_mut().replace(code);

        Ok(())
    }

    fn visit_return(&mut self, node: &CodeNode, expr: CodeNode) -> VisitorResult {
        // We just always dump the return val (or pointer to it) in r13
        let node_ref = node.borrow();
        let expr_ref = expr.borrow();
        let mut code = expr_ref.code.borrow().clone().unwrap_or_default();
        let label = expr_ref.label.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                "Return expected a label for the expression, but found none!".to_string(),
                node_ref.token.clone(),
            )
        })?;

        let expr_type = expr_ref.var_type.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                "Failed to get type of expression".to_string(),
                expr_ref.token.clone(),
            )
        })?;

        let func_table = get_current_function(node)
            .unwrap()
            .borrow()
            .symbol_table
            .clone();

        let func_table = func_table.borrow().clone().unwrap();

        let (_, return_type) =
            func_table
                .iter()
                .find(|(k, _)| k == &"_return")
                .ok_or(CompilerError::new(
                    "Function return type not found in head table".to_string(),
                    node_ref.token.clone(),
                ))?;

        let return_type = return_type.borrow().clone().var_type;

        if expr_type != return_type {
            return Err(CompilerError::new(
                format!("Return type mismatch (expected {return_type}, got {expr_type})!",),
                node_ref.token.clone(),
            )
            .into());
        }

        if is_reg(&label) {
            code.push_str(&format!("add r13, r0, {label}\n"));
            self.free_register(label);
        } else {
            code.push_str(&format!("lw r13, {label}\n"));
        }

        node_ref.code.borrow_mut().replace(code);
        node_ref.label.borrow_mut().replace("r13".to_string());

        Ok(())
    }

    fn visit_program(&mut self, node: &CodeNode, classes_or_funcs: Vec<CodeNode>) -> VisitorResult {
        let mut code = String::new();

        classes_or_funcs.iter().for_each(|child| {
            if let Some(child_code) = child.borrow().code.borrow().clone() {
                code.push_str(&child_code);
                code.push('\n');
            };
        });

        node.borrow().code.borrow_mut().replace(code.clone());
        self.code.push_str(&code);
        Ok(())
    }

    fn visit_function(&mut self, node: &CodeNode, head: CodeNode, body: CodeNode) -> VisitorResult {
        let node_ref = node.borrow();
        let head_ref = head.borrow();
        let head_table = head_ref.symbol_table.borrow();

        let (func_name, _) = head_table
            .as_ref()
            .unwrap()
            .iter()
            .find(|(_, value)| value.borrow().var_type == VarType::Function)
            .ok_or(CompilerError::new(
                "Function not found in head table".to_string(),
                node_ref.token.clone(),
            ))?;

        let func_label = head_ref.label.borrow().clone().unwrap();

        let mut code = String::new();

        code.push_str(&format!("% func {func_name}:\n"));
        code.push_str(&format!("{func_label} nop\n"));
        code.push_str("% push return address\n");
        code.push_str("sw 0(r14), r15\n");

        body.children().for_each(|child| {
            if let Some(child_code) = child.borrow().code.borrow().clone() {
                code.push_str(&child_code);
            };
        });

        code.push_str("% return\n");
        code.push_str("lw r15, 0(r14)\n");
        code.push_str("jr r15\n");
        code.push_str(&format!("% end func {func_name}\n"));

        node_ref.code.borrow_mut().replace(code);
        Ok(())
    }

    fn visit_function_call(
        &mut self,
        node: &CodeNode,
        id: Type,
        param_list: CodeNode,
    ) -> VisitorResult {
        let node_ref = node.borrow();
        let func_name = match id {
            Type::Id(id) => id,
            _ => {
                return Err(CompilerError::new(
                    format!("Expected identifier at '{}'!", node.borrow().value),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let param_types = param_list
            .children()
            .map(|c| {
                let c_ref = c.borrow();
                let c_type = c_ref.var_type.borrow().clone();
                c_type.map_or_else(|| String::from("?"), |v| v.to_string())
            })
            .collect::<Vec<_>>()
            .join(", ");

        let func_signature = format!("{}({})", func_name, param_types);
        let global_table = get_global_table(node)?;

        let func_data = global_table.get(&func_signature).ok_or_else(|| {
            node_ref
                .code
                .borrow_mut()
                .replace("addi r13, r0 , 0".to_string());
            node_ref.label.borrow_mut().replace("r13".to_string());
            CompilerError::new(
                format!(
                    "Failed to generate function call, '{}' not found!",
                    func_signature
                ),
                node_ref.token.clone(),
            )
        })?;
        let func_label = func_data.borrow().label.clone().unwrap();
        let func_table = func_data.borrow().table.clone().unwrap();

        let us_func = get_current_function(node).ok_or_else(|| {
            CompilerError::new(
                "Function call not inside a function!".to_string(),
                node_ref.token.clone(),
            )
        })?;
        let us_ref = us_func.borrow();
        let us_size = us_ref
            .symbol_table
            .borrow()
            .as_ref()
            .unwrap()
            .values()
            .fold(0, |acc, v| {
                let v_ref = v.borrow();
                acc + v_ref.size
            });

        // We know the function exists, and that the types match
        // so now we need to:
        //   - Setup the stack frame
        //   - Push the parameters
        //   - Call the function

        // Setup the stack frame
        let mut code = String::new();
        code.push_str(&format!("% Call function {func_signature}\n"));
        // Run the init code for the params, if any so `add(1 + 3, a + b)` works
        for child in param_list.children() {
            if let Some(child_code) = child.borrow().code.borrow().clone() {
                code.push_str(&child_code);
            };
        }

        // Push the parameters
        let mut offset: isize = -4;
        code.push_str("% Push parameters\n");

        let mut push_code = String::new();

        for child in param_list.children() {
            let child_ref = child.borrow();
            let child_label = child_ref.label.borrow().clone().unwrap();
            let child_type = child_ref.var_type.borrow().clone().unwrap();

            let (param_name, param_data) = func_table
                .iter()
                .find(|e| {
                    let (_, v) = e;
                    v.borrow().var_type.eq_variant(&child_type) && v.borrow().offset == offset
                })
                .ok_or_else(|| {
                    CompilerError::new(
                        format!(
                            "Parameter '{}' not found in function '{}'!",
                            child_type, func_signature
                        ),
                        node_ref.token.clone(),
                    )
                })?;

            let param_size = param_data.borrow().size;

            push_code.push_str(&format!("% Push parameter '{param_name}'\n"));
            let c_reg = if is_reg(&child_label) {
                push_code.push_str(&format!("sw {offset}(r14), {child_label}\n"));
                child_label
            } else {
                let reg = self.get_register();
                code.push_str(&format!("lw {reg}, {child_label}\n"));
                push_code.push_str(&format!("sw {offset}(r14), {reg}\n"));
                reg
            };
            self.free_register(c_reg);

            offset -= param_size as isize;
        }

        code.push_str(&format!("addi r14, r14, -{us_size}\n"));
        code.push_str(&push_code);

        code.push_str("% Call function\n");
        code.push_str(&format!("jl r15, {func_label}\n"));

        code.push_str("% Return stack pointer\n");
        code.push_str(&format!("addi r14, r14, {us_size}\n"));

        // return is always stored in 13
        let ret_reg = self.get_register();
        code.push_str("% Store return value\n");
        code.push_str(&format!("add {ret_reg}, r0, r13\n"));

        code.push_str(&format!("% End function call {func_signature}\n\n"));

        node_ref.code.borrow_mut().replace(code);
        node_ref.label.borrow_mut().replace(ret_reg);
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
