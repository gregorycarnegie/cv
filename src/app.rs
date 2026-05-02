use gloo_timers::future::sleep;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;

use crate::commands::{ALIASES, ASCII, COMMANDS, run_command};

#[derive(Clone)]
struct OutputItem {
    id: usize,
    make_view: SendWrapper<Rc<dyn Fn() -> AnyView>>,
}

fn push_view(
    items: RwSignal<Vec<OutputItem>>,
    counter: RwSignal<usize>,
    f: impl Fn() -> AnyView + 'static,
) {
    let id = counter.get_untracked();
    counter.update_untracked(|n| *n += 1);
    items.update(|v| {
        v.push(OutputItem {
            id,
            make_view: SendWrapper::new(Rc::new(f)),
        })
    });
}

fn prompt_view(cmd: String) -> AnyView {
    view! {
        <>
            <span class="green">"gregory"</span>
            <span class="muted">"@"</span>
            <span class="cyan">"carnegie"</span>
            <span class="muted">":"</span>
            <span class="purple">"~"</span>
            <span class="amber">"❯"</span>
            " "
            {cmd}
        </>
    }
    .into_any()
}

fn execute_cmd(
    raw: &str,
    items: RwSignal<Vec<OutputItem>>,
    counter: RwSignal<usize>,
    history: RwSignal<Vec<String>>,
    h_idx: RwSignal<i32>,
) {
    let cmd = raw.trim().to_string();
    if cmd.is_empty() {
        return;
    }

    let echo = cmd.clone();
    push_view(items, counter, move || prompt_view(echo.clone()));

    history.update(|h| h.insert(0, cmd.clone()));
    h_idx.set(-1);

    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let name = parts[0].to_lowercase();

    if name == "clear" {
        items.set(vec![]);
        counter.set(0);
        return;
    }
    if name.starts_with("rm") {
        push_view(items, counter, || {
            view! {
                <>
                    <span class="pink">"rm: permission denied"</span>
                    " "
                    <span class="muted">"(nice try)"</span>
                </>
            }
            .into_any()
        });
        return;
    }

    push_view(items, counter, move || run_command(&name));
}

#[component]
pub fn App() -> impl IntoView {
    let items: RwSignal<Vec<OutputItem>> = RwSignal::new(vec![]);
    let counter: RwSignal<usize> = RwSignal::new(0);
    let history: RwSignal<Vec<String>> = RwSignal::new(vec![]);
    let h_idx: RwSignal<i32> = RwSignal::new(-1);
    let input_ref = NodeRef::<leptos::html::Input>::new();
    let bottom_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        let _ = items.get();
        spawn_local(async move {
            if let Some(el) = bottom_ref.get_untracked() {
                el.scroll_into_view();
            }
        });
    });

    spawn_local(async move {
        push_view(items, counter, || {
            view! { <span class="muted">"Last login: Fri Apr 24 09:14:22 on ttys002"</span> }
                .into_any()
        });
        sleep(Duration::from_millis(80)).await;

        push_view(items, counter, || {
            view! { <span class="muted">"booting carnegie@portfolio v2.6.0 ..."</span> }.into_any()
        });
        sleep(Duration::from_millis(80)).await;

        push_view(items, counter, || {
            view! { <><span class="green">"✓"</span>" identity verified — gregory@carnegie"</> }
                .into_any()
        });
        sleep(Duration::from_millis(80)).await;

        push_view(items, counter, || {
            view! {
                <>
                    <span class="green">"✓"</span>
                    " rust toolchain · "
                    <span class="green">"✓"</span>
                    " gpu drivers · "
                    <span class="green">"✓"</span>
                    " coffee"
                </>
            }
            .into_any()
        });
        sleep(Duration::from_millis(220)).await;

        push_view(items, counter, || {
            view! {
                <>
                    <pre class="ascii">{ASCII}</pre>
                    <div class="line">
                        <span class="amber bold">"Welcome."</span>
                        " This is an interactive CV. Type "
                        <span class="amber">"help"</span>
                        " to get started, or use the buttons below."
                    </div>
                    <div class="line muted">
                        "Loaded · Rust · Python · PowerShell · wgpu · Axum · Polars · Maud · MCP · Entra ID · SCCM · Intune · Three.js · and too much else to list."
                    </div>
                </>
            }
            .into_any()
        });
        sleep(Duration::from_millis(400)).await;

        execute_cmd("whoami", items, counter, history, h_idx);
        sleep(Duration::from_millis(220)).await;

        execute_cmd("neofetch", items, counter, history, h_idx);

        if let Some(input) = input_ref.get_untracked() {
            let _ = input.focus();
        }
    });

    view! {
        <>
            <div class="topbar">
                <div class="dots">
                    <span class="dot r"></span>
                    <span class="dot y"></span>
                    <span class="dot g"></span>
                </div>
                <div class="tabs">
                    <span class="tab">"about — zsh"</span>
                    <span class="tab">"projects — zsh"</span>
                    <span class="tab">"contact — zsh"</span>
                </div>
                <div class="title"><b>"gregory@carnegie"</b>": ~ — zsh — 92×40"</div>
                <div style="width:60px"></div>
            </div>

            <div
                class="terminal"
                on:click=move |_| {
                    if let Some(input) = input_ref.get_untracked() {
                        let _ = input.focus();
                    }
                }
            >
                <div class="term-body">
                    <For
                        each=move || items.get()
                        key=|item| item.id
                        children=move |item| view! {
                            <div class="line">{(*item.make_view)()}</div>
                        }
                    />
                    <div node_ref=bottom_ref />
                </div>

                <form
                    class="prompt prompt-line"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        if let Some(input) = input_ref.get_untracked() {
                            let value = input.value();
                            execute_cmd(&value, items, counter, history, h_idx);
                            input.set_value("");
                        }
                    }
                >
                    <span class="user">"gregory"</span>
                    <span class="sep">"@"</span>
                    <span class="host">"carnegie"</span>
                    <span class="sep">":"</span>
                    <span class="path">"~"</span>
                    <span class="arrow">"❯"</span>
                    <input
                        name="cmd"
                        spellcheck="false"
                        autocapitalize="off"
                        autofocus
                        node_ref=input_ref
                        on:keydown=move |ev| {
                            match ev.key().as_str() {
                                "ArrowUp" => {
                                    ev.prevent_default();
                                    let hist = history.get_untracked();
                                    if !hist.is_empty() {
                                        let new_idx = (h_idx.get_untracked() + 1)
                                            .min(hist.len() as i32 - 1);
                                        h_idx.set(new_idx);
                                        if let Some(input) = input_ref.get_untracked() {
                                            let val = hist[new_idx as usize].clone();
                                            let len = val.len() as u32;
                                            input.set_value(&val);
                                            let _ = input.set_selection_range(len, len);
                                        }
                                    }
                                }
                                "ArrowDown" => {
                                    ev.prevent_default();
                                    let new_idx = h_idx.get_untracked() - 1;
                                    if new_idx >= 0 {
                                        h_idx.set(new_idx);
                                        let hist = history.get_untracked();
                                        if let Some(input) = input_ref.get_untracked() {
                                            input.set_value(&hist[new_idx as usize]);
                                        }
                                    } else {
                                        h_idx.set(-1);
                                        if let Some(input) = input_ref.get_untracked() {
                                            input.set_value("");
                                        }
                                    }
                                }
                                "Tab" => {
                                    ev.prevent_default();
                                    if let Some(input) = input_ref.get_untracked() {
                                        let raw = input.value();
                                        let q = raw.trim().to_lowercase();
                                        if q.is_empty() {
                                            return;
                                        }
                                        let all: Vec<&str> = COMMANDS
                                            .iter()
                                            .chain(ALIASES.iter().map(|(a, _)| a))
                                            .copied()
                                            .collect();
                                        let matches: Vec<&str> = all
                                            .into_iter()
                                            .filter(|k| k.starts_with(q.as_str()))
                                            .collect();
                                        if matches.len() == 1 {
                                            input.set_value(matches[0]);
                                        } else if matches.len() > 1 {
                                            let joined = matches.join("  ");
                                            push_view(items, counter, move || {
                                                let joined = joined.clone();
                                                view! { <span class="muted">{joined}</span> }
                                                    .into_any()
                                            });
                                        }
                                    }
                                }
                                "l" if ev.ctrl_key() || ev.meta_key() => {
                                    ev.prevent_default();
                                    items.set(vec![]);
                                    counter.set(0);
                                }
                                _ => {}
                            }
                        }
                    />
                </form>

                <div class="quickbar" aria-label="Quick commands">
                    {["help", "about", "skills", "projects", "experience", "contact", "neofetch", "clear"]
                        .into_iter()
                        .map(|cmd| {
                            let cmd_s = cmd.to_string();
                            view! {
                                <button
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        execute_cmd(&cmd_s, items, counter, history, h_idx);
                                        if let Some(input) = input_ref.get_untracked() {
                                            let _ = input.focus();
                                        }
                                    }
                                >
                                    {cmd}
                                </button>
                            }
                        })
                        .collect_view()
                    }
                </div>
            </div>

            <div class="hint">
                <span>
                    "Try: "
                    <span class="kbd">"help"</span>
                    " "
                    <span class="kbd">"about"</span>
                    " "
                    <span class="kbd">"projects"</span>
                    " "
                    <span class="kbd">"contact"</span>
                    " · "
                    <span class="kbd">"↑"</span>
                    <span class="kbd">"↓"</span>
                    " history · "
                    <span class="kbd">"Tab"</span>
                    " complete · "
                    <span class="kbd">"Ctrl"</span>
                    "+"
                    <span class="kbd">"L"</span>
                    " clear"
                </span>
                <span>"gregory.carnegie@live.com · london"</span>
            </div>
        </>
    }
}
