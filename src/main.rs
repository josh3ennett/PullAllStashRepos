extern crate docopt;
extern crate hyper;
extern crate rustc_serialize;

use docopt::Docopt;
use hyper::Client;
use hyper::client::Request;
use hyper::Url;
use hyper::header::{Headers, HeaderFormat, Header, Basic, Authorization};
use rustc_serialize::json;
use rustc_serialize::base64::{ToBase64, MIME};
use std::io::Read;


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
pub struct ProjectInfoStruct  {
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
    data_values: Vec<ProjectInfoStruct>
}

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_url: String,
    arg_username: String,
    arg_password: String,
    arg_outdir: String,
    flag_verbos: bool
}
static USAGE: &'static str = "
Usage: PullAllStashRepos [Options] <url> <outdir> <username> [<password>]

Options:
    -v, --verbos  show everything.
";

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);

    let mut client = Client::new();
    let mut headers = Headers::new();

    let userName = args.arg_username;
    let password = args.arg_password;
    let outputDirectory = args.arg_outdir;

    //Authorization
    let authHeader: Authorization<Basic> = Authorization(Basic{
        username: userName,
        password: Some(password)
    }) ;

    headers.set(authHeader);

    let mut projectsUrl = args.arg_url.to_string() + "/rest/api/1.0/projects/";
    let url = Url::parse(&projectsUrl).unwrap();

    // TODO use basic auth
    let mut res = client
        .get(url)
        .headers(headers)
        .send()
        .unwrap();

    let mut bodyText = String::new();

    &res.read_to_string(&mut bodyText);

    println!("{:?}", bodyText);
}

fn promptForPassword() -> String {
    "this should return the password the user entered".to_string()
}