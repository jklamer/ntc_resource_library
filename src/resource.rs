use serde::{Serialize, Deserialize};
use yew::prelude::*;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Resource{
    File{ description: String},
    Link{ description: String,  link: String}
}


impl Resource {
    
    pub fn view(&self) -> Html {
        
        match self {
            Resource::File { description } => {
                html!{
                    <>
                        <h3>{"A file"}</h3>
                        <p>{description}</p>
                    </>
                }
            },
            Resource::Link { description, link } => {
                html! {
                    <>
                    <h3>{"A link"}</h3>
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
        let r = Resource::File { description: "Jacks file".into() };
        let string_r = serde_json::to_string(&r).unwrap();
        let r2 = serde_json::from_str::<Resource>(&string_r).unwrap();
        assert_eq!(r, r2);
    }
}
