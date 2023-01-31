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
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

async fn fetch_cats() -> Option<Vec<String>> {
    Some(vec!["da".to_string()])
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let counter = create_rw_signal(cx, 1);

    let items_resource = create_resource(cx, move || counter.get(), move |_| fetch_cats());

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=move |_| counter.update(|n| *n += 1)>"Refresh"</button>
        <Suspense fallback=move || view! { cx, <p>"Loading (Suspense Fallback)..."</p> }>
            {move || {
                items_resource.read().map(|data| match data {
                    None => view! { cx,  <><pre>"Error"</pre></>},
                    Some(fetched_items) => view! { cx, <>
                        {
                            fetched_items.iter()
                            .map(|_src| {
                                view! { cx,
                                    <p>"dada"</p>
                                }
                            })
                            .collect::<Vec<_>>()
                        }
                    </>
                    },
                })
            }}
        </Suspense>


    }
}
