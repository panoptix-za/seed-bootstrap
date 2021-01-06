use super::button::Button;
use seed::{prelude::*, *};
use std::{borrow::Cow, rc::Rc};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement, MouseEvent};

type PopperInstance = JsValue;

// ------ ------
//     Init
// ------ ------

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    Model {
        expanded: false,
        toggle: ElRef::default(),
        popup: ElRef::default(),
        popup_style: String::new(),
        id: Uuid::new_v4().to_string(),
        popper_data: None,
        _window_click_stream: orders
            .stream_with_handle(streams::window_event(Ev::Click, |event| {
                Msg::Collapse(Some(event.target().expect("event target")))
            })),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    expanded: bool,
    toggle: ElRef<HtmlElement>,
    popup: ElRef<HtmlElement>,
    popup_style: String,
    id: String,
    popper_data: Option<PopperData>,
    _window_click_stream: StreamHandle,
}

// ------ PopperData ------

struct PopperData {
    popper_instance: PopperInstance,
    _on_apply_styles: Closure<dyn FnMut(String)>,
}

impl Drop for PopperData {
    fn drop(&mut self) {
        destroy_popper(&self.popper_instance)
    }
}

// ------ ------
//    Update
// ------ ------
#[derive(Debug)]
pub enum Msg {
    ToggleClicked,
    UpdatePopper,
    Collapse(Option<EventTarget>),
    OnApplyStyles(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleClicked => {
            model.expanded = !model.expanded;
            if model.expanded {
                if model.popper_data.is_none() {
                    model.popper_data = Some(show_popper(&model.toggle, &model.popup, orders));
                }
                orders.after_next_render(|_| Msg::UpdatePopper);
            }
        }
        Msg::UpdatePopper => {
            if let Some(popper_data) = &model.popper_data {
                update_popper(&popper_data.popper_instance);
            }
        }
        Msg::Collapse(event_target) => {
            if model.expanded {
                match event_target {
                    Some(event_target) => {
                        let target = event_target
                            .dyn_into::<web_sys::Node>()
                            .expect("event_target into node");
                        let is_target_in_toggle = model
                            .toggle
                            .get()
                            .expect("get dropdown toggle")
                            .contains(Some(&target));
                        let is_target_in_popup = model
                            .popup
                            .get()
                            .expect("get dropdopwn popup")
                            .contains(Some(&target));
                        if !is_target_in_toggle && !is_target_in_popup {
                            model.expanded = false;
                        }
                    }
                    None => {
                        model.expanded = false;
                    }
                }
            }
        }
        Msg::OnApplyStyles(popup_style) => model.popup_style = popup_style,
    }
}

fn show_popper(
    toggle: &ElRef<HtmlElement>,
    popup: &ElRef<HtmlElement>,
    orders: &mut impl Orders<Msg>,
) -> PopperData {
    let (app, msg_mapper) = (orders.clone_app(), orders.msg_mapper());

    let closure =
        Closure::new(move |popup_style| app.update(msg_mapper(Msg::OnApplyStyles(popup_style))));
    let closure_as_js_value = closure.as_ref().clone();

    let popper_instance = create_popper(
        toggle.get().expect("get toggle"),
        popup.get().expect("get popup"),
        closure_as_js_value,
    );
    PopperData {
        popper_instance,
        _on_apply_styles: closure,
    }
}

#[wasm_bindgen(module = "/js/popper_wrapper.js")]
extern "C" {
    fn create_popper(
        toggle_element: HtmlElement,
        popup_element: HtmlElement,
        _on_apply_styles: JsValue,
    ) -> PopperInstance;

    fn update_popper(popper_instance: &PopperInstance);
    fn destroy_popper(popper_instance: &PopperInstance);
}

// ------ ------
//     View
// ------ ------

// ------ Dropdown ------

pub struct Dropdown<Ms: 'static, ItemValue> {
    id: Option<Cow<'static, str>>,
    items: Vec<Item<ItemValue>>,
    on_item_clicks: Vec<Rc<dyn Fn(MouseEvent, ItemValue) -> Ms>>,
    toggle: Button<Ms>,
}

impl<Ms: 'static, ItemValue: Clone + 'static> Dropdown<Ms, ItemValue> {
    pub fn new(title: impl Into<Cow<'static, str>>) -> Self {
        Self::default().title(title)
    }

    pub fn title(mut self, title: impl Into<Cow<'static, str>>) -> Self {
        self.toggle = self.toggle.title(title);
        self
    }

    pub fn id(mut self, id: impl Into<Cow<'static, str>>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn items(mut self, items: Vec<Item<ItemValue>>) -> Self {
        self.items = items;
        self
    }

    pub fn add_on_item_click(
        mut self,
        on_item_click: impl FnOnce(MouseEvent, ItemValue) -> Ms + Clone + 'static,
    ) -> Self {
        self.on_item_clicks.push(Rc::new(move |event, item_value| {
            on_item_click.clone()(event, item_value)
        }));
        self
    }

    pub fn update_toggle(mut self, f: impl FnOnce(Button<Ms>) -> Button<Ms>) -> Self {
        self.toggle = f(self.toggle);
        self
    }

    pub fn view(self, model: &Model, to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static) -> Node<Ms> {
        let to_msg = move |msg| to_msg.clone()(msg);
        let id = self.id.unwrap_or_else(|| model.id.clone().into());
        let on_item_clicks = self.on_item_clicks.clone();

        let toggle = self
            .toggle
            .add_attrs(id!(id.clone()))
            .add_attrs(C!["dropdown-toggle"])
            .add_attrs(attrs! {
                At::from("aria-haspopup") => "true",
                At::from("aria-expanded") => model.expanded,
            })
            .add_on_click({
                let to_msg = to_msg.clone();
                move |event| {
                    event.prevent_default();
                    to_msg(Msg::ToggleClicked)
                }
            })
            .el_ref(&model.toggle)
            .view_toggle(model.expanded);

        div![
            C!["dropdown"],
            toggle,
            div![
                el_ref(&model.popup),
                C!["dropdown-menu", IF!(model.expanded => "show")],
                attrs! {
                    At::Style => model.popup_style,
                    At::from("aria-labelledby") => id
                },
                self.items
                    .into_iter()
                    .map(move |item| { item.into_element(to_msg.clone(), on_item_clicks.clone()) })
            ],
        ]
    }

    pub fn view_in_nav(
        self,
        model: &Model,
        to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
    ) -> Node<Ms> {
        let to_msg = move |msg| to_msg.clone()(msg);
        let id = self.id.unwrap_or_else(|| model.id.clone().into());
        let on_item_clicks = self.on_item_clicks.clone();

        let toggle = self
            .toggle
            .add_attrs(id!(id.clone()))
            .add_attrs(C!["nav-link", "dropdown-toggle"])
            .add_attrs(attrs! {
                At::from("aria-haspopup") => "true",
                At::from("aria-expanded") => model.expanded,
            })
            .add_on_click({
                let to_msg = to_msg.clone();
                move |event| {
                    event.prevent_default();
                    to_msg(Msg::ToggleClicked)
                }
            })
            .el_ref(&model.toggle)
            .link()
            .view_toggle(model.expanded);

        li![
            C!["nav-item", "dropdown"],
            toggle,
            div![
                el_ref(&model.popup),
                C!["dropdown-menu", IF!(model.expanded => "show")],
                attrs! {
                    At::Style => model.popup_style,
                    At::from("aria-labelledby") => id,
                },
                self.items
                    .into_iter()
                    .map(move |item| { item.into_element(to_msg.clone(), on_item_clicks.clone()) })
            ],
        ]
    }

    pub fn view_in_split_button(
        self,
        model: &Model,
        to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
        scren_reader_title: &str,
    ) -> Vec<Node<Ms>> {
        let to_msg = move |msg| to_msg.clone()(msg);
        let id = self.id.unwrap_or_else(|| model.id.clone().into());
        let on_item_clicks = self.on_item_clicks.clone();

        let toggle = self
            .toggle
            .add_attrs(id!(id.clone()))
            .add_attrs(C!["dropdown-toggle"])
            .add_attrs(attrs! {
                At::from("aria-haspopup") => "true",
                At::from("aria-expanded") => model.expanded,
            })
            .add_attrs(C!["dropdown-toggle-split"])
            .add_on_click({
                let to_msg = to_msg.clone();
                move |event| {
                    event.prevent_default();
                    to_msg(Msg::ToggleClicked)
                }
            })
            .content(span![C!["sr-only"], scren_reader_title])
            .el_ref(&model.toggle)
            .view_toggle(model.expanded);

        vec![
            toggle,
            div![
                el_ref(&model.popup),
                C!["dropdown-menu", IF!(model.expanded => "show")],
                attrs! {
                    At::Style => model.popup_style,
                    At::from("aria-labelledby") => id
                },
                self.items
                    .into_iter()
                    .map(move |item| { item.into_element(to_msg.clone(), on_item_clicks.clone()) })
            ],
        ]
    }
}

impl<Ms, ItemValue> Default for Dropdown<Ms, ItemValue> {
    fn default() -> Self {
        Self {
            id: None,
            items: Vec::new(),
            on_item_clicks: Vec::new(),
            toggle: Button::default(),
        }
    }
}

// ------ Item ------

pub enum Item<ItemValue> {
    Button {
        title: Cow<'static, str>,
        value: ItemValue,
    },
    A {
        title: Cow<'static, str>,
        value: ItemValue,
        href: Cow<'static, str>,
    },
    Divider,
}

impl<ItemValue> Item<ItemValue> {
    pub fn button(title: impl Into<Cow<'static, str>>, value: ItemValue) -> Self {
        Self::Button {
            title: title.into(),
            value,
        }
    }

    pub fn a(
        title: impl Into<Cow<'static, str>>,
        value: ItemValue,
        href: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::A {
            title: title.into(),
            value,
            href: href.into(),
        }
    }

    pub fn divider() -> Self {
        Self::Divider
    }
}

impl<ItemValue: Clone + 'static> Item<ItemValue> {
    fn into_element<Ms: 'static>(
        self,
        to_msg: impl Fn(Msg) -> Ms + Clone + 'static,
        on_item_clicks: Vec<Rc<dyn Fn(MouseEvent, ItemValue) -> Ms>>,
    ) -> Node<Ms> {
        match self {
            Self::Button { title, value } => {
                let mut node = button![
                    C!["dropdown-item"],
                    attrs! {At::Type => "button"},
                    title,
                    ev(Ev::Click, move |_| to_msg(Msg::Collapse(None))),
                ];
                for on_item_click in on_item_clicks {
                    node.add_event_handler(mouse_ev(Ev::Click, {
                        let value = value.clone();
                        move |event| on_item_click(event, value)
                    }));
                }
                node
            }
            Self::A { title, value, href } => {
                let mut node = a![
                    C!["dropdown-item"],
                    attrs! {At::Href => href},
                    title,
                    ev(Ev::Click, move |_| to_msg(Msg::Collapse(None))),
                ];
                for on_item_click in on_item_clicks {
                    node.add_event_handler(mouse_ev(Ev::Click, {
                        let value = value.clone();
                        move |event| on_item_click(event, value)
                    }));
                }
                node
            }
            Self::Divider => div![C!["dropdown-divider"]],
        }
    }
}
