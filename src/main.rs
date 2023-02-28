use roxmltree::{Document, ParsingOptions};
use std::fs;

#[derive(Debug)]
struct Entry {
    ent_seq: Vec<i32>,
    k_ele: Vec<KEle>,
    r_ele: Vec<REle>,
    sense: Vec<Sense>,
}

#[derive(Debug)]
struct KEle {
    keb: String,
    ke_inf: String,
    ke_pri: String,
}

#[derive(Debug)]
struct REle {
    reb: String,
    re_nokanji: String,
    re_restr: String,
    re_inf: String,
    re_pri: String,
}

#[derive(Debug)]
struct Sense {
    stagk: String,
    stagr: String,
    xref: String,
    ant: String,
    pos: String,
    field: String,
    misc: String,
    lsource: String,
    dial: String,
    gloss: Vec<String>,
    pri: String,
    s_inf: String,
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

    let mut entries: Vec<Entry> = vec![];
    // Get all entries
    for node in doc.descendants() {
        // For each entry
        if node.has_tag_name("entry") {
            // Create vector for ids
            let mut ent_seq_vec: Vec<i32> = vec![];

            // Create vector for k_ele
            let mut k_ele_vec: Vec<KEle> = vec![];
            let mut r_ele_vec: Vec<REle> = vec![];

            for node in node.children() {
                if node.has_tag_name("ent_seq") {
                    ent_seq_vec.push(
                        node.text()
                            .expect("error finding ent_seq text")
                            .parse::<i32>()
                            .unwrap(),
                    );
                }
                if node.has_tag_name("k_ele") {
                    let mut keb = String::new();
                    let mut ke_inf = String::new();
                    let mut ke_pri = String::new();

                    for node in node.children() {
                        if node.has_tag_name("keb") {
                            keb = node.text().expect("error finding keb text").to_string();
                        } else if node.has_tag_name("ke_inf") {
                            ke_inf = node.text().expect("error finding ke_inf text").to_string();
                        } else if node.has_tag_name("ke_pri") {
                            ke_pri = node.text().expect("error finding ke_pri text").to_string();
                        }
                    }
                    let k_ele = KEle {
                        keb,
                        ke_inf,
                        ke_pri,
                    };
                    k_ele_vec.push(k_ele);
                }
                if node.has_tag_name("r_ele") {
                    let mut reb = String::new();
                    let mut re_nokanji = String::new();
                    let mut re_restr = String::new();
                    let mut re_inf = String::new();
                    let mut re_pri = String::new();

                    for node in node.children() {
                        if node.has_tag_name("reb") {
                            reb = node.text().expect("error finding reb text").to_string();
                        } else if node.has_tag_name("re_nokanji") {
                            if let Some(t) = node.text() {
                                re_nokanji = t.to_string();
                            }
                        } else if node.has_tag_name("re_restr") {
                            re_restr = node
                                .text()
                                .expect("error finding re_restr text")
                                .to_string();
                        } else if node.has_tag_name("re_inf") {
                            re_inf = node.text().expect("error finding re_inf text").to_string();
                        } else if node.has_tag_name("re_pri") {
                            re_pri = node.text().expect("error finding re_pri text").to_string();
                        }
                    }
                    let r_ele = REle {
                        reb,
                        re_nokanji,
                        re_restr,
                        re_inf,
                        re_pri,
                    };
                    r_ele_vec.push(r_ele);
                }
                if node.has_tag_name("sense") {
                    let mut stagk = String::new();
                    let mut stagr = String::new();
                    let mut xref = String::new();
                    let mut ant = String::new();
                    let mut pos = String::new();
                    let mut field = String::new();
                    let mut misc = String::new();
                    let mut lsource = String::new();
                    let mut dial = String::new();
                    let mut gloss: Vec<String> = vec![];
                    let mut pri = String::new();
                    let mut s_inf = String::new();

                    for node in node.children() {
                        if node.has_tag_name("stagk") {
                            stagk = node.text().expect("error finding stagk text").to_string();
                        } else if node.has_tag_name("stagr") {
                            stagr = node.text().expect("error finding stagr text").to_string();
                        } else if node.has_tag_name("xref") {
                            xref = node.text().expect("error finding xref text").to_string();
                        } else if node.has_tag_name("ant") {
                            ant = node.text().expect("error finding ant text").to_string();
                        } else if node.has_tag_name("pos") {
                            pos = node.text().expect("error finding pos text").to_string();
                        } else if node.has_tag_name("field") {
                            field = node.text().expect("error finding field text").to_string();
                        } else if node.has_tag_name("misc") {
                            misc = node.text().expect("error finding misc text").to_string();
                        }
                    }
                }
            }

            let entry = Entry {
                ent_seq: ent_seq_vec,
                k_ele: k_ele_vec,
                r_ele: r_ele_vec,
            };
            entries.push(entry);
        }
    }
    println!("{entries:?}");
}
