use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OnboardingProps {
    /// Opt in to the browser's geolocation.
    pub on_use_location: Callback<()>,
    /// Skip geolocation and point at a spot on the map instead.
    pub on_pick_spot: Callback<()>,
}

#[function_component(Onboarding)]
pub fn onboarding(props: &OnboardingProps) -> Html {
    let use_location = {
        let cb = props.on_use_location.clone();
        Callback::from(move |_| cb.emit(()))
    };
    let pick_spot = {
        let cb = props.on_pick_spot.clone();
        Callback::from(move |_| cb.emit(()))
    };
    html! {
        <div class="onb">
            <div class="onb-blob-a"></div>
            <div class="onb-blob-b"></div>
            <div class="onb-body">
                <div class="onb-logo">
                    <div class="onb-logo-badge"><span class="mi">{"coffee"}</span></div>
                    <div>
                        <div class="onb-app-name">{"Coffee Breaks"}</div>
                        <div class="onb-tagline">{"find places for refreshment"}</div>
                    </div>
                </div>
                <h1 class="onb-h1">{"Good places to take breaks"}</h1>
                <p class="onb-p">
                    {"Places with coffee, food, bathrooms, places to sit at \
                      shops, stores, malls, parks & more — sorted by what's \
                      closest to you."}
                </p>
            </div>
            <div class="onb-foot">
                <div class="onb-features">
                    <div class="onb-feature">
                        <span class="mi">{"clean_hands"}</span>
                        <div class="onb-feature-text">{"Clean, comfortable places"}</div>
                    </div>
                    <div class="onb-feature">
                        <span class="mi">{"near_me"}</span>
                        <div class="onb-feature-text">{"Directions in one tap"}</div>
                    </div>
                </div>
                <button class="onb-cta" onclick={use_location}>
                    <span class="mi">{"my_location"}</span>{"Use my location"}
                </button>
                <button class="onb-alt" onclick={pick_spot}>
                    <span class="mi">{"pin_drop"}</span>{"Pick a spot on the map instead"}
                </button>
                <div class="onb-note">
                    {"Location is optional — we only use it to sort nearby places."}
                </div>
            </div>
        </div>
    }
}
