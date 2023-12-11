/*
 * Captain Coaster API
 *
 * Welcome to Captain Coaster API! API Keys for authentication can be found in your profile page. Contact us to get support.
 *
 * The version of the OpenAPI document: 1.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CountryReadCoaster : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CountryReadCoaster {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CountryReadCoaster {
    /// 
    pub fn new() -> CountryReadCoaster {
        CountryReadCoaster {
            name: None,
        }
    }
}

