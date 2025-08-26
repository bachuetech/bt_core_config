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
    pub (crate) fn get_app_info(default_name: &'static str, default_version: &'static str, default_authors: &'static str, default_desciption: &'static str) -> Self{
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
}
