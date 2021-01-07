mod page;

use seed::prelude::*;
use std::borrow::Cow;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    Model {
        page: Route::from_url(url).to_page(orders),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    page: Page,
}

// ------ Page ------

enum Page {
    Components(page::components::Model),
    NotFound,
}

// ------ Route ------

pub enum Route {
    Components(page::components::Page),
    Unknown(Url),
}

impl Route {
    fn to_href(&self) -> Cow<str> {
        let mut url = Url::default();
        match self {
            Self::Components(component_page) => {
                url = url.add_path_part("components");
                let slug = match component_page {
                    page::components::Page::Buttons => "buttons",
                    page::components::Page::ButtonGroups => "button-groups",
                    page::components::Page::Dropdowns => "dropdowns",
                    page::components::Page::Navbars => "navbars",
                };
                url = url.add_path_part(slug);
            }
            Self::Unknown(unknown_url) => url = unknown_url.clone(),
        }
        url.to_string().into()
    }

    fn to_page(&self, orders: &mut impl Orders<Msg>) -> Page {
        match self {
            Self::Components(page) => Page::Components(page::components::init(
                *page,
                &mut orders.proxy(Msg::Components),
            )),
            Self::Unknown(_) => Page::NotFound,
        }
    }

    fn from_url(mut url: Url) -> Route {
        match url.next_path_part() {
            None => Self::Components(page::components::Page::default()),
            Some("components") => match url.next_path_part() {
                Some("buttons") => Self::Components(page::components::Page::Buttons),
                Some("button-groups") => Self::Components(page::components::Page::ButtonGroups),
                Some("dropdowns") => Self::Components(page::components::Page::Dropdowns),
                Some("navbars") => Self::Components(page::components::Page::Navbars),
                _ => Self::Components(page::components::Page::default()),
            },
            _ => Self::Unknown(url),
        }
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    Components(page::components::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            let route = Route::from_url(url);
            model.page = route.to_page(orders);
        }
        Msg::Components(msg) => {
            if let Page::Components(model) = &mut model.page {
                page::components::update(msg, model, &mut orders.proxy(Msg::Components));
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    match &model.page {
        Page::Components(model) => page::components::view(model)
            .into_nodes()
            .map_msg(Msg::Components),
        Page::NotFound => page::not_found::view().into_nodes(),
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
