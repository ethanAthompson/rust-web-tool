#![doc = include_str!("./docs/app.md")]

mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use pages::about::About;
use pages::communications::Communications;
use pages::feedback::Feedback;
use pages::home::Home;
use pages::notfound::NotFound;
use pages::stats::Stats;
use pages::tool::Tool;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Cosmicli"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="about" view=About/>
                    <Route path="feedback" view=Feedback/>
                    <Route path="stats" view=Stats/>
                    <Route path="comms" view=Communications/>
                    <Route path="tools" view=Tool/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
