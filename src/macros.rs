#[macro_export]
macro_rules! ast {
    ($method:ident $(,$args:expr)* $(,)?) => (
        $crate::AST::$method($($args),*)
    );
}

#[cfg(test)]
#[macro_export]
macro_rules! ast_zero_literal {
    () => {
        $crate::ast!(new_literal, $crate::tok!(new_int, 0, $crate::Loc::head()))
    };
}

#[macro_export]
macro_rules! tok {
    ($method:ident $(,$args:expr)* $(,)?) => (
        $crate::Token::$method($($args),*)
    );
}

#[cfg(test)]
#[macro_export]
macro_rules! head_tok {
    ($method:ident $(,$args:expr)* $(,)?) => {
        $crate::Token::$method($($args),*, $crate::Loc::head())
    }
}

#[macro_export]
macro_rules! sym {
    ($sym:ident) => {
        $crate::TokenKind::Symbol($crate::Symbol::$sym)
    };
}

#[macro_export]
macro_rules! keyword {
    ($key:ident) => {
        $crate::TokenKind::Keyword($crate::Keyword::$key)
    };
}
