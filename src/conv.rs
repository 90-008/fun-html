//! Common HTML elements (without attributes)
//!
//! Note that you may create your own element with [`Element::new`] or [`Element::new_void`]
//!
//! It is also possible to inline raw HTML with [`raw`] and [`raw_unsafe`]

use alloc::borrow::Cow;

use crate::{
    attr::{content, href, name, rel},
    elt::raw,
    Attribute, Element, ElementInner,
};

/// Renders nothing. Useful fo conditional rendering.
///
/// # Example
///
/// ```
/// use fun_html::{Element, elt::{text, none}};
///
/// let say_hello = true;
/// let element: Element = if say_hello {
///   text("Hello")
/// } else {
///   none()
/// };
/// ```
pub fn none() -> Element {
    Element(ElementInner::None)
}

/// `<div>`
pub fn div(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("div", [], children)
}

/// `<head>`
pub fn head(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("head", [], children)
}

/// `<legend>`
pub fn legend(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("legend", [], children)
}

/// `<meta>`
pub fn meta(attributes: impl IntoIterator<Item = Attribute>) -> Element {
    Element::new_void("meta", attributes)
}

/// `<meta charset="UFT-8">`
#[deprecated(since = "1.5.0", note = "renamed to 'meta_charset_utf8'")]
pub fn meta_charset_utf_8() -> Element {
    raw("<meta charset=\"UTF-8\">")
}

/// `<meta charset="UFT-8">`
pub fn meta_charset_utf8() -> Element {
    raw("<meta charset=\"UTF-8\">")
}

/// `<meta name="viewport" content="width=device-width, initial-scale=1.0">`
pub fn meta_viewport() -> Element {
    raw("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">")
}

/// `<meta name="color-scheme content="{scheme}">
pub fn meta_color_scheme(scheme: impl Into<Cow<'static, str>>) -> Element {
    meta([name("color-scheme"), content(scheme)])
}

/// `<link>`
pub fn link(attributes: impl IntoIterator<Item = Attribute>) -> Element {
    Element::new_void("link", attributes)
}

/// `<link rel="stylesheet" href="{url}">`
pub fn link_stylesheet(url: impl Into<Cow<'static, str>>) -> Element {
    link([rel("stylesheet"), href(url)])
}

/// `<script>`
pub fn script(url: impl Into<Cow<'static, str>>) -> Element {
    Element::new("script", [href(url)], [])
}

/// `<title>`
pub fn title(text: impl Into<Cow<'static, str>>) -> Element {
    Element::new("title", [], [text.into().into()])
}

/// `<body>`
pub fn body(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("body", [], children)
}

/// `<hgroup>`
pub fn hgroup(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("hgroup", [], children)
}

/// `<h1>`
pub fn h1(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("h1", [], children)
}

/// `<h2>`
pub fn h2(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("h2", [], children)
}

/// `<h3>`
pub fn h3(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("h3", [], children)
}

/// `<h4>`
pub fn h4(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("h4", [], children)
}

/// `<h5>`
pub fn h5(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("h5", [], children)
}

/// `<h6>`
pub fn h6(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("h6", [], children)
}

/// `<p>`
pub fn p(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("p", [], children)
}

/// `<br>`
pub fn br() -> Element {
    Element::new_void("br", [])
}

/// `<hr>`
pub fn hr() -> Element {
    Element::new_void("hr", [])
}

/// `<small>`
pub fn small(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("small", [], children)
}

/// `<span>`
pub fn span(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("span", [], children)
}

/// `<table>`
pub fn table(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("table", [], children)
}

/// `<tr>`
pub fn tr(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("tr", [], children)
}

/// `<td>`
pub fn td(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("td", [], children)
}

/// `<th>`
pub fn th(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("th", [], children)
}

/// `<thead>`
pub fn thead(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("thead", [], children)
}

/// `<tbody>`
pub fn tbody(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("tbody", [], children)
}

/// `<tfoot>`
pub fn tfoot(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("tfoot", [], children)
}

/// `<section>`
pub fn section(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("section", [], children)
}

/// `<article>`
pub fn article(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("article", [], children)
}

/// `<header>`
pub fn header(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("header", [], children)
}

/// `<main>`
pub fn main(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("main", [], children)
}

/// `<footer>`
pub fn footer(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("footer", [], children)
}

/// `<a>`
pub fn a(
    href: impl Into<Cow<'static, str>>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("a", [Attribute::new("href", href)], children)
}

/// `<img>`
pub fn img(attributes: impl IntoIterator<Item = Attribute>) -> Element {
    Element::new_void("img", attributes)
}

/// `<ul>`
pub fn ul(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("ul", [], children)
}

/// `<ol>`
pub fn ol(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("ol", [], children)
}

/// `<li>`
pub fn li(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new("li", [], children)
}

/// `<form>`
pub fn form(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("form", attributes, children)
}

/// `<fieldset>`
pub fn fieldset(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("fieldset", attributes, children)
}

/// `<input>`
pub fn input(attributes: impl IntoIterator<Item = Attribute>) -> Element {
    Element::new_void("input", attributes)
}

/// `<textarea>`
pub fn textarea(
    attributes: impl IntoIterator<Item = Attribute>,
    text: impl Into<Cow<'static, str>>,
) -> Element {
    Element::new("textarea", attributes, [text.into().into()])
}

/// `<select>`
pub fn select(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("select", attributes, children)
}

/// `<option>`
pub fn option(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("option", attributes, children)
}

/// `<button>`
pub fn button(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("button", attributes, children)
}

/// `<label>`
pub fn label(
    attributes: impl IntoIterator<Item = Attribute>,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    Element::new("label", attributes, children)
}

/// HTML escaped text
pub fn text(value: impl Into<Cow<'static, str>>) -> Element {
    ElementInner::Text(value.into()).into()
}
