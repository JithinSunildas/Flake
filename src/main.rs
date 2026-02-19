use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

static BOILERPLATE: &str = r#"
{
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
          ];

          # Sets up the environment so Rust can find the linker
          shellHook = ''
            fish
          '';
        };
      }
    );
}
"#;

const PATH: &str = "flake.nix";

const HELP_MENU: &str = r#"
FLAKE MANAGER (v0.1.0)
A quick-and-simple tool to scaffold and modify Nix flakes.

USAGE:
    flake [COMMAND] [ARGUMENTS]

COMMANDS:
    create                   Creates a flake.nix file in the current directory
    add <pkg1> <pkg2> ...    Inserts packages into buildInputs in flake.nix.
                             Creates a new flake.nix if one doesn't exist.
    help                     Shows this glorious menu.

EXAMPLES:
    flake create
    flake add python313Packages.numpy gcc rust-analyzer
"#;

fn add(args: &Vec<String>, index: &usize) {
    let mut file = match OpenOptions::new().read(true).write(true).open(PATH) {
        Err(_) => {
            create();
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(PATH)
                .unwrap()
        }
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("Couldnt open {}: {}", PATH, why),
        Ok(_) => println!("Opening {PATH}"),
    }

    let mut target_index = None;
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    for (idx, line) in lines.iter().enumerate() {
        if line.contains("buildInputs = with pkgs; [") {
            target_index = Some(idx);
            break;
        }
    }

    let target = target_index.expect("Corrupted file: buildInputs block not found");
    for pkg in &args[index + 1..] {
        lines.insert(target + 1, format!("            {pkg}"));
    }

    let new_content = lines.join("\n");
    std::fs::write(PATH, new_content).expect("Error while writing data to file: {PATH}");
}

fn create() -> File {
    if Path::new("flake.nix").exists() {
        panic!("File exists!");
    } else {
        println!("Creating {PATH}");

        let mut file = File::create(PATH).expect("Failed to create file 'flake.nix'");
        file.write_all(BOILERPLATE.as_bytes())
            .expect("Error writing to the buffer");
        file
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{HELP_MENU}");
        return;
    }
    for (i, command) in args[1..].iter().enumerate() {
        match command.as_str() {
            "add" => {
                add(&args, &(i + 1));
                break;
            }
            "create" => {
                create();
                break;
            }
            "help" | "-h" | "--help" => {
                println!("{HELP_MENU}");
                return;
            }
            _ => {
                println!("{HELP_MENU}");
                return;
            }
        };
    }
}
