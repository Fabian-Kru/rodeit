/*
 * Captain Coaster API
 *
 * Welcome to Captain Coaster API! API Keys for authentication can be found in your profile page. Contact us to get support.
 *
 * The version of the OpenAPI document: 1.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ParkReadPark : Park



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParkReadPark {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<crate::models::CountryReadPark>>,
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
}

impl ParkReadPark {
    /// Park
    pub fn new() -> ParkReadPark {
        ParkReadPark {
            id: None,
            name: None,
            country: None,
            latitude: None,
            longitude: None,
        }
    }
}

