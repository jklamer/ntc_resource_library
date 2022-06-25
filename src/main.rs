pub mod resource;

use reqwasm::http::{Request, Response};
use resource::Topic;
use yew::prelude::*;
use crate::resource::Resource;

#[derive(Clone, Properties, PartialEq)]
struct ResourceListProperties {
    topic: Topic
}

#[function_component(ResourceList)]
fn resource_list(props : &ResourceListProperties) -> Html {
    let resources = use_state(|| vec![]);
    {
        let resources = resources.clone();
        let topic = props.topic.clone();
        use_effect_with_deps(move |_| {
            let topic = topic.clone();
            let resources = resources.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response: Response = match Request::get(&topic.topic_resource_path()).send().await {
                        Ok(response) => response,
                        Err(error) => {
                            panic!("Error requesting resources for topic {}: {error:?}", topic.topic_resource_path());
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
    html!{
        {for {(*resources).iter().map(Resource::view)} }
    }
}



#[derive(Clone, Debug, Properties, PartialEq)]
struct ResourceFolderProperties {
    topic: Topic,
}

#[function_component(ResourceFolder)]
fn folder(props: &ResourceFolderProperties) -> Html {

    let sub_topics = use_state(|| vec![]);
    {
        let sub_topics = sub_topics.clone();
        let topic = props.topic.clone();
        use_effect_with_deps(move |_| {
            let sub_topics = sub_topics.clone();
            let topic = topic.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let topic = topic.clone();
                let response: Response = match Request::get(&topic.sub_topics_file()).send().await {
                            Ok(response) => response,
                            Err(error) => {
                            panic!("Error requesting topics from {:?}: {error:?}",topic.sub_topics_file());
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
                    sub_topics.set(fetched_topics)
                });
                || ()
            }, ());
    }

    let expanded = use_state(|| false);

    let on_folder_click = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(! *expanded)
        })
    };
    
    html!{
        <div>
        <h2 onclick={on_folder_click}>
            {props.topic.get_name()}
        </h2>
        if *expanded {
            {for (*sub_topics).iter().map(|sub_topic| {
                html!{
                    <ResourceFolder topic={props.topic.sub_topic(sub_topic)}/>
                }
            })}
            <ResourceList topic={props.topic.clone()}/>
        }
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct ResourceLibraryProperties {
    topics: Vec<Topic>
}

#[function_component(ResourceLibrary)]
fn library(props: &ResourceLibraryProperties) -> Html {
    props.topics.iter().map(|topic| {
        html! {
            <div>
                <ResourceFolder topic={topic.clone()}/>
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
                topics.set(fetched_topics.into_iter().map(Topic::new).collect())
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
