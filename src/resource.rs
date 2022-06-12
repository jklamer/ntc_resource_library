use serde::{Serialize, Deserialize};
use yew::prelude::*;


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
