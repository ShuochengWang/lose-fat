use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let (y, set_y) = create_signal(0);
    let double_count = move || count() * 2;
    let values = vec![0, 1, 2];
    let counters = (1..=5).map(|idx| create_signal(idx));
    let counter_buttons = counters.map(|(cnt, set_cnt)| {
        view! {
            <li>
                <button on:click=move |_| set_cnt.update(|x| *x += 1)>{cnt}</button>
            </li>
        }
    }).collect_view();

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

        <p>{values.clone()}</p>
        <ul>{values.clone().into_iter().map(|v| view! { <li>{v}</li> }).collect::<Vec<_>>()}</ul>
        <ul>{values.clone().into_iter().map(|v| view! { <li>{v}</li> }).collect_view()}</ul>

        <ul>{counter_buttons}</ul>

        <StaticList length=5/>
        <DynamicList init_length=5/>
    }
}

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    #[prop(default = 100)]
    max: u16, 
    progress: F) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

#[component]
fn StaticList(length: usize) -> impl IntoView {
    let counters = (1..=length).map(|idx| create_signal(idx));
    let counter_buttons = counters.into_iter().map(|(cnt, set_cnt)| {
        view! {
            <li>
                <button on:click=move |_| { set_cnt.update(|n| *n += 1) }>{cnt}</button>
            </li>
        }
    }).collect_view();

    view! { <ul>{counter_buttons}</ul> }
}

#[component]
fn DynamicList(init_length: usize) -> impl IntoView {
    let mut next_key = init_length;
    let init_counters = (0..init_length).map(|key| (key, create_signal(key+1))).collect::<Vec<_>>();
    let (counters, set_counters) = create_signal(init_counters);
    let add_counter = move |_| {
        let signal = create_signal(next_key + 1);
        set_counters.update(move |counters| {
            counters.push((next_key, signal));
        });
        next_key += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add button"</button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    children=move |(key, (cnt, set_cnt))| {
                        view! {
                            <li>
                                <button on:click=move |_| set_cnt.update(|n| *n += 1)>{cnt}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .update(|counters| {
                                            counters.retain(|(id, _)| id != &key)
                                        });
                                }>"remove button"</button>
                            </li>
                        }
                    }
                />

            </ul>
        </div>
    }
}

fn main() {
    mount_to_body(App)
}
