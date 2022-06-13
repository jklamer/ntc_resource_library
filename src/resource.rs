use std::borrow::Borrow;

use serde::{Serialize, Deserialize};
use yew::prelude::*;

const RESOUCES_FILE: &str = "resources.json";
const TOPICS_FILE: &str = "topics.json";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Resource{
    File{ name: String, description: String},
    Link{ name: String, description: String,  link: String}
}


impl Resource {
    
    pub fn view(&self) -> Html {
        
        match self {
            Resource::File { name, description } => {
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
        }
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct Topic {
    path: Vec<String>,
    topic_name: String,
}

impl Topic {

    pub fn new(base_topic: String) -> Topic {
        Topic { path: vec![], topic_name: base_topic}
    }

    pub fn get_name(&self) -> String {
        self.topic_name.clone()
    }

    pub fn sub_topic(&self, sub_topic: impl Borrow<String>) -> Topic {
        let mut path = self.path.clone();
        path.push(self.topic_name.clone());
        Topic { path, topic_name: sub_topic.borrow().clone()}
    }

    pub fn topic_resource_path(&self) -> String {
        if self.path.is_empty() {
            String::from("/") + &self.topic_name + "/" + RESOUCES_FILE
        } else {
            String::from("/") + self.path.join("/").as_ref()+ "/"+ &self.topic_name + "/" + RESOUCES_FILE
        }
    }

    pub fn sub_topics_file(&self) -> String {
        if self.path.is_empty() {
            String::from("/") + &self.topic_name+ "/" + TOPICS_FILE
        } else {
            String::from("/") + self.path.join("/").as_ref() + "/"+ &self.topic_name+ "/" + TOPICS_FILE
        }
    }
}


mod tests {

    use super::*;

    #[test]
    fn test_serde() {
        let r = Resource::File { name:"file name".into(), description: "Jacks file".into() };
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
