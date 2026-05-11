use dioxus::prelude::*;
use components::{Button, Dialog, Input, Switch};

#[component]
fn ShipDialog() -> Element {
    let mut email = use_signal(String::new);

    rsx! {
        Dialog { title: "Invite a teammate",
            Input { value: email, placeholder: "name@team.dev" }
            Switch { label: "Notify on accept" }
            Button { onclick: move |_| invite(email()), "Send →" }
        }
    }
}
