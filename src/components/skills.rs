use leptos::prelude::*;

struct Skill {
    name: &'static str,
    desc: &'static str,
}

const SKILLS: &[Skill] = &[
    Skill {
        name: "Active Directory & Entra ID",
        desc: "User lifecycle management, batch operations, group policy administration, MFA configuration, and Entra ID / Azure AD hybrid environments.",
    },
    Skill {
        name: "PowerShell",
        desc: "Scripting automation for AD management, network monitoring, device deployment, and report generation across Windows infrastructure.",
    },
    Skill {
        name: "Python & Rust",
        desc: "Advanced Python 3 certified via Codecademy. Building production desktop apps and data tooling in Rust. Author of multiple published projects.",
    },
    Skill {
        name: "SCCM / Device Deployment",
        desc: "Imaging and deploying Windows machines at scale, software packaging, and endpoint management across a multi-device estate.",
    },
    Skill {
        name: "Azure & Microsoft 365",
        desc: "SharePoint access control, Exchange Online, Teams administration, and managing Microsoft 365 tenants for educational institutions.",
    },
    Skill {
        name: "Networking & Security",
        desc: "Network monitoring, access control systems (Paxton/InVentry), BitLocker encryption, BYOD mobile device management, VPS administration.",
    },
    Skill {
        name: "Linux",
        desc: "Daily driver on Ubuntu and Fedora. Comfortable with SSH, PuTTY, VPS setup, and command-line administration.",
    },
    Skill {
        name: "Excel & Data Analysis",
        desc: "Pivot tables, Power Query, XLOOKUP, and macro development. Designed reporting solutions used across multiple organisations.",
    },
    Skill {
        name: "Hardware & Infrastructure",
        desc: "PC building, server hardware upgrades, component-level diagnosis, and managing physical infrastructure across a sixth form college.",
    },
];

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <section id="skills">
            <p class="section-label reveal">"Technical Competencies"</p>
            <h2 class="section-title reveal">
                "What I Bring"<br/>"to the Table"
            </h2>

            <div class="skills-grid reveal">
                {SKILLS.iter().map(|skill| view! {
                    <div class="skill-card">
                        <div class="skill-name">{skill.name}</div>
                        <div class="skill-desc">{skill.desc}</div>
                    </div>
                }).collect_view()}
            </div>
        </section>
    }
}
