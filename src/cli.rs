mod secrets;
mod api;
use std::env;
use gemini_rs::Conversation;
use colored::Colorize;
use figleter::FIGfont;

#[tokio::main]
async fn main() {
    let api_key : String = secrets::getAPIKey();
    let arguments = parse_arguments();
    if arguments != String::from("++END++") {
        let mut convo = Conversation::new(api_key.clone(), "gemini-1.5-flash".to_string());
        let res = convo.prompt(&arguments).await;
        println!("{res}");
    }
}
pub fn parse_arguments() -> String {
    let args : Vec<String> = env::args().collect();
    let arg : Vec<String>   = Vec::new();
    if args.len() <= 1 {
        println!("    [+-------------------------------------------+]");
        println!("    | {}  |","No arguments given Use --help to get help".red().on_yellow());
        println!("    [+-------------------------------------------+]");
        return String::from("++END++");
    }
    match &args[1][..] {
        "--help" => return show_help(),
        "-q" => return String::from(&args[2]),
        "-s" => return system_message(args),
        _  => return String::from("User didn't use command line arguments, as them to use -q <their query> to send you anything and they can check --help if not"),

    }
}

fn system_message(args : Vec<String>) -> String {
    if args.len() <= 4 {
        return String::from("The user didn't provide enough arguments advise them they need to read the help with -h")
    }
    return format!("[system message] {} [EOF] [users query] {} [EOF]", String::from(&args[2]), String::from(&args[4]) )
}

fn show_help() -> String {
    let mut arg_help: Vec<String> = Vec::new();
    arg_help.push(String::from("--help         :    Shows this help message"));
    arg_help.push(String::from("-q <Query>     :    Use -q then your query for Gemini to respond to. no history saved so fresh instance each time"));
    arg_help.push(String::from("-s <System>    :    Used to prepend system instructions before the query     Use as -s <System> -q <Query>"));
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