use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="hero">
            <div class="hero-text">
                <p class="hero-tag">"Available for new opportunities"</p>
                <h1 class="hero-name">
                    "Gregory"<br/><span>"Carnegie"</span>
                </h1>
                <p class="hero-title">"IT Technician & Developer"</p>
                <p class="hero-desc">
                    "Experienced IT professional with 6+ years in infrastructure support, \
                     systems administration, and software development. Specialising in Windows \
                     environments, Active Directory, and building custom tools that solve real \
                     operational problems."
                </p>
                <div class="hero-actions">
                    <a href="#skills" class="btn">"View Skills"</a>
                    <a href="#experience" class="btn btn-ghost">"View Experience"</a>
                </div>
            </div>

            <div class="hero-stats">
                <div class="stat-item">
                    <div class="stat-num">"6+"</div>
                    <div class="stat-label">"Years in IT"</div>
                </div>
                <div class="stat-divider"></div>
                <div class="stat-item">
                    <div class="stat-num">"700"</div>
                    <div class="stat-label">"Users Supported"</div>
                </div>
                <div class="stat-divider"></div>
                <div class="stat-item">
                    <div class="stat-num">"BSc"</div>
                    <div class="stat-label">"Physics, Nottingham"</div>
                </div>
            </div>
        </div>
    }
}
