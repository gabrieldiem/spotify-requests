const ARGENTINA_ISO3166_CODE: &str = "AR";
//const US_ISO3166_CODE: &str = "US";

pub(crate) struct Market {}

impl Market {
    pub fn argentina() -> String {
        ARGENTINA_ISO3166_CODE.to_string()
    }

    /*pub fn united_states() -> String {
        US_ISO3166_CODE.to_string()
    }*/
}
