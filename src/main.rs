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
use std::process::Command;
use std::str;

#[derive(RustcDecodable, Debug)]
pub struct HrefStruct{
    href: String,
    name: String,
}

#[derive(RustcDecodable, Debug)]
pub struct RepoLinksStruct{
    clone: Vec<HrefStruct>
}

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
    description: String,
    public: bool,
    //type: String,
    link: LinkStruct,
    //links: RepoLinksStruct
}

#[derive(RustcDecodable, Debug)]
pub struct RepoStruct {
    slug: String,
    id: u32,
    name: String,
    scmId: String,
    state: String,
    statusMessage: String,
    forkable: bool,
    //project: Vec<ProjectInfoStruct>,
    public: bool,
    link: LinkStruct,
    cloneUrl: String,
    links:  RepoLinksStruct
}

#[derive(RustcDecodable, Debug)]
pub struct ResponseStruct {
    size: u16,
    limit: u16,
    isLastPage: bool,
    values: Vec<RepoStruct>
}

// Command line arguments.
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

    let userName: String = args.arg_username;
    let password: String = args.arg_password;
    let outputDirectory: String = args.arg_outdir;

    let baseUrl = args.arg_url.to_string() + "/rest/api/1.0";

    let projectsUrl =  baseUrl.clone().to_string() + "/repos?limit=1000";
    let urlProj = Url::parse(&projectsUrl).unwrap();

    let decodedRepos: DecodeResult<ResponseStruct> = get_json_from_api(urlProj.clone(), userName.clone(), password.clone());

    for proj in decodedRepos.unwrap().values.iter() {
        for hrefStruct in proj.links.clone.iter() {
            if &hrefStruct.name == "ssh" {
                let cloneUrl = &hrefStruct.href;

                println!("Cloning {:?} to {}", cloneUrl, &outputDirectory);

                //TODO try to get git2-rc building on windows so we don't have to shell out

                let output = Command::new("git")
                    .arg("init")
                    //.arg("clone")
                    //.arg(cloneUrl)
                    //.arg(format!("clone {} {}", cloneUrl, &outputDirectory))
                    //.arg("--depth=1")
                    //.arg("-b 1")
                    //.arg(format!("clone {} {}", cloneUrl, &outputDirectory))
                    .output()
                    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

                let outputMsg = output.stdout;

                println!("{:?}", outputMsg);

                break;
            }
        }

    }
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