use std::borrow::Borrow;

use serde::{Serialize, Deserialize};
use yew::prelude::*;

const RESOUCES_FILE: &str = "resources.json";
const TOPICS_FILE: &str = "topics.json";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Resource{
    File{ name: String, description: String, filename: String},
    Link{ name: String, description: String,  link: String},
    Image{ name: String, description: String, source: ImageSource}
}

impl Resource {
    
    pub fn view(&self) -> Html {
        
        match self {
            Resource::File { name, description, filename } => {
                html!{
                    <>
                        <h3>{name}</h3>
                        <p>{description}</p>
                    </>
                }
            },
            Resource::Link { name, description, link } => {
                html! {
                    <>
                        <h3>{name}</h3>
                        <p>{description}</p>
                        <p>{link}</p>
                    </>
                }
            },
            Resource::Image { name, description, source } => {
                match source {
                    ImageSource::File { filename } => {
                        html!{}
                    },
                    ImageSource::Link { link } => {
                        html!{}
                    },
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImageSource {
    File {filename: String},
    Link {link: String}
}

impl ImageSource {

    pub fn from_row(filename: Option<String>, link: Option<String>) -> ImageSource {
        assert!(filename.is_some() ^ link.is_some());
        if filename.is_some() {
            ImageSource::File {filename: filename.unwrap()}
        } else {
            ImageSource::Link { link: link.unwrap() }
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Topic {
    path: Vec<String>,
    topic_name: String,
}

impl Topic {

    pub fn new(base_topic: String) -> Topic {
        Topic { path: vec![], topic_name: base_topic}
    }

    pub fn parse_from_input(topic_str: String) -> Topic {
        let mut sub_topics: Vec<String> =  topic_str.split("/").map(|s| s.to_string()).collect();
        let topic = sub_topics.pop().expect("Empy Topic");
        Topic { path: sub_topics, topic_name: topic }
    }

    pub fn get_name(&self) -> String {
        self.topic_name.clone()
    }

    pub fn sub_topic(&self, sub_topic: impl Borrow<String>) -> Topic {
        let mut path = self.path.clone();
        path.push(self.topic_name.clone());
        Topic { path, topic_name: sub_topic.borrow().clone()}
    }

    pub fn full_topic_path(&self) -> String {
        if self.path.is_empty() {
            String::from("/") + &self.topic_name + "/"
        } else {
            String::from("/") + self.path.join("/").as_ref()+ "/"+ &self.topic_name + "/"
        }
    }

    pub fn topic_resource_path(&self) -> String {
       self.full_topic_path() + RESOUCES_FILE
    }

    pub fn sub_topics_file(&self) -> String {
        self.full_topic_path() + TOPICS_FILE
    }
}


mod tests {

    use super::*;

    #[test]
    fn test_serde() {
        let r = Resource::File { name:"file name".into(), description: "Jacks file".into(),  filename: "hey.pdf".into()};
        let string_r = serde_json::to_string(&r).unwrap();
        println!("{string_r}");
        let r2 = serde_json::from_str::<Resource>(&string_r).unwrap();
        assert_eq!(r, r2);

        let r = Resource::Link { name: "link name".into(), description: "Jacks link".into(), link:"https://newteachercollab.com/".into() };
        let string_r = serde_json::to_string(&r).unwrap();
        println!("{string_r}");
        let r2 = serde_json::from_str::<Resource>(&string_r).unwrap();
        assert_eq!(r, r2);
    }
}
