//Init_0, TIt All Begins Here

use std::io::stdin;
struct Command {
    #[allow(unused)]
    alias: String,
    linux_command: String,
    command_type: CommandType,
}

enum CommandType {
    #[allow(unused)]
    FileManagement,
    DirectoryManagement,
}

struct Arguments {
    args: Vec<String>,
}

struct Options {
    #[allow(unused)]
    options: Vec<String>,
}

//function to parse through user input

fn parse_input(input: &str) -> (Command, Arguments, Options) {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let command = match tokens[0] {
        "list" | "ls" => Command {
            alias: "ls".to_string(),
            linux_command: "ls".to_string(),
            command_type: CommandType::DirectoryManagement,
        },
        "hack" | "cd" => Command {
            alias: "hack".to_string(),
            linux_command: "cd".to_string(),
            command_type: CommandType::FileManagement,
        },
        "hnw" | "pwd" => Command {
            alias: "hnw".to_string(),
            linux_command: "pwd".to_string(),
            command_type: CommandType::DirectoryManagement,
        },
        "jenga" | "mkdir" => Command {
            alias: "jenga".to_string(),
            linux_command: "mkdir".to_string(),
            command_type: CommandType::DirectoryManagement,
        },
        "toka" | "rmdir" => Command {
            alias: "toa".to_string(),
            linux_command: "rmdir".to_string(),
            command_type: CommandType::DirectoryManagement,
        },
        "toa" | "rm" => Command {
            alias: "toa".to_string(),
            linux_command: "rm".to_string(),
            command_type: CommandType::FileManagement,
        },
        "kp" | "cp" => Command {
            alias: "kp".to_string(),
            linux_command: "cp".to_string(),
            command_type: CommandType::FileManagement,
        },
        "hms" | "mv" => Command {
            alias: "hms".to_string(),
            linux_command: "mv".to_string(),
            command_type: CommandType::DirectoryManagement,
        },
        "gusa" | "touch" => Command {
            alias: "gusa".to_string(),
            linux_command: "touch".to_string(),
            command_type: CommandType::FileManagement,
        },
        "soma" | "cat" => Command {
            alias: "soma".to_string(),
            linux_command: "cat".to_string(),
            command_type: CommandType::FileManagement,
        },
        "kurasa" | "more" => Command {
            alias: "kurasa".to_string(),
            linux_command: "more".to_string(),
            command_type: CommandType::FileManagement,
        },
        "urasa" | "less" => Command {
            alias: "urasa".to_string(),
            linux_command: "less".to_string(),
            command_type: CommandType::FileManagement,
        },
        "tafuta" | "find" => Command {
            alias: "tafuta".to_string(),
            linux_command: "find".to_string(),
            command_type: CommandType::FileManagement,
        },
        "neno" | "grep" => Command {
            alias: "neno".to_string(),
            linux_command: "grep".to_string(),
            command_type: CommandType::FileManagement,
        },
        "futa" | "clear" => Command {
            alias: "futa".to_string(),
            linux_command: "clear".to_string(),
            command_type: CommandType::FileManagement,
        },
        "wapi" | "locate" => Command {
            alias: "wapi".to_string(),
            linux_command: "locate".to_string(),
            command_type: CommandType::DirectoryManagement,
        },
        _=> Command {
            alias: tokens[0].to_string(),
            linux_command: tokens[0].to_string(),
            command_type: CommandType::FileManagement, //Defaulting To FILE mANAGEMENT
        },
    };

    let arguments = Arguments {
        args: tokens[1..].iter().map(|s| s.to_string()).collect(),
    };

    let options = Options {
        options: Vec::new(), //Chech here later
    };

    (command, arguments, options)
}

fn execute_command(command: Command, arguments: Arguments, _options: Options) {
    let mut cmd = std::process::Command::new(command.linux_command);
    cmd.args(arguments.args);
    //HAndle Options Later

    match cmd.status() {
        Ok(status) => println!("Command exited with status: {}", status),
        Err(e) => println!("ERROR EXECUTING COMMAND: {}", e),
    }
}
fn main() {
    //let user_input = "wapi home";
    let mut input = String::new();
    println!("HackThePlanet:");
    stdin().read_line(&mut input).expect("failed to read line");

    //clear
    input = input.trim().to_string();

    let (command, arguments, options) = parse_input(&input);

    //println!("Command: {}", command.alias);
    //println!("Arguments: {:?}", arguments.args);
    //println!("Options: {:?}", options.options); //implement later

    execute_command(command, arguments, options);
}