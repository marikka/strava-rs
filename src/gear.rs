//Individual pieces of gear
use super::api::{v3, AccessToken, ResourceState};
use super::error::Result;
use super::http::get;
use serde::Deserialize;

/// Gear type able to represent bikes/shoes/etc.
///
/// Represents equipment used during an activity. Gear has summary and detail
/// representations.
///
/// See: http://strava.github.io/api/v3/gear/
#[derive(Debug, Deserialize)]
pub struct Gear {
    pub id: String,
    pub primary: bool,
    pub name: String,
    pub distance: f32,
    pub resource_state: ResourceState,
    pub brand_name: Option<String>,
    pub model_name: Option<String>,
    pub frame_type: Option<FrameType>,
    pub description: Option<String>,
}

impl Gear {
    /// Get an Gear by id (the only option)
    pub async fn get(token: &AccessToken, id: String) -> Result<Gear> {
        let url = v3(Some(token), format!("gear/{}", id));
        Ok(get::<Gear>(&url[..]).await?)
    }
}

/// Frame type for bikes
#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum FrameType {
    MTB,
    Cross,
    Road,
    TimeTrial,
}

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use super::Gear;
    use api::AccessToken;
    #[test]
    #[test]
    #[test]
    fn get_gear() {
        //TODO find a real way to test this since it only works with gear id's of stuff you own
        let id = "g2164144".to_string();
        let token = AccessToken::new_from_env().unwrap();
        let gear = Gear::get(&token, id);
        println!("{:?}", gear);
    }
}
