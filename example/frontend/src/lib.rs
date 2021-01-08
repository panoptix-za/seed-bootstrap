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
    Components(page::Model),
    NotFound,
}

// ------ Route ------

pub enum Route {
    Components(page::Page),
    Unknown(Url),
}

impl Route {
    fn to_href(&self) -> Cow<str> {
        let mut url = Url::default();
        match self {
            Self::Components(component_page) => {
                url = url.add_path_part("components");
                let slug = match component_page {
                    page::Page::Buttons => "buttons",
                    page::Page::ButtonGroups => "button-groups",
                    page::Page::Dropdowns => "dropdowns",
                    page::Page::Navbars => "navbars",
                };
                url = url.add_path_part(slug);
            }
            Self::Unknown(unknown_url) => url = unknown_url.clone(),
        }
        url.to_string().into()
    }

    fn to_page(&self, orders: &mut impl Orders<Msg>) -> Page {
        match self {
            Self::Components(page) => {
                Page::Components(page::init(*page, &mut orders.proxy(Msg::Components)))
            }
            Self::Unknown(_) => Page::NotFound,
        }
    }

    fn from_url(mut url: Url) -> Route {
        match url.next_path_part() {
            None => Self::Components(page::Page::default()),
            Some("components") => match url.next_path_part() {
                Some("buttons") => Self::Components(page::Page::Buttons),
                Some("button-groups") => Self::Components(page::Page::ButtonGroups),
                Some("dropdowns") => Self::Components(page::Page::Dropdowns),
                Some("navbars") => Self::Components(page::Page::Navbars),
                _ => Self::Components(page::Page::default()),
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
    Components(page::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            let route = Route::from_url(url);
            model.page = route.to_page(orders);
        }
        Msg::Components(msg) => {
            if let Page::Components(model) = &mut model.page {
                page::update(msg, model, &mut orders.proxy(Msg::Components));
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    match &model.page {
        Page::Components(model) => page::view(model).into_nodes().map_msg(Msg::Components),
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
