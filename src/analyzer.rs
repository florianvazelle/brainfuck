use std::cmp;

use crate::parser::Ast;

fn analyze_alloc_size(parsers: &[Ast]) -> i32 {
  let mut i: i32 = 0;
  let mut imax: i32 = 0;

  for val in parsers {
      match val {
          Ast::Gt(j) => i += j,
          Ast::Lt(j) => i -= *j,
          Ast::Bracket(expr) => i += analyze_alloc_size(expr),
          _ => continue,
      }

      imax = cmp::max(imax, i);
  }

  imax
}

pub fn analyze(parsers: &[Ast]) -> i32 {
  let size = analyze_alloc_size(parsers);

  if size <= 0 {
      panic!()
  }

  size
}
