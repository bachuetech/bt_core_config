
use bt_any_error::any_err::AnyErr;
use bt_logger::{log_error, log_info, log_warning};
use bt_yaml_utils::{get_yaml, get_yaml_from_string};
use yaml_rust2::Yaml;

const SRV_YML_CONFIG: &str = "config/core/server-config.yml";
const SRV_YML_CONFIG_ENV_VAR_NAME: &str = "BT_SRV_CONFIGYMLFILE";

const DEFAULT_PORT: i64  = 23339;
const DEFAULT_HOST: &str = "localhost";

#[derive(Debug)]
pub struct ServerConfig {
    host: String,
    port: u16,
    secure: bool,
}

impl ServerConfig {
    /// Constructor. Reading from YAML file
    /// Arguments:
    /// run_env: Receives the current running environment (The file may contain several environments)
    /// embed_config: Content of the YML config file. None to use env variable or default.
    pub fn new(running_environment: &str, embed_config: Option<&str>) -> Result<Self, AnyErr> {

        let srv_config: Yaml = if let Some(yml_cfg) = embed_config {
                                    get_yaml_from_string(yml_cfg)?
                                } else {
                                    get_yaml(SRV_YML_CONFIG_ENV_VAR_NAME, SRV_YML_CONFIG)?
                                };

        let svr_environment: &str;

        if running_environment.trim().is_empty() || srv_config[running_environment].is_badvalue(){
            log_error!("","Invalid Running Environment '{}'. Will use default to continue.",running_environment);  
            #[cfg(debug_assertions)]
                const RUN_ENV: &str = "dev";
            #[cfg(not(debug_assertions))]
                const RUN_ENV: &str = "prod";                     
            svr_environment = srv_config["environment"].as_str().unwrap_or(RUN_ENV);
            log_warning!("","Could not find Running Environment '{}'. Using current default '{}' to continue.",running_environment, svr_environment);                    
        }else{
            svr_environment = running_environment;
            log_info!("","Using current environment '{}'.",&svr_environment);
        }

        let mut srv_port = srv_config[svr_environment]["server"]["port"]
            .as_i64()
            .unwrap_or(DEFAULT_PORT);
        srv_port = if !(0..=65535).contains(&srv_port) {
            DEFAULT_PORT
        } else {
            srv_port
        };

        Ok(Self {
            host: srv_config[svr_environment]["server"]["host"]
                .as_str()
                .unwrap_or(DEFAULT_HOST)
                .to_string(),
            port: srv_port as u16,
            secure: srv_config[svr_environment]["server"]["secure"]
                .as_bool()
                .unwrap_or(true),
        })
    }

    pub fn get_tcp_listener(&self) -> String {
        format!("{}:{}", self.host.clone(), self.port)
    }

    pub fn is_secure(&self) -> bool {
        self.secure
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_host(&self) -> String {
        self.host.clone()
    }    
}

pub fn get_srv_config(current_env: &str,  embed_config: Option<&str>) -> Result<ServerConfig, AnyErr> {
    ServerConfig::new(current_env,  embed_config)
}

//***********/
// UNIT TEST 
//***********/
#[cfg(test)]
mod server_config_tests {
    use bt_logger::{build_logger, LogLevel, LogTarget};

    use crate::server_config::get_srv_config;

    use super::ServerConfig;


    #[test]
    pub fn test_svr_conf_unkown_env(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR, None);
        let c_env = "UNKNOWN";
        let sc = get_srv_config(c_env, None).unwrap();
        println!("{:?}",&sc);
        assert_eq!(sc.get_port(),23332);
        assert_eq!(sc.host,"0.0.0.0");
        assert_eq!(sc.is_secure(),false);
    }

    #[test]
    pub fn test_svr_conf_tpc_listener(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let c_env = "UNKNOWN";
        let sc = ServerConfig::new(c_env, None).unwrap();
        println!("{:?}",&sc);
        let res = format!("{}:{}", "0.0.0.0", 23332);
        assert_eq!(sc.get_host(),"0.0.0.0");
        assert_eq!(sc.get_tcp_listener(),res);
    }

    #[test]
    pub fn test_svr_conf_tpc_listener_dev(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let c_env = "dev";
        let sc = ServerConfig::new(c_env, None).unwrap();
        println!("{:?}",&sc);
        let res = format!("{}:{}", "0.0.0.0", 23332);
        assert_eq!(sc.get_tcp_listener(),res);
        assert_eq!(sc.is_secure(),false);
    }

    #[test]
    pub fn test_svr_conf_tpc_listener_dev_from_str(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let c_env = "prod";
        const YML_CONTENT: &str = include_str!("../config/core/server-config.yml");
        let sc = ServerConfig::new(c_env, Some(YML_CONTENT)).unwrap();
        println!("{:?}",&sc);
        let res = format!("{}:{}", "127.0.0.1", 23333);
        assert_eq!(sc.get_tcp_listener(),res);
        assert_eq!(sc.is_secure(),true);
    }

    #[test]
    pub fn test_svr_conf_tpc_listener_empty(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let c_env = "empty";
        let sc = ServerConfig::new(c_env, None).unwrap();
        println!("{:?}",&sc);
        let res = format!("{}:{}", "localhost", 23339);
        assert_eq!(sc.get_tcp_listener(),res);
        assert_eq!(sc.is_secure(),true);
    }
}
