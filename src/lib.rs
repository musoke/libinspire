/// Re-export slog
///
/// Users of this library can, but don't have to use slog to build their own loggers
#[macro_use]
pub extern crate slog ;
extern crate slog_stdlog;

use slog::DrainExt;

#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

extern crate reqwest;
use reqwest::Url;

extern crate select;
use select::document::Document;
use select::predicate::Name;

use std::io::Read;

#[derive(Debug,PartialEq)]
pub struct RecID<'a> {
    pub id: &'a str,
}

/// Create RecID from &str
///
/// Returns a `Result<Self, ()>` as this can fail.
/// In future I may also implement `std::convert::TryFrom`, currently a [nightly only
/// feature](https://github.com/rust-lang/rust/issues/33417).
///
/// # Examples
///
/// ```
/// extern crate libinspire;
/// libinspire::RecID::new("Bekenstein:1973ur");
/// ```
impl<'a> RecID<'a> {
    pub fn new<S: Into<&'a str>>(s: S) -> Result<Self, ()> {
        let s = s.into();
        match validate_recid(s) {
            true => Ok(RecID { id: s }),
            false => Err(()),
        }
    }
}

/// Test whether a string is a valid Inspire bibliographic code
///
/// # Examples
///
/// ```
/// assert!(libinspire::validate_recid("Nambu:1961tp"))
/// ```
pub fn validate_recid(code: &str) -> bool {
    // Use lazy_static to ensure that regexes are compiled only once
    lazy_static! {
        static ref REGEX: Regex = Regex::new(
            r"^[[:alpha:].]+:[[:digit:]]{4}[[:alpha:]]{2,3}$").unwrap();
    }

    REGEX.is_match(code)
}

pub struct Api {
    logger: slog::Logger,
}

impl Api {
    /// Initialize API
    ///
    /// Either provide a custom slog::Logger or default to the standard `log`
    /// crate.
    ///
    /// # Examples
    /// ```
    /// libinspire::Api::init(None);
    /// ```
    pub fn init(logger: Option<slog::Logger>) -> Self {
        Api {
            logger: logger.unwrap_or_else(|| slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
        }
    }

    /// Fetches BibTeX entries from inspire.net.
    ///
    /// # Examples
    ///
    /// ```
    /// let inspire = libinspire::Api::init(None);
    ///
    /// println!("{}", inspire.fetch_bibtex_with_key(
    ///     libinspire::RecID::new("Abramovici:1992ah").unwrap()).expect("Error"));
    /// ```
    pub fn fetch_bibtex_with_key(&self, key: RecID) -> Option<String> {

        let mut api_url: Url = Url::parse("https://inspirehep.net")
            .expect("Unable to parse API URL")
            .join("search")
            .unwrap();
        api_url
            .query_pairs_mut()
            .append_pair("of", "hx")
            .append_pair("p", &key.id);

        debug!(self.logger, "Querying inspire API";
               "URL" => api_url.to_string());
        let mut response = reqwest::get(api_url).expect("Failed to send get request");
        debug!(self.logger, "GET request completed";
               "HTTP response status" => response.status().to_string());

        let mut html = String::new();
        response
            .read_to_string(&mut html)
            .expect("Failed to read response.");

        let document = Document::from(html.as_str());

        Some(document
                 .find(Name("pre"))
                 .first()
                 .expect("No text found.")
                 .text())
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {}
}
