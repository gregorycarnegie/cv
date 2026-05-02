pub const ASCII: &str = "
   \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2557}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2557}       \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2557}\u{2588}\u{2588}\u{2557}   \u{2588}\u{2588}\u{2557}
  \u{2588}\u{2588}\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d} \u{2588}\u{2588}\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}      \u{2588}\u{2588}\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}\u{2588}\u{2588}\u{2551}   \u{2588}\u{2588}\u{2551}
  \u{2588}\u{2588}\u{2551}  \u{2588}\u{2588}\u{2588}\u{2557}\u{2588}\u{2588}\u{2551}     \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2557}\u{2588}\u{2588}\u{2551}     \u{2588}\u{2588}\u{2551}   \u{2588}\u{2588}\u{2551}
  \u{2588}\u{2588}\u{2551}   \u{2588}\u{2588}\u{2551}\u{2588}\u{2588}\u{2551}     \u{2554}\u{2550}\u{2550}\u{2550}\u{255d}\u{2588}\u{2588}\u{2551}     \u{255a}\u{2588}\u{2588}\u{2557} \u{2588}\u{2588}\u{2554}\u{255d}
  \u{255a}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2554}\u{255d}\u{255a}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2557}      \u{255a}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2557} \u{255a}\u{2588}\u{2588}\u{2588}\u{2588}\u{2554}\u{255d}
   \u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}  \u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}       \u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}  \u{255a}\u{2550}\u{2550}\u{2550}\u{255d}
";

pub const COMMANDS: &[&str] = &[
    "about",
    "clear",
    "contact",
    "date",
    "exit",
    "experience",
    "help",
    "ls",
    "neofetch",
    "now",
    "projects",
    "pwd",
    "skills",
    "social",
    "sudo",
    "whoami",
];

pub const ALIASES: &[(&str, &str)] = &[
    ("?", "help"),
    ("cv", "experience"),
    ("h", "help"),
    ("me", "about"),
    ("resume", "experience"),
    ("work", "projects"),
];

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn run_command(raw_name: &str) -> String {
    let name = resolve_alias(raw_name);
    match name.as_str() {
        "help" | "ls" => help(),
        "about" => about(),
        "skills" => skills(),
        "projects" => projects(),
        "experience" => experience(),
        "contact" => contact(),
        "neofetch" => neofetch(),
        "social" => social(),
        "now" => now(),
        "whoami" => whoami(),
        "sudo" => sudo(),
        "date" => date_cmd(),
        "pwd" => pwd(),
        "exit" => exit_cmd(),
        _ => format!(
            r#"<span class="pink">command not found:</span> {}. Try <span class="amber">help</span>."#,
            esc(raw_name)
        ),
    }
}

fn resolve_alias(name: &str) -> String {
    ALIASES
        .iter()
        .find(|(a, _)| *a == name)
        .map(|(_, b)| b.to_string())
        .unwrap_or_else(|| name.to_string())
}

fn help() -> String {
    r#"<span class="amber bold">Available commands:</span>
<dl class="cmds">
  <dt>about</dt><dd>who I am and what I do</dd>
  <dt>skills</dt><dd>languages, frameworks, infrastructure</dd>
  <dt>projects</dt><dd>selected work with links</dd>
  <dt>experience</dt><dd>career history</dd>
  <dt>contact</dt><dd>how to reach me</dd>
  <dt>neofetch</dt><dd>the obligatory system info</dd>
  <dt>social</dt><dd>github, linkedin, etc.</dd>
  <dt>now</dt><dd>what I'm working on right now</dd>
  <dt>whoami</dt><dd>the short answer</dd>
  <dt>clear</dt><dd>clear the screen</dd>
  <dt>sudo</dt><dd>try me</dd>
</dl>
<p class="muted">Tip: press Tab to autocomplete or tap a button above.</p>"#
        .to_string()
}

fn about() -> String {
    r#"<div class="block">
<h3>About</h3>
<p>I'm <span class="bold">Gregory Carnegie</span> — an IT technician at Harris Westminster Sixth Form in London by day, a Rust systems engineer most evenings. I have a Physics degree from Nottingham and about six years keeping Windows fleets, identity systems and users happy.</p>
<p>The day job is broad: Active Directory, Entra ID, SCCM, Group Policy, Intune, M365, device management, AV, and the 600 or so humans who rely on all of it. The night job is narrower and nerdier: Rust, GPU compute, Polars, wgpu, and making tools that feel instant.</p>
<p>I like fast software, careful abstractions, powerlifting, and the occasional live-coded music set in Strudel.</p>
</div>"#
        .to_string()
}

fn skills() -> String {
    r#"<div class="block">
<h3>Skills</h3>
<h4 class="cyan">Languages</h4>
<ul>
  <li><span class="amber">Rust</span> — primary; Axum, Polars, wgpu, Tokio, Maud, egui, tract</li>
  <li><span class="amber">Python</span> — MCP servers, data pipelines, desktop apps</li>
  <li><span class="amber">PowerShell</span> — daily for Windows automation</li>
  <li>TypeScript / JavaScript · C · WGSL</li>
</ul>
<h4 class="cyan">Graphics &amp; GPU</h4>
<ul>
  <li>wgpu / WGSL compute shaders</li>
  <li>Three.js, WebGL, Plotly WebGL</li>
  <li>ONNX Runtime, cross-platform GPU inference</li>
</ul>
<h4 class="cyan">Microsoft / IT</h4>
<ul>
  <li>Active Directory, Entra ID, SCCM, Intune</li>
  <li>Group Policy, M365 admin, Exchange, SharePoint</li>
  <li>Copilot Studio agent design</li>
</ul>
<h4 class="cyan">Data</h4>
<ul>
  <li>Polars, DuckDB, SQLite, Parquet</li>
  <li>Power Query, Excel at an unreasonable level</li>
</ul>
<h4 class="cyan">Platforms &amp; Web</h4>
<ul>
  <li>Linux (Ubuntu / Fedora daily)</li>
  <li>WebAssembly, Axum, Maud templating</li>
  <li>MCP protocol, multi-source agents</li>
</ul>
</div>"#
        .to_string()
}

fn projects() -> String {
    r#"<div class="block">
<h3>Projects</h3>
<div class="plist">
  <div class="row"><span class="idx">01</span><span class="name">Iron Insights</span><span class="desc">Powerlifting analytics on Rust/Axum/Polars/DuckDB, 2.2M+ records, real-time rankings</span><span class="tags">rust · wasm · duckdb</span></div>
  <div class="row"><span class="idx">02</span><span class="name">Face Crop Studio</span><span class="desc">GPU-accelerated batch face detection in Rust using wgpu/WGSL and YuNet ONNX</span><span class="tags">rust · wgpu · onnx</span></div>
  <div class="row"><span class="idx">03</span><span class="name">Claude Job Agent</span><span class="desc">MCP server for conversational UK job search via Adzuna &amp; Reed APIs</span><span class="tags">python · mcp · sqlite</span></div>
  <div class="row"><span class="idx">04</span><span class="name">3D Plate Calculator</span><span class="desc">Single-file HTML/Three.js app rendering any barbell loadout in 3D</span><span class="tags">three.js · webgl</span></div>
  <div class="row"><span class="idx">05</span><span class="name">RustPad</span><span class="desc">Cross-platform rich text editor in Rust + egui — a WordPad replacement</span><span class="tags">rust · egui</span></div>
  <div class="row"><span class="idx">06</span><span class="name">School automation</span><span class="desc">PowerShell AD bulk tools, Entra CSV pipelines, SCCM, Copilot Studio agents, AV docs site</span><span class="tags">powershell · entra · sccm</span></div>
</div>
<p class="muted">› github.com/gregorycarnegie — public repos and more.</p>
</div>"#
        .to_string()
}

fn experience() -> String {
    r#"<div class="block">
<h3>Experience</h3>
<h4 class="cyan">2019 — present · IT Technician</h4>
<p class="muted">Harris Westminster Sixth Form · London</p>
<ul>
  <li>Support 600+ users across hardware, M365, access control, third-party apps</li>
  <li>AD / Entra ID / SCCM / Group Policy / Intune daily</li>
  <li>Python and Rust tooling in production use by ops staff</li>
  <li>Led Biostore → InVentry visitor management migration + Paxton integration</li>
  <li>Authored PowerShell bulk AD management and fleet-monitoring scripts</li>
</ul>
<h4 class="cyan">2018 · IT Support Engineer</h4>
<p class="muted">Fidessa Group PLC · London</p>
<ul>
  <li>Flagged and escalated a BitLocker / SSD encryption security issue</li>
  <li>Off-boarding, shared mailbox permissions, Exchange admin centre</li>
  <li>BlackBerry Work MDM rollout on BYOD devices</li>
</ul>
<h4 class="cyan">2014 — 2018 · Teaching Assistant</h4>
<p class="muted">St Thomas More Catholic School</p>
<ul>
  <li>Built an MS Access database for behaviour tracking and report-card generation</li>
  <li>Authored differentiated lesson resources across English, Maths, Science</li>
</ul>
<h4 class="cyan">2008 — 2011 · BSc (Hons) Physics</h4>
<p class="muted">University of Nottingham</p>
<ul>
  <li>Where the numerics habit started.</li>
</ul>
</div>"#
        .to_string()
}

fn contact() -> String {
    r#"<div class="block">
<h3>Contact</h3>
<dl class="kv">
  <dt>email</dt><dd><a href="mailto:gregory.carnegie@live.com">gregory.carnegie@live.com</a></dd>
  <dt>phone</dt><dd><a href="tel:+447411852611">+44 7411 852 611</a></dd>
  <dt>github</dt><dd><a href="https://github.com/gregorycarnegie" target="_blank" rel="noopener">github.com/gregorycarnegie</a></dd>
  <dt>linkedin</dt><dd><a href="https://linkedin.com/in/gregorycarnegie" target="_blank" rel="noopener">linkedin.com/in/gregorycarnegie</a></dd>
  <dt>location</dt><dd>London, E10 · United Kingdom</dd>
  <dt>status</dt><dd><span class="green">●</span> open to work — Rust, infra, dev-tools</dd>
</dl>
</div>"#
        .to_string()
}

fn social() -> String {
    r#"<div class="block">
<h3>Find me</h3>
<ul>
  <li>› <a href="https://github.com/gregorycarnegie" target="_blank" rel="noopener">github.com/gregorycarnegie</a></li>
  <li>› <a href="https://linkedin.com/in/gregorycarnegie" target="_blank" rel="noopener">linkedin.com/in/gregorycarnegie</a></li>
  <li>› <a href="mailto:gregory.carnegie@live.com">gregory.carnegie@live.com</a></li>
</ul>
</div>"#
        .to_string()
}

pub fn neofetch() -> String {
    format!(
        r#"<pre class="ascii">{}</pre>
<dl class="kv">
  <dt class="amber">user</dt><dd>gregory@carnegie</dd>
  <dt class="amber">os</dt><dd>Human 1.0 · London build</dd>
  <dt class="amber">host</dt><dd>Harris Westminster Sixth Form</dd>
  <dt class="amber">kernel</dt><dd>rustc 1.86.0 (stable)</dd>
  <dt class="amber">uptime</dt><dd>6+ years in IT</dd>
  <dt class="amber">packages</dt><dd>Axum · Polars · wgpu · DuckDB · Tokio</dd>
  <dt class="amber">shell</dt><dd>PowerShell · zsh</dd>
  <dt class="amber">cpu</dt><dd>Physics, Nottingham (Class of 2011)</dd>
  <dt class="amber">gpu</dt><dd>wgpu · WGSL · Three.js</dd>
  <dt class="amber">memory</dt><dd>loaded with powerlifting trivia</dd>
  <dt class="amber">disk</dt><dd>∞ / Parquet</dd>
</dl>"#,
        ASCII
    )
}

fn now() -> String {
    r#"<div class="block">
<h3>Currently</h3>
<ul>
  <li>Pushing <span class="amber">iron_insights2</span> — new UI pass, modular Rust refactor, DOTS scoring polish</li>
  <li>Marketing <span class="amber">Face Crop Studio</span> on itch.io and facecropstudio.com</li>
  <li>Exploring multi-AI CLI orchestration and MCP patterns</li>
  <li>Deadlifting. Probably.</li>
</ul>
</div>"#
        .to_string()
}

fn whoami() -> String {
    r#"<span class="amber">gregory</span> — IT technician / Rust dev / London"#.to_string()
}

fn sudo() -> String {
    r#"<span class="pink">[sudo] password for gregory:</span> <span class="muted">***</span>
<span class="muted">Sorry, user gregory is not in the sudoers file. This incident will be reported.</span>
<span class="green">(just kidding — you can <a href="mailto:gregory.carnegie@live.com">email me</a> instead)</span>"#
        .to_string()
}

fn date_cmd() -> String {
    let date = js_sys::Date::new_0();
    format!(
        r#"<span class="muted">{}</span>"#,
        date.to_utc_string().as_string().unwrap_or_default()
    )
}

fn pwd() -> String {
    r#"<span class="purple">/home/gregory/portfolio</span>"#.to_string()
}

fn exit_cmd() -> String {
    r#"<span class="muted">logout</span>
<span class="amber">Connection to carnegie closed.</span>"#
        .to_string()
}
