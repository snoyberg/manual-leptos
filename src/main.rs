use leptos::*;

fn main() {
    mount_to_body(app)
}

fn app() -> impl IntoView {
    let (name, set_name) = create_signal(None);
    let make_button = move |name: &str| {
        let name = name.to_owned();
        html::button()
            .child(format!("Say hi to {name}!"))
            .on(ev::click, move |_| set_name(Some(name.clone())))
            .into_view()
    };
    [
        say_hi(name).into_view(),
        make_button("Alice"),
        make_button("Bob"),
    ]
}

fn say_hi(name: impl Into<Signal<Option<String>>>) -> impl IntoView {
    let name = name.into();
    let is_set = create_memo(move |_| name().is_some());
    move || {
        if is_set() {
            html::p().child("Hello, ").child(html::b().child(name))
        } else {
            html::p().child("No name selected")
        }
    }
}
