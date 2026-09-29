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
    ("bathrooms", "Bathrooms",    BATHROOMS),
    ("decks",     "Decks & Patios", DECKS),
    ("office",    "Home & Office",  OFFICE),
    ("kitchens",  "Kitchens",      KITCHENS),
    ("basements", "Basements",     BASEMENTS),
];

fn main() {
    leptos::mount::mount_to_body(App);
}

// ── App ──────────────────────────────────────────────────────────────────────

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

// ── Nav ──────────────────────────────────────────────────────────────────────

#[component]
fn Nav() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);
    let (gallery_open, set_gallery_open) = signal(false);

    view! {
        <nav class="fixed top-0 inset-x-0 z-50 h-16 bg-gray-950 flex items-center justify-between px-6">
            <a href="#home" class="flex items-center gap-3 min-h-[44px]"
                on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                <img src=LOGO alt="Red Fox Co Homes" class="w-9 h-9 rounded-full object-contain bg-white p-0.5" />
                <span class="text-white font-semibold text-sm">"Red Fox Co Homes"</span>
            </a>

            // Desktop links
            <ul class="hidden md:flex items-center gap-8 list-none">
                <li><a href="#home"     class="text-gray-400 hover:text-white text-sm transition-colors min-h-[44px] inline-flex items-center">"Home"</a></li>
                <li><a href="#services" class="text-gray-400 hover:text-white text-sm transition-colors min-h-[44px] inline-flex items-center">"Services"</a></li>
                <li class="nav-dropdown relative">
                    <span class="text-gray-400 hover:text-white text-sm transition-colors min-h-[44px] inline-flex items-center cursor-pointer">"Gallery ▾"</span>
                    <ul class="dropdown-menu hidden absolute top-full mt-2 left-1/2 -translate-x-1/2 bg-gray-900 rounded-lg min-w-[160px] py-1 shadow-xl z-50 list-none">
                        <li><a href="#gallery" class="block px-4 py-3 text-gray-400 hover:text-white text-sm transition-colors min-h-[44px] flex items-center">"Bathrooms"</a></li>
                        <li><a href="#gallery" class="block px-4 py-3 text-gray-400 hover:text-white text-sm transition-colors min-h-[44px] flex items-center">"Decks & Patios"</a></li>
                        <li><a href="#gallery" class="block px-4 py-3 text-gray-400 hover:text-white text-sm transition-colors min-h-[44px] flex items-center">"Home & Office"</a></li>
                        <li><a href="#gallery" class="block px-4 py-3 text-gray-400 hover:text-white text-sm transition-colors min-h-[44px] flex items-center">"Kitchens"</a></li>
                        <li><a href="#gallery" class="block px-4 py-3 text-gray-400 hover:text-white text-sm transition-colors min-h-[44px] flex items-center">"Basements"</a></li>
                    </ul>
                </li>
                <li>
                    <a href="#contact" class="bg-brand-700 hover:bg-brand-800 text-white text-sm font-semibold px-4 py-2 rounded-full transition-colors min-h-[44px] inline-flex items-center">
                        "Get a Quote"
                    </a>
                </li>
            </ul>

            // Hamburger
            <button
                class="md:hidden flex items-center justify-center w-11 h-11 text-white hover:bg-white/10 rounded transition-colors"
                on:click=move |_| set_menu_open.update(|v| *v = !*v)
                aria-label="Toggle menu">
                {move || if menu_open.get() { "✕" } else { "☰" }}
            </button>
        </nav>

        // Mobile drawer
        {move || menu_open.get().then(|| view! {
            <div class="mobile-drawer fixed top-16 inset-x-0 z-40 bg-gray-950 border-t border-white/10 flex flex-col">
                <a href="#home"
                    class="text-gray-300 hover:text-white px-6 py-4 text-base border-b border-white/5 flex items-center min-h-[44px] transition-colors"
                    on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                    "Home"
                </a>
                <a href="#services"
                    class="text-gray-300 hover:text-white px-6 py-4 text-base border-b border-white/5 flex items-center min-h-[44px] transition-colors"
                    on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                    "Services"
                </a>
                <button
                    class="text-gray-300 hover:text-white px-6 py-4 text-base border-b border-white/5 flex items-center min-h-[44px] transition-colors w-full text-left"
                    on:click=move |_| set_gallery_open.update(|v| *v = !*v)>
                    {move || if gallery_open.get() { "Gallery ▴" } else { "Gallery ▾" }}
                </button>
                {move || gallery_open.get().then(|| view! {
                    <div class="flex flex-col bg-black/20">
                        <a href="#gallery" class="text-gray-400 hover:text-gray-200 px-10 py-3 text-sm flex items-center min-h-[44px] transition-colors"
                            on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>"Bathrooms"</a>
                        <a href="#gallery" class="text-gray-400 hover:text-gray-200 px-10 py-3 text-sm flex items-center min-h-[44px] transition-colors"
                            on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>"Decks & Patios"</a>
                        <a href="#gallery" class="text-gray-400 hover:text-gray-200 px-10 py-3 text-sm flex items-center min-h-[44px] transition-colors"
                            on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>"Home & Office"</a>
                        <a href="#gallery" class="text-gray-400 hover:text-gray-200 px-10 py-3 text-sm flex items-center min-h-[44px] transition-colors"
                            on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>"Kitchens"</a>
                        <a href="#gallery" class="text-gray-400 hover:text-gray-200 px-10 py-3 text-sm flex items-center min-h-[44px] transition-colors"
                            on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>"Basements"</a>
                    </div>
                })}
                <a href="#contact"
                    class="mx-6 my-4 bg-brand-700 hover:bg-brand-800 text-white text-center py-3 rounded-lg font-semibold transition-colors min-h-[44px] flex items-center justify-center"
                    on:click=move |_| { set_menu_open.set(false); set_gallery_open.set(false); }>
                    "Get a Free Quote"
                </a>
            </div>
        })}
    }
}

// ── Hero ─────────────────────────────────────────────────────────────────────

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section id="home" class="relative min-h-screen flex items-center justify-center text-center overflow-hidden">
            <img src=HERO_BG alt="" aria-hidden="true" class="hero-bg" />
            <div class="hero-overlay"></div>
            <div class="hero-content max-w-2xl px-6 py-16">
                <span class="text-[0.7rem] font-semibold tracking-[3px] uppercase text-white/50 block mb-6">
                    "VETERAN-OWNED · NORTHERN VIRGINIA"
                </span>
                <h1 class="font-serif font-semibold text-white leading-tight mb-6"
                    style="font-size: clamp(2rem, 5vw, 3.5rem);">
                    "Home Improvement With A Veteran's Touch"
                </h1>
                <p class="text-white/60 text-base leading-relaxed max-w-lg mx-auto mb-10">
                    "Dedicated to high-quality home improvement services for the residents of Northern Virginia."
                </p>
                <div class="flex flex-col sm:flex-row gap-3 justify-center">
                    <a href="#gallery"
                        class="bg-brand-700 hover:bg-brand-800 text-white font-semibold px-8 py-3 rounded-md transition-all hover:-translate-y-0.5 inline-flex items-center justify-center min-h-[44px]">
                        "View Our Work"
                    </a>
                    <a href="#contact"
                        class="border border-white/40 hover:border-white hover:bg-white/10 text-white font-semibold px-8 py-3 rounded-md transition-all inline-flex items-center justify-center min-h-[44px]">
                        "Get a Free Quote"
                    </a>
                </div>
            </div>
        </section>
    }
}

// ── TrustBar ─────────────────────────────────────────────────────────────────

#[component]
fn TrustBar() -> impl IntoView {
    view! {
        <section class="bg-gray-50 py-16">
            <div class="max-w-4xl mx-auto px-6 flex flex-col sm:flex-row justify-center gap-12 sm:gap-20 text-center">
                <div>
                    <p class="font-semibold text-sm text-gray-900 mb-1">"Licensed & Insured"</p>
                    <p class="text-gray-500 text-xs">"Serving Northern Virginia since 2020"</p>
                </div>
                <div>
                    <p class="font-semibold text-sm text-gray-900 mb-1">"Veteran-Owned & Operated"</p>
                    <p class="text-gray-500 text-xs">"Proudly serving those who served"</p>
                </div>
                <div>
                    <p class="font-semibold text-sm text-gray-900 mb-1">"Highly Rated"</p>
                    <p class="text-gray-500 text-xs">"5-star quality on every project"</p>
                </div>
            </div>
        </section>
    }
}

// ── Services ─────────────────────────────────────────────────────────────────

#[component]
fn Services() -> impl IntoView {
    let services = [
        ("🏠", "Interior Renovations",  "Kitchens, bathrooms, basements — we transform your existing spaces into something you'll love coming home to."),
        ("🌿", "Exterior Upgrades",     "Custom decks, patios, and outdoor living spaces built to last through every season."),
        ("🔨", "Custom Additions",      "New rooms, media rooms, and built-in storage tailored to your lifestyle and your floor plan."),
        ("🪨", "Hardscaping",           "Fences, retaining walls, and hardscape features that add structure and curb appeal to your property."),
    ];

    view! {
        <section id="services" class="py-32 bg-white">
            <div class="max-w-5xl mx-auto px-6">
                <div class="text-center">
                    <span class="text-xs font-semibold tracking-[3px] uppercase text-gray-400 block mb-4">"WHAT WE DO"</span>
                    <h2 class="font-serif font-semibold text-4xl text-gray-900 mb-4">"Our Services"</h2>
                    <p class="text-gray-500 text-base max-w-md mx-auto leading-relaxed">
                        "From kitchens to custom decks — quality craftsmanship, every time."
                    </p>
                </div>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-x-16 gap-y-14 mt-20">
                    {services.into_iter().map(|(icon, title, desc)| view! {
                        <div>
                            <span class="text-2xl mb-4 block">{icon}</span>
                            <h3 class="font-serif font-semibold text-lg text-gray-900 mb-3">{title}</h3>
                            <p class="text-gray-500 text-sm leading-relaxed">{desc}</p>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

// ── Gallery ──────────────────────────────────────────────────────────────────

#[component]
fn Gallery(lightbox: RwSignal<Option<String>>) -> impl IntoView {
    let active_tab: RwSignal<&'static str> = RwSignal::new("bathrooms");

    view! {
        <section id="gallery" class="py-32 bg-gray-50">
            <div class="max-w-6xl mx-auto px-6">
                <div class="text-center">
                    <span class="text-xs font-semibold tracking-[3px] uppercase text-gray-400 block mb-4">"PROJECT GALLERY"</span>
                    <h2 class="font-serif font-semibold text-4xl text-gray-900">"Our Work"</h2>
                </div>

                // Tabs
                <div class="flex flex-wrap gap-2 justify-center mt-10 mb-16">
                    {CATEGORIES.iter().map(|(id, label, _)| {
                        let id = *id;
                        let label = *label;
                        view! {
                            <button
                                class=move || {
                                    if active_tab.get() == id {
                                        "px-5 py-2.5 rounded-full bg-gray-900 text-white text-sm font-semibold min-h-[44px] inline-flex items-center transition-all"
                                    } else {
                                        "px-5 py-2.5 rounded-full border border-gray-200 text-gray-500 text-sm font-semibold hover:border-gray-900 hover:text-gray-900 transition-all min-h-[44px] inline-flex items-center bg-white"
                                    }
                                }
                                on:click=move |_| active_tab.set(id)>
                                {label}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                // Active category projects
                {move || {
                    let tab = active_tab.get();
                    let projects = CATEGORIES.iter()
                        .find(|(id, _, _)| *id == tab)
                        .map(|(_, _, p)| *p)
                        .unwrap_or(&[]);

                    view! {
                        <div>
                            {projects.iter().map(|(project_name, images)| {
                                let images = *images;
                                view! {
                                    <div class="mb-12">
                                        <p class="text-[0.7rem] font-semibold tracking-[2.5px] uppercase text-gray-400 mb-6 mt-10">
                                            {*project_name}
                                        </p>
                                        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3">
                                            {images.iter().map(|url| {
                                                let url_str = url.to_string();
                                                let url_for_click = url_str.clone();
                                                view! {
                                                    <div class="gallery-item relative overflow-hidden rounded-md aspect-[4/3] cursor-pointer"
                                                        on:click=move |_| lightbox.set(Some(url_for_click.clone()))>
                                                        <img
                                                            src=url_str.clone()
                                                            alt=""
                                                            loading="lazy"
                                                            class="gallery-img w-full h-full object-cover"
                                                        />
                                                        <div class="gallery-overlay absolute inset-0 flex items-center justify-center">
                                                            <span class="zoom-icon text-white text-xl opacity-0 scale-90">"🔍"</span>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }
                }}
            </div>
        </section>
    }
}

// ── Lightbox ─────────────────────────────────────────────────────────────────

#[component]
fn Lightbox(url: String, lightbox: RwSignal<Option<String>>) -> impl IntoView {
    view! {
        <div
            class="lb-overlay fixed inset-0 z-[1000] bg-black/95 flex items-center justify-center p-8 cursor-pointer"
            on:click=move |_| lightbox.set(None)>
            <img
                src=url
                alt=""
                class="lb-img max-h-[90vh] object-contain rounded-md cursor-default"
                style="max-width: min(90vw, 900px);"
                on:click=move |e| e.stop_propagation()
            />
            <button
                class="fixed top-6 right-6 w-11 h-11 rounded-full bg-white/10 hover:bg-white/20 border-none text-white flex items-center justify-center transition-colors z-[1001] text-sm cursor-pointer"
                on:click=move |_| lightbox.set(None)>
                "✕"
            </button>
        </div>
    }
}

// ── Contact ──────────────────────────────────────────────────────────────────

#[component]
fn Contact() -> impl IntoView {
    let (submitted, set_submitted) = signal(false);
    let (name, set_name) = signal(String::new());

    view! {
        <section id="contact" class="py-32 bg-white">
            <div class="max-w-lg mx-auto px-6 text-center">
                <span class="text-xs font-semibold tracking-[3px] uppercase text-gray-400 block mb-4">"GET IN TOUCH"</span>
                <h2 class="font-serif font-semibold text-4xl text-gray-900 mb-4">"Start Your Project"</h2>
                <p class="text-gray-500 text-base leading-relaxed mb-10">
                    "Ready to transform your home? Reach out and we'll get back to you within one business day."
                </p>
                <a href="mailto:redfoxcohomes@gmail.com"
                    class="bg-brand-700 hover:bg-brand-800 text-white font-semibold px-8 py-3 rounded-md transition-all hover:-translate-y-0.5 inline-flex items-center justify-center min-h-[44px] mb-10">
                    "Email Us Directly"
                </a>

                // Divider
                <div class="flex items-center gap-4 mb-10 text-gray-300 text-xs">
                    <span class="flex-1 h-px bg-gray-100"></span>
                    "or"
                    <span class="flex-1 h-px bg-gray-100"></span>
                </div>

                {move || if submitted.get() {
                    view! {
                        <div class="text-center py-20">
                            <span class="text-3xl mb-4 block">"✅"</span>
                            <h3 class="font-serif font-semibold text-2xl text-gray-900 mb-3">
                                "Thanks, " {name.get()} "!"
                            </h3>
                            <p class="text-gray-500 text-sm">"We'll be in touch within one business day."</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <form class="text-left" on:submit=move |e| { e.prevent_default(); set_submitted.set(true); }>
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
                                <div class="flex flex-col gap-1.5">
                                    <label class="text-xs font-semibold text-gray-500">"Name"</label>
                                    <input
                                        type="text"
                                        required
                                        placeholder="Your name"
                                        class="border border-gray-200 rounded-md px-4 py-3 text-sm text-gray-900 bg-white focus:outline-none focus:border-gray-900 transition-colors w-full min-h-[44px]"
                                        on:input=move |e| set_name.set(event_target_value(&e))
                                    />
                                </div>
                                <div class="flex flex-col gap-1.5">
                                    <label class="text-xs font-semibold text-gray-500">"Email"</label>
                                    <input
                                        type="email"
                                        required
                                        placeholder="your@email.com"
                                        class="border border-gray-200 rounded-md px-4 py-3 text-sm text-gray-900 bg-white focus:outline-none focus:border-gray-900 transition-colors w-full min-h-[44px]"
                                    />
                                </div>
                            </div>
                            <div class="flex flex-col gap-1.5 mb-4">
                                <label class="text-xs font-semibold text-gray-500">"Message"</label>
                                <textarea
                                    rows="4"
                                    required
                                    placeholder="Tell us about your project..."
                                    class="border border-gray-200 rounded-md px-4 py-3 text-sm text-gray-900 bg-white focus:outline-none focus:border-gray-900 transition-colors w-full resize-none"
                                ></textarea>
                            </div>
                            <button
                                type="submit"
                                class="w-full bg-brand-700 hover:bg-brand-800 text-white font-semibold py-3 rounded-md transition-colors min-h-[44px] mt-2">
                                "Send Message"
                            </button>
                        </form>
                    }.into_any()
                }}
            </div>
        </section>
    }
}

// ── Footer ───────────────────────────────────────────────────────────────────

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-gray-100 py-10">
            <div class="max-w-5xl mx-auto px-6 flex flex-col sm:flex-row items-center justify-between gap-4 text-center sm:text-left">
                <span class="font-semibold text-sm text-gray-900">"Red Fox Co Homes"</span>
                <span class="text-gray-400 text-xs">"© 2026 Red Fox Co Homes · Northern Virginia"</span>
                <a href="mailto:redfoxcohomes@gmail.com"
                    class="text-gray-400 text-xs hover:text-gray-700 transition-colors">
                    "redfoxcohomes@gmail.com"
                </a>
            </div>
        </footer>
    }
}
