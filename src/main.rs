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
    link: LinkStruct,
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

fn is_already_cloned (repoName: &String) -> bool {
	false
}

fn delete_folder_contents ( path: &String ) {
    //fs::remove_file("a.txt");
}

fn get_arguments () -> (String, String, String, String){

    let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.decode())
    .unwrap_or_else(|e| e.exit());

    let user_name: String = args.arg_username;
    let password: String = args.arg_password;
    let output_directory: String = args.arg_outdir;

    let base_url = args.arg_url.to_string() + "/rest/api/1.0";

    return (user_name, password, output_directory, base_url)
}

// TODO Check to see if the repo exists, if it does do a pull instead of a clone!
fn main() {

    let (userName, password, outputDirectory, baseUrl) = get_arguments();

    //println!("Got Arguments userName:{0}, password:{1}, outputDirectory: {2}, baseUrl: {3}", &userName, &password, &outputDirectory, &baseUrl);

    let projectsUrl =  baseUrl.clone().to_string() + "/repos";
    let urlProj = Url::parse(&projectsUrl).unwrap();

    let decodedRepos: DecodeResult<ResponseStruct> = make_api_request(urlProj.clone(), userName.clone(), password.clone());

    for repo in decodedRepos.unwrap().values.iter() {
        for hrefStruct in repo.links.clone.iter() {
            if &hrefStruct.name == "ssh" {
                let cloneUrl = &hrefStruct.href;

                println!("Cloning {:?} to {}", cloneUrl, &outputDirectory);

				let isRepoAlreadyClonedHere: bool = is_already_cloned(&repo.name);

				let gitCommand = if isRepoAlreadyClonedHere { "pull" } else { "clone" };

				//TODO try to get git2-rc building on windows so we don't have to shell out
                let output = Command::new("git")
                    .arg("clone")
                    .arg(cloneUrl)
                    .arg("--depth=1")
                    .current_dir(&outputDirectory)
                    .spawn();

                break;
            }
        }
    }
}

fn make_api_request<T: Decodable>(url: Url, userName: String, password: String) -> DecodeResult<T> {

    println!("Making api request to {}", url);

    let mut client = Client::new();
    let mut headers = Headers::new();

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

    let decodedProjects: DecodeResult<T> = json::decode(&bodyText);

    decodedProjects
}