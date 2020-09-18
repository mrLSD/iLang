//! Parser tokens for grammar
//!
//! Parse grammar lexical constructions to AST tokens.
//!
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1,
        anychar,
        char,
        multispace0,
        multispace1,
    },
    combinator::{
        map,
        not,
        opt,
    },
    error::ParseError,
    multi::{
        many0,
        many1,
    },
    sequence::tuple,
    sequence::{
        delimited,
        preceded,
        terminated,
    },
    IResult,
    InputTakeAtPosition,
};

use crate::{
    ast,
    ast::{
        ParseResult,
        Span,
    },
    char::AsChar,
};
use nom::bytes::complete::{take_while, take_until, take_while_m_n};
use nom_locate::LocatedSpan;
use nom::combinator::peek;

/// Apply parser func for delimited space
/// ## RULE:
/// ```js
/// [MULTISPACE] parser-func [MULTISPACE]
/// ```
pub fn delimited_space<'a, O, F>(func: F) -> impl Fn(Span<'a>) -> ParseResult<O>
where
    F: Fn(Span<'a>) -> ParseResult<O>,
{
    delimited(multispace0, func, multispace0)
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

/// Get string data
pub fn string(data: Span) -> ParseResult<String> {
    let (_, o) = map(preceded(tag("\""), many0(anychar)), |o| -> String {
        o.into_iter().collect()
    })(data)?;
    let d:IResult<&str, &str> = peek( take_until("\"") )(o.as_str());
    println!("{:#?}", d);
    // let s = res.1.as_str();
    // let x = Span::new(s);
    // let r: ParseResult<Span> = take_until("\"")(x);
    //println!("{:#?}", res.1);
    
    //preceded(tag("\""), terminated(many0(anychar), tag("\"")))(data);
    //let (i, o) = res?;
    //Ok((i, o.into_iter().collect()))
    Ok((data, "".to_string()))
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
pub fn value_list(data: Span) -> ParseResult<ast::ValueList> {
    let val_list = map(
        get_from_brackets(tuple((
            parameter_value,
            many0(preceded(delimited_space(tag(",")), parameter_value)),
        ))),
        |(first, mut second)| {
            let mut res_list = vec![first];
            res_list.append(&mut second);
            res_list
        },
    );
    alt((map(parameter_value, |v| vec![v]), val_list))(data)
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
    delimited_space(alt((tag("public"), tag("internal"), tag("private"))))(data)
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

/// Function body parser
/// ## RULES:
/// ```js
/// function-body = [function-body-statement]*
/// ```
pub fn function_body(data: Span) -> ParseResult<ast::FunctionBody> {
    many0(function_body_statement)(data)
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
            preceded(delimited_space(tag("let")), let_value_list),
            preceded(delimited_space(tag("=")), function_body),
        )),
        |v| ast::LetBinding {
            value_list: v.0,
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
    delimited_space(ident)(data)
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
                terminated(tag("let"), multispace1),
                tuple((
                    opt(map(delimited_space(tag("inline")), |_| {
                        ast::FunctionModifier::Inline
                    })),
                    function_name,
                )),
            ),
            parameter_list,
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
    many1(alt((
        map(delimited_space(namespace), ast::MainStatement::Namespace),
        map(delimited_space(module), ast::MainStatement::Module),
        map(delimited_space(function), ast::MainStatement::Function),
        map(delimited_space(let_binding), ast::MainStatement::LetBinding),
    )))(data)
}
