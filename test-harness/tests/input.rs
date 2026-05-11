//! Port of `playwright/input.spec.ts`. The "Input" in that spec is just a thin wrapper around
//! a native `<input>` whose `oninput` mirrors the value to a sibling element. We test the same
//! flow directly with a `<input>`.

use dioxus::prelude::*;
use dioxus_test::{by_testid, eq, render, text};

#[tokio::test]
async fn typing_updates_a_mirrored_signal() {
    let mut tester = render(|| {
        let mut name = use_signal(String::new);
        rsx! {
            input {
                "data-testid": "input",
                placeholder: "Enter your name",
                value: name,
                oninput: move |e: FormEvent| name.set(e.value()),
            }
            if !name.read().is_empty() {
                p { "data-testid": "greeting", "Hello, {name}!" }
            }
        }
    })
    .build();

    tester.query(by_testid("input")).input("name").await.unwrap();

    tester
        .query(by_testid("greeting"))
        .expect(text(eq("Hello, name!")))
        .await
        .unwrap();
}

#[tokio::test]
async fn empty_input_hides_greeting() {
    let mut tester = render(|| {
        let mut name = use_signal(String::new);
        rsx! {
            input {
                "data-testid": "input",
                value: name,
                oninput: move |e: FormEvent| name.set(e.value()),
            }
            if !name.read().is_empty() {
                p { "data-testid": "greeting", "Hello, {name}!" }
            }
        }
    })
    .build();

    // Greeting absent at start, present after typing, absent after clearing.
    assert!(tester.query(by_testid("greeting")).immediately().is_err());

    let mut input = tester.query(by_testid("input"));
    input.input("name").await.unwrap();
    tester.query(by_testid("greeting")).await.unwrap();

    let mut input = tester.query(by_testid("input"));
    input.input("").await.unwrap();
    assert!(tester.query(by_testid("greeting")).immediately().is_err());
}
