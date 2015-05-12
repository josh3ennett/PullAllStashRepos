extern crate rustc_serialize;
extern crate docopt;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct HrefStruct{
    data_href: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct LinksStruct{
    data_self: Vec<HrefStruct>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct LinkStruct{
    data_url: String,
    dataRel: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ProjectsInfoStruct  {
    data_key: String,
    data_id: u32,
    data_name: String,
    data_description: String,
    data_public: bool,
    data_type: String,
    data_link: LinkStruct,
    data_links: LinksStruct
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ProjectsStruct  {
    data_size: u8,
    data_limit: u8,
    data_vector: bool,
    data_values: Vec<ProjectsInfoStruct>
}

fn main() {
    println!("Hello, world!");
}
