use leptos::*;

fn main() {
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal("Alice".to_owned());
    let change_name = move |_| set_name("Bob".to_owned());
    view! {
        <SayHi name={name()} />
        <button on:click=change_name>Say hi to Bob!</button>
    }
}

#[component]
fn SayHi(name: String) -> impl IntoView {
    view! { <p>Hello, <b>{name}</b></p> }
}
