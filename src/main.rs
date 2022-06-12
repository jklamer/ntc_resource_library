pub mod resource;

use reqwasm::http::{Request, Response};
use serde_json::error;
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
        let topic = topic.clone();
        html! {
            <div>
                <ResourceFolder topic={topic} resources={vec![
                    Resource::File { description: "This file kills facists".into() },
                    Resource::Link { description: "This link kills facists".into(), link: "https://www.newteachercollab.com/about-1".into() }]}/>
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
                            match error {
                                reqwasm::Error::JsError(js_error) => {
                                    let message = js_error.message;
                                    let name = js_error.name;
                                    panic!("name:{name:?} message: {message:?}")
                                },
                                reqwasm::Error::SerdeError(_) => todo!(),
                                reqwasm::Error::Other(_) => todo!(),
                            }
                            panic!("Log other shits")
                        }
                    };
                let fetched_topics: Vec<String> = response.json()
                    .await
                    .unwrap();
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
