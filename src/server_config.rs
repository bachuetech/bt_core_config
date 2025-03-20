use std::process;

use bt_app_codes::process_exit_codes::SERVER_CONFIG_READING_ERROR;
use bt_logger::log_fatal;
use bt_yaml_utils::get_yaml;
use yaml_rust2::Yaml;

const SRV_YML_CONFIG: &str = "config/core/server-config.yml";
const SRV_YML_CONFIG_ENV_VAR_NAME: &str = "BT_SRV_CONFIGYMLFILE";

const DEFAULT_PORT: i64 = 23339;
const DEFAULT_HOST: &str = "localhost";

#[derive(Debug)]
pub struct ServerConfig {
    host: String,
    port: u16,
    secure: bool,
}

impl ServerConfig {
    // Constructor. Reading from YAML file
    pub fn new(run_env: String) -> Self {
        let srv_config: Yaml;
        match get_yaml(SRV_YML_CONFIG_ENV_VAR_NAME, SRV_YML_CONFIG){
            Ok(y_file_conf) => srv_config = y_file_conf,
            Err(e) => {
                log_fatal!("new","Fatal Error Reading SERVER configuration. Application aborted! {}",e.to_string()); 
                process::exit(SERVER_CONFIG_READING_ERROR);
            }, // Exit the program with code -103
        }

        let mut srv_port = srv_config[run_env.as_str()]["server"]["port"]
            .as_i64()
            .unwrap_or(DEFAULT_PORT);
        srv_port = if srv_port < 0 || srv_port > 65535 {
            DEFAULT_PORT
        } else {
            srv_port
        };

        Self {
            host: srv_config[run_env.as_str()]["server"]["host"]
                .as_str()
                .unwrap_or(DEFAULT_HOST)
                .to_string(),
            port: srv_port as u16,
            secure: srv_config[run_env.as_str()]["server"]["secure"]
                .as_bool()
                .unwrap_or(true),
        }
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
}

pub fn get_srv_config(current_env: String) -> ServerConfig {
    ServerConfig::new(current_env)
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
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let c_env = "UNKNOWN";
        let sc = get_srv_config(c_env.to_owned());
        println!("{:?}",&sc);
        assert_eq!(sc.get_port(),23339);
        assert_eq!(sc.host,"localhost");
        assert_eq!(sc.is_secure(),true);
    }

    #[test]
    pub fn test_svr_conf_tpc_listener(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let c_env = "UNKNOWN";
        let sc = ServerConfig::new(c_env.to_owned());
        println!("{:?}",&sc);
        let res = format!("{}:{}", "localhost", 23339);
        assert_eq!(sc.get_tcp_listener(),res);
    }

    #[test]
    pub fn test_svr_conf_tpc_listener_dev(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let c_env = "dev";
        let sc = ServerConfig::new(c_env.to_owned());
        println!("{:?}",&sc);
        let res = format!("{}:{}", "0.0.0.0", 23332);
        assert_eq!(sc.get_tcp_listener(),res);
        assert_eq!(sc.is_secure(),false);
    }

    #[test]
    pub fn test_svr_conf_tpc_listener_empty(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,);
        let c_env = "empty";
        let sc = ServerConfig::new(c_env.to_owned());
        println!("{:?}",&sc);
        let res = format!("{}:{}", "localhost", 23339);
        assert_eq!(sc.get_tcp_listener(),res);
        assert_eq!(sc.is_secure(),true);
    }
}
