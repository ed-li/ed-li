use dioxus::prelude::*;

/// Portfolio page
#[component]
pub fn Portfolio() -> Element {
    rsx! {
        div { class: "flex flex-wrap",
            id: "portfolio",
            div { class: "card bg-base-100 w-96 shadow-sm",
                div { class: "card-body",
                    div { class: "grid grid-cols-2",
                        div {
                            h2 { class: "card-title", "Logger" }
                        }
                        div { class: "card-actions justify-end",
                            Link {
                                to: "https://github.com/ed-li/logger",
                                button { class: "btn btn-psquare btn-sm",
                                    "Code"
                                }
                            }
                        }
                    }
                    p {
                        "General activity tracker with modular support for specialized activities. Prototypical module is for recording and evaluating strength training exercise performance."
                    }
                    div { class: "card-actions justify-end",
                        Link {
                            to: "https://logger-app.fly.dev/",
                            button { class: "btn btn-primary",
                                "Desktop"
                            }
                        }
                        Link {
                            to: "",
                            button { class: "btn btn-disabled",
                                "iOS"
                            }
                        }
                        Link {
                            to: "https://logger-app.fly.dev/",
                            button { class: "btn btn-primary",
                                "Android"
                            }
                        }
                        Link {
                            to: "https://logger-app.fly.dev/",
                            button { class: "btn btn-primary",
                                "Web"
                            }
                        }
                    }
                }
            }
            div { class: "card bg-base-100 w-96 shadow-sm",
                div { class: "card-body",
                    div { class: "grid grid-cols-2",
                        div {
                            h2 { class: "card-title", "PDF Parser" }
                        }
                        div { class: "card-actions justify-end",
                            Link {
                                to: "https://github.com/ed-li/pdf_parser",
                                button { class: "btn btn-psquare btn-sm",
                                    "Code"
                                }
                            }
                        }
                    }
                    p {
                        "API for extracting fields from PDF files. Using an optimized, lightweight VLM in the backend."
                    }
                    div { class: "card-actions justify-end",
                        Link {
                            to: "https://github.com/ed-li/pdf_parser",
                            button { class: "btn btn-primary",
                                "API"
                            }
                        }
                    }
                }
            }
            div { class: "card bg-base-100 w-96 shadow-sm",
                div { class: "card-body",
                    div { class: "grid grid-cols-2",
                        div {
                            h2 { class: "card-title", "Idleverse" }
                        }
                        div { class: "card-actions justify-end",
                            Link {
                                to: "https://github.com/ed-li/idleverse",
                                button { class: "btn btn-psquare btn-sm",
                                    "Code"
                                }
                            }
                        }
                    }
                    p {
                        "Incremental simulation game in an infinite multi-dimensional universe with discrete space. Made using the open-source Bevy engine."
                    }
                    div { class: "card-actions justify-end",
                        Link {
                            to: "https://github.com/ed-li/idleverse",
                            button { class: "btn btn-primary",
                                "Web"
                            }
                        }
                    }
                }
            }
        }
    }
}
