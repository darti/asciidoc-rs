use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::relaxng::NameClass;

use super::{common::trim, identifiers::identifier_or_keyword, Span};

pub(crate) fn cname(input: Span) -> IResult<Span, NameClass> {
    let (input, (n1, n2)) =
        separated_pair(identifier_or_keyword, char(':'), identifier_or_keyword)(input)?;

    Ok((input, NameClass::CName(n1.to_string(), n2.to_string())))
}

pub(crate) fn nsname(input: Span) -> IResult<Span, NameClass> {
    let (input, n) = identifier_or_keyword(input)?;
    let (input, _) = tag(":*")(input)?;

    Ok((input, NameClass::NsName(n.to_string())))
}

pub(crate) fn anyname(input: Span) -> IResult<Span, NameClass> {
    let (input, _) = char('*')(input)?;

    Ok((input, NameClass::AnyName))
}

pub(crate) fn name_class(input: Span) -> IResult<Span, NameClass> {
    let name = map(identifier_or_keyword, |n| NameClass::Name(n.to_string()));

    let parenthesis = delimited(char('('), trim(name_class), char(')'));

    alt((name, cname, parenthesis))(input)
}

#[cfg(test)]
mod tests {

    use crate::relaxng::{
        parser::{
            names::{anyname, cname, name_class, nsname},
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
            NameClass::CName("n1".into(), "simple_name_with_1.dot".into())
        );
    }

    #[test]
    fn simple_anyname() {
        let i = Span::new("*");

        let (_, o) = anyname(i).unwrap();

        assert_eq!(o, NameClass::AnyName);
    }

    #[test]
    fn simple_nsname() {
        let i = Span::new("n1:*");

        let (_, o) = nsname(i).unwrap();

        assert_eq!(o, NameClass::NsName("n1".into()));
    }
}
