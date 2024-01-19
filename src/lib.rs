mod tests;
use std::fs;
use std::env;
use std::collections::HashMap;
use std::ops::Deref;
use serde_json::{Value, Error};
use regex::Regex;

#[derive(Debug)]
pub struct EnvHolder {
    variables : HashMap<String, String>,
    debug : bool,
    file_name: String,
}

impl EnvHolder {
    pub fn new (debug : bool) -> Self {

        let mut env_holder =  Self {
            variables : HashMap::new(),
            debug,
            file_name : String::from(""),
        };

        let file = EnvHolder::check_available_file();

        match file {
            Some(".env") => {
                env_holder.set_var_from_env_file(".env");
                env_holder.file_name = String::from(".env");
            },
            Some(".env.json") => {
                env_holder.set_var_from_json(".env.json");
                env_holder.file_name = String::from(".env.json");
            },
            _ =>  {
             let _ =  fs::write(".env", "path = '.env'").map_err(|err| {
                if debug {
                    println!("Error creating '.env' file: {:?}", err);
                }
                err
             });
             env_holder.file_name = String::from(".env");
            },
        }
        // Variables from the command line take precedence
        env_holder.read_var_from_cmd_line();


        return env_holder;

    }

    pub fn with_debug(&mut self) -> &mut Self {
      self.debug = true;
      self
    }


    pub fn with_file_name(mut self, file_name: &str) -> Self{
            
            if file_name.ends_with("txt") | file_name.ends_with(".env") {
               
                        self.set_var_from_env_file(file_name)
                    }

            else if file_name.ends_with(".json"){
                self.set_var_from_json(file_name)
            }

            else {
                panic!("File not supported!")
            }

            self
    }


    pub fn get_var(&self, env_name: &str) -> Option<&str> {
           let var = self.variables.get(env_name);

           if let Some(var_value) = var {
           return Some(var_value.deref());
           }else {
            return  None;
           }

           
    }

    fn set_var_from_json (&mut self, file_name: &str) {


        assert!(file_name.ends_with(".json"), "Json file required!");

        let file = fs::read_to_string(file_name);


        match file {
            Ok(file) => {
                // Convert to json
                let parsed_file: Result<Value, Error>= serde_json::from_str(&file);
                
                if let Ok(parsed_file) = parsed_file {

                    match parsed_file {
                        Value::Object(json_obj) => {
                            for (key, value) in json_obj {
                                    let value = value.to_string()
                                                            .replace("\"", "")
                                                            .replace("'", "")
                                                            .trim().to_string();
                                            
                                    self.variables.insert(key, value);
                             }
                        },
                        _ => return,
                    }
                    
                }

            },
            Err(err) => {
                if self.debug {
                    println!("Error while reading the file. {}", err)
                }
               
            }
        }
    }

    fn set_var_from_env_file (&mut self, file_name: &str) {

        let pattern = Regex::new(r"\.(txt|env)$").unwrap();

        assert!(pattern.is_match(file_name), "Invalid file extension");

        // Read the .env or .txt file
        let file = fs::read_to_string(file_name);
        
        match file {
            Ok(file) => {
                for line in file.lines() {
                    let trimmed_line = line.trim();
                    if trimmed_line.is_empty() || trimmed_line.starts_with("#"){
                        continue;
                    } else if trimmed_line.contains("=") {
                    if let Some((key, value)) =  line.trim().split_once("="){
                            
                            let key = key.trim_matches('\"').trim().to_string();
                            println!("{}", value);
                            let value = value
                                .replace("\"", "")
                                .replace("'", "")
                                .trim().to_string();
                            println!("{}", value);
                            self.variables.insert(key, value);
                    }
                    }
                }
            },
            Err(err) => {
                if self.debug {
                    println!("Error while reading {}", err);
                }
               
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

  
    pub fn read_var_from_cmd_line (&mut self) {
        let mut contents = String::from("");
        for arg in env::args(){
            println!("arg{}", arg);
            contents.push_str(&(arg + "\n"));

        }
        print!("{}", contents);

        for line in contents.lines(){
            let trimmed_line = line.trim();
            if let Some((key, value)) = trimmed_line.split_once("="){
                self.variables.insert(key.to_string(), value.to_string());
            }
        }
    }
    

}



