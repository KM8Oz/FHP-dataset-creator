use clap::{arg, command, ArgAction, ArgGroup, ColorChoice, Command};
use fhp_dataset_creator::start_monitor;
use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();
    let processname = matches.get_one::<String>("process");
    let _timeout = match matches.get_one::<u64>("timeout") {
        None => 2 as u64,
        Some(m) => m.clone(),
    };
    let _threads = matches.get_flag("threadsafe");
    let _retry = match matches.get_one::<u64>("retrys") {
        Some(m) => m.clone(),
        None => 1 as u64,
    };
    match processname {
        Some(input) => {
            let inp = input.clone();
            let system = System::new_all();
            let processes_with_name: Vec<&sysinfo::Process> = system.processes_by_name(&inp).into_iter()
            .collect();

            if processes_with_name.is_empty() {
                println!("No running processes with the name {}", input);
                return;
            }

            println!("Choose a running process with the name {}: ", input);
            for (i, process) in processes_with_name.iter().enumerate() {
                println!("{}. {}", i + 1, process.name());
            }

            let mut chosen_process: Option<&sysinfo::Process> = None;
            while chosen_process.is_none() {
                println!("Enter the number of the process you want to choose:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                match input.trim().parse::<usize>() {
                    Ok(index) => {
                        if index > 0 && index <= processes_with_name.len() {
                            chosen_process = Some(processes_with_name[index - 1]);
                        }
                    }
                    Err(_) => {}
                }

                if chosen_process.is_none() {
                    println!("Invalid input, please try again");
                }
            }
            
            if chosen_process.is_some() {
                let chosen_process_pid = chosen_process.unwrap().pid();
                println!("🔥 start computing! 🔥");
                start_monitor(chosen_process_pid).await;
            } else {
                println!("no process choosed <{}>!", input);
            }
        }
        None => {
            println!("no process name in args!");
        }
    };
}
fn cli() -> Command {
    command!()
        .version("1.0")
        .author("KM8Oz <contact@dup.company>")
        .args([
            arg!(-p --process <PROCESS> "Process name to be monitored").group("options")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .required(true),
            arg!(-t --timeout <NUMBER> "Single compute iteration timeout in minutes").group("options")
            .default_value("60")
            .value_parser(clap::value_parser!(u64).range(1..120))
            .required(true),
            arg!(threadsafe: --threadsafe <BOOLEAN> "Separate every param in different rantime").group("options")
            .action(ArgAction::SetTrue)
            .required(false),
            arg!(-r --retrys <NUMBER> "How many time a single compute will be tested (>=1)").group("options")
            .default_value("1")
            .value_parser(clap::value_parser!(u64).range(1..2))
            .required(false),
        ])
        .group(ArgGroup::new("options").multiple(true))
        .group(ArgGroup::new("usage").multiple(true))
        .next_help_heading("USAGE")
        .args([
            arg!(-a <example1> "fhp_dataset_creator(.exe) -t 60 -tsafe -p vscode(.exe|.app)").group("usage"),
        ])
        .about(r"███╗░░░███╗░█████╗░██████╗░███████╗  ░██╗░░░░░░░██╗██╗████████╗██╗░░██╗  ██╗░░░░░░█████╗░██╗░░░██╗███████╗
        ████╗░████║██╔══██╗██╔══██╗██╔════╝ ░██║░░██╗░░██║██║╚══██╔══╝██║░░██║  ██║░░░░░██╔══██╗██║░░░██║██╔════╝
        ██╔████╔██║███████║██║░░██║█████╗░░ ░╚██╗████╗██╔╝██║░░░██║░░░███████║  ██║░░░░░██║░░██║╚██╗░██╔╝█████╗░░
        ██║╚██╔╝██║██╔══██║██║░░██║██╔══╝░░ ░░████╔═████║░██║░░░██║░░░██╔══██║  ██║░░░░░██║░░██║░╚████╔╝░██╔══╝░░
        ██║░╚═╝░██║██║░░██║██████╔╝███████╗ ░░╚██╔╝░╚██╔╝░██║░░░██║░░░██║░░██║  ███████╗╚█████╔╝░░╚██╔╝░░███████╗
        ╚═╝░░░░░╚═╝╚═╝░░╚═╝╚═════╝░╚══════╝ ░░░╚═╝░░░╚═╝░░╚═╝░░░╚═╝░░░╚═╝░░╚═╝  ╚══════╝░╚════╝░░░░╚═╝░░░╚══════╝
        
        ██████╗░██╗░░░██╗  ██╗░░██╗███╗░░░███╗░█████╗░░█████╗░███████╗
        ██╔══██╗╚██╗░██╔╝  ██║░██╔╝████╗░████║██╔══██╗██╔══██╗╚════██║
        ██████╦╝░╚████╔╝░  █████═╝░██╔████╔██║╚█████╔╝██║░░██║░░███╔═╝
        ██╔══██╗░░╚██╔╝░░  ██╔═██╗░██║╚██╔╝██║██╔══██╗██║░░██║██╔══╝░░
        ██████╦╝░░░██║░░░  ██║░╚██╗██║░╚═╝░██║╚█████╔╝╚█████╔╝███████╗
        ╚═════╝░░░░╚═╝░░░  ╚═╝░░╚═╝╚═╝░░░░░╚═╝░╚════╝░░╚════╝░╚══════╝@KM8Oz")
        .color(ColorChoice::Always)
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Value {
    Bool(bool),
    String(String),
}
