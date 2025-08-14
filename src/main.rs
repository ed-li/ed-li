use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/portfolio")]
    Portfolio {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const RESUME: Asset = asset!("/assets/resume.pdf");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        Timeline {}
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar bg-base-200 shadow-sm",
            id: "navbar",
            Link { class: "btn btn-ghost text-xl",
                to: Route::Home {},
                "Home"
            }
            Link { class: "btn btn-ghost text-xl",
                to: Route::Portfolio {},
                "Portfolio"
            }
        }

        Outlet::<Route> {}
    }
}

/// Hero
#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: "hero bg-base-200 min-h-screen",
            div { class: "hero-content text-center",
                div { class: "max-w-md",
                    h1 { class: "text-5xl font-bold", "Edward Li" }
                    p { class: "py-6",
                        "Full-Stack Developer"
                        br {}
                        "Machine Learning Engineer"
                        br {}
                        "Blockchain Consultant"
                    }
                    Link {
                        to: "{RESUME}",
                        button { class: "btn btn-primary", "Resume" }
                    }
                }
            }
        }
    }
}

/// Timeline
#[component]
pub fn Timeline() -> Element {
    rsx! {
        ul { class: "timeline timeline-snap-icon max-md:timeline-compact timeline-vertical",
            li {
                div { class: "timeline-middle",
                    svg {
                        class: "h-5 w-5",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            clip_rule: "evenodd",
                            d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            fill_rule: "evenodd",
                        }
                    }
                }
                div { class: "list timeline-start mb-10 md:text-end max-w-xl",
                    time { class: "font-mono italic", "Dec 2021 to Present" }
                    div { class: "text-lg font-black", "Ed Li, LLC" }
                    div { class: "text-lg font-bold", "Software Engineer" }
                    ul { class: "list bg-base-200 rounded-box shadow-md mt-4",
                        li { class: "list-row", "Designing and developing a web-based incremental simulation game using the open-source Bevy engine in Rust" }
                        li { class: "list-row", "Designing and developing a cross-platform workout tracker application using Dioxus, Axum, and SurrealDB in Rust" }
                        li { class: "list-row", "Designed and developed a progressive web application using React, Ruby on Rails, GraphQL, and PostgreSQL to facilitate the listing and sale of digital art published under an artist collective on the WAX blockchain" }
                        li { class: "list-row", "Built and maintained automated testing pipelines for a health technology company’s production software, ensuring the security of sensitive health data while complying with federal regulations" }

                    }
                }
                hr {}
            }
            li {
                hr {}
                div { class: "timeline-middle",
                    svg {
                        class: "h-5 w-5",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            clip_rule: "evenodd",
                            d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            fill_rule: "evenodd",
                        }
                    }
                }
                div { class: "timeline-end mb-10 max-w-xl",
                    time { class: "font-mono italic", "Nov 2020 to Dec 2021" }
                    div { class: "text-lg font-black", "DIA Associates" }
                    div { class: "text-md font-bold", "Data Analyst" }
                    ul { class: "list bg-base-200 rounded-box shadow-md mt-4",
                        li { class: "list-row", "Performed A/B testing on top credit card issuer’s specialized offers to help marketing teams implement better customer targeting and offer terms, thereby increasing customer engagement and retention" }
                        li { class: "list-row", "Identified likely-to-attrite small business and corporate credit card customers to prioritize efforts of outbound attrition prevention team, resulting in hundreds of additional customers retained per month" }
                        li { class: "list-row", "Identified high-spend-potential customers by analyzing early tenure behavior and firmographics, prioritizing efforts of account development team" }
                        li { class: "list-row", "Improved a credit card issuer’s merchant DBA name matching algorithms to enhance accuracy of transaction metadata on customer account statements, leading to fewer false positive fraud escalations" }

                    }
                }
                hr {}
            }
            li {
                hr {}
                div { class: "timeline-middle",
                    svg {
                        class: "h-5 w-5",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            clip_rule: "evenodd",
                            d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            fill_rule: "evenodd",
                        }
                    }
                }
                div { class: "timeline-start mb-10 md:text-end max-w-xl",
                    time { class: "font-mono italic", "May 2018 to Oct 2020" }
                    div { class: "text-lg font-black", "Delphi, LLC" }
                    div { class: "text-md font-bold", "Principal Consultant" }
                    ul { class: "list bg-base-200 rounded-box shadow-md mt-4",
                        li { class: "list-row", "Designed and oversaw the implementation of an economically motivated peer-to-peer monetary asset exchange network for a startup seeking to enhance its existing production software" }
                        li { class: "list-row", "Critiqued and prepared economics expert testimony and reports in preparation for corporate litigation defense, resulting in the court dismissing the majority of the plaintiffs’ claims" }
                    }
                }
                hr {}
            }
            li {
                hr {}
                div { class: "timeline-middle",
                    svg {
                        class: "h-5 w-5",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            clip_rule: "evenodd",
                            d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            fill_rule: "evenodd",
                        }
                    }
                }
                div { class: "timeline-end mb-10 max-w-xl",
                    time { class: "font-mono italic", "Nov 2017 to May 2018" }
                    div { class: "text-lg font-black", "Atana" }
                    div { class: "text-md font-bold", "Vice President of Product" }
                    ul { class: "list bg-base-200 rounded-box shadow-md mt-4",
                        li { class: "list-row", "Oversaw the design of a blockchain-based platform for managing and monetizing electronic health records" }
                        li { class: "list-row", "Presented the platform to investors and advisors for a successful seed round that resulted in $500K funding" }
                        li { class: "list-row", "Facilitated the Company’s entrance in several entrepreneurship and technology competitions, resulting in the receipt of awards, grants, and monetary prizes from ETHDenver, the Ralph S. O’Connor Fund, J.P Morgan, and AARP Foundation" }
                    }
                }
                hr {}
            }
            li {
                hr {}
                div { class: "timeline-middle",
                    svg {
                        class: "h-5 w-5",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            clip_rule: "evenodd",
                            d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            fill_rule: "evenodd",
                        }
                    }
                }
                div { class: "timeline-start mb-10 md:text-end max-w-xl",
                    time { class: "font-mono italic", "Jun 2017 to Aug 2017" }
                    div { class: "text-lg font-black", "J.P. Morgan" }
                    div { class: "text-md font-bold", "Investment Banking Credit Risk Summer Analyst" }
                    ul { class: "list bg-base-200 rounded-box shadow-md mt-4",
                        li { class: "list-row", "Determined the creditworthiness of asset management firms using both internal and industry-standard rating models" }
                        li { class: "list-row", "Presented a quarterly portfolio review of asset management clients to managing directors" }
                    }
                }
                hr {}
            }
            li {
                hr {}
                div { class: "timeline-middle",
                    svg {
                        class: "h-5 w-5",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            clip_rule: "evenodd",
                            d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            fill_rule: "evenodd",
                        }
                    }
                }
                div { class: "timeline-end mb-10 max-w-xl",
                    time { class: "font-mono italic", "May 2015 to May 2020" }
                    div { class: "text-lg font-black", "Johns Hopkins Institute for Applied Economics" }
                    div { class: "text-md font-bold", "Chief Research Assistant for Prof. Steve H. Hanke" }
                    div { class: "text-md font-bold", "Croft Scholarship Fellow" }
                    div { class: "text-md font-bold", "Teaching Assistant" }
                    ul { class: "list bg-base-200 rounded-box shadow-md mt-4",
                        li { class: "list-row", "Independently evaluated potential investments and presented theses to fund managers using a combination of a proprietary discounted cash flow approach, Monte Carlo statistical analysis, and proxy-statement review" }
                        li { class: "list-row", "Led several teams of research assistants on various research projects, including analyzing country-level monetary and financial statistics and monitoring indicators of currency health for the Troubled Currencies Project" }
                        li { class: "list-row", "Published original research papers in academic journals and business magazines, including the Journal of Applied Corporate Finance, on topics ranging from currency board orthodoxy to international trade wars" }
                    }
                }
                hr {}
            }
        }
    }
}

/// Blog page
#[component]
pub fn Portfolio() -> Element {
    rsx! {
        div { class: "flex flex-row p-4 gap-4",
            id: "portfolio",
            div { class: "card bg-base-200 w-96 shadow-sm",
                div { class: "card-body",
                    h2 { class: "card-title", "Logger" }
                    p {
                        "General activity tracker with modular support for specialized activities, namely working out."
                    }
                    Link { class: "card-actions justify-end",
                        to: "https://logger-app.fly.dev/",
                        button { class: "btn btn-primary",
                            "Go"
                        }
                    }
                }
            }
            div { class: "card bg-base-200 w-96 shadow-sm",
                div { class: "card-body",
                    h2 { class: "card-title", "PDF Parser" }
                    p {
                        "API for extracting fields from PDF files using an optimized, lightweight VLM in the backend."
                    }
                    Link { class: "card-actions justify-end",
                        to: "https://github.com/ed-li/pdf_parser",
                        button { class: "btn btn-primary",
                            "Go"
                        }
                    }
                }
            }
        }
    }
}
