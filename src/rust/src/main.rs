use std::env;
use chrono::prelude::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let pass_file_path = "psw.txt";
	let log_file_path = "psw-log.txt";

	let user_name = env::var("username").expect("Can not found username!");
	let password = env::var("password").expect("Can not found password!");
   
	let mut check_password = String::from("");
	check_password.push_str(&user_name);
	check_password.push(',');
	check_password.push_str(&password);

    let password_str = fs::read_to_string(pass_file_path)?;
	let password_lines = password_str.lines();
	
	let mut check_result=false;
	for temp_user_password in password_lines {
        if temp_user_password==check_password {
			check_result=true;
			break;
		}
    }
	
	// todo 文件不存在时，则创建
    let mut log_file = OpenOptions::new().append(true).open(log_file_path)?;
    
	let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    if check_result {
        let log_text = current_time +" Successful authentication: "+ &user_name+"\r\n";
        log_file.write(log_text.as_bytes())?;
    } else {
        let log_text = current_time +" Failed authentication: "+ &user_name+","+ &password+"\r\n";
        log_file.write(log_text.as_bytes())?;
        panic!("failed authentication");
    }

    Ok(())
}