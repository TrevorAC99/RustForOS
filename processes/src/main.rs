use std::{io::{BufRead, BufReader, Read, Write}, process::{Command, Stdio}};

fn main() {
    println!("======== Running example1 ========\n");
    example1();
    println!("\n\n");
    
    println!("======== Running example2 ========\n");
    example2();
    println!("\n\n");
    
    println!("======== Running example3 ========\n");
    example3();
    println!("\n\n");
    
    println!("======== Running example4 ========\n");
    example4();
}

fn echo_command() -> Command {
    if cfg!(target_os = "windows") {
        let mut command = Command::new("cmd");
        command.args(&["/C", "echo hello"]);
        command
    } else {
        let mut command = Command::new("sh");
        command.arg("-c").arg("echo hello");
        command
    }
}

fn example1() {
    let output = echo_command()
        .output()
        // .spawn()
        .expect("failed to execute process");

    let stdout_output = String::from_utf8_lossy(&output.stdout);
    println!("Got \"{}\" from child's stdout", stdout_output);
}

fn example2() {
    let mut child = echo_command()
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    if let Some(child_stdout) = child.stdout.as_mut() {
        let mut buf = String::new();
        child_stdout
            .read_to_string(&mut buf)
            .expect("failed to read from child stdout");

        println!("Got \"{}\" from child's stdout", buf);
    }
}

fn run_shell() -> Command {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    }
}

fn example3() {
    let mut child = run_shell()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all("echo \"Hello, world!\"\nexit\n".as_bytes())
            .expect("Failed to write to stdin");
    }

    {
        let stdout = child.stdout.as_mut().expect("Failed to open stdout.");
        let mut buf = String::new();
        stdout.read_to_string(&mut buf).expect("failed to read from stdout");
        println!("Got \"{}\" from child's stdout", buf);
    }
}

fn example4() {
    let mut child = run_shell()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all("echo \"Hello, world!\"\nexit\n".as_bytes())
            .expect("Failed to write to stdin");
    }

    {
        let stdout = child.stdout.as_mut().expect("Failed to open stdout.");
        let mut buf_stdout = BufReader::new(stdout);
        let mut buf = String::new();
        buf_stdout.read_line(&mut buf).expect("failed to read from stdout");
        println!("Got \"{}\" from child's stdout", buf);

        let mut buf = String::new();
        buf_stdout.read_to_string(&mut buf).expect("failed to read from stdout");
        println!("But wait! There's more! Got \"{}\" from child's stdout", buf);
    }
}
