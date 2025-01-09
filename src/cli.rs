#![allow(warnings)]
mod secrets;
mod lib;
use std::env;
use lib::libapi::API;
use colored::Colorize;
use figleter::FIGfont;

#[tokio::main]
async fn main() {
    let api_key : String = secrets::get_apikey();
    let arguments = parse_arguments().await;
    if arguments != String::from("++END++") {
        let query = API::Query(arguments, api_key);
        //let mut convo = Conversation::new(api_key.clone(), "gemini-1.5-flash".to_string());
        //let res = convo.prompt(&arguments).await;
        let res = query.call().await;
        let msg = match res.response() {
            Ok(m) => m,
            Err(e) => {
                println!("Error: {}", e);
                return;
            },
        };
        println!("{msg}");
    }
}
pub async fn parse_arguments() -> String {
    let args : Vec<String> = env::args().collect();
    let _arg : Vec<String>   = Vec::new();
    if args.len() <= 1 {
        println!("    [+-------------------------------------------+]");
        println!("    | {}  |","No arguments given Use --help to get help".red().on_yellow());
        println!("    [+-------------------------------------------+]");
        return String::from("++END++");
    }
    match &args[1][..] {
        "--help" => return show_help(),
        "-q" => return String::from(format!("{} EOF Reply With Markdown", &args[2])),
        "-s" => return system_message(args),
        "-u" => return get_content(args).await,
        _  => return String::from("User didn't use command line arguments, as them to use -q <their query> to send you anything and they can check --help if not"),

    }
}

async fn get_content(args: Vec<String>) -> String {
if args.len() <= 4 {
        return String::from("The user didn't provide enough arguments advise them they need to read the help with -h")
    }
    let connect = API::Connect(String::from(&args[2]));
    let res = connect.call().await;
    let content = match res.connection() {
        Ok(c) => c,
        Err(e) => return format!("Error: {}", e),
    };
    return format!("[Web Content] {} [EOF] [User Query] {} [EOF] Format Response as Markdown", content.response(), &args[4]);
}

fn system_message(args : Vec<String>) -> String {
    if args.len() <= 4 {
        return String::from("The user didn't provide enough arguments advise them they need to read the help with -h")
    }
    return format!("[system message use this for context] {} [EOF] [users query] {} [EOF] Format Response as Markdown", String::from(&args[2]), String::from(&args[4]) )
}

fn show_help() -> String {
    let mut arg_help: Vec<String> = Vec::new();
    arg_help.push(String::from("--help         :    Shows this help message"));
    arg_help.push(String::from("-q <Query>     :    Use -q then your query for Gemini to respond to. no history saved so fresh instance each time"));
    arg_help.push(String::from("-s <System>    :    Used to prepend system instructions before the query     Use as -s <System> -q <Query>"));
    arg_help.push(String::from("-u <URL>       :    Used to add content from URI. Use as -u <URL> -q <Query>"));
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("[UKJP]");
    println!("{}\n", figure.unwrap());
    println!("  /\\  /\\   4   BBBB");
    println!(" /  \\/  \\  44  B   B");
    println!("/        \\ 44  BBBB ");
    println!("\\        / 44  B   B");
    println!(" \\/  \\/   4444  BBBB ");
    println!("===[Author:Sam Aldis]===");
    for h in arg_help {
        println!("{:?}",h.to_string());
    }
    return String::from("")
}