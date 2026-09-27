mod api;
mod app;
mod components;
mod glue;
mod maps_link;
mod route;

use yew::prelude::*;
use yew_router::prelude::*;

use app::App;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <BrowserRouter>
            <App />
        </BrowserRouter>
    }
}

fn main() {
    // A panic leaves a dead, often blank, page; surface it on screen instead
    // (the PWA has no devtools on a phone) and in the console.
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("panic: {info}");
        web_sys::console::error_1(&msg.as_str().into());
        glue::sb_report_error(&msg);
        glue::sb_show_rescue();
    }));
    yew::Renderer::<Root>::new().render();
}
