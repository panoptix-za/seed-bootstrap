use crate::Route;
use seed::virtual_dom::IntoNodes;
use seed::{prelude::*, *};
use seed_bootstrap::button;
use seed_bootstrap::button_group;
use seed_bootstrap::dropdown;
use seed_bootstrap::navbar;
use std::{collections::HashMap, iter::FromIterator};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod button_groups;
mod buttons;
mod dropdowns;
mod navbars;

// ------ ------
//     Init
// ------ ------

pub fn init(page: Page, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        button_toggled: false,
        dropdowns: HashMap::from_iter(DropdownID::iter().map(|id| {
            let mut orders = orders.proxy(move |msg| Msg::DropdownMsg(msg, id));
            (id, dropdown::init(&mut orders))
        })),
        navbar_expanded: false,
        page,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    button_toggled: bool,
    dropdowns: HashMap<DropdownID, dropdown::Model>,
    navbar_expanded: bool,
    page: Page,
}

#[derive(Eq, PartialEq, Clone, Copy, EnumIter)]
pub enum Page {
    Buttons,
    ButtonGroups,
    Dropdowns,
    Navbars,
}

impl Default for Page {
    fn default() -> Self {
        Self::Buttons
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, EnumIter)]
pub enum DropdownID {
    MenuButton,
    MenuLink,
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    Link,
    Nested,
    NestedVertical,
    SplitButton,
    NavBarButton,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ToggleButton,
    DropdownMsg(dropdown::Msg, DropdownID),
    ToggleNavbar,
    NoOp,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleButton => model.button_toggled = !model.button_toggled,
        Msg::DropdownMsg(msg, id) => dropdown::update(
            msg,
            model.dropdowns.get_mut(&id).unwrap(),
            &mut orders.proxy(move |msg| Msg::DropdownMsg(msg, id)),
        ),
        Msg::ToggleNavbar => {
            model.navbar_expanded = !model.navbar_expanded;
        }
        Msg::NoOp => {
            orders.skip();
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["components-page"],
        navbar(),
        div![
            C!["container-fluid"],
            div![C!["row"], sidebar(model.page), main(model),]
        ]
    ]
}

fn navbar() -> Node<Msg> {
    navbar::NavBar::new(plain!("Seed Bootstrap"), "/components")
        .add_attrs(C!["navbar-dark", "bg-dark", "py-2", "shadow"])
        .view()
}

fn sidebar(page: Page) -> Node<Msg> {
    nav![
        C!["col-md-3", "col-lg-2", "bg-light"],
        div![
            style! {
                St::MarginTop => px(100),
                St::Top => px(100),
            },
            C!["sticky-top"],
            sidebar_items(page),
        ]
    ]
}

fn sidebar_items(selected_page: Page) -> Node<Msg> {
    button_group::ButtonGroup::new("sidebar")
        .add_attrs(C!["w-100"])
        .vertical()
        .content(
            Page::iter()
                .map(|page| {
                    let title = match page {
                        Page::Buttons => "Buttons",
                        Page::ButtonGroups => "Button Groups",
                        Page::Dropdowns => "Dropdowns",
                        Page::Navbars => "Navbars",
                    };
                    button::Button::new(title)
                        .add_attrs(C!["text-left"])
                        .light()
                        .a(
                            Route::Components(page).to_href().to_string(),
                            button::Role::None,
                        )
                        .view_toggle(selected_page == page)
                })
                .collect::<Vec<_>>(),
        )
        .view()
}

fn main(model: &Model) -> Node<Msg> {
    div![
        C![
            "col-md-9",
            "shadow",
            "p-3",
            "bg-white",
            "rounded",
            "min-vh-100"
        ],
        navbar(),
        div![C!["pt-4"]],
        match &model.page {
            Page::Buttons => buttons::view(model),
            Page::ButtonGroups => button_groups::view(model),
            Page::Dropdowns => dropdowns::view(model),
            Page::Navbars => navbars::view(model),
        }
    ]
}

struct ExampleBox {
    title: &'static str,
    content: Vec<Node<Msg>>,
    code: &'static str,
    overflow_x_auto: bool,
}

impl ExampleBox {
    fn new(title: &'static str) -> Self {
        Self {
            title,
            content: Vec::new(),
            code: "",
            overflow_x_auto: false,
        }
    }

    fn overflow_x_auto(mut self) -> Self {
        self.overflow_x_auto = true;
        self
    }

    fn content(mut self, content: impl IntoNodes<Msg>) -> Self {
        self.content = content.into_nodes();
        self
    }

    fn code(mut self, code: &'static str) -> Self {
        self.code = code;
        self
    }

    fn view(self) -> Node<Msg> {
        div![
            h2![self.title],
            div![
                C!["shadow-sm", "p-4", "mb-5", "mt-2"],
                self.content,
                pre![C!["mt-4", "mb-0",], code_block(self.code),]
            ]
        ]
    }
}

impl UpdateEl<Msg> for ExampleBox {
    fn update_el(self, el: &mut El<Msg>) {
        self.view().update_el(el)
    }
}

fn code_block(code: &str) -> Node<Msg> {
    custom![
        Tag::from("code-block"),
        attrs! {
            At::from("lang") => "rust",
            At::from("code") => code,
        }
    ]
}
