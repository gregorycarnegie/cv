use leptos::prelude::*;

struct Project {
    name: &'static str,
    description: &'static str,
    url: &'static str,
    logo: &'static str,
    tech: &'static [&'static str],
}

const PROJECTS: &[Project] = &[
    Project {
        name: "Iron Insights",
        description: "A powerlifting analytics web app built with Leptos and WebAssembly. Track workouts, visualise progress, and analyse training data \u{2014} all running client-side in the browser.",
        url: "https://gregorycarnegie.github.io/iron_insights/",
        logo: "iron_insights_logo.svg",
        tech: &["Rust", "Leptos", "WASM"],
    },
    Project {
        name: "FaceCrop Studio",
        description: "A Windows desktop application that automates the cropping of student passport photos for ID badge production. Built with GPU-accelerated image processing to solve a real operational need at Harris Westminster Sixth Form.",
        url: "https://facecropstudio.com/",
        logo: "facecropstudio_logo.svg",
        tech: &["Rust", "wgsl", "yunet"],
    },
];

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section id="projects">
            <p class="section-label reveal">"Featured Work"</p>
            <h2 class="section-title reveal">
                "Personal"<br/>"Projects"
            </h2>

            <div class="projects-grid reveal">
                {PROJECTS.iter().map(|project| view! {
                    <a href={project.url} target="_blank" class="project-card">
                        <div class="project-logo">
                            <img src={project.logo} alt={project.name} />
                        </div>
                        <div class="project-info">
                            <h3 class="project-name">{project.name}</h3>
                            <p class="project-desc">{project.description}</p>
                            <div class="project-tech">
                                {project.tech.iter().map(|t| view! {
                                    <span class="tech-tag">{*t}</span>
                                }).collect_view()}
                            </div>
                        </div>
                    </a>
                }).collect_view()}
            </div>
        </section>
    }
}
