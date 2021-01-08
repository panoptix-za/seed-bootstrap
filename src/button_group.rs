use seed::virtual_dom::IntoNodes;
use seed::{prelude::*, *};
use std::borrow::Cow;

// ------ ButtonGroup ------

pub struct ButtonGroup<Ms: 'static> {
    role: Role,
    label: Option<Label>,
    content: Vec<Node<Ms>>,
    vertical: bool,
    attrs: Attrs,
    style: Style,
}

impl<Ms> ButtonGroup<Ms> {
    pub fn new(label: impl Into<Cow<'static, str>>) -> Self {
        Self::default().label(label)
    }

    pub fn new_labelled_by(labelled_by: impl Into<Cow<'static, str>>) -> Self {
        Self::default().labelled_by(labelled_by)
    }

    pub fn label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.label = Some(Label::Label(label.into()));
        self
    }

    pub fn labelled_by(mut self, labelled_by: impl Into<Cow<'static, str>>) -> Self {
        self.label = Some(Label::LabelledBy(labelled_by.into()));
        self
    }

    pub fn group(mut self) -> Self {
        self.role = Role::Group;
        self
    }

    pub fn toolbar(mut self) -> Self {
        self.role = Role::Toolbar;
        self
    }

    pub fn content(mut self, content: impl IntoNodes<Ms>) -> Self {
        self.content = content.into_nodes();
        self
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
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
        let class = format!(
            "{}{}",
            match self.role {
                Role::Group => "btn-group",
                Role::Toolbar => "btn-toolbar",
            },
            if self.vertical { "-vertical" } else { "" }
        );
        div![
            self.style,
            C![class.as_str()],
            attrs! {
                At::from("role") => match self.role {
                    Role::Group => "group",
                    Role::Toolbar => "toolbar"
                },
            },
            match self.label {
                Some(Label::Label(label)) => attrs! {At::from("aria-label") => label},
                Some(Label::LabelledBy(labelled_by)) =>
                    attrs! {At::from("aria-labelledby") => labelled_by},
                None => attrs! {},
            },
            self.attrs,
            self.content,
        ]
    }
}

impl<Ms> Default for ButtonGroup<Ms> {
    fn default() -> Self {
        Self {
            role: Role::default(),
            label: None,
            content: Vec::new(),
            vertical: false,
            attrs: Attrs::empty(),
            style: Style::empty(),
        }
    }
}

impl<Ms> UpdateEl<Ms> for ButtonGroup<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        self.view().update_el(el)
    }
}

// ------ Role ------

pub enum Role {
    Group,
    Toolbar,
}

impl Default for Role {
    fn default() -> Self {
        Self::Group
    }
}

// ------ Label ------

pub enum Label {
    Label(Cow<'static, str>),
    LabelledBy(Cow<'static, str>),
}
