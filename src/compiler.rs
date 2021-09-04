use crate::parser::Ast;

fn compile_term(term: &Ast) -> String {
    match term {
        Ast::Bracket(expr) => format!("while(*ptr) {{\n{}}}\n", compile_expr(expr)),
        Ast::Gt(i) => format!("ptr += {};\n", i),
        Ast::Lt(i) => format!("ptr -= {};\n", i),
        Ast::Add(i) => format!("(*ptr) += {};\n", i),
        Ast::Sub(i) => format!("(*ptr) -= {};\n", i),
        Ast::Dot => String::from("putchar(*ptr);\n"),
        Ast::Comma => String::from("(*ptr) = getchar();\n"),
    }
}

fn compile_expr(parsers: &[Ast]) -> String {
    match parsers.len() {
        0 => String::new(),
        1 => compile_term(&parsers[0]),
        _ => format!(
            "{}{}",
            compile_term(&parsers[0]),
            compile_expr(&parsers[1..])
        ),
    }
}

pub fn compile(parsers: &[Ast], size: i32) -> String {
    format!(
        "#include <stdio.h>
#include <stdlib.h>

int main(int argc, char** argv) {{
int len = {};
int* ptr;
ptr = (int*) calloc(len, sizeof(int));
if (ptr == NULL) exit(1);
{}
// if (ptr != NULL) free(ptr);
return 0;
}}",
        size,
        compile_expr(parsers)
    )
}
