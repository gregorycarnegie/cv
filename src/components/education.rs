use leptos::prelude::*;

struct EduEntry {
    years: &'static str,
    institution: &'static str,
    qualification: &'static str,
}

const EDUCATION: &[EduEntry] = &[
    EduEntry {
        years: "2008 — 2011",
        institution: "The University of Nottingham",
        qualification: "BSc (Hons) Physics — 2:2",
    },
    EduEntry {
        years: "2006 — 2008",
        institution: "Sir George Monoux Sixth Form",
        qualification: "A-Level Mathematics (A) \u{00b7} Further Mathematics (C) \u{00b7} Physics (C)",
    },
    EduEntry {
        years: "2001 — 2006",
        institution: "The Lammas Secondary School",
        qualification: "9 GCSEs — Grades A to C",
    },
    EduEntry {
        years: "Certification",
        institution: "Codecademy",
        qualification: "Advanced Python 3",
    },
];

#[component]
pub fn Education() -> impl IntoView {
    view! {
        <section id="education">
            <p class="section-label reveal">"Academic Background"</p>
            <h2 class="section-title reveal">"Education"</h2>

            <div class="edu-grid reveal">
                {EDUCATION.iter().map(|edu| view! {
                    <div class="edu-card">
                        <div class="edu-years">{edu.years}</div>
                        <div class="edu-inst">{edu.institution}</div>
                        <div class="edu-qual">{edu.qualification}</div>
                    </div>
                }).collect_view()}
            </div>
        </section>
    }
}
