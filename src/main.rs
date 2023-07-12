use dioxus::prelude::*;

struct Interface(pub reqwest::Client);

impl Interface {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .cookie_store(true)
            .build()
            .unwrap();
        Self(client)
    }
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider::<Interface>(&cx, || Interface::new());
    let intf = use_shared_state::<Interface>(&cx).unwrap();
    let loading = use_state::<bool>(&cx, || false);
    let text = use_state::<String>(&cx, || String::new());

    let handle_click = move |_| {
        use tokio::time::{sleep, Duration};

        to_owned![intf, text, loading];
        cx.spawn(async move {
            let intf = &intf.read().0;
            for i in 1..100 {
                loading.set(true);
                let response_expect =
                    format!("Failed to get text http://127.0.0.1:8080/{}.txt", &i);
                let response = intf
                    .get(format!("http://127.0.0.1:8080/{}.txt", &i))
                    .send()
                    .await
                    .expect(&response_expect);
                let content = response.text().await.expect("Failed to parse text");
                text.set(content);
                loading.set(false);
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
            "{text}"
            loading_block
        }
    })
}

fn main() {
    dioxus_desktop::launch(app)
}
