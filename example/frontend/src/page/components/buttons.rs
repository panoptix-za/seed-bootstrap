use super::{ExampleBox, Model, Msg};
use seed::{prelude::*, *};
use seed_bootstrap::components::button::{Button, Role, Type};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["pt-5"],
        h1!["Buttons"],
        hr![],
        ExampleBox::new("Styles")
            .content(div![
                Button::new("Primary"),
                " ",
                Button::new("Secondary").secondary(),
                " ",
                Button::new("Success").success(),
                " ",
                Button::new("Danger").danger(),
                " ",
                Button::new("Warning").warning(),
                " ",
                Button::new("Info").info(),
                " ",
                Button::new("Light").light(),
                " ",
                Button::new("Dark").dark(),
                " ",
                Button::new("Link").link(),
            ])
            .code(
r#"Button::new("Primary"),
Button::new("Secondary").secondary(),
Button::new("Success").success(),
Button::new("Danger").danger(),
Button::new("Warning").warning(),
Button::new("Info").info(),
Button::new("Light").light(),
Button::new("Dark").dark(),
Button::new("Link").link(),"#
            ),
        ExampleBox::new("Disable text wrapping")
            .overflow_x_auto()
            .content(div![
                Button::new("Lorem ipsum dolor sit amet, consectetuer adipiscing elit."),
                div![C!["p-1"]],
                Button::new("Lorem ipsum dolor sit amet, consectetuer adipiscing elit.").text_no_wrap(),
            ])
            .code(
r#"Button::new("Lorem ipsum dolor sit amet, consectetuer adipiscing elit."),
Button::new("Lorem ipsum dolor sit amet, consectetuer adipiscing elit.").text_no_wrap(),"#
            ),
        ExampleBox::new("Elements")
            .content(div![
                Button::new("Link").a("#", Role::Button),
                " ",
                Button::new("Button").button(Type::Submit),
                " ",
                Button::new("Input").input(Type::Button),
                " ",
                Button::new("Submit").input(Type::Submit),
                " ",
                Button::new("Reset").input(Type::Reset),
                " ",
                Button::new("Label").label().add_attrs(C!["mb-0"]),
            ])
            .code(
r##"Button::new("Link").a("#", Role::Button),
Button::new("Button").button(Type::Submit),
Button::new("Input").input(Type::Button),
Button::new("Submit").input(Type::Submit),
Button::new("Reset").input(Type::Reset),
Button::new("Label").label().add_attrs(C!["mb-0"]),"##
            ),
        ExampleBox::new("Outline")
            .content(div![
                Button::new("Primary").outline(),
                " ",
                Button::new("Secondary").secondary().outline(),
                " ",
                Button::new("Success").success().outline(),
                " ",
                Button::new("Danger").danger().outline(),
                " ",
                Button::new("Warning").warning().outline(),
                " ",
                Button::new("Info").info().outline(),
                " ",
                Button::new("Light").light().outline(),
                " ",
                Button::new("Dark").dark().outline(),
                " ",
                Button::new("Link").link().outline(),
            ])
            .code(
r#"Button::new("Primary").outline(),
Button::new("Secondary").secondary().outline(),
Button::new("Success").success().outline(),
Button::new("Danger").danger().outline(),
Button::new("Warning").warning().outline(),
Button::new("Info").info().outline(),
Button::new("Light").light().outline(),
Button::new("Dark").dark().outline(),
Button::new("Link").link().outline(),"#
            ),
        ExampleBox::new("Size")
            .content(div![
                Button::new("Small").small(),
                " ",
                Button::new("Medium"),
                " ",
                Button::new("Large").large(),
            ])
            .code(
r#"BButton::new("Small").small(),
Button::new("Medium"),
Button::new("Large").large(),"#
            ),
        ExampleBox::new("Block")
            .content(div![
                Button::new("Block").block(),
            ])
            .code(
r#"Button::new("Block").block(),"#
            ),
        ExampleBox::new("Disabled")
            .content(div![
                Button::new("Disabled button").disabled(true),
                " ",
                Button::new("Disabled link").a("#", Role::None).disabled(true),
            ])
            .code(
r##"Button::new("Disabled button").disabled(true),
Button::new("Disabled link").a("#", Role::None).disabled(true),"##
            ),
        ExampleBox::new("Toggle")
            .content(div![
                Button::new("Toggle").add_on_click(|_| Msg::ToggleButton).view_toggle(model.button_toggled),
            ])
            .code(
r#"Button::new("Toggle").add_on_click(|_| Msg::ToggleButton).view_toggle(model.button_toggled),"#
            ),
        ExampleBox::new("Content")
            .content(div![
                Button::default().content(h3!["Content"]),
            ])
            .code(
r#"Button::default().content(h3!["Content"]),"#
            ),
    ]
}
