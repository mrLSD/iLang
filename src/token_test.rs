use crate::ast::*;
use crate::tokens::*;

#[test]
fn test_name() {
    assert!(ident(Span::new("test")).is_ok());
    assert!(ident(Span::new("123test")).is_err());
    assert!(ident(Span::new("test123")).is_ok());
    assert!(ident(Span::new("test123test")).is_ok());

    let n = ident(Span::new("test123 test"));
    assert!(n.is_ok());
    let n = n.unwrap();
    assert_eq!(n.1, Ident(Span::new("test123")));
    assert_eq!(n.0.fragment(), &" test");

    let n = ident(Span::new("test_123a(test)"));
    assert!(n.is_ok());
    let n = n.unwrap();
    assert_eq!(n.1.clone(), Ident(Span::new("test_123a")));
    assert_eq!(*n.0.fragment(), "(test)");
}

#[test]
fn test_expression_operations() {
    assert_eq!(
        expression_operations(Span::new("+x")).unwrap().1,
        ExpressionOperation::Plus
    );
    assert_eq!(
        expression_operations(Span::new("-x")).unwrap().1,
        ExpressionOperation::Minus
    );

    assert_eq!(
        expression_operations(Span::new("*x")).unwrap().1,
        ExpressionOperation::Multiply
    );
    assert_eq!(
        expression_operations(Span::new("/x")).unwrap().1,
        ExpressionOperation::Divide
    );

    assert_eq!(
        expression_operations(Span::new("<<<x")).unwrap().1,
        ExpressionOperation::ShiftLeft
    );
    assert_eq!(
        expression_operations(Span::new(">>>x")).unwrap().1,
        ExpressionOperation::ShiftRight
    );
}

#[test]
fn test_parameter_value() {
    assert_eq!(
        parameter_value(Span::new("test")).unwrap().1,
        ParameterValue(Ident(Span::new("test")))
    );

    let n = parameter_value(Span::new("asd123 test")).unwrap();
    let fragment = ((n.1).0).0.fragment();
    assert_eq!(fragment, &"asd123");

    let n = parameter_value(Span::new("(asd123) test")).unwrap();
    let fragment = ((n.1).0).0.fragment();
    assert_eq!(fragment, &"asd123");

    let n = parameter_value(Span::new(" ( asd123 ) test")).unwrap();
    let fragment = ((n.1).0).0.fragment();
    assert_eq!(fragment, &"asd123");

    assert!(parameter_value(Span::new("123test")).is_err());
}

#[test]
fn test_get_ident_from_brackets() {
    let res = get_ident_from_brackets(Span::new("test123 test"));
    assert!(res.is_err());

    let n = get_ident_from_brackets(Span::new("(test123) test"));
    assert!(n.is_ok());
    let n = n.unwrap();
    assert_eq!((n.1).0.fragment(), &"test123");
    // Spaces removed
    assert_eq!(n.0.fragment(), &"test");

    let n = get_ident_from_brackets(Span::new(" ( test123 ) test"));
    assert!(n.is_ok());
    let n = n.unwrap();
    assert_eq!((n.1).0.fragment(), &"test123");
    assert_eq!(n.0.fragment(), &"test");
}

#[test]
fn test_parameter_type() {
    let n = parameter_type(Span::new("asd1 test"));
    assert!(n.is_ok());
    let (i, o) = n.unwrap();
    assert_eq!(o.0[0].0.fragment(), &"asd1");
    assert_eq!(i.fragment(), &"test");
    assert_eq!(o.0.len(), 1);

    let n = parameter_type(Span::new(" ( asd1 ) test"));
    assert!(n.is_ok());
    let (i, o) = n.unwrap();
    assert_eq!(o.0[0].0.fragment(), &"asd1");
    assert_eq!(i.fragment(), &"test");
    assert_eq!(o.0.len(), 1);

    let n = parameter_type(Span::new("* asd1 * asd2 * "));
    assert!(n.is_err());

    let n = parameter_type(Span::new(" ( asd1 ) * asd2 * "));
    assert!(n.is_ok());
    let (i, o) = n.unwrap();
    assert_eq!(o.0[0].0.fragment(), &"asd1");
    assert_eq!(o.0[1].0.fragment(), &"asd2");
    assert_eq!(i.fragment(), &"* ");
    assert_eq!(o.0.len(), 2);

    let n = parameter_type(Span::new(" asd1 * asd2 "));
    assert!(n.is_ok());
    let (_, o) = n.unwrap();
    assert_eq!(o.0[0].0.fragment(), &"asd1");
    assert_eq!(o.0[1].0.fragment(), &"asd2");
    assert_eq!(o.0.len(), 2);

    let n = parameter_type(Span::new(" ( asd1 ) * ( asd2 ) "));
    assert!(n.is_ok());
    let (_, o) = n.unwrap();
    assert_eq!(o.0[0].0.fragment(), &"asd1");
    assert_eq!(o.0[1].0.fragment(), &"asd2");
    assert_eq!(o.0.len(), 2);

    let n = parameter_type(Span::new("asd1 * ( asd2 ) * asd3"));
    assert!(n.is_ok());
    let (_, o) = n.unwrap();
    assert_eq!(o.0[0].0.fragment(), &"asd1");
    assert_eq!(o.0[1].0.fragment(), &"asd2");
    assert_eq!(o.0[2].0.fragment(), &"asd3");
    assert_eq!(o.0.len(), 3);

    let n = parameter_type(Span::new("* asd1 * ( asd2 ) * asd3"));
    assert!(n.is_err());

    let n = parameter_type(Span::new("(asd1 * ( asd2 ) * asd3)"));
    assert!(n.is_ok());
    let (_, o) = n.unwrap();
    assert_eq!(o.0[0].0.fragment(), &"asd1");
    assert_eq!(o.0[1].0.fragment(), &"asd2");
    assert_eq!(o.0[2].0.fragment(), &"asd3");
    assert_eq!(o.0.len(), 3);
}

#[test]
fn test_parameter_value_type() {
    let n = parameter_value_type(Span::new("val1: type1"));
    assert!(n.is_ok());
    let (_, o) = n.unwrap();
    assert_eq!(((o.0).0).0.fragment(), &"val1");
    assert_eq!(((o.1).0[0]).0.fragment(), &"type1");

    let n = parameter_value_type(Span::new(" ( val1 ) : ( type1 ) "));
    assert!(n.is_ok());
    let (_, o) = n.unwrap();
    assert_eq!(((o.0).0).0.fragment(), &"val1");
    assert_eq!(((o.1).0[0]).0.fragment(), &"type1");

    let n = parameter_value_type(Span::new(" ( ( val1 ) : ( type1 ) ) "));
    assert!(n.is_ok());
    let (_, o) = n.unwrap();
    assert_eq!(((o.0).0).0.fragment(), &"val1");
    assert_eq!(((o.1).0[0]).0.fragment(), &"type1");

    let n = parameter_value_type(Span::new("val1: type1 * type2"));
    assert!(n.is_ok());
    let (_, o) = n.unwrap();
    assert_eq!(((o.0).0).0.fragment(), &"val1");
    assert_eq!(((o.1).0[0]).0.fragment(), &"type1");
    assert_eq!(((o.1).0[1]).0.fragment(), &"type2");

    let n = parameter_value_type(Span::new("val1: (type1 * (type2) * type3)"));
    assert!(n.is_ok());
    let (_, o) = n.unwrap();
    assert_eq!(((o.0).0).0.fragment(), &"val1");
    assert_eq!(((o.1).0[0]).0.fragment(), &"type1");
    assert_eq!(((o.1).0[1]).0.fragment(), &"type2");
    assert_eq!(((o.1).0[2]).0.fragment(), &"type3");

    let n = parameter_value_type(Span::new("val1: (type1 * (type2) * type3) test"));
    assert!(n.is_ok());
    let (i, o) = n.unwrap();
    assert_eq!(((o.0).0).0.fragment(), &"val1");
    assert_eq!(((o.1).0[0]).0.fragment(), &"type1");
    assert_eq!(((o.1).0[1]).0.fragment(), &"type2");
    assert_eq!(((o.1).0[2]).0.fragment(), &"type3");
    assert_eq!(i.fragment(), &"test");

    let n = parameter_value_type(Span::new("val1: (type1 * (type2 * type3))"));
    assert!(n.is_err());

    let n = parameter_value_type(Span::new("val1: (type1 * type2"));
    assert!(n.is_err());
}

#[test]
fn test_parameter_list_brackets() {
    let n = parameter_list_brackets(Span::new("val1: type1"));
    assert!(n.is_ok());
}