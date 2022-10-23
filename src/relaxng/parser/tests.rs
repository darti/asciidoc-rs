use super::super::NamespaceBuilder;
use super::*;
use indoc::indoc;
use url::Url;

#[test]
fn test_ws_none() {
    let i = Span::new("test");

    let (_, o) = ws(tag::<&str, LocatedSpan<&str>, ()>("test"))(i).unwrap();

    assert_eq!(o.to_string(), "test");
}

#[test]
fn test_ws_left() {
    let i = Span::new(" test");

    let (_, o) = ws(tag::<&str, LocatedSpan<&str>, ()>("test"))(i).unwrap();

    assert_eq!(o.to_string(), "test");
}

#[test]
fn test_ws_right() {
    let i = Span::new("test ");

    let (_, o) = ws(tag::<&str, LocatedSpan<&str>, ()>("test"))(i).unwrap();

    assert_eq!(o.to_string(), "test");
}

#[test]
fn test_ws_both() {
    let i = Span::new(" test ");

    let (_, o) = ws(tag::<&str, LocatedSpan<&str>, ()>("test"))(i).unwrap();

    assert_eq!(o.to_string(), "test");
}

#[test]
fn test_quoted() {
    let i = Span::new("\"test\"");

    let (_, o) = quoted(i).unwrap();

    assert_eq!(o.to_string(), "test");
}

#[test]
#[should_panic]
fn test_quoted_open_left() {
    let i = Span::new("test\"");

    quoted(i).unwrap();
}

#[test]
#[should_panic]
fn test_quoted_open_right() {
    let i = Span::new("\"test");

    quoted(i).unwrap();
}

#[test]
fn test_parse_namespace() {
    let i = Span::new(indoc! {"
            namespace html = \"http://www.w3.org/1999/xhtml\"
        "});

    let (_, (d, o)) = parse_namespace(i).unwrap();

    let r = NamespaceBuilder::default()
        .name("html".into())
        .url(Url::parse("http://www.w3.org/1999/xhtml").unwrap())
        .build()
        .unwrap();

    assert_eq!(o, r);
}

#[test]
fn test_parse_namespaces() {
    let i = Span::new(indoc! {"
            namespace html = \"http://www.w3.org/1999/xhtml\"
            namespace rng = \"http://relaxng.org/ns/structure/1.0\"
        "});

    let (_, o) = parse_namespaces(i).unwrap();

    let r = vec![
        (
            false,
            NamespaceBuilder::default()
                .name("html".into())
                .url(Url::parse("http://www.w3.org/1999/xhtml").unwrap())
                .build()
                .unwrap(),
        ),
        (
            false,
            NamespaceBuilder::default()
                .name("rng".into())
                .url(Url::parse("http://relaxng.org/ns/structure/1.0").unwrap())
                .build()
                .unwrap(),
        ),
    ];

    assert_eq!(o, r);
}

#[test]
fn test_parse_namespaces_comments() {
    let i = Span::new(indoc! {"
            namespace html = \"http://www.w3.org/1999/xhtml\" # comment 1
            namespace rng = \"http://relaxng.org/ns/structure/1.0\"  # comment 2
        "});

    let (_, o) = parse_namespaces(i).unwrap();

    let r = vec![
        (
            false,
            NamespaceBuilder::default()
                .name("html".into())
                .url(Url::parse("http://www.w3.org/1999/xhtml").unwrap())
                .build()
                .unwrap(),
        ),
        (
            false,
            NamespaceBuilder::default()
                .name("rng".into())
                .url(Url::parse("http://relaxng.org/ns/structure/1.0").unwrap())
                .build()
                .unwrap(),
        ),
    ];

    assert_eq!(o, r);
}

#[test]
fn test_parse_namespace_default() {
    let i = Span::new(indoc! {"
            default namespace html = \"http://www.w3.org/1999/xhtml\"
            namespace rng = \"http://relaxng.org/ns/structure/1.0\"
        "});

    let (_, o) = parse_namespaces(i).unwrap();

    let r = vec![
        (
            true,
            NamespaceBuilder::default()
                .name("html".into())
                .url(Url::parse("http://www.w3.org/1999/xhtml").unwrap())
                .build()
                .unwrap(),
        ),
        (
            false,
            NamespaceBuilder::default()
                .name("rng".into())
                .url(Url::parse("http://relaxng.org/ns/structure/1.0").unwrap())
                .build()
                .unwrap(),
        ),
    ];

    assert_eq!(o, r);
}

#[test]
fn test_comment() {
    let i = Span::new(indoc! {"
        # Comment
    "});

    let (_, o) = comment(i).unwrap();

    assert_eq!(o, ())
}

#[test]
fn test_comment_eol() {
    let i = Span::new(indoc! {"
            test # Comment
        "});

    let (_, o) = skip_comments_eol(tag("test"))(i).unwrap();

    assert_eq!(o.to_string(), "test")
}

#[test]
fn test_comment_line() {
    let i = Span::new(indoc! {"
            # Comment 1
            # Comment 2
            test
        "});

    let (_, o) = skip_comment_lines(tag("test"))(i).unwrap();

    assert_eq!(o.to_string(), "test")
}

#[test]
fn test_comment_line_and_empty() {
    let i = Span::new(indoc! {"
            # Comment 1

            # Comment 2


            test
"});

    let (_, o) = skip_comment_lines(tag("test"))(i).unwrap();

    assert_eq!(o.to_string(), "test")
}
