use std::{io::{self, BufRead, Write}, process::Command, env::args};

fn kill_processes(processes_to_kill: Vec<u32>) {
    for pid in processes_to_kill {
        let status = Command::new("kill")
            .arg("-KILL")
            .arg(pid.to_string())
            .status();

        match status {
            Ok(s) if s.success() => println!("Successfully killed process with PID: {}", pid),
            _ => eprintln!("Error killing process with PID: {}", pid),
        }
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    let auto_confirm = args.iter().any(|arg| arg == "--yes" || arg == "-y");
    let mut processes_to_kill: Vec<u32> = Vec::new();

    for line in io::stdin().lock().lines() {
        match line {
            Ok(text) => {
                let parts: Vec<&str> = text.trim().split_whitespace().collect();
                if let Some(pid_str) = parts.get(0) {
                    if pid_str.parse::<u32>().is_ok() {                      
                        // Finding grep process
                        let mut grep_found = false;
                        for part in &parts[1..] {
                            if part == &"grep" {
                                // println!("Grep will not be killed. PID: {}", pid_str);
                                grep_found = true;
                                continue;
                            }
                        }
                        if grep_found {
                            continue;
                        }

                        processes_to_kill.push(pid_str.parse::<u32>().unwrap());
                    }
                }
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    if processes_to_kill.len() == 0 {
        println!("No processes to kill.");
        return;
    }

    println!("Processes to kill: {:?}", processes_to_kill);

    if auto_confirm {
        kill_processes(processes_to_kill);
        return;
    }
    else {
        print!("Confirm with 'y' to kill the processes, or any other key to cancel: ");

        io::stdout().flush().expect("Failed to flush stdout");

        let mut confirmation = String::new();

        if let Ok(mut tty) = std::fs::File::open("/dev/tty") {
            let mut reader = io::BufReader::new(&mut tty);
            reader.read_line(&mut confirmation).expect("Failed to read confirmation");
        } else {
            io::stdin().read_line(&mut confirmation).expect("Failed to read confirmation and open tty");
        }

        if confirmation.trim().eq_ignore_ascii_case("y") {
            kill_processes(processes_to_kill);
        } else {
            println!("Process killing cancelled.");
        }
    }
}
