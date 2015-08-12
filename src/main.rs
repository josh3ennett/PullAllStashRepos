extern crate docopt;
extern crate hyper;
extern crate rustc_serialize;

use docopt::Docopt;
use hyper::Client;
use hyper::Url;
use hyper::header::{Headers, Basic, Authorization};
use rustc_serialize::Decodable;
use rustc_serialize::json;
use rustc_serialize::json::{DecodeResult};
use std::fs;
use std::io::Read;
use std::process::Command;

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
    arg_branch: String,
    flag_verbos: bool
}

static USAGE: &'static str = "
Usage: PullAllStashRepos [Options] <url> <outdir> <username> <password> <branch>

Options:
    -v, --verbos  show everything.
";

fn get_arguments () -> (String, String, String, String, String){

    let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.decode())
    .unwrap_or_else(|e| e.exit());

    let user_name: String = args.arg_username;
    let password: String = args.arg_password;
    let output_directory: String = args.arg_outdir;
    let base_url = args.arg_url.to_string() + "/rest/api/1.0";
    let branch = args.arg_branch.to_string();

    return (user_name, password, output_directory, base_url, branch)
}

fn main() {

    let (user_name, password, output_directory, base_url, branch) = get_arguments();

    let projects_url =  base_url.clone().to_string() + "/repos?limit=200";
    let url_proj = Url::parse(&projects_url).unwrap();

    let decoded_repos: DecodeResult<ResponseStruct> = make_api_request(url_proj.clone(), user_name.clone(), password.clone());

    for repo in decoded_repos.unwrap().values.iter() {
        for href_struct in repo.links.clone.iter() {
            if &href_struct.name == "ssh" {
                let clone_url = &href_struct.href;

                println!("Cloning {:?} to {}", clone_url, &output_directory);

				let git_command =  "clone" ;

				//TODO try to get git2-rc building on windows so we don't have to shell out
                let output = Command::new("git")
                    .arg(git_command)
                    .arg(clone_url)
                    .arg(format!("-b {}" , &branch))
                    .arg("--single-branch")
                    .arg("--depth=1")
                    .current_dir(&output_directory)
                    .output()
                    .unwrap_or_else( |e| {
                        panic!("Failed to execute process: {}", e)
                    });

                println!("status: {}", output.status);
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

                break;
            }
        }
    }
}

fn make_api_request<T: Decodable>(url: Url, user_name: String, password: String) -> DecodeResult<T> {

    println!("Making api request to {}", url);

    let client = Client::new();
    let mut headers = Headers::new();

    let auth_header: Authorization<Basic> = Authorization(Basic{
        username: user_name,
        password: Some(password)
    });

    headers.set(auth_header);

    let mut res = client
        .get(url)
        .headers(headers)
        .send()
        .unwrap();

    let mut body_text = String::new();

    &res.read_to_string(&mut body_text);
    println!("{}", &body_text);
    let decoded_projects: DecodeResult<T> = json::decode(&body_text);

    decoded_projects
}