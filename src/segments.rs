//! Specific sections of road and attempts an athlete has made on them
use super::activities::ActivityType;
use super::api::{v3, AccessToken, Paginated, ResourceState};
use super::error::Result;
use super::http::get;
use serde::Deserialize;

/// A specific section(s) of road.
///
/// Segments are available in Summary and Detail versions.
/// http://strava.github.io/api/v3/segments/
#[derive(Debug, Deserialize)]
pub struct Segment {
    id: u32,
    resource_state: ResourceState,
    name: String,
    activity_type: ActivityType,
    distance: f32,
    average_grade: f32,
    maximum_grade: f32,
    elevation_high: f32,
    elevation_low: f32,
    start_latlng: Vec<f32>,
    end_latlng: Vec<f32>,
    climb_category: u8,
    city: Option<String>,
    state: String,
    country: String,
    private: bool,

    // Detail Attributes
    created_at: Option<String>,
    updated_at: Option<String>,
    total_elevation_gain: Option<f32>,
    // TODO map: Option<Map>,
    effort_count: Option<u32>,
    athlete_count: Option<u32>,
    star_count: Option<u32>,
    hazardous: Option<bool>, // Properties on starred segments
                             // TODO atlete_pr_effort: Effort
}

impl Segment {
    /// Fetch a Segment by id
    pub async fn get(token: &AccessToken, id: u32) -> Result<Segment> {
        let url = v3(Some(token), format!("segments/{}", id));
        Ok(get::<Segment>(&url[..]).await?)
    }

    /// Get starred segments
    pub async fn get_starred(token: &AccessToken) -> Result<Paginated<Segment>> {
        let url = v3(Some(token), "segments/starred".to_string());
        let segments = get::<Vec<Segment>>(&url[..]).await?;
        Ok(Paginated::new(url, segments))
    }
}

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use super::Segment;
    use api::AccessToken;

    #[test]
    fn get_segment() {
        let token = AccessToken::new_from_env().unwrap();
        let segment = Segment::get(&token, 646257).unwrap();

        println!("{:?}", segment);
    }

    #[test]
    fn get_stars() {
        let token = AccessToken::new_from_env().unwrap();
        let starred = Segment::get_starred(&token).unwrap();
        println!("{:?}", starred);
    }
}
