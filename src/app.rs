use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=move |cx| view! { cx, <Outlet /> }>
                        <Route path="" view=move |cx| view! { cx, <Outlet /> }>
                            <Route path=""
                                view=move |cx| view! { cx, <HomePage /> }
                            />
                        </Route>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

async fn example() -> String {
    return "example".to_string();
}

/// WITH THIS IT RENDERS AGAIN
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let counter = create_rw_signal(cx, 1);

    let rs = create_resource(cx, move || counter.get(), move |_| example());

    view! { cx,
        <h1>"hydration error with outlet!"</h1>
        <div class="w-full" >
            //BODY
            <button on:click=move |_| counter.update(|n| *n += 1)>"Refresh"</button>
            <Suspense fallback=move || view! { cx, <p>"Loading (Suspense Fallback)..."</p> }>
            {move || { rs.read().map(|s| view! { cx, <p>{s}</p> })}}
            </Suspense>
        </div>
    }
}
