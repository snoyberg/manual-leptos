use leptos::*;

fn main() {
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal(None);
    view! {
        <SayHi name />
        <button on:click=move |_| set_name(Some("Alice".to_owned()))>Say hi to Alice!</button>
        <button on:click=move |_| set_name(Some("Bob".to_owned()))>Say hi to Bob!</button>
    }
}

#[component]
fn SayHi(#[prop(into)] name: Signal<Option<String>>) -> impl IntoView {
    view! {
        <Show
            when=move || name().is_some()
            fallback=|| view! { <p>No name selected</p> }
        >
            <p>Hello, <b>{name}</b></p>
        </Show>
    }
}
