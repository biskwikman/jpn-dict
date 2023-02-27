use roxmltree::{Document, ParsingOptions};
use std::fs;

#[derive(Debug)]
struct Entry {
    ent_seq: Vec<i32>,
    k_ele: Vec<KEle>,
}

#[derive(Debug)]
struct KEle {
    keb: String,
    ke_inf: String,
    ke_pri: String,
}

fn main() {
    let mut dict_string =
        fs::read_to_string("dict_data/JMdict_e").expect("could not read dict file");

    dict_string.retain(|c| c != '\n');

    let opt = ParsingOptions {
        allow_dtd: true,
        nodes_limit: u32::MAX,
    };

    let doc = Document::parse_with_options(&dict_string, opt).unwrap();

    // let mut ent_seq_vec: Vec<i32> = vec![];
    let mut entries: Vec<Entry> = vec![];

    // Get all entries
    for node in doc.descendants() {
        // For each entry
        if node.has_tag_name("entry") {
            // Create vector for ids
            let mut ent_seq_vec: Vec<i32> = vec![];

            // Create vector for k_ele
            let mut k_ele_vec: Vec<KEle> = vec![];

            for node in node.descendants() {
                if node.has_tag_name("ent_seq") {
                    ent_seq_vec.push(
                        node.text()
                            .expect("error finding ent_seq text")
                            .parse::<i32>()
                            .unwrap(),
                    );
                }
                if node.has_tag_name("k_ele") {
                    for node in node.descendants() {
                        let mut keb = String::new();
                        let mut ke_inf = String::new();
                        let mut ke_pri = String::new();
                        if node.has_tag_name("keb") {
                            keb = node.text().expect("error finding keb text").to_string();
                        }
                        if node.has_tag_name("ke_inf") {
                            ke_inf = node.text().expect("error finding ke_inf text").to_string();
                        }
                        if node.has_tag_name("ke_pri") {
                            ke_pri = node.text().expect("error finding ke_pri text").to_string();
                        }
                        println!("keb {}", &keb);
                        let k_ele = KEle {
                            keb,
                            ke_inf,
                            ke_pri,
                        };
                        // k_ele_vec.push(k_ele);
                    }
                }
            }
            let entry = Entry {
                ent_seq: ent_seq_vec,
                k_ele: k_ele_vec,
            };
            entries.push(entry);
        }
    }
    // println!("{entries:?}");
}
