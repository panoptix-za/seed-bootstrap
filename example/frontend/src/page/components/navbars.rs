use super::{DropdownID, ExampleBox, Model, Msg};
use seed::{prelude::*, *};
use seed_bootstrap::button::{self, Button};
use seed_bootstrap::dropdown::{self, Dropdown};
use seed_bootstrap::navbar::{Nav, NavBar, NavLink};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["pt-5"],
        h1!["Navbars"],
        hr![],
        ExampleBox::new("Basic example")
            .content(div![
                NavBar::new(plain!("Navbar"), "#")
                    .fixed_top(false)
                    .update_toggle(|toggle| toggle.add_on_click(|_| Msg::ToggleNavbar))
                    .add_attrs(C!["navbar-expand-lg", "navbar-light", "bg-light"])
                    .content(vec![
                        Nav::default()
                            .add_attrs(C!["mr-auto"])
                            .content(vec![
                                NavLink::new("Home", "#").active(true).view(),
                                NavLink::new("Link", "#").view(),
                                NavLink::new("Disabled", "#").disabled(true).view(),
                                Dropdown::new("Dropdown")
                                    .items(vec![
                                        dropdown::Item::button("Action", ()),
                                        dropdown::Item::button("Another action", ()),
                                        dropdown::Item::divider(),
                                        dropdown::Item::button("Something else here", ()),
                                    ])
                                    .view_in_nav(&model.dropdowns[&DropdownID::NavBarButton], |msg| Msg::DropdownMsg(msg, DropdownID::NavBarButton)),
                            ]
                            ).view(),
                        // --- search ---
                        form![C!["form-inline", "my-2", "my-lg-0"],
                            input![C!["form-control", "mr-sm-2"], attrs!{At::Type => "text", At::Placeholder => "Search", At::from("aria-label") => "Search"}],
                            Button::new("Search").outline().success().button(button::Type::Submit).add_attrs(C!["my-2", "my-sm-0"]),
                        ]
                    ])
                    .view_collapsable(model.navbar_expanded, "navbarsExampleDefault")
            ])
            .code(
r##"NavBar::new(plain!("Navbar"), "#")
    .fixed_top(false)
    .update_toggle(|toggle| toggle.add_on_click(|_| Msg::ToggleNavbar))
    .add_attrs(C!["navbar-expand-lg", "navbar-light", "bg-light"])
    .content(vec![
        Nav::default()
            .add_attrs(C!["mr-auto"])
            .content(vec![
                NavLink::new("Home", "#").active(true).view(),
                NavLink::new("Link", "#").view(),
                NavLink::new("Disabled", "#").disabled(true).view(),
                Dropdown::new("Dropdown")
                    .items(vec![
                        dropdown::Item::button("Action", ()),
                        dropdown::Item::button("Another action", ()),
                        dropdown::Item::divider(),
                        dropdown::Item::button("Something else here", ()),
                    ])
                    .view_in_nav(&model.dropdowns[&DropdownID::NavBarButton], |msg| Msg::DropdownMsg(msg, DropdownID::NavBarButton)),
            ]
            ).view(),
        // --- search ---
        form![C!["form-inline", "my-2", "my-lg-0"],
            input![C!["form-control", "mr-sm-2"], attrs!{At::Type => "text", At::Placeholder => "Search", At::from("aria-label") => "Search"}],
            Button::new("Search").outline().success().button(button::Type::Submit).add_attrs(C!["my-2", "my-sm-0"]),
        ]
    ])
    .view_collapsable(model.navbar_expanded, "navbarsExampleDefault")"##
            ),
    ]
}
