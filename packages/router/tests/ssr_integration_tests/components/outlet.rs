use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{render, test_routes};

#[test]
fn index() {
    assert_eq!("<p>0: index</p>", render(App));

    #[allow(non_snake_case)]
    fn App(cx: Scope) -> Element {
        cx.render(rsx! {
            Router {
                routes: test_routes(&cx),
                init_only: true,
                Outlet {}
            }
        })
    }
}

#[test]
fn route() {
    assert_eq!("<p>0: test</p><p>1: index</p>", render(App));

    #[allow(non_snake_case)]
    fn App(cx: Scope) -> Element {
        cx.render(rsx! {
            Router {
                routes: test_routes(&cx),
                initial_path: "/test",

                Outlet {}
            }
        })
    }
}

#[test]
fn nested_route() {
    assert_eq!(
        "<p>0: test</p><p>1: nest</p><!--placeholder-->",
        render(App)
    );

    #[allow(non_snake_case)]
    fn App(cx: Scope) -> Element {
        cx.render(rsx! {
            Router {
                routes: test_routes(&cx),
                initial_path: "/test/nest",

                Outlet {}
            }
        })
    }
}

#[test]
fn with_depth() {
    assert_eq!("<p>1: index</p>", render(App));

    #[allow(non_snake_case)]
    fn App(cx: Scope) -> Element {
        cx.render(rsx! {
            Router{
                routes: test_routes(&cx),
                initial_path: "/test",

                Outlet {
                    depth: 1
                }
            }
        })
    }
}

#[test]
fn with_depth_inheritance() {
    assert_eq!("<p>1: nest</p><p>2: double-nest</p>", render(App));

    #[allow(non_snake_case)]
    fn App(cx: Scope) -> Element {
        cx.render(rsx! {
            Router{
                routes: test_routes(&cx),
                initial_path: "/test/nest/double-nest",

                Outlet {
                    depth: 1
                }
            }
        })
    }
}

#[test]
fn with_name() {
    assert_eq!("<p>1: index, other</p>", render(App));

    #[allow(non_snake_case)]
    fn App(cx: Scope) -> Element {
        cx.render(rsx! {
            Router {
                routes: test_routes(&cx),
                initial_path: "/test",

                Outlet {
                    name: "other"
                }
            }
        })
    }
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "`Outlet` can only be used as a descendent of a `Router`"]
fn without_router_panic_in_debug() {
    render(OutletWithoutRouter);
}

#[cfg(not(debug_assertions))]
#[test]
fn without_router_ignore_in_release() {
    assert_eq!("<!--placeholder-->", render(OutletWithoutRouter));
}

#[allow(non_snake_case)]
fn OutletWithoutRouter(cx: Scope) -> Element {
    cx.render(rsx! {
        Outlet {}
    })
}
