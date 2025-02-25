# External Navigation

In modern apps, and especially on the web, we often want to send our users to an
other website. [`ExternalTarget`] allows us to make a [`Link`] navigate to an
external page.

> You might already now about
> [external navigation failures](../failures/external.md). The [`Link`]
> component doesn't rely on the code path where those originate. Therefore a
> [`Link`] will never trigger an external navigation failure.

Strictly speaking, a [`Link`] is not necessary for navigating to external
targets, since by definition the router cannot handle them internally. However,
the [`Link`] component is more convenient to use, as it automatically sets the
`rel` attribute for the link, when the target is external.

## Code Example
```rust
# // Hidden lines (like this one) make the documentation tests work.
# extern crate dioxus;
use dioxus::prelude::*;
# extern crate dioxus_router;
use dioxus_router::prelude::*;
# extern crate dioxus_ssr;

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            routes: use_segment(&cx, Default::default).clone(),
            # init_only: true,

            // links need to be inside a router, even if they navigate to an
            // external page
            Link {
                target: ExternalTarget(String::from("https://dioxuslabs.com/")),
                "Go to the dioxus home page"
            }
            Link {
                target: "https://dioxuslabs.com/", // short form
                "Go to the dioxus home page 2"
            }
        }
    })
}
#
# let mut vdom = VirtualDom::new(App);
# vdom.rebuild();
# let html = dioxus_ssr::render_vdom(&vdom);
# assert_eq!(
#     format!(
#         "<a {attr1} {attr2}>{text}</a><a {attr1} {attr2}>{text} 2</a>",
#         attr1 = r#"href="https://dioxuslabs.com/" dioxus-prevent-default="""#,
#         attr2 = r#"class="" id="" rel="noopener noreferrer" target="""#,
#         text = "Go to the dioxus home page"
#     ),
#     html
# )
```

> Note that the short form for an [`ExternalTarget`] looks like the short form
> for an [`InternalTarget`]. The router will create an [`ExternalTarget`] only
> if the URL is absolute.

[`ExternalTarget`]: https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html#variant.ExternalTarget
[`InternalTarget`]: https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html#variant.InternalTarget
[`Link`]: https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Link.html
