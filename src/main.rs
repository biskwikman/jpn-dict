use roxmltree::{Document, ParsingOptions};
use std::fs;

#[derive(Debug)]
struct Entry<'a> {
    ent_seq: Option<&'a Vec<i32>>,
}

fn main() {
    let mut dict_string =
        fs::read_to_string("sourceFolder/JMdict_e").expect("could not read dict file");
    dict_string.retain(|c| c != '\n');

    let opt = ParsingOptions {
        allow_dtd: true,
        nodes_limit: u32::MAX,
    };

    let doc = Document::parse_with_options(&dict_string, opt).unwrap();

    let mut ent_seq_vec: Vec<i32> = vec![];
    // Get all entries
    for node in doc.descendants() {
        if node.has_tag_name("entry") {
            for node in node.descendants() {
                ent_seq_vec = vec![];
                if node.has_tag_name("ent_seq") {
                    ent_seq_vec.push(
                        node.text()
                            .expect("error finding ent_seq text")
                            .parse()
                            .unwrap(),
                    );
                }
            }
            let entry = Entry {
                ent_seq: Some(&ent_seq_vec),
            };

            println!("{entry:?}");
        }
    }
}
