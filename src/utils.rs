use std::sync::OnceLock;

use bt_logger::log_error;

static APP_BASE_URL: OnceLock<&'static str> = OnceLock::new();

pub(crate) fn init_app_base_url(base_url: &str){
    let leaked_url: &'static str = Box::leak(base_url.to_owned().into_boxed_str());
    if let Err(e) = APP_BASE_URL.set(&leaked_url){
        log_error!("","Cannot set intial App Base URL. Already initialized? Error: {}",e);
    }
}

///This function default to "/" if there is no app_base_url initialize
pub fn get_app_base_url() -> &'static str {
    match APP_BASE_URL.get(){
        Some(abu) => abu,
        None => {
            log_error!("","No APP BASE URL initialize. Return default root '/'");
            "/"
        },
    }
}

pub fn build_full_route(path_route: &str) -> String{
    build_app_route(get_app_base_url(), path_route)
}

pub fn build_app_route(root_path: &str, path_route: &str) -> String{
    format!("{}{}",root_path,path_route)
}

//***********/
// UNIT TEST 
//***********/
#[cfg(test)]
mod utils_app_base_url_tests {
    use bt_logger::{build_logger, LogLevel, LogTarget};

use crate::utils::{build_full_route, get_app_base_url, init_app_base_url};
   /*#[test]
    pub fn test_app_base_url_get_failure(){
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        assert_ne!("/",get_app_base_url())
    }*/  
    
    #[test]
    pub fn test_app_base_url_set_success(){
        const BASE_URL: &str = "/base_url";
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        init_app_base_url(BASE_URL);
        assert_eq!(BASE_URL,get_app_base_url())
    }

     #[test]    
    pub fn test_full_route_success(){
        const BASE_URL: &str = "/base_url";
        const PATH_ROUTE: &str = "/path/here";
        build_logger("BACHUETECH","APP_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        init_app_base_url(BASE_URL);
        assert_eq!(format!("{}{}",BASE_URL,PATH_ROUTE),build_full_route(PATH_ROUTE))
    }      
}