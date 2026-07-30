//! Unit tests for lib.rs.

use super::*;

#[test]
fn truthiness() {
    assert!(!Value::Nil.is_truthy());
    assert!(!Value::Bool(false).is_truthy());
    assert!(Value::Bool(true).is_truthy());
    assert!(Value::Int(0).is_truthy());
    assert!(Value::str("").is_truthy());
}

#[test]
fn equality() {
    assert_eq!(Value::Int(1), Value::Int(1));
    assert_ne!(Value::Int(1), Value::Float(1.0));
    assert_ne!(Value::Nil, Value::Bool(false));
    let l = Value::List(List::from_vec(vec![Value::Int(1), Value::Int(2)]));
    let v = Value::Vector(Rc::new(vec![Value::Int(1), Value::Int(2)]));
    assert_eq!(l, v); // Lists and vectors have sequential equality.
}

#[test]
fn printing() {
    assert_eq!(pr_str(&Value::str("hi\n")), "\"hi\\n\"");
    assert_eq!(print_str(&Value::str("hi")), "hi");
    assert_eq!(pr_str(&Value::Float(1.0)), "1.0");
    let l = Value::List(List::from_vec(vec![Value::Int(1), Value::Int(2)]));
    assert_eq!(pr_str(&l), "(1 2)");
}

#[test]
fn persistent_list_keeps_count_and_order() {
    let empty = List::empty();
    assert_eq!(empty.count(), 0);
    assert_eq!(empty.iter().next(), None);

    let list = List::cons(Value::Int(1), List::cons(Value::Int(2), List::empty()));
    assert_eq!(list.count(), 2);
    assert_eq!(
        list.iter().cloned().collect::<Vec<_>>(),
        vec![Value::Int(1), Value::Int(2)]
    );
}

#[test]
fn function_methods_prefer_fixed_arity_then_variadic() {
    let fixed = FnMethod {
        params: vec!["x".into(), "y".into()],
        rest: None,
        body: vec![],
    };
    let variadic = FnMethod {
        params: vec!["x".into()],
        rest: Some("more".into()),
        body: vec![],
    };
    let closure = Closure {
        name: Some("f".into()),
        methods: vec![variadic, fixed],
        env: None,
    };
    assert_eq!(closure.method_for(2).unwrap().rest, None);
    assert_eq!(closure.method_for(3).unwrap().rest.as_deref(), Some("more"));
    assert!(closure.method_for(0).is_none());
}

#[test]
fn scope_lookup_prefers_latest_local_then_parent() {
    let parent = Scope::child(
        None,
        vec![
            ("x".into(), Value::Int(1)),
            ("parent".into(), Value::Int(9)),
        ],
    );
    let child = Scope::child(
        Some(parent),
        vec![("x".into(), Value::Int(2)), ("x".into(), Value::Int(3))],
    );
    assert_eq!(child.lookup("x"), Some(Value::Int(3)));
    assert_eq!(child.lookup("parent"), Some(Value::Int(9)));
    assert_eq!(child.lookup("missing"), None);
}

#[test]
fn type_names_and_callability_cover_runtime_values() {
    let native = Value::Native(NativeFn::new("id", |args| {
        Ok(args.first().cloned().unwrap_or(Value::Nil))
    }));
    let closure = Value::Fn(Rc::new(Closure {
        name: None,
        methods: vec![],
        env: None,
    }));
    let values = vec![
        (Value::Nil, "nil"),
        (Value::Bool(true), "boolean"),
        (Value::Int(1), "integer"),
        (Value::Float(1.0), "float"),
        (Value::Char('x'), "char"),
        (Value::str("x"), "string"),
        (Value::symbol(Name::simple("x")), "symbol"),
        (Value::keyword(Name::simple("x")), "keyword"),
        (Value::List(List::empty()), "list"),
        (Value::Vector(Rc::new(vec![])), "vector"),
        (Value::Map(Rc::new(vec![])), "map"),
        (Value::Set(Rc::new(vec![])), "set"),
        (closure, "function"),
        (native, "function"),
    ];
    for (value, expected) in values {
        assert_eq!(value.type_name(), expected);
    }
    assert!(Value::keyword(Name::simple("k")).is_callable());
    assert!(!Value::Int(1).is_callable());
}

#[test]
fn maps_and_sets_compare_without_insertion_order() {
    let a = Value::Map(Rc::new(vec![
        (Value::keyword(Name::simple("a")), Value::Int(1)),
        (Value::keyword(Name::simple("b")), Value::Int(2)),
    ]));
    let b = Value::Map(Rc::new(vec![
        (Value::keyword(Name::simple("b")), Value::Int(2)),
        (Value::keyword(Name::simple("a")), Value::Int(1)),
    ]));
    assert_eq!(a, b);

    let a = Value::Set(Rc::new(vec![Value::Int(1), Value::Int(2)]));
    let b = Value::Set(Rc::new(vec![Value::Int(2), Value::Int(1)]));
    assert_eq!(a, b);
}

#[test]
fn printing_covers_collections_functions_and_escapes() {
    let named = Value::Fn(Rc::new(Closure {
        name: Some("sum".into()),
        methods: vec![],
        env: None,
    }));
    let anonymous = Value::Fn(Rc::new(Closure {
        name: None,
        methods: vec![],
        env: None,
    }));
    let native = Value::Native(NativeFn::new("id", |_| Ok(Value::Nil)));
    assert_eq!(pr_str(&named), "#<fn sum>");
    assert_eq!(pr_str(&anonymous), "#<fn anonymous>");
    assert_eq!(pr_str(&native), "#<native id>");
    assert_eq!(pr_str(&Value::Char('\t')), "\\tab");
    assert_eq!(print_str(&Value::Char('\t')), "\t");
    assert_eq!(pr_str(&Value::symbol(Name::qualified("a", "b"))), "a/b");
    assert_eq!(pr_str(&Value::keyword(Name::simple("k"))), ":k");
    assert_eq!(
        pr_str(&Value::Vector(Rc::new(vec![
            Value::Int(1),
            Value::Bool(true)
        ]))),
        "[1 true]"
    );
    assert_eq!(
        pr_str(&Value::Set(Rc::new(vec![Value::Int(1), Value::Int(2)]))),
        "#{1 2}"
    );
    assert_eq!(
        pr_str(&Value::Map(Rc::new(vec![(Value::Int(1), Value::str("x"))]))),
        "{1 \"x\"}"
    );
}
