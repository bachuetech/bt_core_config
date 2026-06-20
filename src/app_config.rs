use std::collections::HashMap;

use bt_any_error::any_err::AnyErr;
use bt_logger::{log_error, log_info, log_warning};
use bt_yaml_utils::{get_yaml, get_yaml_from_string};
use yaml_rust2::Yaml;

use crate::app_info::AppInfo;
use crate::utils::init_app_base_url;

const APP_YML_CONFIG: &str = "config/core/app-config.yml";
const APP_YML_CONFIG_ENV_VAR_NAME: &str = "BT_APP_CONFIGYMLFILE";

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
    host: Option<String>,
    port: Option<i64>,
    secure: Option<bool>,
    end_point: String,
}

impl AppConfig {
    // Constructor to read from YAML file
    pub fn new(running_environment: &str, app_info: &AppInfo, embed_config: Option<&str>) -> Result<Self, AnyErr> {
        let app_config: Yaml = if let Some(yml_cfg) = embed_config {
            get_yaml_from_string(yml_cfg)?
        }else {
            get_yaml(APP_YML_CONFIG_ENV_VAR_NAME, APP_YML_CONFIG)?
        };

        let app_environment: &str;

        if running_environment.trim().is_empty() || app_config[running_environment].is_badvalue(){
            log_error!("","Invalid Running Environment '{}'. Will use default to continue.",running_environment);  
            #[cfg(debug_assertions)]
                const RUN_ENV: &str = "dev";
            #[cfg(not(debug_assertions))]
                const RUN_ENV: &str = "prod";                     
            app_environment = app_config["environment"].as_str().unwrap_or(RUN_ENV);
            log_warning!("","Could not find Running Environment '{}'. Using current default '{}' to continue.",running_environment, app_environment);                    
        }else{
            app_environment = running_environment;
            log_info!("","Using current environment '{}'.",&app_environment);
        }

        let mut end_points = HashMap::new();
        for ep_value in app_config[app_environment]["end_points"].clone() {
            end_points.insert(
                ep_value["id"].as_str().unwrap().to_string(),
                ep_value["path"].as_str().unwrap_or(&format!("/{}",&ep_value["id"].as_str().unwrap().to_string())).to_string(),
            );
        }

        //Location of the Remote AI Agent
        let agent_cfg = AgentConfig{
            host: app_config[app_environment]["agent"]["host"].as_str().map(|s| s.to_string()), //.unwrap_or(DEFAULT_AGENT_HOST).to_owned(),
            port: app_config[app_environment]["agent"]["port"].as_i64(), //.unwrap_or(DEFAULT_AGENT_PORT.into()).try_into().unwrap_or(DEFAULT_AGENT_PORT),
            secure: app_config[app_environment]["agent"]["secure"].as_bool(), //.unwrap_or(true),
            end_point: app_config[app_environment]["agent"]["end_point"].as_str().unwrap_or("").to_owned(),
        };

        //Application Information
        let app_name = app_config["app_name"]
            .as_str()
            .unwrap_or(app_info.package_name);
        let app_ver = app_info.version;
        
        let app_path = app_config[app_environment]["app_path"]
                .as_str()
                .unwrap_or("/app")
                .to_string();
        
        init_app_base_url(&app_path);

        Ok(Self {
            name: app_name.to_owned(),
            version: app_ver.to_owned(),
            environment: app_environment.to_owned(),
            files_app_dir: app_config[app_environment]["files_app_dir"]
                .as_str()
                .unwrap_or("site")
                .to_string(),
            app_path,
            api_path: app_config[app_environment]["api_path"]
                .as_str()
                .unwrap_or("/api")
                .to_string(),
            end_points,
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
        let mut agent_url = if self.agent.secure.is_some() {
                                    if self.agent.secure.unwrap() {"https://".to_owned()} else {"http://".to_owned()}
                                }else {"".to_owned()};
        if self.agent.host.is_some() {
            agent_url.push_str(&self.agent.host.clone().unwrap());
        }
        if let Some(agent_port) = self.agent.port {
            agent_url.push_str(&format!(":{}",agent_port));
        }
        format!("{}{}",agent_url,self.agent.end_point)
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
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");
        let ac = AppConfig::new("", &app_info, None);
        println!("{:?}",&ac);
        assert_eq!(ac.unwrap().get_agent_url(),"");
    }

    #[test]
    pub fn test_agent_config_unknown_env(){
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let er = "UNKNOWN";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");        
        let ac = AppConfig::new(er, &app_info, None);
        println!("{:?}",&ac);
        assert_eq!(ac.unwrap().get_agent_url(),"");
    }

    #[test]
    pub fn test_agent_config_success_env(){
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let er = "jeremy_dev";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let ac = AppConfig::new(er, &app_info, None);
        println!("{:?}",&ac);
        assert_eq!(ac.unwrap().get_agent_url(),"http://localhost:23332/ai/api/chat");
    }

    #[test]
    pub fn test_app_config_default_env(){
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let acr = AppConfig::new("", &app_info, None);
        println!("{:?}",&acr);
        let ac = acr.unwrap();
        assert_eq!(ac.files_app_dir,"site");
        assert_eq!(ac.app_path,"/app");
        assert_eq!(ac.api_path,"/none/api/");
        assert_eq!(ac.get_environment(),"devNone");
        assert_eq!(ac.get_version(),"0.6.0");
    }

    #[test]
    pub fn test_app_config_unkown_env(){
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let er = "UNKNOWN";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let acr = AppConfig::new(er, &app_info, None);
        println!("{:?}",&acr);
        let ac = acr.unwrap();        
        assert_eq!(ac.files_app_dir,"site");
        assert_eq!(ac.app_path,"/app");
        assert_eq!(ac.api_path,"/none/api/");
        assert_eq!(ac.get_environment(),"devNone");
        assert_eq!(ac.get_version(),"0.6.0");
    }

    #[test]
    pub fn test_app_config_empty_env(){
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let er = "empty";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let acr = AppConfig::new(er, &app_info, None);
        println!("{:?}",&acr);
        let ac = acr.unwrap();        
        assert_eq!(ac.files_app_dir,"site");
        assert_eq!(ac.app_path,"/app");
        assert_eq!(ac.api_path,"/api");
        assert_eq!(ac.get_environment(),er);
        assert_eq!(ac.get_version(),"0.6.0");
    }    

    #[test]
    pub fn test_app_config_success(){
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let er = "jeremy_dev";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let acr = AppConfig::new(er, &app_info, None);
        println!("{:?}",&acr);
        let ac = acr.unwrap();        
        assert_eq!(ac.get_app_name(),"BACHUETECH AI");
        assert_eq!(ac.get_file_app_dir() ,"site");
        assert_eq!(ac.get_app_path(),"/jeremy");
        assert_eq!(ac.get_api_path(),"/ai/api/");
        assert_eq!(ac.get_environment(),er);
        assert_eq!(ac.get_version(),"0.6.0");
    }

   #[test]
    pub fn test_app_config_embeded_success(){
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let er = "embed_dev";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");
        const YML_CONTENT: &str = include_str!("../config/core/app-config.yml");        
        let acr = AppConfig::new(er, &app_info, Some(YML_CONTENT));
        println!("{:?}",&acr);
        let ac = acr.unwrap();        
        assert_eq!(ac.get_app_name(),"BACHUETECH AI");
        assert_eq!(ac.get_file_app_dir() ,"site");
        assert_eq!(ac.get_app_path(),"/embeded");
        assert_eq!(ac.get_api_path(),"/ai/api/");
        assert_eq!(ac.get_environment(),er);
        assert_eq!(ac.get_version(),"0.6.0");
    }    

    #[test]
    pub fn test_end_points(){
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let er = "dev";
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");                
        let ac = AppConfig::new(er, &app_info, None);
        println!("{:?}",&ac);     
        assert_eq!(ac.unwrap().get_end_point("chat"),"/chat");
    }
    
}