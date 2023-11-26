/*
 * Captain Coaster API
 *
 * Welcome to Captain Coaster API! API Keys for authentication can be found in your profile page. Contact us to get support.
 *
 * The version of the OpenAPI document: 1.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LaunchReadLaunch : Launch



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LaunchReadLaunch {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LaunchReadLaunch {
    /// Launch
    pub fn new() -> LaunchReadLaunch {
        LaunchReadLaunch {
            name: None,
        }
    }
}


