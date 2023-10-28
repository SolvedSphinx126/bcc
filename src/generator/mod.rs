use crate::parser::ast_node::*;

pub fn generate(pgrm: Program) -> Vec<String> {
    generate_function(pgrm.fns)
}

fn generate_function(fctn: Function) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    lines.push(format!("  .globl {}", fctn.name.value));
    lines.push(format!("{}:", fctn.name.value));
    lines.append(&mut generate_statement(fctn.statements));
    lines
}

fn generate_statement(stmt: Statement) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    lines.append(&mut generate_expression(stmt.result));
    lines.push("  ret".to_string());
    lines
}

fn generate_expression(exp: Expression) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    lines.push(format!("  mov ${}, %rax", exp.value));
    lines
}
