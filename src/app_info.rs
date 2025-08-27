use std::env;

///Initialize basic information gathering data from Cargo.toml or default values.

pub struct AppInfo{
    pub package_name: &'static str,
    pub version: &'static str,
    pub authors: &'static str,
    pub description: &'static str
}

pub struct CargoPkg{
    pub pkg_name: Option<&'static str>,
    pub pkg_version: Option<&'static str>,
    pub pkg_authors: Option<&'static str>,
    pub pkg_desc: Option<&'static str>,
}

impl AppInfo {
    //This seems to be a useless function. But useful as a snippet
    ///Retrieves the BT_CORE_CONFIG crate info.
    /// Useful for testing purpose (e.g. unit test). Do not use in PROD!
    #[cfg(debug_assertions)]
    #[deprecated(
        since = "0.2.1",
        note = "This function is for testing only. Avoid using in production.")]
    pub fn get_app_info(default_name: &'static str, default_version: &'static str, default_authors: &'static str, default_desciption: &'static str) -> Self{
        println!("AppInfo::get_app_info() is safe to run in tests only!");

        let pkg_name: &'static str = match option_env!("CARGO_PKG_NAME"){
                                            Some(v) => if v.is_empty() {
                                                                    default_name
                                                                }else{
                                                                    v
                                                                },
                                            None => default_name,
                                        };   
        let pkg_version: &str = match option_env!("CARGO_PKG_VERSION"){
                                            Some(v) => if v.is_empty() {
                                                                        default_version
                                                                    }else{
                                                                        v
                                                                    },
                                            None => default_version,
                                        };
        let pkg_authors: &str  = match option_env!("CARGO_PKG_AUTHORS"){
                                            Some(v) => if v.is_empty() {
                                                                        default_authors
                                                                    }else{
                                                                        v
                                                                    },
                                            None => default_authors,
                                        };
        let pkg_desc: &str = match option_env!("CARGO_PKG_DESCRIPTION"){
                                            Some(v) => if v.is_empty() {
                                                                        default_desciption
                                                                    }else{
                                                                        v
                                                                    },
                                            None => default_desciption,
                                        };
        Self { package_name: pkg_name, version: pkg_version, authors: pkg_authors, description: pkg_desc }
    }


    pub fn new(cargo_pkg: CargoPkg, default_name: &'static str, default_version: &'static str, default_authors: &'static str, default_desciption: &'static str) -> Self{
        let pkg_name: &'static str = match cargo_pkg.pkg_name{
                                            Some(v) => if v.is_empty() {
                                                                    default_name
                                                                }else{
                                                                    v
                                                                },
                                            None => default_name,
                                        };   
        let pkg_version: &str = match cargo_pkg.pkg_version{
                                            Some(v) => if v.is_empty() {
                                                                        default_version
                                                                    }else{
                                                                        v
                                                                    },
                                            None => default_version,
                                        };
        let pkg_authors: &str  = match cargo_pkg.pkg_authors{
                                            Some(v) => if v.is_empty() {
                                                                        default_authors
                                                                    }else{
                                                                        v
                                                                    },
                                            None => default_authors,
                                        };
        let pkg_desc: &str = match cargo_pkg.pkg_desc{
                                            Some(v) => if v.is_empty() {
                                                                        default_desciption
                                                                    }else{
                                                                        v
                                                                    },
                                            None => default_desciption,
                                        };
        Self { package_name: pkg_name, version: pkg_version, authors: pkg_authors, description: pkg_desc }
    }

    fn get_app_name(package_name: Option<&str>) -> String{
       let pkg_name = package_name.unwrap_or("").trim();
       if pkg_name.len() > 0 
        { pkg_name.to_string() 
        } else { //Get the name from current executable
            //gets the full path to the currently running executable.
            //extracts the filename without its extension (path.file_stem())
            //remove the suffix
            let full_name = env::current_exe()
                .ok()
                .and_then(|path| path.file_stem().map(|s| s.to_string_lossy().into_owned()))
                .unwrap_or_else(|| "BT_UnkownApp".to_string())
                .split('-').next().map(|base| base.to_string()).unwrap_or("BT_UnkownApp".to_owned());

            if full_name.trim().len() > 0{
                full_name.trim().to_owned()
            }else{
                "BT_UnkownApp".to_string()
            }
        }
    }
}


//***********/
// UNIT TEST 
//***********/
#[cfg(test)]
mod app_config_tests {
    use crate::app_info::AppInfo;

    #[test]
    pub fn test_get_app_name_with_package_success(){
        let name = AppInfo::get_app_name(Some("test"));
        assert_eq!(name,"test");
    }

    #[test]
    pub fn test_get_app_name_with_none_success(){
        let name = AppInfo::get_app_name(None);
        assert_eq!(name,"bt_core_config");
    }

    #[test]
    pub fn test_get_app_name_with_empty_success(){
        let name = AppInfo::get_app_name(Some(""));
        assert_eq!(name,"bt_core_config");
    }       
}