use super::{DropdownID, ExampleBox, Model, Msg};
use seed::{prelude::*, *};
use seed_bootstrap::components::button::Button;
use seed_bootstrap::components::button_group::ButtonGroup;
use seed_bootstrap::components::dropdown::{Dropdown, Item};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["pt-5"],
        h1!["Button groups"],
        hr![],
        ExampleBox::new("Basic example")
            .content(div![
                ButtonGroup::new("Basic example")
                    .content(vec![
                        Button::new("Left").view(),
                        Button::new("Middle").view(),
                        Button::new("Right").view(),
                    ])
            ])
            .code(
r#"ButtonGroup::new("Basic example")
    .content(vec![
        Button::new("Left").view(),
        Button::new("Middle").view(),
        Button::new("Right").view(),
    ])"#
            ),
        ExampleBox::new("Button toolbar")
            .content(div![
                ButtonGroup::new("Toolbar with button groups")
                    .toolbar()
                    .content(vec![
                        ButtonGroup::new("First group")
                            .add_attrs(C!["mr-2"])
                            .content(vec![
                                Button::new("1").secondary().view(),
                                Button::new("2").secondary().view(),
                                Button::new("3").secondary().view(),
                                Button::new("4").secondary().view(),
                            ]).view(),
                        ButtonGroup::new("Second group")
                            .add_attrs(C!["mr-2"])
                            .content(vec![
                                Button::new("5").secondary().view(),
                                Button::new("6").secondary().view(),
                                Button::new("7").secondary().view(),
                            ]).view(),
                        ButtonGroup::new("Third group")
                            .content(
                                Button::new("8").secondary().view()
                            ).view(),
                    ])
            ])
            .code(
r#"ButtonGroup::new("Toolbar with button groups")
    .toolbar()
    .content(vec![
        ButtonGroup::new("First group")
            .add_attrs(C!["mr-2"])
            .content(vec![
                Button::new("1").secondary().view(),
                Button::new("2").secondary().view(),
                Button::new("3").secondary().view(),
                Button::new("4").secondary().view(),
            ]).view(),
        ButtonGroup::new("Second group")
            .add_attrs(C!["mr-2"])
            .content(vec![
                Button::new("5").secondary().view(),
                Button::new("6").secondary().view(),
                Button::new("7").secondary().view(),
            ]).view(),
        ButtonGroup::new("Third group")
            .content(
                Button::new("8").secondary().view()
            ).view(),
    ])"#
            ),
        ExampleBox::new("Nesting")
            .content(div![
                ButtonGroup::new("Button group with nested dropdown")
                    .content(vec![
                        Button::new("1").view(),
                        Button::new("2").view(),
                        ButtonGroup::default()
                            .content(
                                Dropdown::new("Dropdown ")
                                    .items(vec![Item::a("Dropdown link", (), "#"), Item::a("Dropdown link", (), "#")])
                                    .add_on_item_click(|event, _| { event.prevent_default(); Msg::NoOp })
                                    .view(&model.dropdowns[&DropdownID::Nested], |msg| Msg::DropdownMsg(msg, DropdownID::Nested))
                            ).view(),
                    ])
            ])
            .code(
r##"ButtonGroup::new("Button group with nested dropdown")
    .content(vec![
        Button::new("1").view(),
        Button::new("2").view(),
        ButtonGroup::default()
            .content(
                Dropdown::new("Dropdown ")
                    .items(vec![Item::a("Dropdown link", (), "#"), Item::a("Dropdown link", (), "#")])
                    .add_on_item_click(|event, _| { event.prevent_default(); Msg::NoOp })
                    .view(&model.dropdowns[&DropdownID::Nested], |msg| Msg::DropdownMsg(msg, DropdownID::Nested))
            ).view(),
    ])"##
            ),
        ExampleBox::new("Vertical variation")
            .content(div![
                ButtonGroup::new("Button group with nested dropdown - vertical")
                    .vertical()
                    .content(vec![
                        Button::new("Button").secondary().view(),
                        Button::new("Button").secondary().view(),
                        ButtonGroup::default()
                            .content(
                                Dropdown::new("Dropdown ")
                                    .items(vec![Item::a("Dropdown link", (), "#"), Item::a("Dropdown link", (), "#")])
                                    .update_toggle(|toggle| toggle.secondary())
                                    .add_on_item_click(|event, _| { event.prevent_default(); Msg::NoOp })
                                    .view(&model.dropdowns[&DropdownID::NestedVertical], |msg| Msg::DropdownMsg(msg, DropdownID::NestedVertical))
                            ).view(),
                    ])
            ])
            .code(
r##"ButtonGroup::new("Button group with nested dropdown - vertical")
    .vertical()
    .content(vec![
        Button::new("Button").secondary().view(),
        Button::new("Button").secondary().view(),
        ButtonGroup::default()
            .content(
                Dropdown::new("Dropdown ")
                    .items(vec![Item::a("Dropdown link", (), "#"), Item::a("Dropdown link", (), "#")])
                    .update_toggle(|toggle| toggle.secondary())
                    .add_on_item_click(|event, _| { event.prevent_default(); Msg::NoOp })
                    .view(&model.dropdowns[&DropdownID::NestedVertical], |msg| Msg::DropdownMsg(msg, DropdownID::NestedVertical))
            ).view(),
    ])"##
            ),
    ]
}
