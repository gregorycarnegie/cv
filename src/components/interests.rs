use leptos::prelude::*;

struct Interest {
    icon: &'static str,
    label: &'static str,
}

const INTERESTS: &[Interest] = &[
    Interest {
        icon: "\u{1F5A5}\u{FE0F}",
        label: "DIY PC Building",
    },
    Interest {
        icon: "\u{1F980}",
        label: "Rust Programming",
    },
    Interest {
        icon: "\u{1F3CB}\u{FE0F}",
        label: "Powerlifting",
    },
    Interest {
        icon: "\u{1F9D7}",
        label: "Bouldering",
    },
    Interest {
        icon: "\u{1F3A8}",
        label: "Blender & 3D Art",
    },
    Interest {
        icon: "\u{1F52C}",
        label: "Photogrammetry",
    },
    Interest {
        icon: "\u{1F3B5}",
        label: "Live Coding Music",
    },
    Interest {
        icon: "\u{26A1}",
        label: "GPU Computing",
    },
];

#[component]
pub fn Interests() -> impl IntoView {
    view! {
        <section>
            <p class="section-label reveal">"Beyond the Desk"</p>
            <h2 class="section-title reveal">"Personal Interests"</h2>

            <div class="interests-grid reveal">
                {INTERESTS.iter().map(|interest| view! {
                    <div class="interest-pill">
                        <span class="interest-icon">{interest.icon}</span>
                        {" "}{interest.label}
                    </div>
                }).collect_view()}
            </div>
        </section>
    }
}
