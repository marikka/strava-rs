//strava clubs
use super::api::{v3, AccessToken, ResourceState};
use super::error::Result;
use super::http::get;
use serde::Deserialize;

/// Clubs represent groups of athletes on Strava.
///
/// They can be public or private. Only members of private clubs can access
/// their details. The object is returned in summary or detailed
/// representations.
///
/// See: http://strava.github.io/api/v3/clubs/
#[derive(Debug, Deserialize)]
pub struct Club {
    id: i32,
    resource_state: ResourceState,
    name: String,

    profile_medium: Option<String>,
    profile: Option<String>,
    cover_photo: Option<String>,
    cover_photo_small: Option<String>,
    sport_type: Option<SportType>,
    city: Option<String>,
    state: Option<String>,
    country: Option<String>,
    private: Option<bool>,
    member_count: Option<i32>,
    featured: Option<bool>,
    verified: Option<bool>,
    url: Option<String>,

    description: Option<String>,
    club_type: Option<ClubType>,
    membership: Option<String>,
    admin: Option<bool>,
    owner: Option<bool>,
    following_count: Option<i32>,
}

impl Club {
    /// Get an Gear by id (the only option)
    pub async fn get(token: &AccessToken, id: String) -> Result<Club> {
        let url = v3(Some(token), format!("clubs/{}", id));
        Ok(get::<Club>(&url[..]).await?)
    }
}

/// Types of sports
#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum SportType {
    cycling,
    running,
    triathlon,
    other,
}

/// Types of clubs
#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ClubType {
    casual_club,
    racing_team,
    shop,
    company,
    other,
}

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use super::Club;
    use api::AccessToken;
    #[test]
    #[test]
    fn get_club() {
        let id = "1".to_string();
        let token = AccessToken::new_from_env().unwrap();
        let club = Club::get(&token, id);
        println!("{:?}", club);
    }
}
