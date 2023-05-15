use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;
use stylist::style;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
    #[at("/counter")]
    Counter,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::HelloServer => html! { <HelloServer /> },
        Route::Counter => html! { <Counter /> },
    }
}

#[styled_component(Title)]
fn title() -> Html {

    let img_src = String::from("https://i.ibb.co/K5NKKX9/SGR4.png");

    let stylesheet = style!(
        r#"
        .title-a {
            color: orange;
            height: auto;
            font-size: 400%;
            font-family: 'Unbounded', cursive;
            font-weight: 600;
            text-align: left;
            padding: 20px;
            background-color: #2e3192;
        }
        
        
        
        .container {
            width: 30%;
            clear: both;
            color: #000000;
            font-family: 'Unbounded', cursive;
            font-size: 130%;
            text-align: left;
            padding-top: 0px;
            padding-bottom: 20px;
            padding-right: 10px;
            padding-left: 10px;
            margin: 25;
            font-weight: bold;
        }


        .image {
            width: 1.3em;
            height: auto;
            margin-left: 2%;
            vertical-align: -15%;
        }
        "#
    ).expect("Failed to mount style");

    html! {
        <div class={stylesheet}>
            <h1 class="title-a">{"sugarcube beta testing"}<a href="https://ibb.co/ZfH22Vh"><img class="image" alt="SGR4" src={img_src}/></a></h1>
        </div>
    }
}

#[styled_component(App)]
fn app() -> Html {
    
    html! {
        <BrowserRouter>
            <Title />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[styled_component(Home)]
fn home() -> Html {
    let stylesheet = style!(
        r#"                
        .container-text {
            width: 90%;
            clear: both;
            color: #000000;
            font-family: 'Unbounded', cursive;
            font-weight: 300;
            font-size: 130%;
            text-align: left;
            padding: 10px;
        }

        .subtitle-a {
            color: #ff674d;
            font-size: 175%;
            font-family: 'Unbounded', cursive;
            font-weight: 400;
            text-align: left;
            padding: 20px;
        }"#
    ).expect("Failed to mount style");
    
    html! {
        <div class={stylesheet}>
        <h2 class="subtitle-a">{"Welcome to sugarcube's homepage!"}</h2>
        
        <p class="container-text">{"

        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam sit amet est ut turpis ullamcorper volutpat. Etiam convallis turpis sed urna sagittis aliquam. Aliquam euismod neque ac nulla feugiat fringilla. Aliquam a fermentum leo, eget laoreet lorem. Praesent nec porta dui. Aliquam elementum ac felis quis tincidunt. Duis ac lacus accumsan, imperdiet nisl in, consequat arcu. Integer vitae metus tristique, aliquet justo dapibus, imperdiet nunc. Nam sit amet augue posuere, faucibus neque a, semper neque.
        
        Vivamus feugiat ipsum a metus scelerisque porttitor. Aenean consequat, magna non aliquam faucibus, nisl nibh dictum lectus, tincidunt mattis magna dolor vel magna. Praesent finibus velit id ornare sodales. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Cras vel dapibus lectus, ac convallis sapien. Fusce suscipit a orci eu sollicitudin. Pellentesque non ultricies mi. Maecenas facilisis dui mi, ut euismod lorem pellentesque eget. Suspendisse dapibus, nibh et laoreet porttitor, nisi enim iaculis eros, quis tristique orci risus non nunc. Nunc euismod, neque ut faucibus sodales, libero orci semper ante, non consectetur lorem mi ut est. Pellentesque eget tristique leo.
        
        Fusce vehicula interdum massa at placerat. Mauris fermentum dolor in tempor condimentum. In tempus rutrum ex, consequat placerat justo. Mauris vel neque hendrerit, laoreet arcu ut, rutrum diam. Proin tempor, odio eget commodo dapibus, dui mi posuere erat, sit amet blandit turpis mauris ac tellus. Suspendisse commodo vestibulum ex, quis tincidunt ante congue ac. Sed dictum nisi a ipsum vulputate, sed mattis mi molestie. Nulla imperdiet ligula sollicitudin venenatis suscipit. Donec mollis tristique ante, eu dignissim purus aliquet ac. Interdum et malesuada fames ac ante ipsum primis in faucibus. Proin libero velit, maximus a nisi et, iaculis venenatis velit. Duis sed orci nulla. Donec lectus mauris, vehicula non turpis a, aliquam venenatis erat. Vivamus vitae venenatis elit, at ultrices dolor. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tempor ac enim imperdiet efficitur.
        
        Mauris tempus sem urna, non placerat est commodo at. Nunc sed malesuada ipsum. Quisque sit amet consectetur orci. Pellentesque venenatis augue nec turpis sodales, a molestie dui cursus. Integer sagittis lorem non justo fringilla, ut malesuada augue sagittis. Suspendisse potenti. Mauris pretium ligula nec nisi ornare consequat.
        
        Nulla non massa tincidunt, pulvinar dui quis, feugiat eros. Sed rutrum nec dolor vel laoreet. Nullam suscipit lacus a lectus egestas eleifend. Donec nisi lectus, scelerisque in semper ac, suscipit et massa. Curabitur nec massa ac nunc ultricies ornare. Donec et interdum metus, non aliquam magna. Suspendisse convallis mauris vel magna cursus scelerisque. Sed cursus at quam non sodales. Donec venenatis dui non dignissim rutrum. Quisque feugiat luctus tellus, sit amet vestibulum urna varius et. Integer eu maximus ex, id scelerisque ex. Vestibulum congue, turpis ut ultrices lacinia, massa metus auctor tellus, ut pharetra ipsum est sed nisi. "}</p>
        </div>
    }
}

#[function_component(HelloServer)]
fn hello_server() -> Html {
     
    let data = use_state(|| None);

    // Request `/api/hello` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/hello").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>{"Got server response: "}{data}</div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}

#[function_component(Counter)]
fn counter() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
