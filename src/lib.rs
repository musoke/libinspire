/// Re-export slog
///
/// Users of this library can, but don't have to use slog to build their own loggers
#[macro_use]
pub extern crate slog ;
extern crate slog_stdlog;

use slog::DrainExt;

extern crate regex;
use regex::Regex;

extern crate reqwest;
use reqwest::Url;

extern crate select;
use select::document::Document;
use select::predicate::Name;

use std::io::Read;

pub struct Inspire {
    logger: slog::Logger,
}

impl Inspire {
    /// Initialize 'Inspirer'
    ///
    /// Either provide a custom slog::Logger or default to the standard `log`
    /// crate.
    ///
    /// # Examples
    /// ```
    /// libinspire::Inspire::init(None);
    /// ```
    pub fn init(logger: Option<slog::Logger>) -> Self {
        Inspire {
            logger: logger.unwrap_or_else(|| slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
        }
    }

    /// Fetches BibTeX entries from inspire.net.
    ///
    /// # Examples
    ///
    /// ```
    /// let inspire = libinspire::Inspire::init(None);
    ///
    /// println!("{}", inspire.fetch_bibtex_with_key(
    ///     "Abramovici:1992ah".to_string()).expect("Error"));
    /// ```
    pub fn fetch_bibtex_with_key(&self, key: String) -> Option<String> {

        let mut api_url: Url = Url::parse("https://inspirehep.net")
            .expect("Unable to parse API URL")
            .join("search")
            .unwrap();
        api_url
            .query_pairs_mut()
            .append_pair("of", "hx")
            .append_pair("p", &key);

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
    #[test]
    fn it_works() {}
}
