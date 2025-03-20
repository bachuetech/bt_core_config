use std::{collections::HashMap, process};

use bt_app_codes::process_exit_codes::APP_CONFIG_READING_ERROR;
use bt_logger::{log_fatal, log_info, log_warning};
use bt_yaml_utils::get_yaml;
use yaml_rust2::Yaml;

const APP_YML_CONFIG: &str = "config/core/app-config.yml";
const APP_YML_CONFIG_ENV_VAR_NAME: &str = "BT_AIAPP_CONFIGYMLFILE";

const APP_DEFAULT_NAME: &str = "BACHUETECH";
const APP_DEFAULT_VERSION: &str = "x0.0.1d";

#[derive(Clone, Debug)]
pub struct AppConfig {
    name: String,
    version: String,
    environment: String,
    files_app_dir: String,
    app_path: String,
    api_path: String,
    end_points: HashMap<String, String>
}

impl AppConfig {
    // Constructor to read from YAML file
    pub fn new(running_environment: Option<String>) -> Self {
        let app_config: Yaml;
        
        match  get_yaml(APP_YML_CONFIG_ENV_VAR_NAME, APP_YML_CONFIG) {
            Ok(y_file_conf) => app_config = y_file_conf,
            Err(e) => {log_fatal!("new","Fatal Error Reading APP configuration. Application aborted! {}",e.to_string()); process::exit(APP_CONFIG_READING_ERROR);}, // Exit the program with code -101 },
        }
        let app_environment: String;

        match running_environment{
            Some(re) => {
                if app_config[re.as_str()].is_badvalue(){
                    app_environment = app_config["environment"].as_str().unwrap_or("dev").to_owned();
                    log_warning!("new","Could not find Running Environment {}. Using current default '{}' to continue.",&re, &app_environment);                    
                }else{
                    app_environment = re.clone();
                }
            },
            None => {
                app_environment = app_config["environment"].as_str().unwrap_or("dev").to_owned();
                log_info!("new","Using current environment '{}' from app config file.",&app_environment);
            },
        }

        let mut end_points = HashMap::new();
        for value in app_config[app_environment.as_str()]["end_points"].clone() {
            end_points.insert(
                value["id"].as_str().unwrap().to_string(),
                value["path"].as_str().unwrap_or(&format!("/{}",&value["id"].as_str().unwrap().to_string())).to_string(),
            );
        }

        let app_name = app_config["app_name"]
            .as_str()
            .unwrap_or(APP_DEFAULT_NAME);
        let app_ver = app_config["version"]
            .as_str()
            .unwrap_or(APP_DEFAULT_VERSION);

        Self {
            name: app_name.to_owned(),
            version: app_ver.to_owned(),
            environment: app_environment.to_owned(),
            files_app_dir: app_config[app_environment.as_str()]["files_app_dir"]
                .as_str()
                .unwrap_or("site")
                .to_string(),
            app_path: app_config[app_environment.as_str()]["app_path"]
                .as_str()
                .unwrap_or("/app")
                .to_string(),
            api_path: app_config[app_environment.as_str()]["api_path"]
                .as_str()
                .unwrap_or("/api")
                .to_string(),
            end_points: end_points,
        }
    }

    pub fn get_environment(&self) -> String {
        self.environment.clone()
    }

    pub fn get_file_app_dir(&self) -> String {
        self.files_app_dir.clone()
    }

    pub fn get_app_path(&self) -> String {
        self.app_path.clone()
    }

    pub fn get_api_path(&self) -> String {
        self.api_path.clone()
    }

    pub fn get_end_point(&self, end_point_name: &str) -> String {
        self.end_points
            .get(end_point_name)
            .unwrap_or(&format!("/{}",&end_point_name.to_string()))
            .to_string()
    }

    pub fn get_app_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }
}


//***********/
// UNIT TEST 
//***********/
#[cfg(test)]
mod app_config_tests {
    use bt_logger::{build_logger, LogLevel, LogTarget};

    use super::AppConfig;


    #[test]
    pub fn test_app_config_default_env(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let ac = AppConfig::new(None);
        println!("{:?}",&ac);
        assert_eq!(ac.files_app_dir,"site");
        assert_eq!(ac.app_path,"/app");
        assert_eq!(ac.api_path,"/ai/api/");
        assert_eq!(ac.get_environment(),"dev");
        assert_eq!(ac.get_version(),"v0.1.0");
    }

    #[test]
    pub fn test_app_config_unkown_env(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "UNKONW";
        let ac = AppConfig::new(Some(er.to_owned()));
        println!("{:?}",&ac);
        assert_eq!(ac.files_app_dir,"site");
        assert_eq!(ac.app_path,"/app");
        assert_eq!(ac.api_path,"/ai/api/");
        assert_eq!(ac.get_environment(),"dev");
        assert_eq!(ac.get_version(),"v0.1.0");
    }

    #[test]
    pub fn test_app_config_empty_env(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "empty";
        let ac = AppConfig::new(Some(er.to_owned()));
        println!("{:?}",&ac);
        assert_eq!(ac.files_app_dir,"site");
        assert_eq!(ac.app_path,"/app");
        assert_eq!(ac.api_path,"/api");
        assert_eq!(ac.get_environment(),er);
        assert_eq!(ac.get_version(),"v0.1.0");
    }    

    #[test]
    pub fn test_app_config_success(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "jeremy_dev";
        let ac = AppConfig::new(Some(er.to_owned()));
        println!("{:?}",&ac);
        assert_eq!(ac.get_app_name(),"BACHUETECH AI");
        assert_eq!(ac.get_file_app_dir() ,"site");
        assert_eq!(ac.get_app_path(),"/jeremy");
        assert_eq!(ac.get_api_path(),"/ai/api/");
        assert_eq!(ac.get_environment(),er);
        assert_eq!(ac.get_version(),"v0.1.0");
    }

    #[test]
    pub fn test_end_points(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "dev";
        let ac = AppConfig::new(Some(er.to_owned()));
        println!("{:?}",&ac);     
        assert_eq!(ac.get_end_point("chat"),"/chat");
    }
    
}