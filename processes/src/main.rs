use std::{io::{BufRead, BufReader, Read, Write}, process::{Command, Stdio}};

fn main() {
    println!("Running example pipe_data_between_two_processes()\n\n");

    pipe_data_between_two_processes();

    println!("\n\nRunning example run_shell_and_interact_with_stdio_and_stdout()\n\n");

    run_shell_and_interact_with_stdio_and_stdout();

    println!("\n\nDone with examples");
}

/// Runs one process that echos three lines and pipes that output into a process
/// that runs grep to look for the word "world". This example will only run on
/// *nix operating systems since this example uses the commands `echo` and `grep`.
/// When spawning a process, you can tell it to pipe it's stdin, stdout, or stderr.
/// Since the struct Stdio implements From<ChildStderr>, From<ChildStdin>, and 
/// From<ChildStdout>, the piped io stream from one process can be directly given
/// to different new process as its stdin, stdout, or stderr.
fn pipe_data_between_two_processes() {
    if cfg!(not(target_os = "windows")) {
        let mut echo = Command::new("echo")
            .arg("Line 1: Hello\nLine 2: Hello world!\nLine 3: world!")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute echo");
        
        let mut grep = Command::new("grep")
            .arg("world")
            .stdin(echo.stdout.take().unwrap())
            .spawn()
            .expect("Failed to execute grep");

        echo.wait().expect("Failed to wait for echo");
        grep.wait().expect("Failed to wait for grep");
    } else {
        println!("Running on windows so pipe_data_between_two_processes() will not run.");
    }
}

fn run_shell() -> Command {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    }
}

/// This runs a process that starts a shell in interactive mode. The parent process
/// then writes to the child processes piped stdin and reads from its piped stdout.
/// This example uses the shell aliased to `sh` on *nix systems and uses `cmd` on
/// Windows.
fn run_shell_and_interact_with_stdio_and_stdout() {
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
