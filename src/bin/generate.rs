use std::{collections::HashMap, cell::RefCell, path::Path};

use clap::Parser;
use serde::Deserialize;
use ntc_resource_library::resource::{Resource, Topic, ImageSource};


#[derive(Parser, Debug)]
#[clap(author, version, about = "Generate the resources index for the NTC library", long_about = None)]
struct Args {
    #[clap(value_parser)]
    file: String,

    #[clap(value_parser)]
    output_path: String,
}


#[derive(Debug, Deserialize)]
struct ResourceRow {
    name: String,
    description: String,
    link: Option<String>,
    filename: Option<String>,
    alt_text:Option<String>,
    resource_type:String,
    topics: String
}

fn main() {
    let args = Args::parse();
    let mut reader = csv::Reader::from_path(&args.file).expect("File should be valid csv in releative path to executable");
    let mut topic_map : HashMap<Topic, RefCell<Vec<Resource>>> = HashMap::new();
    for row_result  in reader.deserialize() {
        let row: ResourceRow =  row_result.expect("Unabled to derserialize row with error {row_result:?}");
        let (topics, resource) = match row.resource_type.to_ascii_lowercase().as_str() {
            "file" => parse_file(&row),
            "link" => parse_link(&row),
            "image" => parse_image(&row),
            _ => panic!{"Unknown resource type in row {row:?}"}
        };

        for topic in topics {
            if !topic_map.contains_key(&topic) {
                topic_map.insert(topic.clone(), RefCell::from(vec![]));
            }
            topic_map.get(&topic).unwrap().borrow_mut().push(resource.clone())
        }
    }

    //dbg!(&topic_map);
    let mut topic_list = topic_map.into_iter()
    .map(|(topic, rc)| {
        (topic,  rc.into_inner())})
    .collect::<Vec<(Topic,Vec<Resource>)>>();
    topic_list.sort_by(|(topic1, _), (topic2, _)| {
        topic1.full_topic_path().len().cmp(&topic2.full_topic_path().len()).reverse()
    });
    dbg!(&topic_list);
    let output_path = Path::new(args.output_path.as_str());
    std::fs::create_dir(output_path);
    dbg!(output_path.canonicalize());
    for (topic, resources) in topic_list.iter() {
        
    }
}

fn make_sub_topics_file_structure(output_path: &Path, topic: &Topic) {
    
}

#[inline]
fn parse_topics_from_row(row: &ResourceRow) -> Vec<Topic> {
    row.topics.split(",").map(|s| s.to_string()).map(Topic::parse_from_input).collect()
}

fn parse_file(row: &ResourceRow) -> (Vec<Topic>, Resource) {
    assert!(row.filename.is_some() && row.link.is_none() && row.alt_text.is_none(), "File {} must have filename only, no link or alt text accepted.", row.name);
    (parse_topics_from_row(row), Resource::File { name: row.name.clone(), description: row.description.clone(), filename: row.filename.clone().unwrap()})
}

fn parse_link(row: &ResourceRow) -> (Vec<Topic>, Resource) {
    assert!(row.link.is_some() && row.filename.is_none() && row.alt_text.is_none(), "Link {} must only have link and no filename and no alt text", row.name);
    (parse_topics_from_row(row), Resource::Link { name: row.name.clone(), description: row.description.clone(), link: row.link.clone().unwrap() })
}

fn parse_image(row: &ResourceRow) -> (Vec<Topic>, Resource) {
    assert!((row.link.is_some() ^ row.filename.is_some()) && row.alt_text.is_some(), "Image {} must only one of a link or file name with alt text", row.name);
    (parse_topics_from_row(row),Resource::Image { name: row.name.clone(), description: row.description.clone(), source: ImageSource::from_row(row.filename.clone(), row.link.clone())})
}
