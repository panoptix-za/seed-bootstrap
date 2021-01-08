use seed::{prelude::*, *};
use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

pub struct FormGroup<'a, Ms: 'static> {
    id: Cow<'a, str>,
    label: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    input_event: Option<Rc<dyn Fn(String) -> Ms>>,
    input_type: InputType,
    is_invalid: bool,
    invalid_feedback: Option<Cow<'a, str>>,
    is_warning: bool,
    warning_feedback: Option<Cow<'a, str>>,
    help_text: Vec<Node<Ms>>,
    group_attrs: Attrs,
    input_attrs: Attrs,
}

impl<'a, Ms> FormGroup<'a, Ms> {
    pub fn new(id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            id: id.into(),
            label: None,
            value: None,
            input_event: None,
            input_type: InputType::Text,
            is_invalid: false,
            invalid_feedback: None,
            is_warning: false,
            warning_feedback: None,
            help_text: Vec::new(),
            group_attrs: Attrs::empty(),
            input_attrs: Attrs::empty(),
        }
    }

    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn on_input(mut self, input_event: impl Fn(String) -> Ms + Clone + 'static) -> Self {
        self.input_event = Some(Rc::new(input_event));
        self
    }

    pub fn text(mut self) -> Self {
        self.input_type = InputType::Text;
        self
    }
    pub fn number(mut self) -> Self {
        self.input_type = InputType::Number;
        self
    }
    pub fn password(mut self) -> Self {
        self.input_type = InputType::Password;
        self
    }
    pub fn textarea(mut self) -> Self {
        self.input_type = InputType::Textarea;
        self
    }
    pub fn checkbox(mut self) -> Self {
        self.input_type = InputType::Checkbox;
        self
    }

    pub fn select(mut self, options: Vec<(String, String)>, include_none_option: bool) -> Self {
        self.input_type = InputType::Select {
            options,
            include_none_option,
        };
        self
    }

    pub fn invalid(mut self, is_invalid: bool) -> Self {
        self.is_invalid = is_invalid;
        self
    }

    pub fn invalid_feedback(mut self, invalid_feedback: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.invalid_feedback = invalid_feedback.map(|s| s.into());
        self
    }

    pub fn warning(mut self, is_warning: bool) -> Self {
        self.is_warning = is_warning;
        self
    }

    pub fn warning_feedback(mut self, warning_feedback: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.warning_feedback = warning_feedback.map(|s| s.into());
        self
    }

    pub fn help_text(mut self, help_text: impl Into<Cow<'static, str>>) -> Self {
        self.help_text = Node::new_text(help_text).into_nodes();
        self
    }
    pub fn help_nodes(mut self, help_nodes: impl IntoNodes<Ms>) -> Self {
        self.help_text = help_nodes.into_nodes();
        self
    }

    pub fn group_attrs(mut self, attrs: Attrs) -> Self {
        self.group_attrs.merge(attrs);
        self
    }

    pub fn input_attrs(mut self, attrs: Attrs) -> Self {
        self.input_attrs.merge(attrs);
        self
    }

    pub fn view(self) -> Node<Ms> {
        if self.input_type == InputType::Checkbox {
            self.view_checkbox()
        } else {
            self.view_textfield()
        }
    }

    fn view_checkbox(self) -> Node<Ms> {
        let is_checked = self
            .value
            .as_ref()
            .map(|value| value == "true")
            .unwrap_or(false);
        let click_event_text = if is_checked {
            "false".to_string()
        } else {
            "true".to_string()
        };
        div![
            C!["form-group form-check"],
            &self.group_attrs,
            input![
                C!["form-check-input", IF!(self.is_invalid => "is-invalid")],
                &self.input_attrs,
                id![&self.id],
                attrs![
                    At::Type => "checkbox",
                    At::Value => "true",
                    At::Checked => is_checked.as_at_value()
                ],
                self.input_event.clone().map(|input_event| {
                    input_ev(Ev::Input, move |_event| input_event(click_event_text))
                })
            ],
            self.label.as_ref().map(|label| {
                label![
                    C!["form-check-label"],
                    attrs![
                        At::For => self.id
                    ],
                    label
                ]
            }),
            if !self.help_text.is_empty() {
                small![C!["form-text text-muted"], &self.help_text]
            } else {
                empty![]
            },
            self.invalid_feedback
                .as_ref()
                .filter(|_| self.is_invalid)
                .map(|err| div![C!["invalid-feedback"], err]),
            self.warning_feedback
                .as_ref()
                .filter(|_| self.is_warning)
                .map(|err| small![C!["form-text text-warning"], err])
        ]
    }

    fn view_textfield(self) -> Node<Ms> {
        div![
            C!["form-group"],
            &self.group_attrs,
            self.label.as_ref().map(|label| {
                label![
                    attrs![
                        At::For => self.id
                    ],
                    label
                ]
            }),
            match &self.input_type {
                InputType::Text | InputType::Number | InputType::Password => input![
                    C!["form-control", IF!(self.is_invalid => "is-invalid")],
                    &self.input_attrs,
                    id![&self.id],
                    attrs![
                        At::Type => &self.input_type,
                    ],
                    self.value.as_ref().map(|value| attrs![At::Value => value]),
                    self.input_event.clone().map(|input_event| {
                        input_ev(Ev::Input, move |event| input_event(event))
                    })
                ],
                InputType::Textarea => textarea![
                    C!["form-control", IF!(self.is_invalid => "is-invalid")],
                    &self.input_attrs,
                    id![&self.id],
                    self.value.as_ref().map(
                        |value| attrs![At::Value => value, At::Rows => value.split('\n').count(), At::Wrap => "off"]
                    ),
                    self.input_event.clone().map(|input_event| {
                        input_ev(Ev::Input, move |event| input_event(event))
                    })
                ],
                InputType::Select { options, include_none_option } => select![
                    C!["custom-select", IF!(self.is_invalid => "is-invalid")],
                    if *include_none_option { option![
                        attrs! {
                            At::Value => "",
                            At::Selected => self.value.is_none().as_at_value()
                        }
                    ] } else { empty![] },
                    options.iter().map(|(key, display)| {
                        option![
                            attrs! {
                                At::Value => &key,
                                At::Selected => self.value.as_ref().map(|value| value == key).unwrap_or(false).as_at_value()
                            },
                            &display
                        ]
                    }),
                    self.input_event.clone().map(|input_event| {
                        input_ev(Ev::Input, move |event| input_event(event))
                    })
                ],
                InputType::Checkbox => empty![],
            },
            if !self.help_text.is_empty() { small![C!["form-text text-muted"], &self.help_text] } else { empty![] },
            self.invalid_feedback
                .as_ref()
                .filter(|_| self.is_invalid)
                .map(|err| div![C!["invalid-feedback"], err]),
            self.warning_feedback
                .as_ref()
                .filter(|_| self.is_warning)
                .map(|err| small![C!["form-text text-warning"], err])
        ]
    }
}

impl<Ms> UpdateEl<Ms> for FormGroup<'_, Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        self.view().update_el(el)
    }
}

#[derive(PartialEq)]
enum InputType {
    Text,
    Number,
    Password,
    Textarea,
    Checkbox,
    Select {
        options: Vec<(String, String)>,
        include_none_option: bool,
    },
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Text => write!(f, "text"),
            Self::Number => write!(f, "number"),
            Self::Password => write!(f, "password"),
            Self::Textarea => write!(f, "textarea"),
            Self::Checkbox => write!(f, "checkbox"),
            Self::Select { .. } => write!(f, "select"),
        }
    }
}
