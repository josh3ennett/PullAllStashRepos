extern crate docopt;
extern crate hyper;
extern crate rustc_serialize;

use docopt::Docopt;
use hyper::Client;
use hyper::client::Request;
use hyper::Url;
use hyper::header::{Headers, HeaderFormat, Header, Basic, Authorization};
use rustc_serialize::Decodable;
use rustc_serialize::json;
use rustc_serialize::json::{DecodeResult};
use std::io::Read;

#[derive(RustcDecodable, Debug)]
pub struct HrefStruct{
    href: String
}

/*#[derive(RustcDecodable, Debug)]
pub struct LinksStruct{
    self: Vec<HrefStruct>
}*/

#[derive(RustcDecodable, Debug)]
pub struct LinkStruct{
    url: String,
    rel: String
}

#[derive(RustcDecodable, Debug)]
pub struct ProjectInfoStruct  {
    key: String,
    id: u32,
    name: String,
    //description: String,
    public: bool,
    //type: String,
    link: LinkStruct,
    //links: LinksStruct
}

#[derive(RustcDecodable, Debug)]
pub struct ProjectsStruct  {
    //size: u8,
    //limit: u8,
    isLastPage: bool,
    values: Vec<ProjectInfoStruct>
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
Usage: PullAllStashRepos [Options] <url> <outdir> <username> <password>

Options:
    -v, --verbos  show everything.
";

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);

    let userName: String = args.arg_username;
    let password: String = args.arg_password;
    let outputDirectory: String = args.arg_outdir;

    let baseUrl = args.arg_url.to_string() + "/rest/api/1.0";

    let projectsUrl =  baseUrl.clone().to_string() + "/projects/";
    let urlProj = Url::parse(&projectsUrl).unwrap();

    let decodedProjects: DecodeResult<ProjectsStruct> = get_json_from_api(urlProj.clone(), userName.clone(), password.clone());

    //TODO loop through projects, get repo, clone repo to outDir
    for proj in decodedProjects.unwrap().values.iter() {

        let projUrl = baseUrl.clone() + &proj.clone().link.url;
        let url = Url::parse(&projUrl).unwrap();

        let decodedProj: DecodeResult<ProjectsStruct> = get_json_from_api(url, userName.clone(), password.clone());

        //TOOD output raw text? println!("{:?}", &decodedProj);

        println!("{:?}", &decodedProj);
    }

    //println!("{:?}", &bodyText);
    //println!("{:?}", &decodedProjects);
}

fn get_json_from_api<T: Decodable>(url: Url, userName: String, password: String) -> DecodeResult<T> {

    let mut client = Client::new();
    let mut headers = Headers::new();

    //Authorization
    let authHeader: Authorization<Basic> = Authorization(Basic{
        username: userName,
        password: Some(password)
    });

    headers.set(authHeader);

    let mut res = client
        .get(url)
        .headers(headers)
        .send()
        .unwrap();

    let mut bodyText = String::new();

    &res.read_to_string(&mut bodyText);

    //println!("{:?}", &bodyText);

    //Get Projects
    let decodedProjects: DecodeResult<T> = json::decode(&bodyText);

    decodedProjects
}