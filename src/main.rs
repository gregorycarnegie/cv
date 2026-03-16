mod components;

use components::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Nav/>
        <Hero/>
        <Skills/>
        <SectionDivider/>
        <Projects/>
        <SectionDivider/>
        <Experience/>
        <SectionDivider/>
        <Education/>
        <SectionDivider/>
        <Interests/>
        <Footer/>
    }
}

#[component]
fn SectionDivider() -> impl IntoView {
    view! {
        <div class="section-divider"></div>
    }
}
