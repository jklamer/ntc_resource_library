pub mod resource;

use reqwasm::http::{Request, Response};
use yew::prelude::*;
use crate::resource::Resource;


#[derive(Clone, Properties, PartialEq)]
struct ResourceFolderProperties {
    topic: String,
    resources: Vec<Resource>,
}

#[function_component(ResourceFolder)]
fn folder(props: &ResourceFolderProperties) -> Html {

    let expanded = use_state(|| false);

    let on_folder_click = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(! *expanded)
        })
    };
    
    let resources = props.resources.iter().map(Resource::view);

    html!{
        <div>
        <h2 onclick={on_folder_click}>
            {&props.topic}
        </h2>
        if *expanded {
            {for resources }
        }
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct ResourceLibraryProperties {
    topics: Vec<String>
}

#[function_component(ResourceLibrary)]
fn library(props: &ResourceLibraryProperties) -> Html {
    props.topics.iter().map(|topic|
    {
        let resources = use_state(|| vec![]);
        {
            let resources = resources.clone();
            let topic = topic.clone();
            use_effect_with_deps(move |_| {
                let topic = topic.clone();
                let resources = resources.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response: Response = match Request::get(&("/".to_owned() + &topic + "/resources.json")).send().await {
                            Ok(response) => response,
                            Err(error) => {
                               panic!("Error requesting resources for topic {topic}: {error:?}");
                            }
                        };
                    let fetched_resources: Vec<Resource> = match response.json()
                        .await
                        {
                            Ok(json) => json ,
                            Err(error) => {
                                panic!("Json parse error {error:?}")
                            }
                        };
                    resources.set(fetched_resources);
                });
                || ()
            }, ());
        }
        let topic = topic.clone();

        html! {
            <div>
                <ResourceFolder topic={topic} resources={(*resources).clone()}/>
            </div>
        }
    }).collect::<Html>()
}

#[function_component(App)]
fn app() -> Html {
    let topics = use_state(|| vec![]);
    {
        let topics = topics.clone();
        use_effect_with_deps(move |_| {
            let topics = topics.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response: Response = match Request::get("/topics.json").send().await {
                        Ok(response) => response,
                        Err(error) => {
                           panic!("Error requesting topics: {error:?}");
                        }
                    };
                let fetched_topics: Vec<String> = match response.json()
                    .await
                    {
                        Ok(json) => json ,
                        Err(error) => {
                            panic!("Json parse error {error:?}")
                        }
                    };
                topics.set(fetched_topics)
            });
            || ()
        }, ());
    }
    html! {
        <div>
            <h1>{"Resource Library"}</h1>
            <ResourceLibrary topics={(*topics).clone()}/>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
