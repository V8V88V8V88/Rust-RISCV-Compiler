use crate::ast::{Expr, Stmt};
use std::collections::HashMap;

struct CodeGenerator {
    output: String,
    variables: HashMap<String, i32>,
    temp_counter: i32,
    label_counter: i32,
}

impl CodeGenerator {
    fn new() -> Self {
        CodeGenerator {
            output: String::new(),
            variables: HashMap::new(),
            temp_counter: 0,
            label_counter: 0,
        }
    }

    fn generate(&mut self, ast: Vec<Stmt>) -> String {
        self.output.push_str(".text\n");
        self.output.push_str(".globl main\n");
        self.output.push_str("main:\n");

        for stmt in ast {
            self.generate_stmt(stmt);
        }

        self.output.push_str("    li a0, 0\n");
        self.output.push_str("    ret\n");

        self.output.clone()
    }

    fn generate_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Assign(name, expr) => {
                let temp = self.generate_expr(expr);
                let offset = self.variables.len() as i32 * 4;
                self.variables.insert(name, offset);
                self.output
                    .push_str(&format!("    sw {}, {}(sp)\n", temp, offset));
            }
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.generate_stmt(stmt);
                }
            }
            Stmt::Expr(expr) => {
                self.generate_expr(expr);
            }
        }
    }

    fn generate_expr(&mut self, expr: Expr) -> String {
        match expr {
            Expr::Const(value) => {
                let temp = self.new_temp();
                self.output
                    .push_str(&format!("    li {}, {}\n", temp, value));
                temp
            }
            Expr::Var(name) => {
                let temp = self.new_temp();
                let offset = self.variables[&name];
                self.output
                    .push_str(&format!("    lw {}, {}(sp)\n", temp, offset));
                temp
            }
            Expr::Add(left, right) => {
                let left_temp = self.generate_expr(*left);
                let right_temp = self.generate_expr(*right);
                let result_temp = self.new_temp();
                self.output.push_str(&format!(
                    "    add {}, {}, {}\n",
                    result_temp, left_temp, right_temp
                ));
                result_temp
            }
            Expr::Sub(left, right) => {
                let left_temp = self.generate_expr(*left);
                let right_temp = self.generate_expr(*right);
                let result_temp = self.new_temp();
                self.output.push_str(&format!(
                    "    sub {}, {}, {}\n",
                    result_temp, left_temp, right_temp
                ));
                result_temp
            }
            Expr::Mul(left, right) => {
                let left_temp = self.generate_expr(*left);
                let right_temp = self.generate_expr(*right);
                let result_temp = self.new_temp();
                self.output.push_str(&format!(
                    "    mul {}, {}, {}\n",
                    result_temp, left_temp, right_temp
                ));
                result_temp
            }
            Expr::Div(left, right) => {
                let left_temp = self.generate_expr(*left);
                let right_temp = self.generate_expr(*right);
                let result_temp = self.new_temp();
                self.output.push_str(&format!(
                    "    div {}, {}, {}\n",
                    result_temp, left_temp, right_temp
                ));
                result_temp
            }
            Expr::Greater(left, right) => {
                let left_temp = self.generate_expr(*left);
                let right_temp = self.generate_expr(*right);
                let result_temp = self.new_temp();
                self.output.push_str(&format!(
                    "    sgt {}, {}, {}\n",
                    result_temp, left_temp, right_temp
                ));
                result_temp
            }
            Expr::If(condition, then_branch, else_branch) => {
                let condition_temp = self.generate_expr(*condition);
                let then_label = self.new_label();
                let else_label = self.new_label();
                let end_label = self.new_label();

                self.output
                    .push_str(&format!("    bnez {}, {}\n", condition_temp, then_label));
                self.output.push_str(&format!("    j {}\n", else_label));

                self.output.push_str(&format!("{}:\n", then_label));
                self.generate_stmt(*then_branch);
                self.output.push_str(&format!("    j {}\n", end_label));

                self.output.push_str(&format!("{}:\n", else_label));
                if let Some(else_branch) = else_branch {
                    self.generate_stmt(*else_branch);
                }

                self.output.push_str(&format!("{}:\n", end_label));

                self.new_temp()
            }
        }
    }

    fn new_temp(&mut self) -> String {
        let temp = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;
        label
    }
}

pub fn generate(ast: Vec<Stmt>) -> String {
    let mut generator = CodeGenerator::new();
    generator.generate(ast)
}
