use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="home-navbar">
            <div>
                <h1 class="home-title">"Cosmicli"</h1>
            </div>

            <div class="content-center">
               <MenuToggler/>
            </div>
        </nav>
    }
}

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
       <h1> "Welcome to Cosmicli" </h1>
       <p> "Time is a fundamental aspect of our lives" </p>
    }
}

/// Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Nav/>
        <Dashboard/>
    }
}

/// Menu Toggler
#[component]
pub fn MenuToggler() -> impl IntoView {
    // Creates Read & Write signals for the toggler
    let (is_pressed, set_is_pressed) = create_signal(true);

    view! {
        // toggles on each click
        <button on:click=move |_| { set_is_pressed(!is_pressed.get()) }>
            // Shows a Menu Icon based on the Read Signal (is_pressed)
            // If True then Menu Open, else fallback to Menu Closed
            <Show when=move || {is_pressed.get()} fallback=|| view! {<MenuClosed class="home-icon"/>}>
                <MenuOpen class="home-icon"/>
            </Show>
        </button>
    }
}

/// MenuOpen Icon
#[component]
pub fn MenuOpen(class: &'static str) -> impl IntoView {
    view! {
            <svg class={class} xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 24 24">
    <path d="M 2 5 L 2 7 L 22 7 L 22 5 L 2 5 z M 2 11 L 2 13 L 22 13 L 22 11 L 2 11 z M 2 17 L 2 19 L 22 19 L 22 17 L 2 17 z"></path>
    </svg>
        }
}

/// Menu Close Icon
#[component]
pub fn MenuClosed(class: &'static str) -> impl IntoView {
    view! {
            <svg class={class} xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 50 50">
    <path d="M 7.71875 6.28125 L 6.28125 7.71875 L 23.5625 25 L 6.28125 42.28125 L 7.71875 43.71875 L 25 26.4375 L 42.28125 43.71875 L 43.71875 42.28125 L 26.4375 25 L 43.71875 7.71875 L 42.28125 6.28125 L 25 23.5625 Z"></path>
    </svg>
        }
}
