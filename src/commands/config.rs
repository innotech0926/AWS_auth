use crate::utils::extend_path;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Write};
use std::{fs, io};
use tracing::{error, info};

// The name of the file and directory for the configuration will not be custom by the moment
pub const CONFIG_FILE_PATH: &str = "~/.aws/aws-sso-auth.json";

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct Configuration {
    profile_name: String,
    parameters: Parameters,
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
struct Parameters {
    start_url: String,
    aws_region: String,
}

impl Configuration {
    pub fn new(start_url: String, aws_region: String, profile_name: String) -> Self {
        Self {
            profile_name,
            parameters: Parameters {
                start_url,
                aws_region,
            },
        }
    }

    // This function will write configuration file in
    // TO DO: allow multiple profiles with different AWS accounts
    // TO DO: if you append to the config file with different configurations, implement the possibility of update existing configuration
    pub fn write_config_file(&self) {
        let file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(extend_path(CONFIG_FILE_PATH))
        {
            Ok(file) => file,
            Err(err) => {
                error!("Can't create configuration file. {}", err);
                std::process::exit(1);
            }
        };

        let config = Configuration {
            profile_name: self.profile_name.to_string(),
            parameters: Parameters {
                start_url: self.parameters.start_url.to_string(),
                aws_region: self.parameters.aws_region.to_string(),
            },
        };

        match serde_json::to_writer(file, &config) {
            Ok(_) => {
                info!("Configuration file saved!");
            }
            Err(err) => {
                error!("Can't write configuration file. {}", err)
            }
        }
    }
}

// pub fn read_config_file()  {
//     let file = OpenOptions::new()
//         .read(true)
//         .open(extend_path(CONFIG_FILE_PATH)).expect_err("Can't open config file");

//     let config: Configuration = serde_json::from_reader(file).expect("No puedo serializar la data!!");
// }
