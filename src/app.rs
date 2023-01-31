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
                    <Route path="/" view=|cx| view! { cx,
                        <HomePage/>
                     }/>
                    <Route path="/simple" view=|cx| view! { cx,
                        <HomePage2/>
                     }/>
                </Routes>
            </main>
        </Router>
    }
}

async fn _fetch_cats() -> Option<Vec<String>> {
    Some(vec!["da".to_string()])
}

/// WITH THIS IT RENDERS AGAIN
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let counter = create_rw_signal(cx, 1);
    let items_resource = create_resource(cx, move || counter.get(), move |_| _fetch_cats());

    view! { cx,
        <A href="/simple">"Simple"</A>
        <h1>"Double render error with hydration error!"</h1>
        <button on:click=move |_| counter.update(|n| *n += 1)>"Refresh"</button>
        <Suspense fallback=move || view! { cx, <p>"Loading (Suspense Fallback)..."</p> }>
            {move || {
                items_resource.read().map(|data| match data {
                    None => view! { cx,  <p>"Error"</p>},
                    Some(_fetched_items) => view! { cx,
                        <p>"dada"</p>
                    },
                })
            }}
        </Suspense>


    }
}

/// WITH THIS ITS JUST HYDRATION ERROR
#[component]
fn HomePage2(cx: Scope) -> impl IntoView {
    view! { cx,
        <A href="/">"Double render with hydration error"</A>
        <h1>"Simple Hydration error!"</h1>
        <Suspense fallback=move || view! { cx, <p>"Loading (Suspense Fallback)..."</p> }>
            <p>"asdafasfaf"</p>
        </Suspense>
    }
}
