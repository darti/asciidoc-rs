use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    combinator::map,
    error::ParseError,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::relaxng::NameClass;

use super::{common::trim, identifiers::identifier_or_keyword, Span};

pub(crate) fn cname(input: Span) -> IResult<Span, NameClass> {
    let (input, (n1, n2)) =
        separated_pair(identifier_or_keyword, char(':'), identifier_or_keyword)(input)?;

    Ok((
        input,
        NameClass::CName(n1.to_string(), Some(n2.to_string())),
    ))
}

pub(crate) fn name_class(input: Span) -> IResult<Span, NameClass> {
    let name = map(identifier_or_keyword, |n| NameClass::Name(n.to_string()));

    let parenthesis = delimited(char('('), trim(name_class), char(')'));

    let cname = map(
        separated_pair(identifier_or_keyword, char(':'), identifier_or_keyword),
        |(n1, n2)| NameClass::CName(n1.to_string(), Some(n2.to_string())),
    );

    alt((name, cname, parenthesis))(input)
}

#[cfg(test)]
mod tests {
    use nom::combinator::{all_consuming, complete};

    use crate::relaxng::{
        parser::{
            names::{cname, name_class},
            Span,
        },
        NameClass,
    };

    #[test]
    fn simple_name() {
        let i = Span::new("simple_name_with_1.dot");

        let (_, o) = name_class(i).unwrap();

        assert_eq!(o, NameClass::Name("simple_name_with_1.dot".into()));
    }

    #[test]
    fn simple_name_with_parenthesis() {
        let i = Span::new("( simple_name_with_1.dot\n)  ");

        let (_, o) = name_class(i).unwrap();

        assert_eq!(o, NameClass::Name("simple_name_with_1.dot".into()));
    }

    #[test]
    fn simple_cname() {
        let i = Span::new("n1:simple_name_with_1.dot");

        let (_, o) = cname(i).unwrap();

        assert_eq!(
            o,
            NameClass::CName("n1".into(), Some("simple_name_with_1.dot".into()))
        );
    }
}
