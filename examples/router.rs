#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let routes = use_segment(&cx, || {
        Segment::new()
            .index(Home as Component)
            .fixed(
                "blog",
                Route::new(RcNone).nested(
                    Segment::new()
                        .index(BlogList as Component)
                        .parameter(("post", BlogPost as Component)),
                ),
            )
            .fixed(
                "users",
                Route::new(RcNone).nested(
                    Segment::new()
                        .index(UserList as Component)
                        .parameter(("name", User as Component)),
                ),
            )
    });

    cx.render(rsx! {
        Router {
            routes: routes.clone(),
            fallback: RcComponent(RouteNotFound),

            ul {
                Link { target: "/".into(),  li { "Go home!" } }

                Link { target: "/users".into(),  li { "List all users" } }
                Link { target: "/users/bill".into(),  li { "Show user \"bill\"" } }
                Link { target: "/users/franz?bold".into(), li { "Show user \"franz\""}}

                Link { target: "/blog".into(), li { "List all blog posts" } }
                Link { target: "/blog/5".into(), li { "Blog post 5" } }
            }
            Outlet { }
        }
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        "Home"
    })
}

fn BlogList(cx: Scope) -> Element {
    cx.render(rsx! {
        "Blog list"
    })
}

fn BlogPost(cx: Scope) -> Element {
    let route = use_route(&cx)?;
    let post = route.parameters.get("post")?;

    cx.render(rsx! {
        div {
            h1 { "Reading blog post: {post}" }
            p { "example blog post" }
        }
    })
}

fn RouteNotFound(cx: Scope) -> Element {
    cx.render(rsx! {
        "Error 404: Route Not Found"
    })
}

fn User(cx: Scope) -> Element {
    let route = use_route(&cx)?;
    let params = route.query_params().unwrap_or_default();

    let name = route.parameters.get("name")?;

    // if bold is specified without content => true
    // if bold is specified => parse, false if invalid
    // default to false
    let bold: bool = params
        .get("bold")
        .and_then(|bold| match bold.is_empty() {
            true => Some(true),
            false => bold.parse().ok(),
        })
        .unwrap_or_default();

    cx.render(rsx! {
        div {
            h1 { "Showing user: {name}" }
            p { "example user content" }

            if bold {
                rsx!{ b { "bold" } }
            } else {
                rsx!{ i { "italic" } }
            }
        }
    })
}

fn UserList(cx: Scope) -> Element {
    cx.render(rsx! {
        "User list"
    })
}
