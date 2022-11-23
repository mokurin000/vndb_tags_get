use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize)]
struct Tag {
    id: u32,
    name: String,
    description: String,
    meta: bool,
    searchable: bool,
    applicable: bool,
    vns: u32,
    cat: String,
    aliases: Vec<String>,
    parents: Vec<u32>,
}

/* English descrption is avaliable on VNDB's official site.
name         type              null?            Description

id 	         integer            no 	Tag ID
name 	     string 	        no 	Tag 名
description  string 	        no 	可能包含一些格式化代码，在 https://vndb.org/d9#3 介绍。
meta 	     bool 	            no 	这个是不是元标签。这个字段仅为了向后兼容存在，与searchable相反。
searchable 	 bool 	            no 	是否可能用这个标签过滤VN。
applicable 	 bool 	            no 	这个标签是否可以对VN使用。
vns 	     integer 	        no 	（包括子标签） 标签下有多少VN
cat 	     string 	        no 	标签分类: "cont" for 内容, "ero" for 性相关, "tech" for 技术性细节
aliases 	 array of strings 	no 	（可能为空） 别名的列表。
parents 	 array of integers 	no 	父标签列表（对于根标签是空的）
*/

fn process() -> Vec<Tag> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let v: Vec<Tag> = serde_json::from_str(buffer.as_str()).unwrap();
    v
}

fn main() {
    let mut input = process();
    input.sort_unstable_by(|a, b| b.vns.cmp(&a.vns));
    let out: io::Stdout = std::io::stdout();
    let mut lock = out.lock();
    writeln!(
        lock,
        "Tag | id | Comment\n------------|--------------------|------------------- "
    )
    .unwrap();
    for tag in input.iter().filter(|x| x.vns != 0) {
        let html_description = vndb_to_html(&tag.description);
        let name = &tag.name;
        let id = tag.id;
        writeln!(
            lock,
            "{name}| [g{id}](https://vndb.org/g{id}) | {html_description}",
        )
        .unwrap();
    }
}

fn vndb_to_html(x: &str) -> String {
    x.to_lowercase()
        .replace("[url=", "<a href=")
        .replace("[/url]", "</a>")
        .replace("[", "<")
        .replace("]", ">")
        .replace("\n", "<br>")
}
