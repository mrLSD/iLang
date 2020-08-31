use crate::ast::*;
use crate::token::*;

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
fn test_parameter_type() {
    let (i, o) = parameter_type(Span::new("val1 val2")).unwrap();
    assert_eq!(o[0].fragment(), &"val1");
    assert_eq!(i.fragment(), &"val2");
    assert_eq!(o.len(), 1);

    let (i, o) = parameter_type(Span::new(" ( val1 ) val2")).unwrap();
    assert_eq!(o[0].fragment(), &"val1");
    assert_eq!(i.fragment(), &"val2");
    assert_eq!(o.len(), 1);

    let n = parameter_type(Span::new("* asd1 * asd2 * "));
    assert!(n.is_err());

    let (i, o) = parameter_type(Span::new(" ( asd1 ) * asd2 * ")).unwrap();
    assert_eq!(o[0].fragment(), &"asd1");
    assert_eq!(o[1].fragment(), &"asd2");
    assert_eq!(i.fragment(), &"* ");
    assert_eq!(o.len(), 2);

    let (_, o) = parameter_type(Span::new(" asd1 * asd2 ")).unwrap();
    assert_eq!(o[0].fragment(), &"asd1");
    assert_eq!(o[1].fragment(), &"asd2");
    assert_eq!(o.len(), 2);

    let (_, o) = parameter_type(Span::new(" ( asd1 ) * ( asd2 ) ")).unwrap();
    assert_eq!(o[0].fragment(), &"asd1");
    assert_eq!(o[1].fragment(), &"asd2");
    assert_eq!(o.len(), 2);

    let (_, o) = parameter_type(Span::new("asd1 * ( asd2 ) * asd3")).unwrap();
    assert_eq!(o[0].fragment(), &"asd1");
    assert_eq!(o[1].fragment(), &"asd2");
    assert_eq!(o[2].fragment(), &"asd3");
    assert_eq!(o.len(), 3);

    let n = parameter_type(Span::new("* asd1 * ( asd2 ) * asd3"));
    assert!(n.is_err());

    let (_, o) = parameter_type(Span::new("(asd1 * ( asd2 ) * asd3)")).unwrap();
    assert_eq!(o[0].fragment(), &"asd1");
    assert_eq!(o[1].fragment(), &"asd2");
    assert_eq!(o[2].fragment(), &"asd3");
    assert_eq!(o.len(), 3);
}

#[test]
fn test_parameter_value_type() {
    match parameter_value_type(Span::new("val1: type1")).unwrap() {
        (_, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
        }
        _ => unimplemented!(),
    }

    match parameter_value_type(Span::new(" ( val1 ) : ( type1 ) ")).unwrap() {
        (_, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
        }
        _ => unimplemented!(),
    }

    match parameter_value_type(Span::new(" ( ( val1 ) : ( type1 ) ) ")).unwrap() {
        (_, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
        }
        _ => unimplemented!(),
    }

    match parameter_value_type(Span::new("val1: type1 * type2")).unwrap() {
        (_, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
            assert_eq!(t[1].fragment(), &"type2");
        }
        _ => unimplemented!(),
    }

    match parameter_value_type(Span::new("val1: (type1 * (type2) * type3)")).unwrap() {
        (_, ParameterValueType::ValueType(v, t)) => {
            assert_eq!(v.fragment(), &"val1");
            assert_eq!(t[0].fragment(), &"type1");
            assert_eq!(t[1].fragment(), &"type2");
            assert_eq!(t[2].fragment(), &"type3");
        }
        _ => unimplemented!(),
    }

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
fn test_parameter_value_list() {
    match parameter_value_list(Span::new("val1")).unwrap() {
        (_, ParameterValueList::ParameterValue(v)) => assert_eq!(v.fragment(), &"val1"),
        _ => unimplemented!(),
    }

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
fn test_parameter_list() {
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
fn test_value_list() {
    let x = value_list(Span::new("val1")).unwrap().1;
    assert_eq!(x[0].fragment(), &"val1");

    let x = value_list(Span::new("val1, val2")).unwrap();
    assert_eq!(x.1[0].fragment(), &"val1");
    assert_eq!(x.0.fragment(), &", val2");

    let x = value_list(Span::new("(val1)")).unwrap().1;
    assert_eq!(x[0].fragment(), &"val1");

    let x = value_list(Span::new("(val1, val2)")).unwrap().1;
    assert_eq!(x[0].fragment(), &"val1");
    assert_eq!(x[1].fragment(), &"val2");

    let x = value_list(Span::new("(val1, (val2))")).unwrap().1;
    assert_eq!(x[0].fragment(), &"val1");
    assert_eq!(x[1].fragment(), &"val2");

    let x = value_list(Span::new("((val1), (val2))")).unwrap().1;
    assert_eq!(x[0].fragment(), &"val1");
    assert_eq!(x[1].fragment(), &"val2");
}

#[test]
fn test_let_value_list() {
    match let_value_list(Span::new("val1")).unwrap().1[0] {
        ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val1"),
        _ => unimplemented!(),
    }

    let res = let_value_list(Span::new("val1, val2")).unwrap().1;
    match res[0] {
        ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val1"),
        _ => unimplemented!(),
    }
    match res[1] {
        ParameterValueList::ParameterValue(v) => assert_eq!(v.fragment(), &"val2"),
        _ => unimplemented!(),
    }

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

    let res = module(Span::new("module test1.test2")).unwrap().1;
    assert_eq!(res.module_name.len(), 2);
    assert_eq!(res.module_name[0].fragment(), &"test1");
    assert_eq!(res.module_name[1].fragment(), &"test2");
    assert_eq!(res.accessibility, None);

    let res = module(Span::new("module public test1.test2.test3"))
        .unwrap()
        .1;
    assert_eq!(res.module_name.len(), 3);
    assert_eq!(res.module_name[0].fragment(), &"test1");
    assert_eq!(res.module_name[1].fragment(), &"test2");
    assert_eq!(res.module_name[2].fragment(), &"test3");
    assert_eq!(res.accessibility.unwrap().fragment(), &"public");

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
    let res = module(Span::new("test"));
    assert!(res.is_err());

    let res = namespace(Span::new("namespace test1")).unwrap().1;
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].fragment(), &"test1");

    let res = namespace(Span::new("namespace test1.test2")).unwrap().1;
    assert_eq!(res.len(), 2);
    assert_eq!(res[0].fragment(), &"test1");
    assert_eq!(res[1].fragment(), &"test2");

    let res = namespace(Span::new("namespace test1.test2.test3"))
        .unwrap()
        .1;
    assert_eq!(res.len(), 3);
    assert_eq!(res[0].fragment(), &"test1");
    assert_eq!(res[1].fragment(), &"test2");
    assert_eq!(res[2].fragment(), &"test3");

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
    //println!("{:#?}", res);
}
