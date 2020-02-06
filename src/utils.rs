use regex::Captures;
use regex::Regex;

// decimal numeric character to unicode
pub fn dnc_unicode(str: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r##"(&#(\d{1,5});)"##).unwrap();
    }
    RE.replace_all(str, |caps: &Captures| { format!("{}", std::char::from_u32(caps[2].parse::<u32>().ok().unwrap()).unwrap()) }).to_string()
}
