use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let (y, set_y) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }

            class:red=move || count() % 2 == 1
        >
            "click me: "
            {count}
        </button>

        <div
            style="position: absolute"
            style:left=move || format!("{}px", x() + 100)
            style:top=move || format!("{}px", y() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", 100 + x(), 100 + y())
            style=("--columns", x)
        >
            <button on:click=move |_| {
                set_x.update(|x| *x += 1);
            }>"Increase X"</button>
            <button on:click=move |_| {
                set_y.update(|y| *y += 1);
            }>"Increase Y"</button>
            "Move when coordinates change"
        </div>

        <ProgressBar progress=count/>
        <ProgressBar progress=double_count/>
        <p>"Double count: " {double_count}</p>
    }
}

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    #[prop(default = 100)]
    max: u16, 
    progress: F) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
