#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(Identifier),

    Operator(Operator),
    Delimiter(Delimiter),

    Separator,       // ,
    Comment(String), // //
}
impl Token {
    pub fn parse_keydentifier(input: String) -> Self {
        if input == "_" {
            return Self::Identifier(Identifier::Discard);
        }

        input
            .as_str()
            .try_into()
            .map(Self::Keyword)
            .unwrap_or_else(|_| Self::Identifier(Identifier::Named(input)))
    }
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Constant, // given
    Variable, // let
    Function, // fn
}
// TODO: consider a real error here
impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "given" => Ok(Keyword::Constant),
            "let" => Ok(Keyword::Variable),
            "fn" => Ok(Keyword::Function),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Named(String),
    Discard, // _
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Assignment, // =
    Type,       // :
    ReturnType, // ->
}

#[derive(Debug, PartialEq)]
pub enum Delimiter {
    OpenParen,  // (
    CloseParen, // )
    OpenBrace,  // {
    CloseBrace, // }
}
