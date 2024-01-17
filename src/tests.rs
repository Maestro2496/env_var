#[cfg(test)]

use super::*;

#[test]
fn test_check_available_file() {
    
    let result = EnvHolder::check_available_file();

    assert!(result.is_some());

    match result {
        Some(".env") => println!(".env file found!"),
        Some(".env.json") => println!(".env.json file found!"),
        None => println!("No file found"),
        _ => panic!("Unexpected Behavior")
    }

}

#[test]
fn test_set_var_from_env_file() {
    let env_holder = EnvHolder::new();

    let url = env_holder.get_var("job");
    
    if let Some(url_value) = url {
        assert_eq!(url_value, "eddyagossou.me");
    }else {
        assert_eq!(url, None);
    }
    
}

#[test]
fn test_set_var_from_json_file() {
    let env_holder = EnvHolder::new();
    let url = env_holder.get_var("url");
    if let Some(url_value) = url {
        assert_eq!(url_value, "eddyagossou.me");
    }else {
       assert_eq!(url, None);
    }
}