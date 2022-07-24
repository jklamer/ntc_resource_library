pub mod resource;

use reqwasm::http::{Request, Response};
use resource::Topic;
use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::resource::Resource;

const STYLE_FILE: &str = include_str!("main.css");

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
    style: FolderStyle
}

#[derive(Clone, Debug, PartialEq)]
enum FolderStyle {
    Style1,
    Style2,
    Style3,
}

impl FolderStyle {

    fn inner_style(&self) -> FolderStyle {
        match self {
            FolderStyle::Style1 => Self::Style2,
            FolderStyle::Style2 => Self::Style3,
            FolderStyle::Style3 => Self::Style1,
        }
    }

    fn get_class(&self) -> &'static str {
        match self {
            FolderStyle::Style1 => "folder style1",
            FolderStyle::Style2 => "folder style2",
            FolderStyle::Style3 => "folder style3",
        }
    }
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
        <div class={props.style.get_class()}>
        <h2 onclick={on_folder_click}>
            {props.topic.get_name()} 
            if *expanded {
                {"\u{1F4C2}"}
            } else {
                {"\u{1F4C1}"}
            }
        </h2>
        if *expanded {
            {for (*sub_topics).iter().map(|sub_topic| {
                html!{
                    <ResourceFolder topic={props.topic.sub_topic(sub_topic)} style={props.style.inner_style()}/>
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
                <ResourceFolder topic={topic.clone()} style={FolderStyle::Style1}/>
            </div>
        }
    }).collect::<Html>()
}

#[styled_component(MyApp)]
fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).expect("Should be able to parse CSS");
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
    let font_load = 
    "@font-face 
        { 
                font-family: NTC;
                src: url(\"https://static.parastorage.com/services/third-party/fonts/user-site-fonts/fonts/0078f486-8e52-42c0-ad81-3c8d3d43f48e.woff2\") ; 
        }
        @font-face 
        { 
            font-family: NTC;
            font-weight: bold; 
            src: url(\"https://static.parastorage.com/services/third-party/fonts/user-site-fonts/fonts/d513e15e-8f35-4129-ad05-481815e52625.woff2\") ;
        }";


    html! {
        <>
        <style>
        {font_load}
        </style>
        <div class={stylesheet}>
            <div class="banner">
                <span>{"Resource Library"}</span>
            </div>
            <div>
            <p>{"Annie is making a description for me"}</p>
            </div>
            <div>
                <ResourceLibrary topics={(*topics).clone()}/>
            </div>
        </div>
        </>
    }
}




fn main() {
    yew::start_app::<MyApp>();
}
