use std::env;
use std::process;
use std::fs;
use std::path::{Path,PathBuf};
use std::ffi::{OsStr,OsString};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let tar_dir = Path::new(&config.target_dir);

    if let Ok(entries) = fs::read_dir(Path::new(tar_dir)){
        for entry in entries{
            if let Ok(entry) = entry{
                let path = entry.path();

                if path.is_file() && path.file_name().unwrap_or_default() == OsStr::new(&config.target_file){
                    let new_path = path.with_file_name(&config.new_name);
                    match fs::rename(&path, &new_path){
                        Ok(_) => println!("File renamed from {:?} to {:?}", path, new_path),
                        Err(e) => eprintln!("Error in renamining file: {}", e),

                    }
                }
            }
        }
    }

}

struct Config{
    target_dir: String,
    target_file: String,
    new_name: String
}

impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 4{
            return Err("not enough arguments");
        }
        let target_dir = args[1].clone();
        let target_file = args[2].clone();
        let new_name = args[3].clone();
        Ok(Config {target_dir,target_file,new_name})
    }
}
