use std::process::{Command, exit};
use names::Generator;

fn update_commit_push(){

    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command.");

    if !add_command.status.success(){
        eprint!("Error: failed to add files to the repository.");
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to execute git commit command.");

    if !commit_command.status.success(){
        eprint!("Error: failed to commit files to the repository.");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .expect("Failed to execute git push command.");
    
    if !push_command.status.success(){
        eprint!("Error: failed to push changes to the repository.");
        exit(1);
    }

    println!("Succesfully added, committed, and pushed all changes.")
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    update_commit_push();
}
