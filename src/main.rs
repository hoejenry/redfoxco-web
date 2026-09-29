use leptos::prelude::*;

const LOGO: &str = "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/vecteezy_fox-logo-design-free-template_29325468.HEIC.jpg";
const HERO_BG: &str = "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes04-1024x768.jpeg";

const BATHROOMS: &[(&str, &[&str])] = &[
    ("Master Bathroom Upgrade", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_1559-2-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_1556-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_1554-768x1024.jpg",
    ]),
    ("Basement Bathroom Refresh", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0303-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0430-768x1024.jpg",
    ]),
    ("Jack and Jill Bathroom", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0299-768x1024.jpg",
    ]),
    ("Luxury Suite Remodel", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes02-1024x768.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes03-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes04-1-1024x768.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes05-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes06-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes07-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes08-1024x768.jpeg",
    ]),
];

const DECKS: &[(&str, &[&str])] = &[
    ("Custom Backyard Oasis", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0536-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0535-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0533-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0532-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0531-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0508-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0503-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0506-768x1024.jpg",
    ]),
    ("Screened-In Deck", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_9425-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_9648-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_9647-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_9642-768x1024.jpg",
    ]),
    ("Updated Deck", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes22-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes20-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes24-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes21-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes23-768x1024.jpeg",
    ]),
];

const OFFICE: &[(&str, &[&str])] = &[
    ("Office Update", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_9397-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_9706-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_9705-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_9704-768x1024.jpg",
    ]),
    ("Hidden Office", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes09-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes10-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes11-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes12-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes13-768x1024.jpeg",
    ]),
    ("Custom Built-ins", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_2101.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_2102.jpg",
    ]),
];

const KITCHENS: &[(&str, &[&str])] = &[
    ("Kitchen Remodel", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_2033-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_2025-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_2034-768x1024.jpg",
    ]),
];

const BASEMENTS: &[(&str, &[&str])] = &[
    ("New Finished Basement", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0428-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0403-768x1024.jpg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/06/IMG_0427-768x1024.jpg",
    ]),
    ("Basement Remodel", &[
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes14-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes15-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes16-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes17-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes18-768x1024.jpeg",
        "https://www.redfoxcohomes.com/wp-content/uploads/2026/05/redfoxcohomes19-768x1024.jpeg",
    ]),
];

const CATEGORIES: &[(&str, &str, &[(&str, &[&str])])] = &[
    ("bathrooms", "Bathrooms", BATHROOMS),
    ("decks", "Decks & Patios", DECKS),
    ("office", "Home & Office", OFFICE),
    ("kitchens", "Kitchens", KITCHENS),
    ("basements", "Basements", BASEMENTS),
];

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let lightbox: RwSignal<Option<String>> = RwSignal::new(None);

    let _esc = window_event_listener(leptos::ev::keydown, move |e: leptos::ev::KeyboardEvent| {
        if e.key() == "Escape" {
            lightbox.set(None);
        }
    });

    view! {
        <Nav />
        <main>
            <Hero />
            <TrustBar />
            <Services />
            <Gallery lightbox />
            <Contact />
        </main>
        <Footer />
        {move || lightbox.get().map(|url| view! { <Lightbox url lightbox /> })}
    }
}

// ── NAV ─────────────────────────────────────────────────────────────────────

#[component]
fn Nav() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);
    let (gallery_open, set_gallery_open) = signal(false);

    view! {
        <nav class="nav">
            <a href="#home" class="nav-brand-link"
                on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                <img src=LOGO alt="Red Fox Co Homes logo" class="nav-logo" />
                <span class="nav-brand">"Red Fox Co Homes"</span>
            </a>

            <ul class="nav-links">
                <li><a href="#home" class="nav-link">"Home"</a></li>
                <li><a href="#services" class="nav-link">"Services"</a></li>
                <li class="nav-dropdown">
                    <span class="nav-link nav-dropdown-toggle">"Gallery ▾"</span>
                    <ul class="nav-dropdown-menu">
                        <li><a href="#gallery" class="nav-dropdown-item">"Bathrooms"</a></li>
                        <li><a href="#gallery" class="nav-dropdown-item">"Decks & Patios"</a></li>
                        <li><a href="#gallery" class="nav-dropdown-item">"Home & Office"</a></li>
                        <li><a href="#gallery" class="nav-dropdown-item">"Kitchens"</a></li>
                        <li><a href="#gallery" class="nav-dropdown-item">"Basements"</a></li>
                    </ul>
                </li>
                <li><a href="#contact" class="nav-cta">"Get a Quote"</a></li>
            </ul>

            <button class="hamburger" aria-label="Toggle navigation"
                on:click=move |_| set_menu_open.update(|v| *v = !*v)>
                {move || if menu_open.get() { "✕" } else { "☰" }}
            </button>
        </nav>

        {move || menu_open.get().then(|| view! {
            <div class="mobile-drawer">
                <a href="#home" class="mobile-nav-link"
                    on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                    "Home"
                </a>
                <a href="#services" class="mobile-nav-link"
                    on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                    "Services"
                </a>
                <button class="mobile-gallery-btn"
                    on:click=move |_| set_gallery_open.update(|v| *v = !*v)>
                    "Gallery "
                    {move || if gallery_open.get() { "▴" } else { "▾" }}
                </button>
                {move || gallery_open.get().then(|| view! {
                    <div class="mobile-sub-group">
                        <a href="#gallery" class="mobile-sub-link"
                            on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                            "Bathrooms"
                        </a>
                        <a href="#gallery" class="mobile-sub-link"
                            on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                            "Decks & Patios"
                        </a>
                        <a href="#gallery" class="mobile-sub-link"
                            on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                            "Home & Office"
                        </a>
                        <a href="#gallery" class="mobile-sub-link"
                            on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                            "Kitchens"
                        </a>
                        <a href="#gallery" class="mobile-sub-link"
                            on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                            "Basements"
                        </a>
                    </div>
                })}
                <a href="#contact" class="mobile-cta-link"
                    on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                    "Get a Free Quote"
                </a>
            </div>
        })}
    }
}

// ── HERO ─────────────────────────────────────────────────────────────────────

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section id="home" class="hero">
            <img src=HERO_BG alt="" aria-hidden="true" class="hero-bg" />
            <div class="hero-overlay" />
            <div class="hero-content">
                <p class="eyebrow hero-eyebrow">"VETERAN-OWNED · NORTHERN VIRGINIA"</p>
                <h1 class="hero-heading">"Home Improvement With A Veteran's Touch"</h1>
                <p class="hero-sub">
                    "Dedicated to high-quality home improvement services for the residents of Northern Virginia."
                </p>
                <div class="hero-actions">
                    <a href="#gallery" class="btn btn-accent">"View Our Work"</a>
                    <a href="#contact" class="btn btn-outline">"Get a Free Quote"</a>
                </div>
            </div>
        </section>
    }
}

// ── TRUST BAR ────────────────────────────────────────────────────────────────

#[component]
fn TrustBar() -> impl IntoView {
    view! {
        <div class="trust-bar">
            <div class="trust-item">
                <span class="trust-icon">"🛡️"</span>
                <div>
                    <strong class="trust-label">"Licensed & Insured"</strong>
                    <p class="trust-sub">"Your project is fully protected"</p>
                </div>
            </div>
            <div class="trust-divider" />
            <div class="trust-item">
                <span class="trust-icon">"⭐"</span>
                <div>
                    <strong class="trust-label">"Veteran-Owned & Operated"</strong>
                    <p class="trust-sub">"Discipline and integrity in every job"</p>
                </div>
            </div>
            <div class="trust-divider" />
            <div class="trust-item">
                <span class="trust-icon">"📍"</span>
                <div>
                    <strong class="trust-label">"Serving Northern Virginia"</strong>
                    <p class="trust-sub">"Local expertise, community roots"</p>
                </div>
            </div>
        </div>
    }
}

// ── SERVICES ─────────────────────────────────────────────────────────────────

#[component]
fn Services() -> impl IntoView {
    let cards = [
        ("🏠", "Interior Renovations", "Kitchens, bathrooms, basements — complete interior transformations built to last."),
        ("🌿", "Exterior Upgrades", "Custom decks and patios designed for how you live outdoors."),
        ("🔨", "Custom Additions", "New rooms, media spaces, and built-ins that expand your home's potential."),
        ("🪨", "Hardscaping", "Fences, retaining walls, and stonework that add function and curb appeal."),
    ];

    view! {
        <section id="services" class="section section-bg">
            <div class="container">
                <p class="eyebrow">"WHAT WE DO"</p>
                <h2>"Our Services"</h2>
                <p class="section-sub">"From kitchens to custom decks — quality work, every time."</p>
                <div class="services-grid">
                    {cards.into_iter().map(|(icon, title, desc)| view! {
                        <div class="service-card">
                            <span class="service-icon">{icon}</span>
                            <h3>{title}</h3>
                            <p class="service-desc">{desc}</p>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

// ── GALLERY ──────────────────────────────────────────────────────────────────

#[component]
fn Gallery(lightbox: RwSignal<Option<String>>) -> impl IntoView {
    let active_tab: RwSignal<&'static str> = RwSignal::new("bathrooms");

    view! {
        <section id="gallery" class="section">
            <div class="container">
                <p class="eyebrow">"PROJECT GALLERY"</p>
                <h2>"Our Work"</h2>
                <p class="section-sub">"Browse completed projects across all categories."</p>

                <div class="tab-bar">
                    {CATEGORIES.iter().map(|(id, label, _)| {
                        let id = *id;
                        view! {
                            <button
                                class=move || if active_tab.get() == id { "tab-btn tab-btn-active" } else { "tab-btn" }
                                on:click=move |_| active_tab.set(id)>
                                {*label}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                {move || {
                    let current = active_tab.get();
                    CATEGORIES.iter()
                        .find(|(id, _, _)| *id == current)
                        .map(|(_, _, projects)| {
                            projects.iter().map(|(proj_name, images)| {
                                let images = *images;
                                view! {
                                    <div class="project-block">
                                        <h3 class="project-heading">{*proj_name}</h3>
                                        <div class="photo-grid">
                                            {images.iter().map(|url| {
                                                let url = url.to_string();
                                                let url_lb = url.clone();
                                                view! {
                                                    <button class="photo-cell"
                                                        aria-label="Open image"
                                                        on:click=move |_| lightbox.set(Some(url_lb.clone()))>
                                                        <img src={url.clone()} alt="" class="photo-img" loading="lazy" />
                                                        <div class="photo-overlay">
                                                            <span class="photo-zoom">"🔍"</span>
                                                        </div>
                                                    </button>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        })
                        .unwrap_or_default()
                }}
            </div>
        </section>
    }
}

// ── LIGHTBOX ─────────────────────────────────────────────────────────────────

#[component]
fn Lightbox(url: String, lightbox: RwSignal<Option<String>>) -> impl IntoView {
    view! {
        <div class="lightbox-overlay" on:click=move |_| lightbox.set(None)>
            <button class="lightbox-close" on:click=move |_| lightbox.set(None) aria-label="Close">
                "✕"
            </button>
            <img
                src={url}
                alt="Project photo"
                class="lightbox-img"
                on:click=move |e| e.stop_propagation()
            />
        </div>
    }
}

// ── CONTACT ──────────────────────────────────────────────────────────────────

#[component]
fn Contact() -> impl IntoView {
    let (submitted, set_submitted) = signal(false);
    let (name, set_name) = signal(String::new());

    view! {
        <section id="contact" class="section section-alt">
            <div class="container">
                <div class="contact-wrap">
                    <p class="eyebrow">"GET IN TOUCH"</p>
                    <h2>"Start Your Project"</h2>
                    <p class="section-sub">
                        "Ready to transform your home? Reach out and we'll get back to you within one business day."
                    </p>
                    <a href="mailto:redfoxcohomes@gmail.com" class="btn btn-accent btn-email">
                        "Email Us Directly"
                    </a>
                    <div class="contact-divider">
                        <span>"— or fill out the form —"</span>
                    </div>

                    {move || if submitted.get() {
                        view! {
                            <div class="success-card">
                                <p class="success-icon">"✅"</p>
                                <h3>"Message Received!"</h3>
                                <p>"Thanks, " {name.get()} ". We'll be in touch within one business day."</p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <form class="contact-form"
                                on:submit=move |ev| {
                                    ev.prevent_default();
                                    set_submitted.set(true);
                                }>
                                <div class="form-row">
                                    <div class="form-field">
                                        <label class="form-label" for="cf-name">"Your Name"</label>
                                        <input id="cf-name" class="form-input" type="text"
                                            placeholder="Jane Smith" required
                                            on:input=move |ev| set_name.set(event_target_value(&ev)) />
                                    </div>
                                    <div class="form-field">
                                        <label class="form-label" for="cf-email">"Email Address"</label>
                                        <input id="cf-email" class="form-input" type="email"
                                            placeholder="jane@example.com" required />
                                    </div>
                                </div>
                                <div class="form-field">
                                    <label class="form-label" for="cf-msg">"Tell Us About Your Project"</label>
                                    <textarea id="cf-msg" class="form-input form-textarea"
                                        rows="4" placeholder="Describe what you'd like done..." required>
                                    </textarea>
                                </div>
                                <button type="submit" class="btn btn-accent btn-submit">"Send Message"</button>
                            </form>
                        }.into_any()
                    }}
                </div>
            </div>
        </section>
    }
}

// ── FOOTER ───────────────────────────────────────────────────────────────────

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container footer-inner">
                <div class="footer-brand">
                    <img src=LOGO alt="Red Fox Co Homes" class="footer-logo" />
                    <span class="footer-name">"Red Fox Co Homes"</span>
                </div>
                <div class="footer-right">
                    <a href="mailto:redfoxcohomes@gmail.com" class="footer-email">
                        "redfoxcohomes@gmail.com"
                    </a>
                    <p class="footer-copy">"© 2026 Red Fox Co Homes · Northern Virginia"</p>
                </div>
            </div>
        </footer>
    }
}
