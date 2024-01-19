#[cfg(test)]

use super::*;

#[test]
fn test_check_available_file() {
    
    let result = EnvHolder::check_available_file();

    assert!(result.is_some());
    if result.is_some(){
        match result {
            Some(".env") => println!(".env file found!"),
            Some(".env.json") => println!(".env.json file found!"),
            None => println!("No file found"),
            _ => panic!("Unexpected Behavior")
        }
    }else{
        assert_eq!(fs::metadata(".env").is_ok(), true)
    }
    

}

#[test]
fn test_set_var_from_env_file() {
    let env_holder = EnvHolder::new(true);
    

    let url = env_holder.get_var("path");
    
    if let Some(url_value) = url {
        assert_eq!(url_value, ".env");
    }else {
        assert_eq!(url, None);
    }
    
}

#[test]
fn test_set_var_from_json_file() {
    let env_holder = EnvHolder::new(true);
    let url = env_holder.get_var("path");
    if let Some(url_value) = url {
        assert_eq!(url_value, ".env");
    }else {
       assert_eq!(url, None);
    }
}
#[test]
fn test_init_with_file_name() {
    fs::write("test.env", "path=test.json").ok();

    let env_holder = EnvHolder::new(true).with_file_name("test.env");

   

    let path = env_holder.get_var("path");
    if let Some(path_value) = path {
        assert_eq!(path_value, "test.json");
    }else {
       assert_eq!(path, None);
    }

    fs::remove_file("test.env").ok();
}

// #[test]
// fn test_read_from_cmd_line (){
//     // Must run cargo test with some env variables
    
//     let env_holder = EnvHolder::new(true);
   

//     if let Some(url) = env_holder.get_var("url") {
//         print!("{}", url);
//         assert_eq!(url, "test")
//     }else{
//        panic!("")
//     }
    
    
// }