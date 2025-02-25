use std::sync::{Arc, RwLockReadGuard};

use dioxus::prelude::*;
use dioxus_router::{
    history::{ControlledHistory, HistoryController, HistoryProvider, MemoryHistory},
    prelude::*,
};
use dioxus_ssr::render_vdom;

use crate::{render, test_routes};

#[test]
fn render_when_init_only() {
    // prepare history
    let (mut controller, controlled) = HistoryController::new(MemoryHistory::new());
    controller.push(String::from("/test"));

    // first render
    let mut app = VirtualDom::new_with_props(
        App,
        AppProps {
            history: controlled,
        },
    );
    app.rebuild();
    let pre = render_vdom(&app);

    // second render
    controller.push(String::from("/test/nest"));
    app.rebuild();
    let post = render_vdom(&app);

    assert_ne!(pre, post);
    assert_eq!(pre, "<p>0: test</p><p>1: index</p>");
    assert_eq!(post, "<p>0: test</p><p>1: nest</p><!--placeholder-->");

    #[derive(Props)]
    struct AppProps {
        history: ControlledHistory,
    }

    impl PartialEq for AppProps {
        fn eq(&self, _: &Self) -> bool {
            false
        }
    }

    #[allow(non_snake_case)]
    fn App(cx: Scope<AppProps>) -> Element {
        let history = cx.use_hook(|| {
            let history = cx.props.history.clone();
            return move || -> Box<dyn HistoryProvider> { Box::new(history.clone()) };
        });

        cx.render(rsx! {
            Router {
                history: history,
                init_only: true,
                routes: test_routes(&cx),

                Outlet { }
            }
        })
    }
}

#[test]
fn with_initial_path() {
    assert_eq!("<p>0: test</p><p>1: index</p>", render(App));

    #[allow(non_snake_case)]
    fn App(cx: Scope) -> Element {
        cx.render(rsx! {
            Router {
                routes: test_routes(&cx),
                initial_path: "/test",

                Outlet { }
            }
        })
    }
}

#[test]
fn with_update_callback() {
    assert_eq!("<p>0: test</p><p>1: index</p>", render(App));

    #[allow(non_snake_case)]
    fn App(cx: Scope) -> Element {
        let update_fn = cx.use_hook(|| {
            Arc::new(
                |state: RwLockReadGuard<RouterState>| -> Option<NavigationTarget> {
                    (state.path == "/").then(|| "/redirect".into())
                },
            )
        });

        cx.render(rsx! {
            Router {
                routes: test_routes(&cx),
                update_callback: update_fn.clone(),
                init_only: true,

                Outlet { }
            }
        })
    }
}

#[test]
fn non_nested_router() {
    render(App);

    #[allow(non_snake_case)]
    fn App(cx: Scope) -> Element {
        cx.render(rsx! {
            Router {
                routes: test_routes(&cx)
            }
        })
    }
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "`Router` can not be used as a descendent of a `Router`"]
fn nested_routers_panic_in_debug() {
    render(NestedRouters);
}

#[cfg(not(debug_assertions))]
#[test]
fn nested_routes_ignore_in_release() {
    assert_eq!("<!--placeholder-->", render(NestedRouters));
}

#[allow(non_snake_case)]
fn NestedRouters(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            routes: test_routes(&cx),
            Router {
                routes: test_routes(&cx),
            }
        }
    })
}
