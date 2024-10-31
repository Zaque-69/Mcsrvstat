use reqwest::blocking::get;
use std::env;

use crate::file::write_server_info;
use crate::file::format_words;
use crate::file::clear_server_data;

mod r#struct;
use r#struct::MinecraftServerInfo;
use serde::Deserialize;

fn final_address() -> Result<String, Box<dyn std::error::Error>> {
    // Collecting the argument and return the URL
    let arg: Vec<String> = env::args().collect();
    if arg.len() != 2 {
        return Err("You don't have 2 arguments!".into());
    }

    let server_address = &arg[1];
    let final_url : String = format!("https://api.mcsrvstat.us/3/{}", server_address);

    Ok(final_url)
}

fn server_data<MinecraftServerInfo: for<'de> Deserialize<'de>>() -> Result<MinecraftServerInfo, Box<dyn std::error::Error>>  {
    // Returning the parsed data
    let url = final_address()?;
    let response = get(url)?;
    let server_info = response.json::<MinecraftServerInfo>()?;

    Ok(server_info)
}


fn simple_data<T, F>(field: Option<T>, to_string: F) -> Result<(), Box<dyn std::error::Error>>
    // Making a function to get all the simple data
where
    F: FnOnce(T) -> String,
{
    if let Some(value) = field {
        write_server_info("serverinfo", &to_string(value))?;
    }
    Ok(())
}

fn complex_data(data : MinecraftServerInfo) -> Result<(), Box<dyn std::error::Error>> {
    // Making a function to write the complex data 
    if let Some(players) = &data.players {
        let players_info = format!("{} / {}", players.online, players.max);
        write_server_info("serverinfo", &players_info)?;
    }

    if let Some(motd) = &data.motd {
        let motto = motd.raw.join(" ");
        write_server_info("serverinfo", &motto)?;
    }

    if let Some(debug) = &data.debug {
        let debug_info = format_words(&debug
            .to_strings()
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>());
        write_server_info("serverinfo", &debug_info)?;
    }

    Ok(())
}

pub fn write_info() -> Result<(), Box<dyn std::error::Error>> {
    // Writing all the data in a file for TUI
    let data : MinecraftServerInfo = server_data()?;
    if data.online {
        // Deteting the file with the info before writing
        clear_server_data()?;
        write_server_info("serverinfo", "true")?;
        
        // Writing all the simple data 
        simple_data(data.ip, |ip| ip.to_string())?; 
        simple_data(data.port, |port| port.to_string())?; 
        simple_data(data.version, |version| version.to_string())?;
        simple_data(data.hostname, |hostname| hostname.to_string())?;

        // Writing the complex data, with more structures        
        complex_data(server_data()?)?;
    }
    else {
        println!("The server is offline!");
    }

    Ok(())
}
