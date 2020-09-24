use crate::ast::*;
use crate::token::*;
use nom::multi::many1;

#[test]
fn test_name() {
    assert!(ident(Span::new("test")).is_ok());
    assert!(ident(Span::new("123test")).is_err());
    assert!(ident(Span::new("test123")).is_ok());
    assert!(ident(Span::new("test123test")).is_ok());

    let n = ident(Span::new("test123 test"));
    assert!(n.is_ok());
    let n = n.unwrap();
    assert_eq!(n.1.fragment(), &"test123");
    assert_eq!(n.0.fragment(), &" test");

    let n = ident(Span::new("test_123a(test)"));
    assert!(n.is_ok());
    let n = n.unwrap();
    assert_eq!(n.1.fragment(), &"test_123a");
    assert_eq!(n.0.fragment(), &"(test)");
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
    let res = parameter_value(Span::new("val1")).unwrap().1;
    assert_eq!(res.fragment(), &"val1");

    let n = parameter_value(Span::new("asd123 test")).unwrap();
    assert_eq!(n.1.fragment(), &"asd123");

    let n = parameter_value(Span::new("(asd123) test")).unwrap();
    assert_eq!(n.1.fragment(), &"asd123");

    let n = parameter_value(Span::new(" ( asd123 ) test")).unwrap();
    let fragment = n.1.fragment();
    assert_eq!(fragment, &"asd123");

    assert!(parameter_value(Span::new("123test")).is_err());
}

#[test]
fn test_get_ident_from_brackets() {
    let res = get_ident_from_brackets(Span::new("test123 test"));
    assert!(res.is_err());

    let n = get_ident_from_brackets(Span::new("(test123) test")).unwrap();
    assert_eq!(n.1.fragment(), &"test123");
    // Spaces removed
    assert_eq!(n.0.fragment(), &"test");

    let n = get_ident_from_brackets(Span::new(" ( test123 ) test")).unwrap();
    assert_eq!(n.1.fragment(), &"test123");
    assert_eq!(n.0.fragment(), &"test");
}

#[test]
fn test_parameter_type_simple() {
    let (i, o) = parameter_type(Span::new("val1 val2")).unwrap();
    assert_eq!(o[0].fragment(), &"val1");
    assert_eq!(i.fragment(), &"val2");
    assert_eq!(o.len(), 1);

    let (_, o) = parameter_type(Span::new(" asd1 * asd2 ")).unwrap();
    assert_eq!(o[0].fragment(), &"asd1");
    assert_eq!(o[1].fragment(), &"asd2");
    assert_eq!(o.len(), 2);
}

#[test]
fn test_parameter_type_first_bracket() {
    let (i, o) = parameter_type(Span::new(" ( val1 ) val2")).unwrap();
    assert_eq!(o[0].fragment(), &"val1");
    assert_eq!(i.fragment(), &"val2");
    assert_eq!(o.len(), 1);
}

#[test]
fn test_parameter_type_failed() {
    let n = parameter_type(Span::new("* asd1 * asd2 * "));
    assert!(n.is_err());

    let n = parameter_type(Span::new("* asd1 * ( asd2 ) * asd3"));
    assert!(n.is_err());
}

#[test]
fn test_parameter_type_partly() {
    let (i, o) = parameter_type(Span::new(" ( asd1 ) * asd2 * ")).unwrap();
    assert_eq!(o[0].fragment(), &"asd1");
    assert_eq!(o[1].fragment(), &"asd2");
    assert_eq!(i.fragment(), &"* ");
    assert_eq!(o.len(), 2);
}

#[test]
fn test_parameter_type_bracketts_compound() {
    let (_, o) = parameter_type(Span::new(" ( asd1 ) * ( asd2 ) ")).unwrap();
    assert_eq!(o[0].fragment(), &"asd1");
    assert_eq!(o[1].fragment(), &"asd2");
    assert_eq!(o.len(), 2);
}

#[test]
fn test_parameter_type_sequence() {
    let (_, o) = parameter_type(Span::new("asd1 * ( asd2 ) * asd3")).unwrap();
    assert_eq!(o[0].fragment(), &"asd1");
    assert_eq!(o[1].fragment(), &"asd2");
    assert_eq!(o[2].fragment(), &"asd3");
    assert_eq!(o.len(), 3);
}

#[test]
fn test_parameter_type_sequence_and_brackets() {
    let (_, o) = parameter_type(Span::new("(asd1 * ( asd2 ) * asd3)")).unwrap();
    assert_eq!(o[0].fragment(), &"asd1");
    assert_eq!(o[1].fragment(), &"asd2");
    assert_eq!(o[2].fragment(), &"asd3");
    assert_eq!(o.len(), 3);
}

#[test]
fn test_parameter_value_type_simple() {
    match parameter_value_type(Span::new("val1: type1")).unwrap() {
        (_, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_value_type_brackets() {
    match parameter_value_type(Span::new(" ( val1 ) : ( type1 ) ")).unwrap() {
        (_, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_value_type_all_brackets() {
    match parameter_value_type(Span::new(" ( ( val1 ) : ( type1 ) ) ")).unwrap() {
        (_, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_value_type_sequnce() {
    match parameter_value_type(Span::new("val1: type1 * type2")).unwrap() {
        (_, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
            assert_eq!(t[1].fragment(), &"type2");
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_value_type_sequnce_compound_brackets() {
    match parameter_value_type(Span::new("val1: (type1 * (type2) * type3)")).unwrap() {
        (_, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
            assert_eq!(t[1].fragment(), &"type2");
            assert_eq!(t[2].fragment(), &"type3");
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_value_type_sequnce_partial() {
    match parameter_value_type(Span::new("val1: (type1 * (type2) * type3) test")).unwrap() {
        (i, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
            assert_eq!(t[1].fragment(), &"type2");
            assert_eq!(t[2].fragment(), &"type3");
            assert_eq!(i.fragment(), &"test");
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_value_type_failed() {
    let n = parameter_value_type(Span::new("val1: (type1 * (type2 * type3))"));
    assert!(n.is_err());

    let n = parameter_value_type(Span::new("val1: (type1 * type2"));
    assert!(n.is_err());
}

#[test]
fn test_parameter_list_brackets() {
    match parameter_list_brackets(Span::new("(val1, val2)")).unwrap() {
        (_, ParameterValueList::ParameterList(x)) => {
            assert_eq!(x.len(), 2);
            match &x[0] {
                ParameterValueType::Value(v) => assert_eq!(v.fragment(), &"val1"),
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::Value(v) => assert_eq!(v.fragment(), &"val2"),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_list_value_type() {
    match parameter_list_brackets(Span::new("(val1: type1)")).unwrap() {
        (_, ParameterValueList::ParameterList(x)) => {
            assert_eq!(x.len(), 1);
            match &x[0] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val1");
                    assert_eq!(t[0].fragment(), &"type1");
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_list_multi_value_type() {
    match parameter_list_brackets(Span::new("(val1: type1, val2: type2)")).unwrap() {
        (_, ParameterValueList::ParameterList(x)) => {
            assert_eq!(x.len(), 2);
            match &x[0] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val1");
                    assert_eq!(t[0].fragment(), &"type1");
                }
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val2");
                    assert_eq!(t[0].fragment(), &"type2");
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_list_multi_value_one_type() {
    match parameter_list_brackets(Span::new("(val1, val2: type2)")).unwrap() {
        (_, ParameterValueList::ParameterList(x)) => {
            assert_eq!(x.len(), 2);
            match &x[0] {
                ParameterValueType::Value(v) => {
                    assert_eq!(v.fragment(), &"val1");
                }
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val2");
                    assert_eq!(t[0].fragment(), &"type2");
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_list_multi_value_one_type_and_brackets() {
    match parameter_list_brackets(Span::new("(val1, val2: type2, (val3: type3))")).unwrap() {
        (_, ParameterValueList::ParameterList(x)) => {
            assert_eq!(x.len(), 3);
            match &x[0] {
                ParameterValueType::Value(v) => {
                    assert_eq!(v.fragment(), &"val1");
                }
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val2");
                    assert_eq!(t[0].fragment(), &"type2");
                }
                _ => unimplemented!(),
            }
            match &x[2] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val3");
                    assert_eq!(t[0].fragment(), &"type3");
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_list_multi_value_one_type_and_sequence() {
    match parameter_list_brackets(Span::new("(val1, val2: type2, val3, val4)")).unwrap() {
        (_, ParameterValueList::ParameterList(x)) => {
            assert_eq!(x.len(), 4);
            match &x[0] {
                ParameterValueType::Value(v) => {
                    assert_eq!(v.fragment(), &"val1");
                }
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val2");
                    assert_eq!(t[0].fragment(), &"type2");
                }
                _ => unimplemented!(),
            }
            match &x[2] {
                ParameterValueType::Value(v) => {
                    assert_eq!(v.fragment(), &"val3");
                }
                _ => unimplemented!(),
            }
            match &x[3] {
                ParameterValueType::Value(v) => {
                    assert_eq!(v.fragment(), &"val4");
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_value_list_one_value() {
    match parameter_value_list(Span::new("val1")).unwrap() {
        (_, ParameterValueList::ParameterValue(v)) => assert_eq!(v.fragment(), &"val1"),
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_value_list_sequence() {
    match parameter_value_list(Span::new("(val1, val2)")).unwrap() {
        (_, ParameterValueList::ParameterList(x)) => {
            assert_eq!(x.len(), 2);
            match &x[0] {
                ParameterValueType::Value(v) => assert_eq!(v.fragment(), &"val1"),
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::Value(v) => assert_eq!(v.fragment(), &"val2"),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_value_list_value_type_sequence() {
    match parameter_list_brackets(Span::new("(val1, val2: type2, val3, (val4: type4))")).unwrap() {
        (_, ParameterValueList::ParameterList(x)) => {
            assert_eq!(x.len(), 4);
            match &x[0] {
                ParameterValueType::Value(v) => {
                    assert_eq!(v.fragment(), &"val1");
                }
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val2");
                    assert_eq!(t[0].fragment(), &"type2");
                }
                _ => unimplemented!(),
            }
            match &x[2] {
                ParameterValueType::Value(v) => {
                    assert_eq!(v.fragment(), &"val3");
                }
                _ => unimplemented!(),
            }
            match &x[3] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val4");
                    assert_eq!(t[0].fragment(), &"type4");
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_list_sequnce() {
    match parameter_list(Span::new("val1 val2")).unwrap() {
        (_, ParameterList::ParameterValueList(x)) => {
            assert_eq!(x.len(), 2);
            match &x[0] {
                ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val1"),
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val2"),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_list_sequnce_and_types() {
    match parameter_list(Span::new("val1 (val2: type2) val3 (val4: type4)")).unwrap() {
        (_, ParameterList::ParameterValueList(x)) => {
            assert_eq!(x.len(), 4);
            match &x[0] {
                ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val1"),
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueList::ParameterList(v) => match &v[0] {
                    ParameterValueType::ValueType(v, t) => {
                        assert_eq!(v.fragment(), &"val2");
                        assert_eq!(t[0].fragment(), &"type2");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
            match &x[2] {
                ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val3"),
                _ => unimplemented!(),
            }
            match &x[3] {
                ParameterValueList::ParameterList(v) => match &v[0] {
                    ParameterValueType::ValueType(v, t) => {
                        assert_eq!(v.fragment(), &"val4");
                        assert_eq!(t[0].fragment(), &"type4");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_parameter_list_sequnce_and_types_brackets() {
    match parameter_list(Span::new("(val1, val2, (val3: type3 * type4)) val4")).unwrap() {
        (_, ParameterList::ParameterValueList(x)) => {
            assert_eq!(x.len(), 2);
            match &x[0] {
                ParameterValueList::ParameterList(v) => {
                    match &v[0] {
                        ParameterValueType::Value(v) => {
                            assert_eq!(v.fragment(), &"val1");
                        }
                        _ => unimplemented!(),
                    }
                    match &v[1] {
                        ParameterValueType::Value(v) => {
                            assert_eq!(v.fragment(), &"val2");
                        }
                        _ => unimplemented!(),
                    }
                    match &v[2] {
                        ParameterValueType::ValueType(v, t) => {
                            assert_eq!(v.fragment(), &"val3");
                            assert_eq!(t[0].fragment(), &"type3");
                            assert_eq!(t[1].fragment(), &"type4");
                        }
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val4"),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_value_list_one() {
    let x = value_list(Span::new("val1")).unwrap().1;
    let x = if let ValueExpression::ParameterValue(v) = &x[0] {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(x.fragment(), &"val1");

    let x = value_list(Span::new("100")).unwrap().1;
    let x = if let ValueExpression::TypeExpression(v) = &x[0] {
        if let BasicTypeExpression::Number(v) = v {
            v
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    };
    assert_eq!(x, &100.);

    let x = value_list(Span::new("\"test\"")).unwrap().1;
    let x = if let ValueExpression::TypeExpression(v) = &x[0] {
        if let BasicTypeExpression::String(v) = v {
            v
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    };
    assert_eq!(x, &String::from("test"));

    let x = value_list(Span::new("true")).unwrap().1;
    let x = if let ValueExpression::TypeExpression(v) = &x[0] {
        if let BasicTypeExpression::Bool(v) = v {
            v
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    };
    assert_eq!(x, &true);
}

#[test]
fn test_value_list_many() {
    let data = Span::new(r#"true 10 "test""#);
    let x = many1(value_list)(data).unwrap().1;
    assert_eq!(x.len(), 3);
    if let ValueExpression::TypeExpression(v) = &x[0][0] {
        if let BasicTypeExpression::Bool(v) = v {
            assert_eq!(v, &true);
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    }
    if let ValueExpression::TypeExpression(v) = &x[1][0] {
        if let BasicTypeExpression::Number(v) = v {
            assert_eq!(v, &10.);
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    }
    if let ValueExpression::TypeExpression(v) = &x[2][0] {
        if let BasicTypeExpression::String(v) = v {
            assert_eq!(v, &String::from("test"));
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    }
}

#[test]
fn test_value_list_sequence() {
    let x = value_list(Span::new("val1, val2")).unwrap();
    assert_eq!(x.0.fragment(), &", val2");
    let x = if let ValueExpression::ParameterValue(v) = &x.1[0] {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(x.fragment(), &"val1");
}

#[test]
fn test_value_list_brackets() {
    let x = value_list(Span::new("(val1)")).unwrap().1;
    let x = if let ValueExpression::ParameterValue(v) = &x[0] {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(x.fragment(), &"val1");

    let x = value_list(Span::new("(100)")).unwrap().1;
    let x = if let ValueExpression::TypeExpression(v) = &x[0] {
        if let BasicTypeExpression::Number(v) = v {
            v
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    };
    assert_eq!(x, &100.);
}

#[test]
fn test_value_list_brackets_sequence() {
    let x = value_list(Span::new(r#"(val1, 100, true, "test")"#))
        .unwrap()
        .1;
    assert_eq!(x.len(), 4);
    if let ValueExpression::ParameterValue(v) = &x[0] {
        assert_eq!(v.fragment(), &"val1");
    } else {
        unimplemented!()
    }

    if let ValueExpression::TypeExpression(v) = &x[1] {
        if let BasicTypeExpression::Number(v) = v {
            assert_eq!(v, &100.);
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    }

    if let ValueExpression::TypeExpression(v) = &x[2] {
        if let BasicTypeExpression::Bool(v) = v {
            assert_eq!(v, &true);
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    }

    if let ValueExpression::TypeExpression(v) = &x[3] {
        if let BasicTypeExpression::String(v) = v {
            assert_eq!(v, &String::from("test"));
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    }
}

#[test]
fn test_value_list_multi_brackets() {
    let x = value_list(Span::new("(val1, (val2))")).unwrap().1;
    let v = if let ValueExpression::ParameterValue(v) = &x[0] {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(v.fragment(), &"val1");

    let v = if let ValueExpression::ParameterValue(v) = &x[1] {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(v.fragment(), &"val2");
}

#[test]
fn test_value_list_multi_brackets_sequence() {
    let x = value_list(Span::new("((val1), (val2))")).unwrap().1;
    let v = if let ValueExpression::ParameterValue(v) = &x[0] {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(v.fragment(), &"val1");

    let v = if let ValueExpression::ParameterValue(v) = &x[1] {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(v.fragment(), &"val2");
}

#[test]
fn test_let_value_list_one() {
    match let_value_list(Span::new("val1")).unwrap().1[0] {
        ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val1"),
        _ => unimplemented!(),
    }
}

#[test]
fn test_let_value_list_sequnce() {
    let res = let_value_list(Span::new("val1, val2")).unwrap().1;
    match res[0] {
        ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val1"),
        _ => unimplemented!(),
    }
    match res[1] {
        ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val2"),
        _ => unimplemented!(),
    }
}

#[test]
fn test_let_value_list_sequnce_brackets() {
    match let_value_list(Span::new("(val1, val2)")).unwrap().1[0] {
        ParameterValueList::ParameterList(ref x) => {
            assert_eq!(x.len(), 2);
            match &x[0] {
                ParameterValueType::Value(v) => assert_eq!(v.fragment(), &"val1"),
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::Value(v) => assert_eq!(v.fragment(), &"val2"),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_let_value_list_sequnce_list() {
    let res = let_value_list(Span::new("(val1, val2), (val3, val4)"))
        .unwrap()
        .1;
    match res[0] {
        ParameterValueList::ParameterList(ref x) => {
            assert_eq!(x.len(), 2);
            match &x[0] {
                ParameterValueType::Value(v) => assert_eq!(v.fragment(), &"val1"),
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::Value(v) => assert_eq!(v.fragment(), &"val2"),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
    match res[1] {
        ParameterValueList::ParameterList(ref x) => {
            assert_eq!(x.len(), 2);
            match &x[0] {
                ParameterValueType::Value(v) => assert_eq!(v.fragment(), &"val3"),
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::Value(v) => assert_eq!(v.fragment(), &"val4"),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_let_value_list_sequnce_list_abd_type() {
    let res = let_value_list(Span::new(
        "(val1, val2: type2, val3, (val4: type4)), (val5: type5)",
    ))
    .unwrap()
    .1;
    match res[0] {
        ParameterValueList::ParameterList(ref x) => {
            assert_eq!(x.len(), 4);
            match &x[0] {
                ParameterValueType::Value(v) => {
                    assert_eq!(v.fragment(), &"val1");
                }
                _ => unimplemented!(),
            }
            match &x[1] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val2");
                    assert_eq!(t[0].fragment(), &"type2");
                }
                _ => unimplemented!(),
            }
            match &x[2] {
                ParameterValueType::Value(v) => {
                    assert_eq!(v.fragment(), &"val3");
                }
                _ => unimplemented!(),
            }
            match &x[3] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val4");
                    assert_eq!(t[0].fragment(), &"type4");
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
    match res[1] {
        ParameterValueList::ParameterList(ref x) => {
            assert_eq!(x.len(), 1);
            match &x[0] {
                ParameterValueType::ValueType(v, t) => {
                    assert_eq!(v.fragment(), &"val5");
                    assert_eq!(t[0].fragment(), &"type5");
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_module() {
    let res = module(Span::new("test"));
    assert!(res.is_err());

    let res = module(Span::new("module test1")).unwrap().1;
    assert_eq!(res.module_name.len(), 1);
    assert_eq!(res.module_name[0].fragment(), &"test1");
    assert_eq!(res.accessibility, None);
}

#[test]
fn test_module_sequence_two() {
    let res = module(Span::new("module test1.test2")).unwrap().1;
    assert_eq!(res.module_name.len(), 2);
    assert_eq!(res.module_name[0].fragment(), &"test1");
    assert_eq!(res.module_name[1].fragment(), &"test2");
    assert_eq!(res.accessibility, None);
}

#[test]
fn test_module_sequence_public() {
    let res = module(Span::new("module public test1.test2.test3"))
        .unwrap()
        .1;
    assert_eq!(res.module_name.len(), 3);
    assert_eq!(res.module_name[0].fragment(), &"test1");
    assert_eq!(res.module_name[1].fragment(), &"test2");
    assert_eq!(res.module_name[2].fragment(), &"test3");
    assert_eq!(res.accessibility.unwrap().fragment(), &"public");
}

#[test]
fn test_module_sequence_fail() {
    let res = module(Span::new("module test1 .test2")).unwrap();
    assert_eq!(res.1.module_name.len(), 1);
    assert_eq!(res.0.fragment(), &" .test2");

    let res = module(Span::new("module test1. test2")).unwrap();
    assert_eq!(res.1.module_name.len(), 1);
    assert_eq!(res.0.fragment(), &". test2");

    // Space delimiter before module
    let res = module(Span::new(" module test1"));
    assert!(res.is_err());

    let res = module(Span::new("module"));
    assert!(res.is_err());
}

#[test]
fn test_namespace() {
    let res = namespace(Span::new("namespace test1")).unwrap().1;
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].fragment(), &"test1");
}

#[test]
fn test_namespace_sequence_two() {
    let res = namespace(Span::new("namespace test1.test2")).unwrap().1;
    assert_eq!(res.len(), 2);
    assert_eq!(res[0].fragment(), &"test1");
    assert_eq!(res[1].fragment(), &"test2");
}

#[test]
fn test_namespace_sequence_three() {
    let res = namespace(Span::new("namespace test1.test2.test3"))
        .unwrap()
        .1;
    assert_eq!(res.len(), 3);
    assert_eq!(res[0].fragment(), &"test1");
    assert_eq!(res[1].fragment(), &"test2");
    assert_eq!(res[2].fragment(), &"test3");
}

#[test]
fn test_namespace_fail() {
    let res = module(Span::new("test"));
    assert!(res.is_err());

    let res = namespace(Span::new("namespace test1 .test2")).unwrap();
    assert_eq!(res.1.len(), 1);
    assert_eq!(res.0.fragment(), &" .test2");

    let res = namespace(Span::new("namespace test1. test2")).unwrap();
    assert_eq!(res.1.len(), 1);
    assert_eq!(res.0.fragment(), &". test2");

    // Space delimiter before module
    let res = namespace(Span::new(" namespace test1"));
    assert!(res.is_err());

    let res = namespace(Span::new("namespace"));
    assert!(res.is_err());
}

#[test]
fn test_function_value_brackets() {
    match function_value(Span::new("(val1, (val2))")).unwrap().1 {
        FunctionValue::ValueList(x) => {
            let v = if let ValueExpression::ParameterValue(v) = &x[0] {
                v
            } else {
                unimplemented!()
            };
            assert_eq!(v.fragment(), &"val1");

            let v = if let ValueExpression::ParameterValue(v) = &x[1] {
                v
            } else {
                unimplemented!()
            };
            assert_eq!(v.fragment(), &"val2");
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_value_with_expressions() {
    let x = function_value(Span::new("((val1, (val2)) + func1 val3)")).unwrap();
    assert_eq!(x.0.fragment(), &"");
    let x = x.1;
    match x {
        FunctionValue::Expression(v) => {
            match v.function_statement {
                ExpressionFunctionValueCall::FunctionValue(ref x) => match x {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 2);
                        if let ValueExpression::ParameterValue(v) = &v[0] {
                            assert_eq!(v.fragment(), &"val1");
                        } else {
                            unimplemented!()
                        }
                        if let ValueExpression::ParameterValue(v) = &v[1] {
                            assert_eq!(v.fragment(), &"val2");
                        } else {
                            unimplemented!()
                        }
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
            let x = v.operation_statement.as_ref();
            assert_eq!(x.unwrap(), &ExpressionOperation::Plus);
            match v.expression.unwrap().function_statement {
                ExpressionFunctionValueCall::FunctionCall(x) => {
                    assert_eq!(x.function_call_name[0].fragment(), &"func1");
                    assert_eq!(x.function_value.len(), 1);
                    match &x.function_value[0] {
                        FunctionValue::ValueList(v) => {
                            assert_eq!(v.len(), 1);
                            if let ValueExpression::ParameterValue(v) = &v[0] {
                                assert_eq!(v.fragment(), &"val3");
                            } else {
                                unimplemented!()
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_call_name() {
    let x = function_call_name(Span::new("func1")).unwrap().1;
    assert_eq!(x.len(), 1);
    assert_eq!(x[0].fragment(), &"func1");
}

#[test]
fn test_function_call_name_sequence_and_value() {
    let x = function_call_name(Span::new("func1.func2 val1")).unwrap().1;
    assert_eq!(x.len(), 2);
    assert_eq!(x[0].fragment(), &"func1");
    assert_eq!(x[1].fragment(), &"func2");

    let x = function_call_name(Span::new("func1.func2 val1")).unwrap();
    assert_eq!(x.1.len(), 2);
    assert_eq!(x.1[0].fragment(), &"func1");
    assert_eq!(x.1[1].fragment(), &"func2");
    assert_eq!(x.0.fragment(), &" val1");

    let x = function_call_name(Span::new("func1.func2.func3"))
        .unwrap()
        .1;
    assert_eq!(x.len(), 3);
    assert_eq!(x[0].fragment(), &"func1");
    assert_eq!(x[1].fragment(), &"func2");
    assert_eq!(x[2].fragment(), &"func3");
}

#[test]
fn test_function_call_func_val() {
    let x = function_call(Span::new("func1 val1"));
    let x = x.unwrap().1;
    assert_eq!(x.function_call_name.len(), 1);
    assert_eq!(x.function_value.len(), 1);
    assert_eq!(x.function_call_name[0].fragment(), &"func1");
    match &x.function_value[0] {
        FunctionValue::ValueList(v) => {
            assert_eq!(v.len(), 1);
            if let ValueExpression::ParameterValue(v) = &v[0] {
                assert_eq!(v.fragment(), &"val1");
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }

    let x = function_call(Span::new("func1 val1"));
    let x = x.unwrap().1;
    assert_eq!(x.function_call_name.len(), 1);
    assert_eq!(x.function_value.len(), 1);
    assert_eq!(x.function_call_name[0].fragment(), &"func1");
    match &x.function_value[0] {
        FunctionValue::ValueList(v) => {
            assert_eq!(v.len(), 1);
            if let ValueExpression::ParameterValue(v) = &v[0] {
                assert_eq!(v.fragment(), &"val1");
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_call_func_val_sequence() {
    let x = function_call(Span::new(r#"func1.func2 val1 10 true "test1""#))
        .unwrap()
        .1;
    assert_eq!(x.function_call_name.len(), 2);
    assert_eq!(x.function_value.len(), 4);
    assert_eq!(x.function_call_name[0].fragment(), &"func1");
    assert_eq!(x.function_call_name[1].fragment(), &"func2");
    match &x.function_value[0] {
        FunctionValue::ValueList(v) => {
            assert_eq!(v.len(), 1);
            if let ValueExpression::ParameterValue(v) = &v[0] {
                assert_eq!(v.fragment(), &"val1");
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
    match &x.function_value[1] {
        FunctionValue::ValueList(v) => {
            assert_eq!(v.len(), 1);
            if let ValueExpression::TypeExpression(x) = &v[0] {
                if let BasicTypeExpression::Number(n) = x {
                    assert_eq!(n, &10.);
                } else {
                    unimplemented!()
                }
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
    match &x.function_value[2] {
        FunctionValue::ValueList(v) => {
            assert_eq!(v.len(), 1);
            if let ValueExpression::TypeExpression(x) = &v[0] {
                if let BasicTypeExpression::Bool(b) = x {
                    assert_eq!(b, &true);
                } else {
                    unimplemented!()
                }
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
    match &x.function_value[3] {
        FunctionValue::ValueList(v) => {
            assert_eq!(v.len(), 1);
            if let ValueExpression::TypeExpression(x) = &v[0] {
                if let BasicTypeExpression::String(s) = x {
                    assert_eq!(s, &String::from("test1"));
                } else {
                    unimplemented!()
                }
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_call_func_val_sequence_brackets() {
    let x = function_call(Span::new(r#"func1.func2 (val1) ("str")"#))
        .unwrap()
        .1;
    assert_eq!(x.function_call_name.len(), 2);
    assert_eq!(x.function_value.len(), 2);
    assert_eq!(x.function_call_name[0].fragment(), &"func1");
    assert_eq!(x.function_call_name[1].fragment(), &"func2");
    match &x.function_value[0] {
        FunctionValue::ValueList(v) => {
            assert_eq!(v.len(), 1);
            if let ValueExpression::ParameterValue(v) = v[0] {
                assert_eq!(v.fragment(), &"val1");
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
    match &x.function_value[1] {
        FunctionValue::ValueList(v) => {
            assert_eq!(v.len(), 1);
            if let ValueExpression::TypeExpression(v) = &v[0] {
                if let BasicTypeExpression::String(s) = v {
                    assert_eq!(s, &String::from("str"));
                } else {
                    unimplemented!()
                }
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_call_func_val_multi_sequence() {
    let x = function_call(Span::new("func1.func2 (val1, val2)"))
        .unwrap()
        .1;
    assert_eq!(x.function_call_name.len(), 2);
    assert_eq!(x.function_value.len(), 1);
    assert_eq!(x.function_call_name[0].fragment(), &"func1");
    assert_eq!(x.function_call_name[1].fragment(), &"func2");
    match &x.function_value[0] {
        FunctionValue::ValueList(v) => {
            assert_eq!(v.len(), 2);
            if let ValueExpression::ParameterValue(x) = &v[0] {
                assert_eq!(x.fragment(), &"val1");
            } else {
                unimplemented!()
            }
            if let ValueExpression::ParameterValue(x) = &v[1] {
                assert_eq!(x.fragment(), &"val2");
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_call_func_val_partial() {
    let x = function_call(Span::new("func1 val1, val2")).unwrap();
    assert_eq!(x.1.function_call_name.len(), 1);
    assert_eq!(x.1.function_value.len(), 1);
    assert_eq!(x.0.fragment(), &", val2");
}

#[test]
fn test_function_call_func_val_multi_brackets() {
    let x = function_call(Span::new("func1.func2 ((val1), (val2))"))
        .unwrap()
        .1;
    assert_eq!(x.function_call_name.len(), 2);
    assert_eq!(x.function_value.len(), 1);
    assert_eq!(x.function_call_name[0].fragment(), &"func1");
    assert_eq!(x.function_call_name[1].fragment(), &"func2");
    match &x.function_value[0] {
        FunctionValue::ValueList(v) => {
            assert_eq!(v.len(), 2);
            if let ValueExpression::ParameterValue(x) = &v[0] {
                assert_eq!(x.fragment(), &"val1");
            } else {
                unimplemented!()
            }
            if let ValueExpression::ParameterValue(x) = &v[1] {
                assert_eq!(x.fragment(), &"val2");
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_call_empty_brackets() {
    let x = function_call(Span::new("func1 ()")).unwrap().1;
    assert_eq!(x.function_call_name.len(), 1);
    assert_eq!(x.function_value.len(), 0);
    assert_eq!(x.function_call_name[0].fragment(), &"func1");
}

#[test]
fn test_function_call_func_val_in_func() {
    let x = function_call(Span::new("func1 (func2 val2)")).unwrap().1;
    assert_eq!(x.function_call_name.len(), 1);
    assert_eq!(x.function_value.len(), 1);
    assert_eq!(x.function_call_name[0].fragment(), &"func1");
    match &x.function_value[0] {
        FunctionValue::Expression(v) => match &v.function_statement {
            ExpressionFunctionValueCall::FunctionCall(v) => {
                assert_eq!(v.function_call_name[0].fragment(), &"func2");
                match &v.function_value[0] {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        if let ValueExpression::ParameterValue(x) = &v[0] {
                            assert_eq!(x.fragment(), &"val2");
                        } else {
                            unimplemented!()
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_func_empty_param() {
    let x = expression(Span::new("func1 ()")).unwrap().1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionCall(x) => {
            assert_eq!(x.function_call_name.len(), 1);
            assert_eq!(x.function_value.len(), 0);
            assert_eq!(x.function_call_name[0].fragment(), &"func1");
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_func_multi_val_params() {
    let x = expression(Span::new("func1 (val1, val2)")).unwrap().1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionCall(x) => {
            assert_eq!(x.function_call_name.len(), 1);
            assert_eq!(x.function_value.len(), 1);
            assert_eq!(x.function_call_name[0].fragment(), &"func1");
            match &x.function_value[0] {
                FunctionValue::ValueList(v) => {
                    assert_eq!(v.len(), 2);
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val1");
                    } else {
                        unimplemented!()
                    }
                    if let ValueExpression::ParameterValue(x) = &v[1] {
                        assert_eq!(x.fragment(), &"val2");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_multi_func_sequence_params() {
    let x = expression(Span::new("func1.func2 val1 val2")).unwrap().1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionCall(x) => {
            assert_eq!(x.function_call_name.len(), 2);
            assert_eq!(x.function_value.len(), 2);
            assert_eq!(x.function_call_name[0].fragment(), &"func1");
            assert_eq!(x.function_call_name[1].fragment(), &"func2");
            match &x.function_value[0] {
                FunctionValue::ValueList(v) => {
                    assert_eq!(v.len(), 1);
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val1");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
            match &x.function_value[1] {
                FunctionValue::ValueList(v) => {
                    assert_eq!(v.len(), 1);
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val2");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_multi_func_sequence_params_multi_params() {
    let x = expression(Span::new("func1 val1 (val2, val3)")).unwrap().1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionCall(x) => {
            assert_eq!(x.function_call_name.len(), 1);
            assert_eq!(x.function_value.len(), 2);
            assert_eq!(x.function_call_name[0].fragment(), &"func1");
            match &x.function_value[0] {
                FunctionValue::ValueList(v) => {
                    assert_eq!(v.len(), 1);
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val1");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
            match &x.function_value[1] {
                FunctionValue::ValueList(v) => {
                    assert_eq!(v.len(), 2);
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val2");
                    } else {
                        unimplemented!()
                    }
                    if let ValueExpression::ParameterValue(x) = &v[1] {
                        assert_eq!(x.fragment(), &"val3");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_multi_func_sequence_params_brackets() {
    let x = expression(Span::new("(func1 (val1, val2))")).unwrap().1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionCall(x) => {
            assert_eq!(x.function_call_name.len(), 1);
            assert_eq!(x.function_value.len(), 1);
            assert_eq!(x.function_call_name[0].fragment(), &"func1");
            match &x.function_value[0] {
                FunctionValue::ValueList(v) => {
                    assert_eq!(v.len(), 2);
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val1");
                    } else {
                        unimplemented!()
                    }
                    if let ValueExpression::ParameterValue(x) = &v[1] {
                        assert_eq!(x.fragment(), &"val2");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_func_params_in_brackets() {
    let x = expression(Span::new("(func1 val1 val2)")).unwrap().1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionCall(x) => {
            assert_eq!(x.function_call_name.len(), 1);
            assert_eq!(x.function_value.len(), 2);
            assert_eq!(x.function_call_name[0].fragment(), &"func1");
            match &x.function_value[0] {
                FunctionValue::ValueList(v) => {
                    assert_eq!(v.len(), 1);
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val1");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
            match &x.function_value[1] {
                FunctionValue::ValueList(v) => {
                    assert_eq!(v.len(), 1);
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val2");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_values_plus() {
    let x = expression(Span::new("val1 + val2")).unwrap().1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionValue(x) => match x {
            FunctionValue::ValueList(v) => {
                assert_eq!(v.len(), 1);
                if let ValueExpression::ParameterValue(x) = &v[0] {
                    assert_eq!(x.fragment(), &"val1");
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    assert_eq!(x.operation_statement.unwrap(), ExpressionOperation::Plus);
    match x.expression.unwrap().function_statement {
        ExpressionFunctionValueCall::FunctionValue(v) => match v {
            FunctionValue::ValueList(v) => {
                if let ValueExpression::ParameterValue(x) = &v[0] {
                    assert_eq!(x.fragment(), &"val2");
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_func_params_plus_value() {
    let x = expression(Span::new("(func1 val1) + val2")).unwrap().1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionCall(x) => {
            assert_eq!(x.function_call_name[0].fragment(), &"func1");
            assert_eq!(x.function_value.len(), 1);
            match &x.function_value[0] {
                FunctionValue::ValueList(v) => {
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val1");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
    assert_eq!(&x.operation_statement.unwrap(), &ExpressionOperation::Plus);
    match &x.expression.unwrap().function_statement {
        ExpressionFunctionValueCall::FunctionValue(v) => match v {
            FunctionValue::ValueList(v) => {
                if let ValueExpression::ParameterValue(x) = &v[0] {
                    assert_eq!(x.fragment(), &"val2");
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_func_params_plus_func_params() {
    let x = expression(Span::new("(func1 val1) + (func2 val2)"))
        .unwrap()
        .1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionCall(x) => {
            assert_eq!(x.function_call_name[0].fragment(), &"func1");
            assert_eq!(x.function_value.len(), 1);
            match &x.function_value[0] {
                FunctionValue::ValueList(v) => {
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val1");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
    assert_eq!(&x.operation_statement.unwrap(), &ExpressionOperation::Plus);
    match &x.expression.unwrap().function_statement {
        ExpressionFunctionValueCall::FunctionCall(v) => {
            assert_eq!(v.function_call_name[0].fragment(), &"func2");
            assert_eq!(v.function_value.len(), 1);
            match &v.function_value[0] {
                FunctionValue::ValueList(v) => {
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val2");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_brackets_func_params_plus_func_params() {
    let x = expression(Span::new("((func1 val1) + (func2 val2))"))
        .unwrap()
        .1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionValue(x) => match &x {
            FunctionValue::Expression(x) => {
                match x.function_statement {
                    ExpressionFunctionValueCall::FunctionCall(ref x) => {
                        assert_eq!(x.function_call_name[0].fragment(), &"func1");
                        assert_eq!(x.function_value.len(), 1);
                        match &x.function_value[0] {
                            FunctionValue::ValueList(v) => {
                                if let ValueExpression::ParameterValue(x) = &v[0] {
                                    assert_eq!(x.fragment(), &"val1");
                                } else {
                                    unimplemented!()
                                }
                            }
                            _ => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                }
                let v = x.operation_statement.as_ref();
                assert_eq!(v.unwrap(), &ExpressionOperation::Plus);
                let e = x.expression.as_ref();
                match e.unwrap().function_statement {
                    ExpressionFunctionValueCall::FunctionCall(ref v) => {
                        assert_eq!(v.function_call_name[0].fragment(), &"func2");
                        assert_eq!(v.function_value.len(), 1);
                        match &v.function_value[0] {
                            FunctionValue::ValueList(v) => {
                                assert_eq!(v.len(), 1);
                                if let ValueExpression::ParameterValue(x) = &v[0] {
                                    assert_eq!(x.fragment(), &"val2");
                                } else {
                                    unimplemented!()
                                }
                            }
                            _ => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_value_plus_func_params_plus_value() {
    let x = expression(Span::new("val1 + (func1 val2) + val3"))
        .unwrap()
        .1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionValue(x) => match x {
            FunctionValue::ValueList(v) => {
                assert_eq!(v.len(), 1);
                if let ValueExpression::ParameterValue(x) = &v[0] {
                    assert_eq!(x.fragment(), &"val1");
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    assert_eq!(x.operation_statement.unwrap(), ExpressionOperation::Plus);
    let v = x.expression.unwrap();
    match &v.function_statement {
        ExpressionFunctionValueCall::FunctionCall(v) => {
            assert_eq!(v.function_call_name[0].fragment(), &"func1");
            assert_eq!(v.function_value.len(), 1);
            match &v.function_value[0] {
                FunctionValue::ValueList(v) => {
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val2");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
    assert_eq!(v.operation_statement.unwrap(), ExpressionOperation::Plus);
    match &v.expression.unwrap().function_statement {
        ExpressionFunctionValueCall::FunctionValue(v) => match &v {
            FunctionValue::ValueList(v) => {
                assert_eq!(v.len(), 1);
                if let ValueExpression::ParameterValue(x) = &v[0] {
                    assert_eq!(x.fragment(), &"val3");
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_value_plus_func_params_partly_parsed() {
    // Parsed partly
    let x = expression(Span::new("val1 + (func1 val2) val3")).unwrap();
    assert_eq!(x.0.fragment(), &"val3");
    let x = x.1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionValue(x) => match x {
            FunctionValue::ValueList(v) => {
                assert_eq!(v.len(), 1);
                if let ValueExpression::ParameterValue(x) = &v[0] {
                    assert_eq!(x.fragment(), &"val1");
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    assert_eq!(x.operation_statement.unwrap(), ExpressionOperation::Plus);
    let v = x.expression.unwrap();
    match &v.function_statement {
        ExpressionFunctionValueCall::FunctionCall(v) => {
            assert_eq!(v.function_call_name[0].fragment(), &"func1");
            assert_eq!(v.function_value.len(), 1);
            match &v.function_value[0] {
                FunctionValue::ValueList(v) => {
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val2");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_expression_complex_statement() {
    let x = expression(Span::new(
        "(func1 val1) + ((func2 (val2, val3)) + val4 + func5 val5 val6 + func6 ())",
    ))
    .unwrap();
    assert_eq!(x.0.fragment(), &"");
    let x = x.1;
    match x.function_statement {
        ExpressionFunctionValueCall::FunctionCall(v) => {
            assert_eq!(v.function_call_name[0].fragment(), &"func1");
            assert_eq!(v.function_value.len(), 1);
            match &v.function_value[0] {
                FunctionValue::ValueList(v) => {
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val1");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
    assert_eq!(x.operation_statement.unwrap(), ExpressionOperation::Plus);
    match x.expression.unwrap().function_statement {
        ExpressionFunctionValueCall::FunctionValue(ref x) => match &x {
            FunctionValue::Expression(x) => {
                match x.function_statement {
                    ExpressionFunctionValueCall::FunctionCall(ref x) => {
                        assert_eq!(x.function_call_name[0].fragment(), &"func2");
                        assert_eq!(x.function_value.len(), 1);
                        match &x.function_value[0] {
                            FunctionValue::ValueList(v) => {
                                assert_eq!(v.len(), 2);
                                if let ValueExpression::ParameterValue(x) = &v[0] {
                                    assert_eq!(x.fragment(), &"val2");
                                } else {
                                    unimplemented!()
                                }
                                if let ValueExpression::ParameterValue(x) = &v[1] {
                                    assert_eq!(x.fragment(), &"val3");
                                } else {
                                    unimplemented!()
                                }
                            }
                            _ => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                }
                let v = x.operation_statement.as_ref();
                assert_eq!(v.unwrap(), &ExpressionOperation::Plus);

                let e = x.expression.as_ref().unwrap();
                match e.function_statement {
                    ExpressionFunctionValueCall::FunctionValue(ref x) => match x {
                        FunctionValue::ValueList(v) => {
                            assert_eq!(v.len(), 1);
                            if let ValueExpression::ParameterValue(x) = &v[0] {
                                assert_eq!(x.fragment(), &"val4");
                            } else {
                                unimplemented!()
                            }
                        }
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                }
                assert_eq!(v.unwrap(), &ExpressionOperation::Plus);

                let x = e.expression.as_ref().unwrap();
                match x.function_statement {
                    ExpressionFunctionValueCall::FunctionCall(ref x) => {
                        assert_eq!(x.function_call_name[0].fragment(), &"func5");
                        assert_eq!(x.function_value.len(), 2);
                        match &x.function_value[0] {
                            FunctionValue::ValueList(v) => {
                                assert_eq!(v.len(), 1);
                                if let ValueExpression::ParameterValue(x) = &v[0] {
                                    assert_eq!(x.fragment(), &"val5");
                                } else {
                                    unimplemented!()
                                }
                            }
                            _ => unimplemented!(),
                        }
                        match &x.function_value[1] {
                            FunctionValue::ValueList(v) => {
                                assert_eq!(v.len(), 1);
                                if let ValueExpression::ParameterValue(x) = &v[0] {
                                    assert_eq!(x.fragment(), &"val6");
                                } else {
                                    unimplemented!()
                                }
                            }
                            _ => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                }
                let v = &e.operation_statement.as_ref();
                assert_eq!(v.unwrap(), &ExpressionOperation::Plus);
                let x = x.expression.as_ref().unwrap();
                match x.function_statement {
                    ExpressionFunctionValueCall::FunctionCall(ref x) => {
                        assert_eq!(x.function_call_name[0].fragment(), &"func6");
                        assert_eq!(x.function_value.len(), 0);
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_body_statement_func_values() {
    let x = function_body_statement(Span::new("func1 val1 val2")).unwrap();
    assert_eq!(x.0.fragment(), &"");
    match x.1 {
        FunctionBodyStatement::FunctionCall(v) => {
            assert_eq!(v.function_call_name[0].fragment(), &"func1");
            assert_eq!(v.function_value.len(), 2);
            match &v.function_value[0] {
                FunctionValue::ValueList(v) => {
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val1");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
            match &v.function_value[1] {
                FunctionValue::ValueList(v) => {
                    if let ValueExpression::ParameterValue(x) = &v[0] {
                        assert_eq!(x.fragment(), &"val2");
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_body_values_sum() {
    let x = function_body_statement(Span::new("(val1 + val2)")).unwrap();
    assert_eq!(x.0.fragment(), &"");
    match x.1 {
        FunctionBodyStatement::Expression(e) => match e.function_statement {
            ExpressionFunctionValueCall::FunctionValue(v) => match v {
                FunctionValue::Expression(x) => {
                    match x.function_statement {
                        ExpressionFunctionValueCall::FunctionValue(v) => match v {
                            FunctionValue::ValueList(ref v) => {
                                if let ValueExpression::ParameterValue(x) = &v[0] {
                                    assert_eq!(x.fragment(), &"val1");
                                } else {
                                    unimplemented!()
                                }
                            }
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    }
                    assert_eq!(
                        x.operation_statement.as_ref().unwrap(),
                        &ExpressionOperation::Plus
                    );
                    match x.expression.unwrap().function_statement {
                        ExpressionFunctionValueCall::FunctionValue(ref v) => match v {
                            FunctionValue::ValueList(ref v) => {
                                if let ValueExpression::ParameterValue(x) = &v[0] {
                                    assert_eq!(x.fragment(), &"val2");
                                } else {
                                    unimplemented!()
                                }
                            }
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
/*
#[test]
fn test_function_body_let_binding_simple() {
    let x = function_body_statement(Span::new("let val1 = val2")).unwrap();
    assert_eq!(x.0.fragment(), &"");
    match x.1 {
        FunctionBodyStatement::LetBinding(x) => {
            assert_eq!(x.value_list.len(), 1);
            match x.value_list[0] {
                ParameterValueList::ParameterValue(v) => {
                    assert_eq!(v.fragment(), &"val1");
                }
                _ => unimplemented!(),
            }
            assert_eq!(x.function_body.len(), 1);
            match &x.function_body[0] {
                FunctionBodyStatement::Expression(e) => match &e.function_statement {
                    ExpressionFunctionValueCall::FunctionValue(v) => match v {
                        FunctionValue::ValueList(v) => {
                            assert_eq!(v.len(), 1);
                            assert_eq!(v[0].fragment(), &"val2");
                        }
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_body() {
    let x = function_body(Span::new("(func1 val1) (val2, val3)")).unwrap();
    assert_eq!(x.0.fragment(), &"");
    let x = x.1;
    assert_eq!(x.len(), 2);
    match &x[0] {
        FunctionBodyStatement::Expression(e) => match &e.function_statement {
            ExpressionFunctionValueCall::FunctionCall(v) => {
                assert_eq!(v.function_call_name[0].fragment(), &"func1");
                assert_eq!(v.function_value.len(), 1);
                match &v.function_value[0] {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val1");
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }

    match &x[1] {
        FunctionBodyStatement::Expression(e) => match &e.function_statement {
            ExpressionFunctionValueCall::FunctionValue(v) => match v {
                FunctionValue::ValueList(x) => {
                    assert_eq!(x.len(), 2);
                    assert_eq!(x[0].fragment(), &"val2");
                    assert_eq!(x[1].fragment(), &"val3");
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[test]
fn test_let_binding_simple() {
    let x = let_binding(Span::new("let x = y")).unwrap();
    assert_eq!(x.0.fragment(), &"");
    let x = x.1;
    assert_eq!(x.value_list.len(), 1);
    match x.value_list[0] {
        ParameterValueList::ParameterValue(v) => {
            assert_eq!(v.fragment(), &"x");
        }
        _ => unimplemented!(),
    }
    assert_eq!(x.function_body.len(), 1);
    match &x.function_body[0] {
        FunctionBodyStatement::Expression(e) => match &e.function_statement {
            ExpressionFunctionValueCall::FunctionValue(v) => match v {
                FunctionValue::ValueList(v) => {
                    assert_eq!(v.len(), 1);
                    assert_eq!(v[0].fragment(), &"y");
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[test]
fn test_let_binding_value_plus_value() {
    let x = let_binding(Span::new("let val1 = val2 + val3")).unwrap();
    assert_eq!(x.0.fragment(), &"");
    let x = x.1;
    assert_eq!(x.value_list.len(), 1);
    match x.value_list[0] {
        ParameterValueList::ParameterValue(v) => {
            assert_eq!(v.fragment(), &"val1");
        }
        _ => unimplemented!(),
    }
    assert_eq!(x.function_body.len(), 1);
    match &x.function_body[0] {
        FunctionBodyStatement::Expression(e) => {
            match &e.function_statement {
                ExpressionFunctionValueCall::FunctionValue(v) => match v {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val2");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
            assert_eq!(
                e.operation_statement.as_ref().unwrap(),
                &ExpressionOperation::Plus
            );
            match e.expression.as_ref().unwrap().function_statement {
                ExpressionFunctionValueCall::FunctionValue(ref v) => match v {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val3");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_simple() {
    let x = function(Span::new("let func1 val1 = val2")).unwrap();
    assert_eq!(x.0.fragment(), &"");
    let x = x.1;
    assert_eq!(x.function_name.fragment(), &"func1");
    match x.parameter_list {
        ParameterList::ParameterValueList(ref v) => {
            assert_eq!(v.len(), 1);
            match v[0] {
                ParameterValueList::ParameterValue(x) => {
                    assert_eq!(x.fragment(), &"val1");
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
    assert_eq!(x.function_body.len(), 1);
    match &x.function_body[0] {
        FunctionBodyStatement::Expression(x) => match &x.function_statement {
            ExpressionFunctionValueCall::FunctionValue(v) => match v {
                FunctionValue::ValueList(x) => {
                    assert_eq!(x.len(), 1);
                    assert_eq!(x[0].fragment(), &"val2");
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[test]
fn test_function_params_and_simple_expression() {
    let x = function(Span::new(
        "let inline func1 val1 (val2: type1) : return_type = val3 + val4",
    ))
    .unwrap();
    assert_eq!(x.0.fragment(), &"");
    let x = x.1;
    assert_eq!(&x.modifier.unwrap(), &FunctionModifier::Inline);
    assert_eq!(x.function_name.fragment(), &"func1");

    let v = x.return_type.as_ref().unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].fragment(), &"return_type");

    let p = &x.parameter_list;
    match p {
        ParameterList::ParameterValueList(ref v) => {
            assert_eq!(v.len(), 2);
            match &v[0] {
                ParameterValueList::ParameterValue(x) => {
                    assert_eq!(x.fragment(), &"val1");
                }
                _ => unimplemented!(),
            }
            match &v[1] {
                ParameterValueList::ParameterList(x) => {
                    assert_eq!(x.len(), 1);
                    match &x[0] {
                        ParameterValueType::ValueType(v, t) => {
                            assert_eq!(v.fragment(), &"val2");
                            assert_eq!(t.len(), 1);
                            assert_eq!(t[0].fragment(), &"type1");
                        }
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
    assert_eq!(x.function_body.len(), 1);
    match &x.function_body[0] {
        FunctionBodyStatement::Expression(e) => {
            match &e.function_statement {
                ExpressionFunctionValueCall::FunctionValue(x) => match x {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val3");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
            assert_eq!(
                e.operation_statement.as_ref().unwrap(),
                &ExpressionOperation::Plus
            );
            let x = e.expression.as_ref().unwrap();
            match x.function_statement {
                ExpressionFunctionValueCall::FunctionValue(ref x) => match x {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val4");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_main_func_call() {
    let x = main(Span::new(
        "let inline func1 val1 (val2: type1) : return_type = val3 + val4",
    ))
    .unwrap();
    assert_eq!(x.0.fragment(), &"");
    assert_eq!(x.1.len(), 1);
    let x = if let MainStatement::Function(v) = &x.1[0] {
        v.clone()
    } else {
        unimplemented!()
    };
    assert_eq!(&x.modifier.unwrap(), &FunctionModifier::Inline);
    assert_eq!(x.function_name.fragment(), &"func1");

    let v = x.return_type.as_ref().unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].fragment(), &"return_type");

    let p = &x.parameter_list;
    match p {
        ParameterList::ParameterValueList(ref v) => {
            assert_eq!(v.len(), 2);
            match &v[0] {
                ParameterValueList::ParameterValue(x) => {
                    assert_eq!(x.fragment(), &"val1");
                }
                _ => unimplemented!(),
            }
            match &v[1] {
                ParameterValueList::ParameterList(x) => {
                    assert_eq!(x.len(), 1);
                    match &x[0] {
                        ParameterValueType::ValueType(v, t) => {
                            assert_eq!(v.fragment(), &"val2");
                            assert_eq!(t.len(), 1);
                            assert_eq!(t[0].fragment(), &"type1");
                        }
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
    assert_eq!(x.function_body.len(), 1);
    match &x.function_body[0] {
        FunctionBodyStatement::Expression(e) => {
            match &e.function_statement {
                ExpressionFunctionValueCall::FunctionValue(x) => match x {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val3");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
            assert_eq!(
                e.operation_statement.as_ref().unwrap(),
                &ExpressionOperation::Plus
            );
            let x = e.expression.as_ref().unwrap();
            match x.function_statement {
                ExpressionFunctionValueCall::FunctionValue(ref x) => match x {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val4");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_main_func_complex() {
    let x = main(Span::new(
        "module name1.name2\nlet val1 = val2 + val3\nlet inline func1 val4 (val5: type1) : return_type = val6 + val7",
    )).unwrap();
    assert_eq!(x.0.fragment(), &"");
    assert_eq!(x.1.len(), 3);
    let module_name = if let MainStatement::Module(v) = &x.1[0] {
        v.clone()
    } else {
        unimplemented!()
    };
    assert_eq!(module_name.module_name.len(), 2);
    assert_eq!(module_name.module_name[0].fragment(), &"name1");
    assert_eq!(module_name.module_name[1].fragment(), &"name2");

    let let_binding = if let MainStatement::LetBinding(v) = &x.1[1] {
        v.clone()
    } else {
        unimplemented!()
    };
    let function = if let MainStatement::Function(v) = &x.1[2] {
        v.clone()
    } else {
        unimplemented!()
    };

    assert_eq!(let_binding.value_list.len(), 1);
    assert_eq!(let_binding.function_body.len(), 1);
    if let ParameterValueList::ParameterValue(v) = &let_binding.value_list[0] {
        assert_eq!(v.fragment(), &"val1");
    } else {
        unimplemented!()
    }

    match &let_binding.function_body[0] {
        FunctionBodyStatement::Expression(e) => {
            match &e.function_statement {
                ExpressionFunctionValueCall::FunctionValue(x) => match x {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val2");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
            assert_eq!(
                e.operation_statement.as_ref().unwrap(),
                &ExpressionOperation::Plus
            );
            let x = e.expression.as_ref().unwrap();
            match x.function_statement {
                ExpressionFunctionValueCall::FunctionValue(ref x) => match x {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val3");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }

    assert_eq!(&function.modifier.unwrap(), &FunctionModifier::Inline);
    assert_eq!(function.function_name.fragment(), &"func1");

    let v = function.return_type.as_ref().unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].fragment(), &"return_type");

    let p = &function.parameter_list;
    match p {
        ParameterList::ParameterValueList(ref v) => {
            assert_eq!(v.len(), 2);
            match &v[0] {
                ParameterValueList::ParameterValue(x) => {
                    assert_eq!(x.fragment(), &"val4");
                }
                _ => unimplemented!(),
            }
            match &v[1] {
                ParameterValueList::ParameterList(x) => {
                    assert_eq!(x.len(), 1);
                    match &x[0] {
                        ParameterValueType::ValueType(v, t) => {
                            assert_eq!(v.fragment(), &"val5");
                            assert_eq!(t.len(), 1);
                            assert_eq!(t[0].fragment(), &"type1");
                        }
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
    assert_eq!(function.function_body.len(), 1);
    match &function.function_body[0] {
        FunctionBodyStatement::Expression(e) => {
            match &e.function_statement {
                ExpressionFunctionValueCall::FunctionValue(x) => match x {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val6");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
            assert_eq!(
                e.operation_statement.as_ref().unwrap(),
                &ExpressionOperation::Plus
            );
            let x = e.expression.as_ref().unwrap();
            match x.function_statement {
                ExpressionFunctionValueCall::FunctionValue(ref x) => match x {
                    FunctionValue::ValueList(v) => {
                        assert_eq!(v.len(), 1);
                        assert_eq!(v[0].fragment(), &"val7");
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}
*/
#[test]
fn test_expression_value_type() {
    let x = expression_value_type(Span::new("true")).unwrap();
    let x = if let BasicTypeExpression::Bool(v) = x.1 {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(x, true);

    let x = expression_value_type(Span::new("false")).unwrap();
    let x = if let BasicTypeExpression::Bool(v) = x.1 {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(x, false);

    let x = expression_value_type(Span::new("\"string\"")).unwrap();
    let x = if let BasicTypeExpression::String(v) = x.1 {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(x, String::from("string"));

    let x = expression_value_type(Span::new("10")).unwrap();
    let x = if let BasicTypeExpression::Number(v) = x.1 {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(x, 10_f64);

    let x = expression_value_type(Span::new("10.1")).unwrap();
    let x = if let BasicTypeExpression::Number(v) = x.1 {
        v
    } else {
        unimplemented!()
    };
    assert_eq!(x, 10.1_f64);
}
