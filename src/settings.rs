/* Mainframe - manage infrastructure with a few button clicks
 * Copyright © 2021 Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use std::env;
use std::path::Path;

use config::{Config, ConfigError, Environment, File};
use log::{debug, warn};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u32,
    pub domain: String,
    pub cookie_secret: String,
    pub ip: String,
    pub url_prefix: Option<String>,
    pub proxy_has_tls: bool,
}

impl Server {
    #[cfg(not(tarpaulin_include))]
    pub fn get_ip(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub server: Server,
    pub source_code: String,
}

#[cfg(not(tarpaulin_include))]
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // setting default values
        #[cfg(test)]
        s.set_default("database.pool", 2.to_string())
            .expect("Couldn't get the number of CPUs");

        const CURRENT_DIR: &str = "./config/default.toml";
        const ETC: &str = "/etc/mcaptcha/config.toml";

        if let Ok(path) = env::var("MAINFRAME_CONFIG") {
            s.merge(File::with_name(&path))?;
        } else if Path::new(CURRENT_DIR).exists() {
            // merging default config from file
            s.merge(File::with_name(CURRENT_DIR))?;
        } else if Path::new(ETC).exists() {
            s.merge(File::with_name(ETC))?;
        } else {
            log::warn!("configuration file not found");
        }

        s.merge(Environment::with_prefix("MAINFRAME").separator("_"))?;

        check_url(&s);

        match env::var("PORT") {
            Ok(val) => {
                s.set("server.port", val).unwrap();
            }
            Err(e) => warn!("couldn't interpret PORT: {}", e),
        }

        match env::var("DATABASE_URL") {
            Ok(val) => {
                let url = Url::parse(&val).expect("couldn't parse Database URL");
                s.set("database.url", url.to_string())
                    .expect("Couldn't set database username");
            }
            Err(e) => warn!("couldn't interpret DATABASE_URL: {}", e),
        }

        match s.try_into() {
            Ok(val) => Ok(val),
            Err(e) => Err(ConfigError::Message(format!("\n\nError: {}. If it says missing fields, then please refer to https://github.com/mCaptcha/mcaptcha#configuration to learn more about how mcaptcha reads configuration\n\n", e))),
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn check_url(s: &Config) {
    let url = s
        .get::<String>("source_code")
        .expect("Couldn't access source_code");

    Url::parse(&url).expect("Please enter a URL for source_code in settings");
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn url_prefix_test() {
//        let mut settings = Settings::new().unwrap();
//        assert!(settings.server.url_prefix.is_none());
//        settings.server.url_prefix = Some("test".into());
//        settings.server.check_url_prefix();
//        settings.server.url_prefix = Some("    ".into());
//        settings.server.check_url_prefix();
//        assert!(settings.server.url_prefix.is_none());
//    }
//
//    #[test]
//    fn smtp_config_works() {
//        let settings = Settings::new().unwrap();
//        assert!(settings.smtp.is_some());
//        assert_eq!(settings.smtp.as_ref().unwrap().password, "password");
//        assert_eq!(settings.smtp.as_ref().unwrap().username, "admin");
//    }
//}
