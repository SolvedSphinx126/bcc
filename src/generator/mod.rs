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
    if let Expression::Constant(Constant { value: num }) = exp {
        lines.push(format!("  mov ${}, %rax", num));
    } else if let Expression::UnaryOp(Operator::Negation, exp) = exp {
        lines.append(&mut generate_expression(*exp));
        lines.push(format!("  neg %rax"));
    } else if let Expression::UnaryOp(Operator::BitwiseComplement, exp) = exp {
        lines.append(&mut generate_expression(*exp));
        lines.push(format!("  not %rax"));
    } else if let Expression::UnaryOp(Operator::LogicalNegation, exp) = exp {
        lines.append(&mut generate_expression(*exp));
        lines.push(format!("  cmp $0, %rax"));
        lines.push(format!("  mov $0, %rax"));
        lines.push(format!("  sete %al"));
    } else {
        panic!("This should never run, and means that generate_expression can't handle successfully parsed code");
    }
    lines
}
