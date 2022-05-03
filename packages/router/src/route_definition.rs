//! Several data types for defining what component to render for which path.

use std::collections::BTreeMap;

use dioxus_core::Component;

use crate::navigation::InternalNavigationTarget;

/// A collection of routes for a single path segment.
#[derive(Clone)]
pub struct Segment {
    /// The index route is rendered if the [`Segment`] is the first segment to be not specified by
    /// the path.
    pub index: RouteContent,
    /// A fixed route is rendered if it matches the path segment _exactly_.
    pub fixed: Vec<(String, Route)>,
    /// The dynamic route is rendered if no fixed route is matched.
    pub dynamic: DynamicRoute,
}

impl Default for Segment {
    fn default() -> Self {
        Self { index: Default::default(), fixed: Default::default(), dynamic: Default::default() }
    }
}

/// A definition of a static route.
#[derive(Clone)]
pub struct Route {
    /// The name of the route.
    ///
    /// Can be used for name-based navigation. See [NtName] or [ItName].
    ///
    /// Make sure that the name is unique among the routes passed to a
    /// [Router](crate::components::Router).
    ///
    /// [NtName]: crate::navigation::NavigationTarget::NtName
    /// [ItName]: crate::navigation::InternalNavigationTarget::ItName
    pub name: Option<&'static str>,
    /// The content to render if the route is matched.
    pub content: RouteContent,
    /// The routes for the next path segment.
    pub sub: Option<Segment>,
}

impl Default for Route {
    fn default() -> Self {
        Self {
            name: Default::default(),
            content: Default::default(),
            sub: Default::default(),
        }
    }
}

/// A dynamic route definition.
#[derive(Clone)]
pub enum DynamicRoute {
    /// Indicates the absence of an actual dynamic route.
    DrNone,
    /// A dynamic route that treats the actual value of its segment as a variable.
    ///
    /// The value will be accessible to components via [use_route].
    ///
    /// [use_route]: crate::hooks::use_route
    DrVariable {
        /// The name of the route.
        ///
        /// Can be used for name-based navigation. See [NtName] or [ItName].
        ///
        /// Make sure that the name is unique among the routes passed to a
        /// [Router](crate::components::Router).
        ///
        /// [NtName]: crate::navigation::NavigationTarget::NtName
        /// [ItName]: crate::navigation::InternalNavigationTarget::ItName
        name: Option<&'static str>,
        /// The key that the segments value will be accessible under.
        key: &'static str,
        /// The content to render if the route is matched.
        content: RouteContent,
        /// The routes for the next path segment.
        sub: Option<Box<Segment>>,
    },
    /// A fallback that is rendered when no other route matches.
    DrFallback(RouteContent),
}

impl Default for DynamicRoute {
    fn default() -> Self {
        Self::DrNone
    }
}

/// The actual content of a [`Route`] or [`DynamicRoute`].
#[derive(Clone)]
pub enum RouteContent {
    /// Indicates the absence of content.
    ///
    /// When used for an `index` it marks that no index content exists.
    ///
    /// When used for a `fixed` or `dynamic` no components will be rendered. If a nested route is
    /// matched its content will be rendered in the outlet where this segments content would be
    /// rendered.
    RcNone,
    /// A single component.
    RcComponent(Component),
    /// One main and several side components.
    RcMulti(Component, Vec<(&'static str, Component)>),
    /// Causes a redirect when the route is matched.
    ///
    /// Redirects are performed as a _replace_ operation. This means that the original path won't be
    /// part of the history.
    ///
    /// Be careful to not create an infinite loop. While certain [HistoryProvider]s may stop after a
    /// threshold is reached, others (like [MemoryHistoryProvider]) will not.
    ///
    /// [HistoryProvider]: crate::history::HistoryProvider
    /// [MemoryHistoryProvider]: crate::history::MemoryHistoryProvider
    RcRedirect(InternalNavigationTarget),
}

impl RouteContent {
    pub(crate) fn add_to_list(
        &self,
        components: &mut (Vec<Component>, BTreeMap<&'static str, Vec<Component>>),
    ) -> Option<InternalNavigationTarget> {
        match self {
            RouteContent::RcNone => {}
            RouteContent::RcComponent(comp) => components.0.push(*comp),
            RouteContent::RcMulti(main, side) => {
                components.0.push(*main);
                for (name, comp) in side {
                    if let Some(x) = components.1.get_mut(name) {
                        x.push(*comp);
                    } else {
                        components.1.insert(name, vec![*comp]);
                    }
                }
            }
            RouteContent::RcRedirect(t) => return Some(t.clone()),
        }

        None
    }

    /// Returns `true` if the route content is [`TNone`].
    ///
    /// [`TNone`]: RouteContent::TNone
    #[must_use]
    pub fn is_rc_none(&self) -> bool {
        matches!(self, Self::RcNone)
    }
}

impl Default for RouteContent {
    fn default() -> Self {
        Self::RcNone
    }
}
