mod tests;

use std::fs;
use std::collections::HashMap;
use std::ops::Deref;

pub struct EnvHolder {
    variables : HashMap<String, String>
}

impl EnvHolder {
    pub fn new () -> Self {
        let mut env_holder =  Self {
            variables : HashMap::new(),
        };
        let file = EnvHolder::check_available_file();

        match file {
            Some(".env") => env_holder.set_var_from_env_file(),
            Some(".env.json") => {
                // Handle .env.json file here if needed
            }
            _ => {}
        }
       return env_holder;

    }

    pub fn get_var(&self, env_name: &str) -> Option<&str> {
           let var = self.variables.get(env_name);

           if let Some(var_value) = var {
           return Some(var_value.deref());
           }else {
            return  None;
           }

           
    }

    fn set_var_from_env_file (&mut self) {
        // Read the .env file
        let file = fs::read_to_string(".env");
        
        match file {
            Ok(file) => {
                for line in file.lines() {
                    let trimmed_line = line.trim();
                    if trimmed_line.is_empty() || trimmed_line.starts_with("#"){
                        continue;
                    } else if trimmed_line.contains("=") {
                    if let Some((key, value)) =  line.trim().split_once("="){
                            
                            let key = key.trim_matches('\"').to_string();
                            let value = value.trim_matches('\"').to_string();
                            println!("{key} {value}");
                            self.variables.insert(key, value);
                    }
                    }
                }
            },
            Err(err) => {
                println!("Error while reading {}", err);
            }
        }
    }

    /// Checks if a .env or .env.json file is available.
    fn check_available_file () -> Option<&'static str> {
        
        let entries = match fs::read_dir("./") {
            Ok(entries) => entries,
            Err(e) => {
                println!("Error reading directory {}", e);
                return None;
            }
        };

        for entry in entries {
            if let Ok(file) = entry {
            match file.file_name().to_str(){
                Some(".env" )=>  return Some(".env"),
                Some(".env.json") =>  return Some(".env.json"),
                _ => continue,
            }
            }
        }

        None
        
}
}



