use super::button::Button;
use seed::virtual_dom::IntoNodes;
use seed::{prelude::*, *};
use std::borrow::Cow;

use std::rc::Rc;
use web_sys::MouseEvent;

// ------ NavBar ------

pub struct NavBar<Ms: 'static> {
    brand: Brand<Ms>,
    content: Vec<Node<Ms>>,
    toggle: Button<Ms>,
    attrs: Attrs,
    style: Style,
    fixed_top: bool,
}

impl<Ms> NavBar<Ms> {
    pub fn new(content: impl IntoNodes<Ms>, link: impl Into<Cow<'static, str>>) -> Self {
        let toggle = Button::default()
            .no_style()
            .add_attrs(C!["navbar-toggler"])
            .add_attrs(attrs! {
                At::from("aria-expanded") => "false",
                At::from("aria-label") => "Toggle navigation",
            })
            .content(span![C!["navbar-toggler-icon"]]);

        Self {
            brand: Brand::new(content, link),
            content: Vec::new(),
            toggle,
            attrs: Attrs::empty(),
            style: Style::empty(),
            fixed_top: true,
        }
    }

    pub fn fixed_top(mut self, fixed_top: bool) -> Self {
        self.fixed_top = fixed_top;
        self
    }

    pub fn update_brand(mut self, f: impl FnOnce(Brand<Ms>) -> Brand<Ms>) -> Self {
        self.brand = f(self.brand);
        self
    }

    pub fn content(mut self, content: impl IntoNodes<Ms>) -> Self {
        self.content = content.into_nodes();
        self
    }

    pub fn add_attrs(mut self, attrs: Attrs) -> Self {
        self.attrs.merge(attrs);
        self
    }

    pub fn add_style(mut self, style: Style) -> Self {
        self.style.merge(style);
        self
    }

    pub fn update_toggle(mut self, f: impl FnOnce(Button<Ms>) -> Button<Ms>) -> Self {
        self.toggle = f(self.toggle);
        self
    }

    pub fn view(self) -> Node<Ms> {
        nav![
            C!["navbar", IF!(self.fixed_top => "fixed-top")],
            self.style,
            self.attrs,
            self.brand,
            self.content
        ]
    }

    pub fn view_collapsable(
        self,
        expanded: bool,
        content_id: impl Into<Cow<'static, str>>,
    ) -> Node<Ms> {
        let content_id = content_id.into();
        nav![
            C!["navbar", IF!(self.fixed_top => "fixed-top")],
            self.style,
            self.attrs,
            self.brand,
            vec![
                self.toggle
                    .add_attrs(attrs! { At::from("aria-controls") => content_id })
                    .view(),
                div![
                    C!["collapse", "navbar-collapse", IF!(expanded => "show")],
                    id!(content_id),
                    self.content,
                ]
            ]
        ]
    }
}

// ------ Brand ------

pub struct Brand<Ms: 'static> {
    content: Vec<Node<Ms>>,
    link: Cow<'static, str>,
    attrs: Attrs,
    style: Style,
}

impl<Ms> UpdateEl<Ms> for Brand<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        self.view().update_el(el)
    }
}

impl<Ms> Brand<Ms> {
    pub fn new(content: impl IntoNodes<Ms>, link: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: content.into_nodes(),
            link: link.into(),
            attrs: Attrs::empty(),
            style: Style::empty(),
        }
    }

    pub fn content(mut self, content: impl IntoNodes<Ms>) -> Self {
        self.content = content.into_nodes();
        self
    }

    pub fn add_attrs(mut self, attrs: Attrs) -> Self {
        self.attrs.merge(attrs);
        self
    }

    pub fn add_style(mut self, style: Style) -> Self {
        self.style.merge(style);
        self
    }

    pub fn view(self) -> Node<Ms> {
        a![
            C!["navbar-brand"],
            attrs! {At::Href => self.link},
            self.style,
            self.attrs,
            self.content,
        ]
    }
}

// ------ Nav ------

pub struct Nav<Ms: 'static> {
    content: Vec<Node<Ms>>,
    attrs: Attrs,
    style: Style,
}

impl<Ms> Nav<Ms> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: impl IntoNodes<Ms>) -> Self {
        self.content = content.into_nodes();
        self
    }

    pub fn add_attrs(mut self, attrs: Attrs) -> Self {
        self.attrs.merge(attrs);
        self
    }

    pub fn add_style(mut self, style: Style) -> Self {
        self.style.merge(style);
        self
    }

    pub fn view(self) -> Node<Ms> {
        ul![C!["navbar-nav"], self.style, self.attrs, self.content,]
    }
}

impl<Ms> Default for Nav<Ms> {
    fn default() -> Self {
        Self {
            content: Vec::new(),
            attrs: Attrs::empty(),
            style: Style::empty(),
        }
    }
}

// ------ NavLink ------

pub struct NavLink<Ms: 'static> {
    title: Cow<'static, str>,
    link: Cow<'static, str>,
    active: bool,
    disabled: bool,
    icon: Option<Node<Ms>>,
    attrs: Attrs,
    inner_attrs: Attrs,
    style: Style,
    on_clicks: Vec<Rc<dyn Fn(MouseEvent) -> Ms>>,
}

impl<Ms> UpdateEl<Ms> for NavLink<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        self.view().update_el(el)
    }
}

impl<Ms> NavLink<Ms> {
    pub fn new(title: impl Into<Cow<'static, str>>, link: impl Into<Cow<'static, str>>) -> Self {
        Self {
            title: title.into(),
            link: link.into(),
            active: false,
            disabled: false,
            icon: None,
            attrs: Attrs::empty(),
            inner_attrs: Attrs::empty(),
            style: Style::empty(),
            on_clicks: Vec::new(),
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon(mut self, icon: Node<Ms>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn add_attrs(mut self, attrs: Attrs) -> Self {
        self.attrs.merge(attrs);
        self
    }

    pub fn add_inner_attrs(mut self, attrs: Attrs) -> Self {
        self.inner_attrs.merge(attrs);
        self
    }

    pub fn add_style(mut self, style: Style) -> Self {
        self.style.merge(style);
        self
    }

    pub fn add_on_click(
        mut self,
        on_click: impl FnOnce(MouseEvent) -> Ms + Clone + 'static,
    ) -> Self {
        self.on_clicks
            .push(Rc::new(move |event| on_click.clone()(event)));
        self
    }

    pub fn view(self) -> Node<Ms> {
        let mut elem = li![
            C!["nav-item", IF!(self.active => "active")],
            &self.attrs,
            &self.style,
            a![
                C![
                    "nav-link",
                    IF!(self.disabled => "disabled"),
                    IF!(self.active => "active")
                ],
                attrs! {
                    At::Href => self.link
                    At::TabIndex => if self.disabled { AtValue::Some((-1).to_string()) } else { AtValue::Ignored },
                    At::from("aria-disabled") => if self.disabled { AtValue::Some(true.to_string()) } else { AtValue::Ignored },
                },
                self.inner_attrs,
                if let Some(icon) = self.icon {
                    icon
                } else {
                    empty![]
                },
                self.title,
                if self.active {
                    span![C!["sr-only"], " (current)"]
                } else {
                    empty![]
                },
            ],
        ];
        for on_click in self.on_clicks {
            elem.add_event_handler(mouse_ev(Ev::Click, move |event| on_click(event)));
        }
        elem
    }
}
