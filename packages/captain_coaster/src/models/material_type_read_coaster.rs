/*
 * Captain Coaster API
 *
 * Welcome to Captain Coaster API! API Keys for authentication can be found in your profile page. Contact us to get support.
 *
 * The version of the OpenAPI document: 1.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MaterialTypeReadCoaster : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaterialTypeReadCoaster {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MaterialTypeReadCoaster {
    /// 
    pub fn new() -> MaterialTypeReadCoaster {
        MaterialTypeReadCoaster {
            name: None,
        }
    }
}


