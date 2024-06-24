use leptos::*;

fn main() {
    mount_to_body(app)
}

fn app() -> impl IntoView {
    let (name, set_name) = create_signal(None);
    view! {
        {say_hi(name)}
        <button on:click=move |_| set_name(Some("Alice".to_owned()))>Say hi to Alice!</button>
        <button on:click=move |_| set_name(Some("Bob".to_owned()))>Say hi to Bob!</button>
    }
}

fn say_hi(name: impl Into<Signal<Option<String>>>) -> impl IntoView {
    let name = name.into();
    let is_set = create_memo(move |_| name().is_some());
    move || {
        if is_set() {
            view! { <p>Hello, <b>{name}</b></p> }
        } else {
            view! { <p>No name selected</p> }
        }
    }
}
