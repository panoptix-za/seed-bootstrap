use super::{DropdownID, ExampleBox, Model, Msg};
use seed::{prelude::*, *};
use seed_bootstrap::button::{self, Button};
use seed_bootstrap::button_group::ButtonGroup;
use seed_bootstrap::dropdown::{Dropdown, Item};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["pt-5"],
        h1!["Dropdowns"],
        div![C!["p-2"],
            "Add PopperJS to your index.html:",
            div![
                r#"<script src="https://unpkg.com/@popperjs/core@2"></script>"#
            ]
        ],
        div![C!["p-2"],
            "Note 1: Boilerplate will be reduced once features like React Hooks (WIP) are integrated into Seed.",
        ],
        div![C!["p-2"],
            "Note 2: PopperJS initiation can cause popup flash on the first popup render. \
            It should be mitigated once the positioning algorithm is rewritten (in Rust) or by Seed (React) hooks.",
        ],
        hr![],
        ExampleBox::new("Single button")
            .content(div![
                Dropdown::new("Dropdown button ")
                    .items(vec![
                        Item::button("Action", ()),
                        Item::button("Another action", ()),
                        Item::button("Something else here", ())
                    ])
                    .view(&model.dropdowns[&DropdownID::MenuButton], |msg| Msg::DropdownMsg(msg, DropdownID::MenuButton)),
            ])
            .code(
r#"pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    Model {
        dropdown_model: dropdown::init(&mut orders.proxy(Msg::DropdownMsg)),
    }
}

pub struct Model {
    dropdown: dropdown::Model,
}

pub enum Msg {
    DropdownMsg(dropdown::Msg),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::DropdownMsg(msg) => {
            dropdown::update(msg, &mut model.dropdown_model, &mut orders.proxy(Msg::DropdownMsg))
        },
    }
}

Dropdown::new("Dropdown button ")
    .items(vec![
        Item::button("Action", ()),
        Item::button("Another action", ()),
        Item::button("Something else here", ())
    ])
    .view(dropdown_model, Msg::DropdownMsg),"#
            ),
        ExampleBox::new("Single button - Link")
            .content(div![
                Dropdown::new("Dropdown link ")
                    .items(vec![
                        Item::button("Action", ()),
                        Item::button("Another action", ()),
                        Item::button("Something else here", ()),
                    ])
                    .update_toggle(|toggle| toggle.a("#", button::Role::Button))
                    .view(&model.dropdowns[&DropdownID::MenuLink], |msg| Msg::DropdownMsg(msg, DropdownID::MenuLink)),
                ])
            .code(
r##"Dropdown::new("Dropdown link ")
    .items(vec![
        Item::button("Action", ()),
        Item::button("Another action", ()),
        Item::button("Something else here", ()),
    ])
    .update_toggle(|toggle| toggle.a("#", button::Role::Button))
    .view(&model.dropdowns[&DropdownID::MenuLink], |msg| Msg::DropdownMsg(msg, DropdownID::MenuLink)),"##
            ),
        ExampleBox::new("Single button - Styles")
            .content(div![
                ButtonGroup::default()
                    .content(vec![
                        Dropdown::new("Primary")
                            .items(vec![
                                Item::button("Action", ()),
                                Item::button("Another action", ()),
                                Item::button("Something else here", ()),
                                Item::divider(),
                                Item::button("Separated button", ()),
                            ])
                            .view(&model.dropdowns[&DropdownID::Primary], |msg| Msg::DropdownMsg(msg, DropdownID::Primary)),
                        plain!["\u{00a0}"],
                        Dropdown::new("Secondary")
                            .items(vec![
                                Item::button("Action", ()),
                                Item::button("Another action", ()),
                                Item::button("Something else here", ()),
                                Item::divider(),
                                Item::button("Separated button", ()),
                            ])
                            .update_toggle(|toggle| toggle.secondary())
                            .view(&model.dropdowns[&DropdownID::Secondary], |msg| Msg::DropdownMsg(msg, DropdownID::Secondary)),
                        plain!["\u{00a0}"],
                        Dropdown::new("Success")
                            .items(vec![
                                Item::button("Action", ()),
                                Item::button("Another action", ()),
                                Item::button("Something else here", ()),
                                Item::divider(),
                                Item::button("Separated button", ()),
                            ])
                            .update_toggle(|toggle| toggle.success())
                            .view(&model.dropdowns[&DropdownID::Success], |msg| Msg::DropdownMsg(msg, DropdownID::Success)),
                        plain!["\u{00a0}"],
                        Dropdown::new("Info")
                            .items(vec![
                                Item::button("Action", ()),
                                Item::button("Another action", ()),
                                Item::button("Something else here", ()),
                                Item::divider(),
                                Item::button("Separated button", ()),
                            ])
                            .update_toggle(|toggle| toggle.info())
                            .view(&model.dropdowns[&DropdownID::Info], |msg| Msg::DropdownMsg(msg, DropdownID::Info)),
                        plain!["\u{00a0}"],
                        Dropdown::new("Warning")
                            .items(vec![
                                Item::button("Action", ()),
                                Item::button("Another action", ()),
                                Item::button("Something else here", ()),
                                Item::divider(),
                                Item::button("Separated button", ()),
                            ])
                            .update_toggle(|toggle| toggle.warning())
                            .view(&model.dropdowns[&DropdownID::Warning], |msg| Msg::DropdownMsg(msg, DropdownID::Warning)),
                        plain!["\u{00a0}"],
                        Dropdown::new("Danger")
                            .items(vec![
                                Item::button("Action", ()),
                                Item::button("Another action", ()),
                                Item::button("Something else here", ()),
                                Item::divider(),
                                Item::button("Separated button", ()),
                            ])
                            .update_toggle(|toggle| toggle.danger())
                            .view(&model.dropdowns[&DropdownID::Danger], |msg| Msg::DropdownMsg(msg, DropdownID::Danger)),
                        plain!["\u{00a0}"],
                        Dropdown::new("Link")
                            .items(vec![
                                Item::button("Action", ()),
                                Item::button("Another action", ()),
                                Item::button("Something else here", ()),
                                Item::divider(),
                                Item::button("Separated button", ()),
                            ])
                            .update_toggle(|toggle| toggle.link())
                            .view(&model.dropdowns[&DropdownID::Link], |msg| Msg::DropdownMsg(msg, DropdownID::Link)),
                    ])
            ])
            .code(
r#"Dropdown::new("Danger")
    .items(vec![
        Item::button("Action", ()),
        Item::button("Another action", ()),
        Item::button("Something else here", ()),
        Item::divider(),
        Item::button("Separated button", ()),
    ])
    .update_toggle(|toggle| toggle.danger())
    .view(&model.dropdowns[&DropdownID::Danger], |msg| Msg::DropdownMsg(msg, DropdownID::Danger)),"#
            ),
        ExampleBox::new("Split button")
            .content(div![
                ButtonGroup::default()
                    .content(nodes![
                        Button::new("Split button").view(),
                        Dropdown::default()
                            .items(vec![
                                Item::button("Action", ()),
                                Item::button("Another action", ()),
                                Item::button("Something else here", ()),
                                Item::divider(),
                                Item::button("Separated button", ()),
                            ])
                            .view_in_split_button(
                                &model.dropdowns[&DropdownID::SplitButton],
                                |msg| Msg::DropdownMsg(msg, DropdownID::SplitButton),
                                "Toggle dropdown"
                            ),
                    ]).view()
            ])
            .code(
r#"ButtonGroup::default()
    .content(nodes![
        Button::new("Split button").view(),
        Dropdown::default()
            .items(vec![
                Item::button("Action", ()),
                Item::button("Another action", ()),
                Item::button("Something else here", ()),
                Item::divider(),
                Item::button("Separated button", ()),
            ])
            .view_in_split_button(
                &model.dropdowns[&DropdownID::SplitButton],
                |msg| Msg::DropdownMsg(msg, DropdownID::SplitButton),
                "Toggle dropdown"
            ),
    ]).view()"#
            ),
    ]
}
