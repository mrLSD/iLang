//! Parser tokens for grammar
//!
//! Parse grammar lexical constructions to AST tokens.
//!
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1,
        char,
        multispace0,
        multispace1,
        space0,
        space1,
    },
    combinator::{
        map,
        not,
        opt,
        value,
    },
    error::{
        ErrorKind,
        ParseError,
    },
    multi::{
        many0,
        many1,
    },
    number::complete::double,
    sequence::tuple,
    sequence::{
        delimited,
        preceded,
        terminated,
    },
    IResult,
    InputTakeAtPosition,
};

use super::{
    ast,
    ast::BasicTypeExpression,
    ast::{
        ParseResult,
        Span,
    },
    char::AsChar,
    string::parse_string,
};

/// Apply parser func for delimited space
/// ## RULE:
/// ```js
/// [MULTISPACE] parser-func [MULTISPACE]
/// ```
pub fn delimited_space<'a, O, F>(func: F) -> impl Fn(Span<'a>) -> ParseResult<O>
where
    F: Fn(Span<'a>) -> ParseResult<O>,
{
    delimited(space0, func, multispace0)
}

pub fn delimited_white_space<'a, O, F>(func: F) -> impl Fn(Span<'a>) -> ParseResult<O>
where
    F: Fn(Span<'a>) -> ParseResult<O>,
{
    delimited(space0, func, space0)
}

/// Apply parser for brackets case
/// ## RULE:
/// ```js
/// [MULTISPACE] "(" [MULTISPACE] parser-func [MULTISPACE] ")" [MULTISPACE]
/// ```
pub fn get_from_brackets<'a, O, F>(func: F) -> impl Fn(Span<'a>) -> ParseResult<O>
where
    F: Fn(Span<'a>) -> ParseResult<O>,
{
    preceded(
        delimited_space(char('(')),
        terminated(func, delimited_space(char(')'))),
    )
}

/// Parse Ident from brackets
/// ## RULE:
/// ```js
/// [MULTISPACE] "(" [MULTISPACE] ident [MULTISPACE] ")" [MULTISPACE]
/// ```
pub fn get_ident_from_brackets(data: Span) -> ParseResult<ast::Ident> {
    get_from_brackets(ident)(data)
}

/// Alphanum characters with underscores. Based on ASCII.
/// ## RULES:
/// ```js
/// (alpha | number | '_')*
/// ```
pub fn alphanum_and_underscore0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    let f = |c: &char| c.is_alphanumeric() || c.as_char() == '_';
    input.split_at_position_complete(|item| !item.is_a(f))
}

/// Exclude reserved keywords
/// ## RULES:
/// ```js
/// reserved-keywords = !( "let" | "module" | "namespace" | "type" )
/// ```
pub fn reserved_keywords<'a, O, F>(func: F) -> impl Fn(Span<'a>) -> ParseResult<O>
where
    F: Fn(Span<'a>) -> ParseResult<O>,
{
    preceded(
        alt((tag("let"), tag("module"), tag("namespace"), tag("type"))),
        func,
    )
}

/// Get ident token
///
/// First always should be Alpha char.
/// ## RULES:
/// ```js
/// ident = (alpha+)(alpha | number | '_')*
/// ```
pub fn ident(data: Span) -> ParseResult<ast::Ident> {
    let _ = alpha1(data)?;
    let (i, o) = alphanum_and_underscore0(data)?;
    let _ = not(alt((tag("let"), tag("module"), tag("namespace"))))(o)?;
    Ok((i, o))
}

/// Parse expression operations
/// ## RULES:
/// ```js
/// expression-operations = (
///     "+" | "-" |
///     "*" | "/" |
///     "<<<" | ">>>"
/// )
/// ```
pub fn expression_operations(data: Span) -> ParseResult<ast::ExpressionOperation> {
    alt((
        map(tag("+"), |_| ast::ExpressionOperation::Plus),
        map(tag("-"), |_| ast::ExpressionOperation::Minus),
        map(tag("*"), |_| ast::ExpressionOperation::Multiply),
        map(tag("/"), |_| ast::ExpressionOperation::Divide),
        map(tag("<<<"), |_| ast::ExpressionOperation::ShiftLeft),
        map(tag(">>>"), |_| ast::ExpressionOperation::ShiftRight),
    ))(data)
}

/// Parse parameter value
/// ## RULES:
/// ```js
/// parameter-value = ident-value
/// ```
pub fn parameter_value(data: Span) -> ParseResult<ast::ParameterValue> {
    ident_value(data)
}

/// Parse ident value with space and brackets
/// ## RULES:
/// ```js
/// ident-value = (ident | "(" ident ")")
/// ```
pub fn ident_value(data: Span) -> ParseResult<ast::Ident> {
    delimited_space(alt((ident, get_ident_from_brackets)))(data)
}

/// Parse parameter type. It can contain type sequence
/// ## RULES:
/// ```js
/// parameter-type = (ident-value ["*" ident-value] | "(" ident-value ["*" ident-value] ")")+
/// ```
pub fn parameter_type(data: Span) -> ParseResult<ast::ParameterType> {
    let type_list = tuple((
        ident_value,
        many0(preceded(delimited_space(tag("*")), ident_value)),
    ));
    let type_list_bracketes = get_from_brackets(tuple((
        ident_value,
        many0(preceded(delimited_space(tag("*")), ident_value)),
    )));

    map(
        alt((type_list, type_list_bracketes)),
        |(first, mut second)| {
            let mut res_list = vec![first];
            res_list.append(&mut second);
            res_list
        },
    )(data)
}

/// Value-Type parameters parser
/// ## RULES:
/// ```js
/// parameter-value-type = (parameter-value ":" parameter-type | "(" parameter-value ":" parameter-type ")")
/// ```
pub fn parameter_value_type(data: Span) -> ParseResult<ast::ParameterValueType> {
    let value_type = tuple((
        parameter_value,
        preceded(delimited_space(tag(":")), parameter_type),
    ));
    let value_type_bracketes = get_from_brackets(tuple((
        parameter_value,
        preceded(delimited_space(tag(":")), parameter_type),
    )));

    map(alt((value_type, value_type_bracketes)), |o| {
        ast::ParameterValueType::ValueType(o.0, o.1)
    })(data)
}

/// Parameters list with brackets parser
/// ## RULES:
/// ```js
/// parameter-list-brackets = "(" [(
///     parameter-value |
///     parameter-value-type
/// ) [","]]* ")"
/// ```
pub fn parameter_list_brackets(data: Span) -> ParseResult<ast::ParameterValueList> {
    let wrapper_parameter_value = &map(parameter_value, ast::ParameterValueType::Value);
    let (i, (param1, mut param2)) = get_from_brackets(tuple((
        alt((parameter_value_type, wrapper_parameter_value)),
        many0(preceded(
            delimited_space(tag(",")),
            alt((parameter_value_type, wrapper_parameter_value)),
        )),
    )))(data)?;
    let mut res = vec![param1];
    res.append(&mut param2);
    Ok((i, ast::ParameterValueList::ParameterList(res)))
}

/// Parameters value list
/// ## RULES:
/// ```js
/// parameter-value-list = (parameter-value | parameter-list-brackets)
/// ```
pub fn parameter_value_list(data: Span) -> ParseResult<ast::ParameterValueList> {
    alt((
        map(parameter_value, ast::ParameterValueList::ParameterValue),
        parameter_list_brackets,
    ))(data)
}

/// Parameters list
/// ## RULES:
/// ```js
/// parameter-list = (parameter-value-list+ | parameter-list-brackets)
/// ```
pub fn parameter_list(data: Span) -> ParseResult<ast::ParameterList> {
    alt((
        map(
            many1(parameter_value_list),
            ast::ParameterList::ParameterValueList,
        ),
        map(parameter_list_brackets, ast::ParameterList::ParameterList),
    ))(data)
}

/// Value list from parameter values
/// ## RULES:
/// ```js
/// value-list = (parameter-value | "(" (parameter-value [","])* ")")
/// ```
#[allow(clippy::let_and_return)]
pub fn value_list(data: Span) -> ParseResult<ast::ValueList> {
    let val_expr = &alt((
        map(expression_value_type, ast::ValueExpression::TypeExpression),
        map(parameter_value, ast::ValueExpression::ParameterValue),
    ));
    let val_list = map(
        get_from_brackets(tuple((
            val_expr,
            many0(preceded(delimited_space(tag(",")), val_expr)),
        ))),
        |(first, mut second)| {
            let mut res_list = vec![first];
            res_list.append(&mut second);
            res_list
        },
    );
    let res = alt((map(val_expr, |v| vec![v]), val_list))(data);
    res
}

/// Let binding Value list from parameter values list
/// ## RULES:
/// ```js
/// let-value-list = (parameter-value-list [","])+
/// ```
pub fn let_value_list(data: Span) -> ParseResult<ast::LetValueList> {
    map(
        tuple((
            parameter_value_list,
            many0(preceded(delimited_space(tag(",")), parameter_value_list)),
        )),
        |(first, mut second)| {
            let mut res_list = vec![first];
            res_list.append(&mut second);
            res_list
        },
    )(data)
}

/// Let binding Value list from parameter values list
/// ## RULES:
/// ```js
/// namespace = "namespace" (namespace-name ".")* namespace-name
/// namespace-name = ident
/// ```
pub fn namespace(data: Span) -> ParseResult<ast::Namespace> {
    map(
        tuple((
            preceded(terminated(tag("namespace"), multispace1), ident),
            many0(preceded(tag("."), ident)),
        )),
        |(first, mut second)| {
            let mut res_list = vec![first];
            res_list.append(&mut second);
            res_list
        },
    )(data)
}

/// Accessibility modifiers parser
/// ## RULES:
/// ```js
/// accessibility-modifier = ("public" | "internal" | "private")
/// ```
pub fn accessibility_modifier(data: Span) -> ParseResult<ast::AccessibilityModifier> {
    delimited_white_space(alt((tag("public"), tag("internal"), tag("private"))))(data)
}

/// Module parser
/// ## RULES:
/// ```js
/// module = "module" [accessibility-modifier] (qualified-namespace "." )* module-name
/// qualified-namespace = indent
/// module-name = ident
/// ```
pub fn module(data: Span) -> ParseResult<ast::Module> {
    map(
        tuple((
            preceded(
                terminated(tag("module"), multispace1),
                tuple((opt(accessibility_modifier), ident)),
            ),
            many0(preceded(tag("."), ident)),
        )),
        |(first, mut second)| {
            let accessibility = first.0;
            let mut res_list = vec![first.1];
            res_list.append(&mut second);
            ast::Module {
                accessibility,
                module_name: res_list,
            }
        },
    )(data)
}

/// Function value
/// ## RULES:
/// ```js
/// function-value = (value-list | "(" expression ")")
/// ```
pub fn function_value(data: Span) -> ParseResult<ast::FunctionValue> {
    alt((
        map(value_list, ast::FunctionValue::ValueList),
        map(get_from_brackets(expression), |v| {
            ast::FunctionValue::Expression(Box::new(v))
        }),
    ))(data)
}

/// Function value
/// ## RULES:
/// ```js
/// function-call-name = (function-name ".")* function-name
/// function-name = ident
/// ```
pub fn function_call_name(data: Span) -> ParseResult<ast::FunctionCallName> {
    map(
        tuple((ident, many0(preceded(tag("."), ident)))),
        |(first, mut second)| {
            let mut res_list = vec![first];
            res_list.append(&mut second);
            res_list
        },
    )(data)
}

/// Function value
/// ## RULES:
/// ```js
/// function-call = function-call-name (function-value+ | "(" [function-value [","] ]* ")")
/// ```
pub fn function_call(data: Span) -> ParseResult<ast::FunctionCall> {
    let func_val = alt((
        many1(function_value),
        // Detect only empty brackets. Other cases covered via `function_value` parser
        map(get_from_brackets(multispace0), |_| vec![]),
    ));
    map(tuple((function_call_name, func_val)), |v| {
        ast::FunctionCall {
            function_call_name: v.0,
            function_value: v.1,
        }
    })(data)
}

pub fn function_body(data: Span) -> ParseResult<ast::FunctionBody> {
    #[derive(Debug)]
    struct Block {
        line: u32,
        column: usize,
    }
    fn select_block(func_body: &ast::FunctionBodyStatement) -> Block {
        println!("-> select_block");
        match func_body {
            ast::FunctionBodyStatement::Expression(ref e) => match e.function_statement {
                ast::ExpressionFunctionValueCall::FunctionValue(ref x) => match x {
                    ast::FunctionValue::ValueList(ref val_list) => match val_list[0] {
                        ast::ValueExpression::ParameterValue(ref param_val) => {
                            let line = param_val.location_line();
                            let column = param_val.get_column();
                            println!("FunctionBodyStatement.ValueExpression{:?}", (line, column));
                            Block { line, column }
                        }
                        ast::ValueExpression::TypeExpression(ref ty) => {
                            println!("\t# FunctionBodyStatement.TypeExpression: {:#?}", ty);
                            Block { line: 1, column: 1 }
                        }
                    },
                    _ => unimplemented!(),
                },
                ast::ExpressionFunctionValueCall::FunctionCall(ref fn_call) => {
                    let line = fn_call.function_call_name[0].location_line();
                    let column = fn_call.function_call_name[0].get_column();
                    println!("FunctionBodyStatement.FunctionCall{:?}", (line, column));
                    Block { line, column }
                }
            },
            ast::FunctionBodyStatement::LetBinding(ref let_bind) => {
                let line = let_bind.let_position.location_line();
                let offset = let_bind.let_position.get_column();
                println!("# LetBinding{:#?}", (line, offset));
                match let_bind.value_list[0] {
                    ast::ParameterValueList::ParameterValue(ref param_val) => {
                        let line = param_val.location_line();
                        let column = param_val.get_column();
                        println!("LetBinding.ParameterValue{:?}", (line, column));
                        Block { line, column }
                    }
                    ast::ParameterValueList::ParameterList(ref param_val_ty) => {
                        match param_val_ty[0] {
                            ast::ParameterValueType::Value(ref param_val) => {
                                let line = param_val.location_line();
                                let column = param_val.get_column();
                                println!("LetBinding.ParameterList{:?}", (line, column));
                                Block { line, column }
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
            }
            ast::FunctionBodyStatement::FunctionCall(ref fn_call) => {
                let line = fn_call.function_call_name[0].location_line();
                let column = fn_call.function_call_name[0].get_column();
                println!("FunctionCall{:?}", (line, column));
                Block { line, column }
            }
        }
    }

    let mut acc = vec![];
    let mut block: Option<Block> = None;
    let mut inp = data;
    loop {
        match function_body_statement(inp) {
            Err(nom::Err::Error(_)) => {
                println!("-> block {:#?}", block);
                return Ok((inp, acc));
            }
            Err(e) => return Err(e),
            Ok((new_inp, o)) => {
                if new_inp == inp {
                    return Err(nom::Err::Error((inp, ErrorKind::Many0)));
                }
                println!("{:?}", acc.len());
                println!("{:?}", new_inp);
                println!("{:#?}", o);

                match o {
                    ast::FunctionBodyStatement::LetBinding(ref let_bind) => {
                        let new_block = Block {
                            line: let_bind.let_position.location_line(),
                            column: let_bind.let_position.get_column(),
                        };
                        println!("LetBinding{:?}", new_block);
                        if let Some(b) = block {
                            // Check is it first line or same line or column is less
                            if new_block.line == 1
                                || new_block.line <= b.line
                                || new_block.column < b.column
                            {
                                // Return prev statement and decline current
                                return Ok((inp, acc));
                            }
                        } else {
                            // Check is it first line
                            if new_block.line == 1 {
                                return Ok((inp, acc));
                            }
                        }
                        block = Some(new_block);
                    }
                    _ => {
                        let new_block = select_block(&o);
                        println!("-> block {:#?}", block);
                        if let Some(b) = block {
                            // Check is it same line or column is less
                            if new_block.line <= b.line || new_block.column < b.column {
                                // Return prev statement and decline current
                                return Ok((inp, acc));
                            }
                        }
                        acc.push(o.clone());
                        return Ok((new_inp, acc));
                    }
                }
                inp = new_inp;
                acc.push(o.clone());
            }
        }
    }
}

/// Function body parser
/// ## RULES:
/// ```js
/// function-body = [function-body-statement]*
/// ```
pub fn function_body1(data: Span) -> ParseResult<ast::FunctionBody> {
    many0(map(function_body_statement, |f| {
        match f {
            ast::FunctionBodyStatement::Expression(ref e) => match e.function_statement {
                ast::ExpressionFunctionValueCall::FunctionValue(ref x) => match x {
                    ast::FunctionValue::ValueList(ref v) => match v[0] {
                        ast::ValueExpression::ParameterValue(ref p) => {
                            let line = p.location_line();
                            let offset = p.get_column();
                            println!("FunctionBodyStatement.ValueExpression{:?}", (line, offset));
                        }
                        ast::ValueExpression::TypeExpression(ref t) => {
                            println!("\t# FunctionBodyStatement.TypeExpression: {:#?}", t);
                        }
                    },
                    _ => unimplemented!(),
                },
                ast::ExpressionFunctionValueCall::FunctionCall(ref x) => {
                    let line = x.function_call_name[0].location_line();
                    let offset = x.function_call_name[0].get_column();
                    println!("FunctionBodyStatement.FunctionCall{:?}", (line, offset));
                }
            },
            ast::FunctionBodyStatement::LetBinding(ref x) => {
                let line = x.let_position.location_line();
                let offset = x.let_position.get_column();
                println!("# LetBinding{:#?}", (line, offset));
                match x.value_list[0] {
                    ast::ParameterValueList::ParameterValue(ref p) => {
                        let line = p.location_line();
                        let offset = p.get_column();
                        println!("LetBinding.ParameterValue{:?}", (line, offset));
                    }
                    ast::ParameterValueList::ParameterList(ref l) => match l[0] {
                        ast::ParameterValueType::Value(ref v) => {
                            let line = v.location_line();
                            let offset = v.get_column();
                            println!("LetBinding.ParameterList{:?}", (line, offset));
                        }
                        _ => unimplemented!(),
                    },
                }
            }
            ast::FunctionBodyStatement::FunctionCall(ref x) => {
                let line = x.function_call_name[0].location_line();
                let offset = x.function_call_name[0].get_column();
                println!("FunctionCall{:?}", (line, offset));
            }
        };
        f
    }))(data)
}

/// Function body statement parser
/// ## RULES:
/// ```js
/// function-body-statement = (let-binding | function-call | expression)
/// ```
pub fn function_body_statement(data: Span) -> ParseResult<ast::FunctionBodyStatement> {
    alt((
        map(let_binding, ast::FunctionBodyStatement::LetBinding),
        map(function_call, ast::FunctionBodyStatement::FunctionCall),
        map(expression, |v| {
            ast::FunctionBodyStatement::Expression(Box::new(v))
        }),
    ))(data)
}

/// Let binding statement
/// ## RULES:
/// ```js
/// let-binding = "let" let-value-list "=" function-body
/// ```
pub fn let_binding(data: Span) -> ParseResult<ast::LetBinding> {
    map(
        tuple((
            tuple((delimited_space(tag("let")), let_value_list)),
            preceded(delimited_space(tag("=")), function_body),
        )),
        |v| ast::LetBinding {
            let_position: (v.0).0,
            value_list: (v.0).1,
            function_body: v.1,
        },
    )(data)
}

/// Expression parser
/// ## RULES:
/// ```js
/// expression = (
///     function-value |
///     function-call |
///     "(" function-call ")"
/// ) [expression-operations expression]
/// ```
pub fn expression(data: Span) -> ParseResult<ast::Expression> {
    let func = alt((
        map(get_from_brackets(function_call), |v| {
            ast::ExpressionFunctionValueCall::FunctionCall(v)
        }),
        map(delimited_space(function_call), |v| {
            ast::ExpressionFunctionValueCall::FunctionCall(v)
        }),
        map(delimited_space(function_value), |v| {
            ast::ExpressionFunctionValueCall::FunctionValue(v)
        }),
    ));
    map(
        tuple((func, opt(tuple((expression_operations, expression))))),
        |v| {
            let (operation_statement, expression) = if let Some(x) = v.1 {
                (Some(x.0), Some(Box::new(x.1)))
            } else {
                (None, None)
            };
            ast::Expression {
                function_statement: v.0,
                operation_statement,
                expression,
            }
        },
    )(data)
}

/// Function name parser
/// ## RULES:
/// ```js
/// function-name = [MULTISPACE] ident [MULTISPACE]
/// ```
pub fn function_name(data: Span) -> ParseResult<ast::FunctionName> {
    delimited_white_space(ident)(data)
}

/// Return type parser
/// ## RULES:
/// ```js
/// return-type = [MULTISPACE] parameter-type [MULTISPACE]
/// ```
pub fn return_type(data: Span) -> ParseResult<ast::ReturnType> {
    delimited_space(parameter_type)(data)
}

/// Function parser
/// ## RULES:
/// ```js
/// function = "let" ["inline"] function-name parameter-list [ ":" return-type ] "=" function-body
/// ```
pub fn function(data: Span) -> ParseResult<ast::Function> {
    map(
        tuple((
            preceded(
                terminated(tag("let"), space1),
                tuple((
                    opt(map(delimited_white_space(tag("inline")), |_| {
                        ast::FunctionModifier::Inline
                    })),
                    function_name,
                )),
            ),
            alt((
                parameter_list,
                map(get_from_brackets(multispace0), |_| {
                    ast::ParameterList::ParameterValueList(vec![])
                }),
            )),
            opt(preceded(delimited_space(tag(":")), return_type)),
            preceded(delimited_space(tag("=")), function_body),
        )),
        |v| {
            let func_name = v.0;
            ast::Function {
                modifier: func_name.0,
                function_name: func_name.1,
                parameter_list: v.1,
                return_type: v.2,
                function_body: v.3,
            }
        },
    )(data)
}

/// Main statement parser
/// ## RULES:
/// ```js
/// main = (
///     namespace |
///     module    |
///     function  |
///     let-binding
/// )+
/// ```
pub fn main(data: Span) -> ParseResult<ast::Main> {
    let (i, o) = many1(alt((
        map(delimited_space(namespace), ast::MainStatement::Namespace),
        map(delimited_space(module), ast::MainStatement::Module),
        map(delimited_space(function), ast::MainStatement::Function),
        map(delimited_space(let_binding), ast::MainStatement::LetBinding),
    )))(data)
    .unwrap();
    //println!("{:#?}", o);
    Ok((i, o))
}

/// Numbers parser
pub fn number(data: Span) -> ParseResult<ast::BasicTypeExpression> {
    map(double, BasicTypeExpression::Number)(data)
}

/// Boolean parser
pub fn boolean(data: Span) -> ParseResult<ast::BasicTypeExpression> {
    let parse_true = value(true, tag("true"));
    let parse_frue = value(false, tag("false"));
    map(alt((parse_true, parse_frue)), BasicTypeExpression::Bool)(data)
}

/// Expression basic/common types values parser
pub fn expression_value_type(data: Span) -> ParseResult<ast::BasicTypeExpression> {
    delimited_space(alt((parse_string, number, boolean)))(data)
}
