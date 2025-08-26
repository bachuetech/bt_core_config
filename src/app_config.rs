use std::{collections::HashMap, error::Error};

use bt_logger::{log_info, log_warning};
use bt_yaml_utils::{get_yaml, get_yaml_from_string};
use yaml_rust2::Yaml;

use crate::app_info::AppInfo;

const APP_YML_CONFIG: &str = "config/core/app-config.yml";
const APP_YML_CONFIG_ENV_VAR_NAME: &str = "BT_APP_CONFIGYMLFILE";

//const APP_DEFAULT_NAME: &str = "BACHUETECH";
//const APP_DEFAULT_VERSION: &str = "x0.0.1d";

const DEFAULT_AGENT_HOST: &str = "localhost";
const DEFAULT_AGENT_PORT: u16 = 23332;


#[derive(Clone, Debug)]
pub struct AppConfig {
    name: String,
    version: String,
    environment: String,
    agent: AgentConfig,
    files_app_dir: String,
    app_path: String,
    api_path: String,
    end_points: HashMap<String, String>
}

#[derive(Clone, Debug)]
struct AgentConfig{
    host: String,
    port: u16,
    secure: bool,
    end_point: String,
}

impl AppConfig {
    // Constructor to read from YAML file
    pub fn new(running_environment: Option<String>, app_info: &AppInfo, embed_config: Option<&str>) -> Result<Self, Box<dyn Error>> {
        let app_config: Yaml = if let Some(yml_cfg) = embed_config {
            get_yaml_from_string(yml_cfg)?
        }else {
            get_yaml(APP_YML_CONFIG_ENV_VAR_NAME, APP_YML_CONFIG)?
        /*match  get_yaml(APP_YML_CONFIG_ENV_VAR_NAME, APP_YML_CONFIG) {
            Ok(y_file_conf) => app_config = y_file_conf,
            Err(e) => {log_fatal!("new","Fatal Error Reading APP configuration (PEC: {}). Application aborted! {}",APP_CONFIG_READING_ERROR, e.to_string()); process::exit(APP_CONFIG_READING_ERROR);}, // Exit the program with code -101 },
        }*/
        };

        let app_environment: String;

        match running_environment{
            Some(re) => {
                if app_config[re.as_str()].is_badvalue(){
                    app_environment = app_config["environment"].as_str().unwrap_or("dev").to_owned();
                    log_warning!("new","Could not find Running Environment {}. Using current default '{}' to continue.",&re, &app_environment);                    
                }else{
                    app_environment = re.clone();
                    log_info!("new","Using current environment '{}' from app config file.",&app_environment);
                }
            },
            None => {
                app_environment = app_config["environment"].as_str().unwrap_or("dev").to_owned();
                log_info!("new","Using current environment '{}' from app config file.",&app_environment);
            },
        }

        let mut end_points = HashMap::new();
        for ep_value in app_config[app_environment.as_str()]["end_points"].clone() {
            end_points.insert(
                ep_value["id"].as_str().unwrap().to_string(),
                ep_value["path"].as_str().unwrap_or(&format!("/{}",&ep_value["id"].as_str().unwrap().to_string())).to_string(),
            );
        }

        //Location of the Remote AI Agent
        let agent_cfg = AgentConfig{
            host: app_config[app_environment.as_str()]["agent"]["host"].as_str().unwrap_or(DEFAULT_AGENT_HOST).to_owned(),
            port: app_config[app_environment.as_str()]["agent"]["port"].as_i64().unwrap_or(DEFAULT_AGENT_PORT.into()).try_into().unwrap_or(DEFAULT_AGENT_PORT),
            secure: app_config[app_environment.as_str()]["agent"]["secure"].as_bool().unwrap_or(true),
            end_point: app_config[app_environment.as_str()]["agent"]["end_point"].as_str().unwrap_or("/").to_owned(),
        };

        //Application Information
        let app_name = app_config["app_name"]
            .as_str()
            .unwrap_or(&app_info.package_name);
        let app_ver = app_info.version;
        
        /*match option_env!("CARGO_PKG_VERSION"){
                            Some(v) => if v.is_empty() {
                                                        app_config["version"].as_str()
                                                        .unwrap_or(APP_DEFAULT_VERSION)
                                                     }else{v},
                                None => app_config["version"].as_str()
                                                        .unwrap_or(APP_DEFAULT_VERSION)};*/
        Ok(Self {
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
            agent: agent_cfg,
        })
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

    pub fn get_agent_url(&self) -> String{
        let http = if self.agent.secure {"https"} else {"http"};
        format!("{}://{}:{}{}",http,self.agent.host,self.agent.port,self.agent.end_point)
    }
}


//***********/
// UNIT TEST 
//***********/
#[cfg(test)]
mod app_config_tests {
    use bt_logger::{build_logger, LogLevel, LogTarget};

    use crate::app_info::AppInfo;

    use super::AppConfig;


    #[test]
    pub fn test_agent_config_default_env(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");
        let ac = AppConfig::new(None, &app_info, None);
        println!("{:?}",&ac);
        assert_eq!(ac.unwrap().get_agent_url(),"https://localhost:23332/");
    }

    #[test]
    pub fn test_agent_config_unknown_env(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "UNKNOWN";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");        
        let ac = AppConfig::new(Some(er.to_owned()), &app_info, None);
        println!("{:?}",&ac);
        assert_eq!(ac.unwrap().get_agent_url(),"https://localhost:23332/");
    }

    #[test]
    pub fn test_agent_config_success_env(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "jeremy_dev";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let ac = AppConfig::new(Some(er.to_owned()), &app_info, None);
        println!("{:?}",&ac);
        assert_eq!(ac.unwrap().get_agent_url(),"http://localhost:23332/ai/api/chat");
    }

    #[test]
    pub fn test_app_config_default_env(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let acr = AppConfig::new(None, &app_info, None);
        println!("{:?}",&acr);
        let ac = acr.unwrap();
        assert_eq!(ac.files_app_dir,"site");
        assert_eq!(ac.app_path,"/app");
        assert_eq!(ac.api_path,"/none/api/");
        assert_eq!(ac.get_environment(),"devNone");
        assert_eq!(ac.get_version(),"0.2.4");
    }

    #[test]
    pub fn test_app_config_unkown_env(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "UNKNOWN";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let acr = AppConfig::new(Some(er.to_owned()), &app_info, None);
        println!("{:?}",&acr);
        let ac = acr.unwrap();        
        assert_eq!(ac.files_app_dir,"site");
        assert_eq!(ac.app_path,"/app");
        assert_eq!(ac.api_path,"/none/api/");
        assert_eq!(ac.get_environment(),"devNone");
        assert_eq!(ac.get_version(),"0.2.4");
    }

    #[test]
    pub fn test_app_config_empty_env(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "empty";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let acr = AppConfig::new(Some(er.to_owned()), &app_info, None);
        println!("{:?}",&acr);
        let ac = acr.unwrap();        
        assert_eq!(ac.files_app_dir,"site");
        assert_eq!(ac.app_path,"/app");
        assert_eq!(ac.api_path,"/api");
        assert_eq!(ac.get_environment(),er);
        assert_eq!(ac.get_version(),"0.2.4");
    }    

    #[test]
    pub fn test_app_config_success(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "jeremy_dev";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let acr = AppConfig::new(Some(er.to_owned()), &app_info, None);
        println!("{:?}",&acr);
        let ac = acr.unwrap();        
        assert_eq!(ac.get_app_name(),"BACHUETECH AI");
        assert_eq!(ac.get_file_app_dir() ,"site");
        assert_eq!(ac.get_app_path(),"/jeremy");
        assert_eq!(ac.get_api_path(),"/ai/api/");
        assert_eq!(ac.get_environment(),er);
        assert_eq!(ac.get_version(),"0.2.4");
    }

   #[test]
    pub fn test_app_config_embeded_success(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "embed_dev";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");
        const YML_CONTENT: &str = include_str!("../config/core/app-config.yml");        
        let acr = AppConfig::new(Some(er.to_owned()), &app_info, Some(YML_CONTENT));
        println!("{:?}",&acr);
        let ac = acr.unwrap();        
        assert_eq!(ac.get_app_name(),"BACHUETECH AI");
        assert_eq!(ac.get_file_app_dir() ,"site");
        assert_eq!(ac.get_app_path(),"/embeded");
        assert_eq!(ac.get_api_path(),"/ai/api/");
        assert_eq!(ac.get_environment(),er);
        assert_eq!(ac.get_version(),"0.2.4");
    }    

    #[test]
    pub fn test_end_points(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let er = "dev";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let ac = AppConfig::new(Some(er.to_owned()), &app_info, None);
        println!("{:?}",&ac);     
        assert_eq!(ac.unwrap().get_end_point("chat"),"/chat");
    }
    
}