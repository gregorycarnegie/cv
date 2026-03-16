use leptos::prelude::*;

struct Job {
    dates: (&'static str, &'static str),
    company: &'static str,
    role: &'static str,
    summary: &'static str,
    bullets: &'static [&'static str],
}

const JOBS: &[Job] = &[
    Job {
        dates: ("Mar 2019", "Present"),
        company: "Harris Westminster\nSixth Form",
        role: "IT Technician",
        summary: "Senior technician responsible for end-to-end IT operations at a prestigious sixth form college, supporting 600\u{2013}700 staff and students across hardware, software, identity systems, and third-party integrations.",
        bullets: &[
            "Manage Active Directory, Azure AD/Entra ID, and ticket systems (SMC & Halo) \u{2014} overseeing user onboarding and access control across all systems.",
            "Provide hardware and software support for 600\u{2013}700 users including MS365, access control, and third-party applications.",
            "Developed a Windows desktop application in Python to automate the cropping of student passport photos for ID badge production.",
            "Led the deployment of SchoolsBuddy activity sign-up system and oversaw the migration from Biostore to InVentry for visitor management.",
            "Integrated the Paxton access control system with InVentry to enhance physical security.",
            "Conducted hardware upgrades on user PCs and servers; maintained software and hardware inventory audits.",
            "Managed SharePoint access permissions and collaborated with third-party suppliers on system integrations including Wonde and Bromcom.",
            "Produced user-facing guides and posters for Wi-Fi connectivity and digital signage.",
        ],
    },
    Job {
        dates: ("Jul 2018", "Dec 2018"),
        company: "Fidessa Group PLC",
        role: "IT Support Engineer",
        summary: "Office IT support role at a global financial software company, handling security incidents, account management, and mobile device setup.",
        bullets: &[
            "Identified and escalated a critical BitLocker SSD encryption vulnerability, preventing a potential data security breach.",
            "Managed account offboarding processes to ensure secure and compliant departures.",
            "Created email distribution groups and managed shared mailbox permissions via the Exchange Admin Centre.",
            "Assisted users with BlackBerry Work configuration on BYOD devices for secure mobile access.",
        ],
    },
    Job {
        dates: ("Nov 2014", "Jul 2018"),
        company: "St Thomas More\nCatholic School",
        role: "Teaching Assistant",
        summary: "Provided classroom support and developed bespoke internal tools to improve administrative efficiency and student outcomes in Maths, Science, and English.",
        bullets: &[
            "Built a Microsoft Access database to automate student behaviour tracking and report card printing, with accompanying video training materials for staff.",
            "Created a library of differentiated PowerPoint lessons and worksheets across multiple subjects.",
            "Managed classroom behaviour for students with additional needs, fostering a positive and inclusive learning environment.",
        ],
    },
    Job {
        dates: ("Sep 2012", "Jan 2020"),
        company: "Global Tutors\n& Coaches",
        role: "IT Support \u{00b7} Tutor \u{00b7} Administrator",
        summary: "Multi-role tenure spanning administration, tutoring, and IT support for an education services company.",
        bullets: &[
            "Tutored students from primary through A-Level in Mathematics, Further Mathematics, and Physics.",
            "Built Google Sheets automation with macros to streamline invoicing workflows.",
            "Created MS Excel templates for payslips, invoicing, and attendance tracking; set up OneDrive for cloud document management.",
            "Expanded the company\u{2019}s online visibility by establishing a Google Maps presence and maintaining the company website.",
        ],
    },
];

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <section id="experience">
            <p class="section-label reveal">"Career History"</p>
            <h2 class="section-title reveal">
                "Professional"<br/>"Experience"
            </h2>

            <div class="experience-list">
                {JOBS.iter().map(|job| {
                    let company_lines: Vec<&str> = job.company.split('\n').collect();
                    view! {
                        <div class="exp-item reveal">
                            <div class="exp-meta">
                                <div class="exp-dates">
                                    {job.dates.0}<br/>{"\u{2014} "}{job.dates.1}
                                </div>
                                <div class="exp-company" inner_html={
                                    company_lines.join("<br/>")
                                }></div>
                            </div>
                            <div class="exp-content">
                                <div class="exp-role">{job.role}</div>
                                <p class="exp-summary">{job.summary}</p>
                                <ul class="exp-bullets">
                                    {job.bullets.iter().map(|b| view! {
                                        <li>{*b}</li>
                                    }).collect_view()}
                                </ul>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
