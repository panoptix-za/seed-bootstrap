use seed::virtual_dom::IntoNodes;
use seed::{prelude::*, Style as StyleSeed, *};
use std::borrow::Cow;

use std::rc::Rc;
use web_sys::{HtmlElement, MouseEvent};

// ------ Button ------

pub struct Button<Ms: 'static> {
    title: Option<Cow<'static, str>>,
    style: Style,
    outline: bool,
    size: Size,
    block: bool,
    element: Element,
    text_no_wrap: bool,
    attrs: Attrs,
    disabled: bool,
    on_clicks: Vec<Rc<dyn Fn(MouseEvent) -> Ms>>,
    prefix_content: Vec<Node<Ms>>,
    content: Vec<Node<Ms>>,
    el_ref: ElRef<HtmlElement>,
    style_seed: StyleSeed,
}

impl<Ms> Button<Ms> {
    pub fn new(title: impl Into<Cow<'static, str>>) -> Self {
        Self::default().title(title)
    }

    pub fn title(mut self, title: impl Into<Cow<'static, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    // --- style ---

    fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn primary(self) -> Self {
        self.style(Style::Primary)
    }

    pub fn secondary(self) -> Self {
        self.style(Style::Secondary)
    }

    pub fn success(self) -> Self {
        self.style(Style::Success)
    }

    pub fn danger(self) -> Self {
        self.style(Style::Danger)
    }

    pub fn warning(self) -> Self {
        self.style(Style::Warning)
    }

    pub fn info(self) -> Self {
        self.style(Style::Info)
    }

    pub fn light(self) -> Self {
        self.style(Style::Light)
    }

    pub fn dark(self) -> Self {
        self.style(Style::Dark)
    }

    pub fn link(self) -> Self {
        self.style(Style::Link)
    }

    pub fn no_style(self) -> Self {
        self.style(Style::NoStyle)
    }

    // --- // ---

    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    // --- size ---

    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn small(self) -> Self {
        self.size(Size::Small)
    }

    pub fn large(self) -> Self {
        self.size(Size::Large)
    }

    // --- // ---

    pub fn block(mut self) -> Self {
        self.block = true;
        self
    }

    // --- element ---

    fn element(mut self, element: Element) -> Self {
        self.element = element;
        self
    }

    pub fn a(self, href: impl Into<Cow<'static, str>>, role: Role) -> Self {
        self.element(Element::A(Href(href.into()), role))
    }

    pub fn button(self, type_: Type) -> Self {
        self.element(Element::Button(type_))
    }

    pub fn input(self, type_: Type) -> Self {
        self.element(Element::Input(type_))
    }

    pub fn label(self) -> Self {
        self.element(Element::Label)
    }

    // --- // ---

    pub fn text_no_wrap(mut self) -> Self {
        self.text_no_wrap = true;
        self
    }

    pub fn add_attrs(mut self, attrs: Attrs) -> Self {
        self.attrs.merge(attrs);
        self
    }

    pub fn add_style(mut self, style: StyleSeed) -> Self {
        self.style_seed.merge(style);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

    pub fn prefix_content(mut self, prefix_content: impl IntoNodes<Ms>) -> Self {
        self.prefix_content = prefix_content.into_nodes();
        self
    }

    pub fn content(mut self, content: impl IntoNodes<Ms>) -> Self {
        self.content = content.into_nodes();
        self
    }

    pub fn el_ref(mut self, el_ref: &ElRef<HtmlElement>) -> Self {
        self.el_ref = el_ref.clone();
        self
    }

    pub fn view(self) -> Node<Ms> {
        self.view_internal(None)
    }

    pub fn view_toggle(self, active: bool) -> Node<Ms> {
        self.view_internal(Some(active))
    }

    fn view_internal(mut self, active: Option<bool>) -> Node<Ms> {
        let mut content = Vec::new();
        let mut attrs = self.element.to_attrs();

        if let Some(title) = self.title.take() {
            match self.element {
                Element::A(_, _) | Element::Button(_) | Element::Label => {
                    content.push(plain!(title));
                }
                Element::Input(_) => {
                    attrs.add(At::Value, title);
                }
            }
        }

        if self.disabled {
            attrs.add(At::from("aria-disabled"), true);
            attrs.add(At::TabIndex, -1);
            match self.element {
                Element::A(_, _) | Element::Label => {
                    attrs.add(At::Class, "disabled");
                }
                Element::Button(_) | Element::Input(_) => {
                    attrs.add(At::Disabled, AtValue::None);
                }
            }
        }

        if let Some(active) = active {
            if active {
                attrs.add(At::Class, "active")
            }
            attrs.add(At::from("aria-pressed"), active);
            attrs.add(At::from("data-toggle"), "button");
        }

        let style = self.style.as_str();
        let style_class = format!(
            "btn-{}{}",
            if self.outline { "outline-" } else { "" },
            style
        );
        let mut button = custom![
            self.element.to_tag(),
            el_ref(&self.el_ref),
            C![
                IF!(!style.is_empty() => "btn"),
                IF!(!style.is_empty() => &style_class),
                self.size.as_class(),
                IF!(self.text_no_wrap => "text-nowrap"),
                IF!(self.block => "btn-block"),
            ],
            self.style_seed,
            attrs,
            self.attrs,
            self.prefix_content,
            content,
            self.content,
        ];

        if !self.disabled {
            for on_click in self.on_clicks {
                button.add_event_handler(mouse_ev(Ev::Click, move |event| on_click(event)));
            }
        }

        button
    }
}

impl<Ms> Default for Button<Ms> {
    fn default() -> Self {
        Self {
            title: None,
            style: Style::default(),
            outline: false,
            size: Size::default(),
            block: false,
            element: Element::default(),
            text_no_wrap: false,
            attrs: Attrs::empty(),
            disabled: false,
            on_clicks: Vec::new(),
            prefix_content: Vec::new(),
            content: Vec::new(),
            el_ref: ElRef::default(),
            style_seed: StyleSeed::empty(),
        }
    }
}

impl<Ms> UpdateEl<Ms> for Button<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        self.view().update_el(el)
    }
}

// ------ Style ------

enum Style {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link,
    NoStyle,
}

impl Style {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Danger => "danger",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Light => "light",
            Self::Dark => "dark",
            Self::Link => "link",
            Self::NoStyle => "",
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::Primary
    }
}

// ------ Size ------

enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    fn as_class(&self) -> &'static str {
        match self {
            Self::Small => "btn-sm",
            Self::Medium => "",
            Self::Large => "btn-lg",
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Medium
    }
}

// ------ Element ------

enum Element {
    A(Href, Role),
    Button(Type),
    Input(Type),
    Label,
}

impl Element {
    fn to_tag(&self) -> Tag {
        match self {
            Self::A(_, _) => Tag::A,
            Self::Button(_) => Tag::Button,
            Self::Input(_) => Tag::Input,
            Self::Label => Tag::Label,
        }
    }

    fn to_attrs(&self) -> Attrs {
        match self {
            Self::A(href, role) => attrs! {
                At::Href => href.as_str(),
                At::from("role") => match role {
                    Role::Button => AtValue::Some("button".to_owned()),
                    Role::None => AtValue::Ignored,
                }
            },
            Self::Button(type_) | Self::Input(type_) => attrs! {
                At::Type => match type_.as_str() {
                    Some(type_) => AtValue::Some(type_.to_owned()),
                    None => AtValue::Ignored,
                }
            },
            Self::Label => Attrs::empty(),
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        Self::Button(Type::None)
    }
}

// ------ Href ------

pub struct Href(Cow<'static, str>);

impl Href {
    fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

// ------ Role ------

pub enum Role {
    None,
    Button,
}

// ------ Type ------

pub enum Type {
    None,
    Submit,
    Button,
    Reset,
}

impl Type {
    fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Submit => Some("submit"),
            Self::Button => Some("button"),
            Self::Reset => Some("reset"),
        }
    }
}
