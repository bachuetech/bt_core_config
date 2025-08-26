///Initialize basic information gathering data from Cargo.toml or default values.

pub struct AppInfo{
    pub package_name: String,
    pub version: String,
    pub authors: String,
    pub description: String
}


impl AppInfo {
    pub fn get_app_info(default_name: &str, default_version: &str, default_authors: &str, default_desciption: &str) -> Self{
        let pkg_name: &str = match option_env!("CARGO_PKG_NAME"){
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
        Self { package_name: pkg_name.to_owned(), version: pkg_version.to_owned(), authors: pkg_authors.to_owned(), description: pkg_desc.to_owned() }
    }
    
}
