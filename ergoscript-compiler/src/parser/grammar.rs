mod expr;
mod stmt;

use crate::lexer::TokenKind;
use crate::parser::Parser;
use crate::syntax::SyntaxKind;

use super::marker::CompletedMarker;

pub fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    while !p.at_end() {
        stmt::stmt(p);
    }

    m.complete(p, SyntaxKind::Root)
}

#[cfg(test)]
mod tests {
    use crate::parser::check;
    use expect_test::expect;

    #[test]
    fn parse_multiple_statements() {
        check(
            "val a = 1\na",
            expect![[r#"
            Root@0..11
              VariableDef@0..10
                ValKw@0..3 "val"
                Whitespace@3..4 " "
                Ident@4..5 "a"
                Whitespace@5..6 " "
                Equals@6..7 "="
                Whitespace@7..8 " "
                Literal@8..10
                  Number@8..9 "1"
                  Whitespace@9..10 "\n"
              VariableRef@10..11
                Ident@10..11 "a""#]],
        );
    }
}
