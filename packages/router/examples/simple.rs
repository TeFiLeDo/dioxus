use dioxus::prelude::*;

fn main() {
    env_logger::init();
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let routes = use_segment(&cx, || {
        Segment::default()
            .index(RcComponent(Home))
            .fixed("apple", Route::new(RcComponent(Apple)))
            .fixed("potato", Route::new(RcComponent(Potato)).name("potato"))
            .fixed(
                "earth_apple",
                Route::new(RcRedirect(NtName("potato", vec![], QNone))).name("earth apple"),
            )
    });

    cx.render(rsx! {
        Router {
            routes: routes.clone(),
            Outlet { }
        }
    })
}

#[allow(non_snake_case)]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Home" }
        p { "Hi! This is a simple example for Dioxus' router:" }
        ul {
            li {
                Link {
                    target: NtPath(String::from("/apple")),
                    "Read about apples…"
                }
            }
            li {
                Link {
                    target: NtName("potato", vec![], QNone),
                    "Read about potatoes…"
                }
            }
            li {
                Link {
                    target: NtName("earth apple", vec![], QNone),
                    "Read about earth apples (literal translation of a german word for potato)…"
                }
            }
        }
    })
}

#[allow(non_snake_case)]
fn Apple(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Apples" }
        p { "Apples are fruit that grows on trees."}
        p {
            Link {
                target: NtName("root_index", vec![], QNone),
                "Go back to home"
            }
        }
    })
}

#[allow(non_snake_case)]
fn Potato(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Potatoes" }
        p { "Potatoes grow underground. There are many recipes involving potatoes."}
        p {
            Link {
                target: NtName("root_index", vec![], QNone),
                "Go back to home"
            }
        }
    })
}
