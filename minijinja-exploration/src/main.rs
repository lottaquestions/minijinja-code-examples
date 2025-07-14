use minijinja::State;
use minijinja::value::{Enumerator, Object, Value};
use minijinja::{Environment, context};
use std::fmt::format;
use std::io::stdout;
use std::{collections::HashSet, sync::Arc};

fn test_template_usage() {
    let mut env = Environment::new();
    env.add_template("hello", "Hello {{ name }}!").unwrap();
    let tmpl = env.get_template("hello").unwrap();
    println!("{}", tmpl.render(context!(name => "John")).unwrap());
}

fn test_expression_usage() {
    let env = Environment::new();
    let expr = env.compile_expression("number < 42").unwrap();
    let result = expr.eval(context! (number => 23)).unwrap();
    assert_eq!(result.is_true(), true);
}

// Dynamic objects
#[derive(Debug)]
struct Point(f32, f32, f32);

impl Object for Point {
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "x" => Some(Value::from(self.0)),
            "y" => Some(Value::from(self.1)),
            "z" => Some(Value::from(self.2)),
            _ => None,
        }
    }

    fn enumerate(self: &Arc<Self>) -> Enumerator {
        Enumerator::Str(&["x", "y", "z"])
    }
}

fn test_dynamic_objects() {
    let value = Value::from_object(Point(1.0, 2.5, 3.0));
    if let Some(object) = value.as_object() {
        assert_eq!(object.get_value(&Value::from("x")), Some(Value::from(1.0)));
        assert_eq!(object.get_value(&Value::from("y")), Some(Value::from(2.5)));
        assert_eq!(object.get_value(&Value::from("z")), Some(Value::from(3.0)));
    }
}

// Custom filters
fn test_custom_filters() {
    let mut env = Environment::new();
    env.add_filter("repeat", str::repeat);
    env.add_template("hello", "{{ 'Na ' | repeat(3) }} {{ name }}!")
        .unwrap();
    let tmpl = env.get_template("hello").unwrap();
    println!("{}", tmpl.render(context! (name => "Batman")).unwrap());
}

// Environment related
// How to iterate through templates in an environment
fn test_templates_iteration() {
    let mut env = Environment::new();
    env.add_template("hello.txt", "Hello {{ name }}!").unwrap();
    env.add_template("goodbye.txt", "Goodbye {{ name }}!")
        .unwrap();

    for (name, tmpl) in env.templates() {
        println!(
            "The template {} renders to {}",
            name,
            tmpl.render(context! {name => "World"}).unwrap()
        );
    }
}

// Getting a template by name
fn test_get_template_by_name() {
    let mut env = Environment::new();
    env.add_template("hello.txt", "Hello {{ name }} !").unwrap();
    let tmpl = env.get_template("hello.txt").unwrap();
    println!("{}", tmpl.render(context! { name => "World" }).unwrap());
}

// Loading a template from a string
fn test_loading_template_from_a_string() {
    let env = Environment::new();
    let tmpl = env
        .template_from_named_str("template_name", "Hello {{ name }}")
        .unwrap();
    let rv = tmpl.render(context! {name => "World"});
    println!("{}", rv.unwrap());
}

// Parsing and rendering a template from a string in one go
fn test_parse_and_render_from_string_in_one_go() {
    let env = Environment::new();
    let rv = env.render_named_str(
        "template_name",
        "Hello {{ name }}",
        context! { name => "World" },
    );
    println!("{}", rv.unwrap());
}

// Rendering and returning the evaluated state
fn test_render_and_return_evaluated_state() {
    let env = Environment::new();
    let tmpl = env
        .template_from_str("{% set x = 42 %}Hello {{ what }}!")
        .unwrap();
    let (rv, state) = tmpl
        .render_and_return_state(context! { what => "World"})
        .unwrap();
    assert_eq!(rv, "Hello World!");
    assert_eq!(state.lookup("x"), Some(Value::from(42)));

    // Render and send output to stdout
    tmpl.render_to_write(context! { what => "John"}, &mut stdout())
        .unwrap();
}

// Discard output and return internal state
fn test_discard_output_and_return_internal_state() {
    let mut env = Environment::new();
    env.add_template("hello", "Hello {{ name }}!").unwrap();
    let tmpl = env.get_template("hello").unwrap();
    let state = tmpl.eval_to_state(context! { name => "John"}).unwrap();
    println!("{:?}", state.exports());
}

// Returning undeclared variables in the template
fn test_return_undeclared_variables() {
    let mut env = Environment::new();
    env.add_template("x", "{% set x = foo %}{{ x }}{{ bar.baz }}")
        .unwrap();
    let tmpl = env.get_template("x").unwrap();
    let undeclared = tmpl.undeclared_variables(false);
    assert_eq!(
        undeclared,
        HashSet::from(["foo".to_string(), "bar".to_string()])
    );
    let undeclared = tmpl.undeclared_variables(true);
    assert_eq!(
        undeclared,
        HashSet::from(["foo".to_string(), "bar.baz".to_string()])
    );
}

// Custom filters
fn slugify(value: String) -> String {
    value
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

fn append_template(state: &State, value: &Value) -> String {
    format!("{}-{}", value, state.name())
}

fn test_custom_filters_example1_slugify() {
    let mut env = Environment::new();
    env.add_filter("slugify", slugify);

    env.add_template("hello", "Hello {{ name | slugify }}!")
        .unwrap();
    let tmpl = env.get_template("hello").unwrap();
    println!(
        "{}",
        tmpl.render(context!(name => "John Wild Oak")).unwrap()
    );

    env.add_filter("append_template", append_template);
    env.add_template("state_of_the_template", "{{ name | append_template }}")
        .unwrap();
    let tmpl = env.get_template("state_of_the_template").unwrap();
    println!(
        "{}",
        tmpl.render(context!(name => "John Wild Oak")).unwrap()
    );
}


fn main() {
    test_template_usage();
    test_expression_usage();
    test_dynamic_objects();
    test_custom_filters();
    test_templates_iteration();
    test_get_template_by_name();
    test_loading_template_from_a_string();
    test_parse_and_render_from_string_in_one_go();
    test_render_and_return_evaluated_state();
    test_discard_output_and_return_internal_state();
    test_return_undeclared_variables();
    test_custom_filters_example1_slugify();
}
