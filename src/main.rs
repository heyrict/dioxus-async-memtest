use dioxus::prelude::*;
use tokio_stream::StreamExt;

#[inline_props]
fn number_display(cx: Scope, number: u32) -> Element {
    cx.render(rsx! {
        span {
            "{number}"
        }
    })
}

fn app(cx: Scope) -> Element {
    let number = use_state::<u32>(&cx, || 0);
    let loading = use_state::<bool>(&cx, || false);

    let handle_click = move |_| {
        use tokio::time::{sleep, Duration};

        to_owned![number, loading];
        cx.spawn(async move {
            let mut inner_stream = tokio_stream::iter(1..1000);
            while let Some(i) = inner_stream.next().await {
                loading.set(true);
                sleep(Duration::new(0, 50_000)).await;
                number.set(i);
                loading.set(false);
                sleep(Duration::new(0, 200_000)).await;
            }
        })
    };
    let loading_block = if *loading.get() {
        Some(rsx! { div{ "loading" } })
    } else {
        None
    };

    cx.render(rsx! {
        div {
            style: "width: 100%; height: 95vh; display: flex; flex-direction: column; align-items: space-between",
            button {
                onclick: handle_click,
                "Start fetch"
            }
            number_display {
                number: *number.get(),
            }
            loading_block
        }
    })
}

fn main() {
    dioxus_desktop::launch(app)
}
