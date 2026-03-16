use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="footer-name">"Gregory Carnegie \u{2014} IT Technician"</div>
            <div class="footer-note">"Open to opportunities"</div>
        </footer>
    }
}
