#![allow(non_snake_case)]

use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

#[component]
fn App() -> impl IntoView {
    view! {
        <p>"Hello world!"</p>
        <script type="module">"import init from './bevy-breakout.js'; init();"</script>
    }
}
