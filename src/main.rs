use std::io::{stdin,stdout,Write};
use std::os::unix::fs::PermissionsExt;
use std::str::SplitWhitespace;
use std::{env, fs};
use std::fs::{DirEntry, Metadata, read_dir};

// execute bits for a file UNIX
const S_IXUSR: u32 = 0o100;

fn echo_util(tokens: SplitWhitespace<'_>){
    for token in tokens{
        print!("{} ", token);
    }
    println!();
}


fn pathenv_search(util: &str) -> bool{
    let key: &str = "PATH";
    match env::var_os(key){
        Some(paths) => {
            // paths of dirs
            for path in env::split_paths(&paths){
                // ptr to directory
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
                            println!("{} is {}", util, file_path.display());
                            return true;
                        }
                    }
                }
            }
            println!("{}: not found", util);
            false
        }
        _ => {
            println!("Key not defined");
            false
        },
    }
}

fn type_util(mut tokens: SplitWhitespace<'_>){
    let util: &str = tokens.next().unwrap_or("Bad");
    match util {
        "echo" => println!("{} is a shell builtin", util),
        "type" => println!("{} is a shell builtin", util),
        "exit" => println!("{} is a shell builtin", util),
        _ => _ = pathenv_search(util),
    }
}

fn shell_util(mut tokens: SplitWhitespace<'_>){
    // execute shell util calling helper functions or not found
    let command: &str = tokens.next().unwrap_or("Bad");
    match  command {
        "echo" => echo_util(tokens),
        "type" => type_util(tokens),
        _ => println!("{}: command not found",command),
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

        let tokens:SplitWhitespace<'_> = input.split_whitespace();

        shell_util(tokens);
    }
}
