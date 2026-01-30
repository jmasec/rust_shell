use std::intrinsics::simd::simd_and;
use std::io::{stdin,stdout,Write, Error};
use std::os::unix::fs::PermissionsExt;
use std::path::{ PathBuf};
use std::str::SplitWhitespace;
use std::{env, fs};
use std::fs::{DirEntry, Metadata, read_dir};
use std::process::{Command, Output};

// execute bits for a file UNIX
const S_IXUSR: u32 = 0o100;

// tokens will need to own data since the data parsing will not live past the parsing func
struct Tokens {
    command: String, 
    args: Vec<String> 
}


fn echo_util(tokens: SplitWhitespace<'_>){
    for token in tokens{
        print!("{} ", token);
    }
    println!();
}

fn execute_command(executable: &str, args: SplitWhitespace<'_>) -> Option<Output>{
    // let i = myVec.iter() to then send that to execute commands
    let file_path: Option<PathBuf> = pathenv_search(executable);
    // unwrap here because filepath should already be checked to be valid in pathenv_search 
    //let exe_path: PathBuf = file_path.unwrap();
    if let Some(exe_path) = file_path{
        let output: Output = Command::new(exe_path)
            .args(args)
            .output()
            .expect("failed to execute");
        return Some(output)
    }
    None
}


fn pathenv_search(util: &str) -> Option<PathBuf>{
    let key: &str = "PATH";
    match env::var_os(key){
        Some(paths) => {
            // paths of dirs
            for path in env::split_paths(&paths){
                // ptr to directory
                if path.is_dir(){
                    if let Ok(entry) = read_dir(&path){
                    // loop through files in the dir
                    for file in entry{
                        // let dir_entry: fs::DirEntry = file.unwrap();
                        let dir_entry: DirEntry = match file{
                            Ok(e) => e,
                            Err(_) => continue,
                        };
                        let file_path: std::path::PathBuf = dir_entry.path();
                        let file_name: &str = match file_path.file_name().and_then(|n: &std::ffi::OsStr| n.to_str()){
                            Some(n) => n,
                            None => continue,
                        };
                        if util == file_name{
                            let metadata: Metadata = match fs::metadata(&file_path){
                                Ok(m) => m,
                                Err(_) => continue
                            };
                            // check if executabale and skip if not
                            let mode: u32 = metadata.permissions().mode();
                            if mode & S_IXUSR == 0{
                                continue;
                            }
                            return Some(file_path);
                        }
                    }
                }   
                }
            }
            println!("{}: not found", util);
            None
        }
        _ => {
            println!("Key not defined");
            None
        },
    }
}

fn type_util(mut tokens: SplitWhitespace<'_>){
    let util: &str = match tokens.next(){
        Some(u) => u,
        None => return,
    };
    match util {
        "echo" => println!("{} is a shell builtin", util),
        "type" => println!("{} is a shell builtin", util),
        "exit" => println!("{} is a shell builtin", util),
        "cd" => println!("{} is a shell builtin", util),
        _ => {
            let file_path: Option<PathBuf> = pathenv_search(util);
            if let Some(path) = file_path {
                println!("{} is {}", util, path.display());
            }
        },
    }
}

fn pwd_util() -> Option<PathBuf>{
    match env::current_dir(){
        Ok(path) => {
            println!("{}", path.display());
            Some(path)
        },
        Err(e) => {
            println!("Failed to get current working dir {}", e);
            None
        }
    }
}

fn change_directory_util(mut tokens: SplitWhitespace<'_>){
    let directory: &str = match tokens.next(){
        Some(d) => d,
        None => return,
    };

    if directory == "~"{
        if let Some(home_dir) = dir::home_dir(){
            let result: Result<(), Error> = env::set_current_dir(home_dir);
            match result{
                Ok(()) => return,
                Err(_) => println!("cd : {}: No such file or directory", directory),
            }   
        }
    }
     
    let result: Result<(), Error> = env::set_current_dir(directory);
    match result{
        Ok(()) => return,
        Err(_) => println!("cd : {}: No such file or directory", directory),
    }
}


// I am going to consume the iterator into a Vector Strings, as I consume each, I will process for any special characters
// then I will change my functions to consume a reference to the vector to basically just read from it, shouldnt need mut
// like I do with the iterator. I also need to look into handling errors in Rust to do it with more of a methedology, 
// right now I have a lot of silent errors
// Actually I may need to write my own parser at this point
// struct to handle tokens, just command and then vec of tokens
// 1. read until first space, thats the command
// 2. read next char, is it a special char? single or double quotes, backslash and the combinations made from this
//      - if single quote follow rules, prolly read till next is found and just keep everything as the token including the single quotes
//      - empty quotes ignored
//      - next to each other == concat them
//      - remove the actual quotes when processed
/*
Tokenizer Rules

*/
fn tokenizer(input: &str){
    // can I put it all into a stack vector, with delimintor between tokens?
    // let words: Vec<&str> = input.split(" ").collect();
    let mut tokens: Tokens = Tokens{
        command : String::new(),
        args : Vec::new(),
    };

    // get command, probably faster than split_whitespace to then ignore the rest
    while let Some(c) = input.chars().next(){
        if c.is_whitespace(){
            break;
        }
        else{
            tokens.command.push(c);
        }
    }
    // let mut command: String = String::new();
    // let mut args: Vec<String> = Vec::new();
    let mut single_quote: bool = false;
    let mut is_space: bool = false;
    let mut quote_last: bool = false;

    // dbg!(words);

    while let Some(c) = input.chars().next(){
        match c {
            '\'' => {
                // if single quote and last char was quote, ignore or concat them
                // if single quote is already set, then this must be end quote
                if single_quote && quote_last{
                    single_quote = false;
                    continue;
                }
                else if is_space {
                    // let tmp_str: String = String::new();
                }
                // single quote is false 
                else{

                }
                // let last_index: usize = tokens.args.len();
                // tokens.args[last_index].push(c);
                // single_quote = true;

            },
            ' ' => {
                // if we saw a single quote, then we need to keep the space and its not a normal space but a char
                if single_quote{
                    let last_index: usize = tokens.args.len();
                    tokens.args[last_index].push(c);
                    is_space = true;
                }
                // if we are not in a quote then its just a space, set the flag and move forward
                else{
                    is_space = true;
                    continue;
                }
            },
            _ => {
                // if we have character we push on to the latest string, unless. it was a space last, then we make a new one
                // pushing to vec moves the ownership
                if is_space{
                    let mut tmp_str: String = String::new();
                    tmp_str.push(c);
                    tokens.args.push(tmp_str);
                }
                else{
                    let last_index: usize = tokens.args.len();
                    tokens.args[last_index].push(c);
                }
            }, // normal char
        }
    }
}


fn shell_util(mut tokens: SplitWhitespace<'_>){
    // execute shell util calling helper functions or not found
    let command: &str = tokens.next().unwrap_or("Bad");

    match  command {
        "echo" => echo_util(tokens),
        "type" => type_util(tokens),
        "pwd" => {
            _ = pwd_util();
        }
        "cd" => change_directory_util(tokens),
        _ => {
            // let output: Output = execute_command(command, tokens);
            if let Some(output) =  execute_command(command, tokens){
                if output.status.success() {
                let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
                }
                else{
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Command failed with status {}", output.status);
                    eprintln!("Error {}", stderr);
                }   
            }
        },
    }

}

fn main() {
    loop {
        let mut input: String = String::new();
        print!("$ ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        let input: &str = input.trim();

        if input == "exit" {
            break;
        }

        //let tokens:SplitWhitespace<'_> = input.split_whitespace();
        // let tokens
        tokenizer(input);

        //shell_util(tokens);
    }
}
