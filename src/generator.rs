// Generate the assembly from an in order traveral of the AST

use crate::ast::{Ast, TokenType};

pub fn emit_asm_in_order(ast_root: &Ast) -> String {
    /*
    if ast_root.children.is_empty() {
        return "".to_string();
    };
    */

    match &ast_root.token_type {
        TokenType::AstRoot => { format!(".globl {}", emit_asm_in_order(&ast_root.children[0])) },
        TokenType::Function(fn_name) => { format!("_{}\n_{}:\n{}", fn_name, fn_name, emit_asm_in_order(&ast_root.children[0])) },
        TokenType::Body => format!("movl ${}, %eax\nret\n", emit_asm_in_order(&ast_root.children[0])),
        TokenType::Expression(num) => {
            //println!("where is my num {}", num);
            format!("{}", num)
        },
    }
}
