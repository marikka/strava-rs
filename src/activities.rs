use super::api::{v3, AccessToken, ResourceState};
use super::athletes::Athlete;
use super::error::Result;
use super::http::get;
use super::segmentefforts::SegmentEffort;
use serde::Deserialize;

/// Activity Types
#[derive(Debug, Deserialize)]
pub enum ActivityType {
    Ride,
    Run,
    Swim,
    Hike,
    Walk,
    AlpineSki,
    BackcountrySki,
    Canoeing,
    Crossfit,
    EBikeRide,
    Elliptical,
    IceSkate,
    InlineSkate,
    Kayaking,
    Kitesurf,
    NordicSki,
    RockClimbing,
    RollerSki,
    Rowing,
    Snowboard,
    Snowshoe,
    StairStepper,
    StandUpPaddling,
    Surfing,
    WeightTraining,
    Windsurf,
    Workout,
    Yoga,
    Unknown,
}

#[derive(Debug, Deserialize)]
pub enum WorkoutType {
    DefaultRun = 0,
    RaceRun = 1,
    LongRun = 2,
    WorkoutRun = 3,
    DefaultRide = 10,
    RaceRide = 11,
    WorkoutRide = 12,
}

#[derive(Debug, Deserialize)]
pub struct Activity {
    // Meta representation
    pub id: u64,
    pub resource_state: ResourceState,

    // Summary representation
    pub external_id: String,
    pub upload_id: u64,
    pub athlete: Athlete,
    pub name: String,
    pub distance: f32,
    pub moving_time: i32,
    pub elapsed_time: i32,
    pub total_elevation_gain: f32,
    pub r#type: ActivityType,
    pub start_date: String,       //TODO decode time from string
    pub start_date_local: String, //TODO decode time from string
    pub timezone: String,
    pub start_latlng: Cords,
    pub end_latlng: Cords,
    pub achievement_count: i32,
    pub kudos_count: i32,
    pub comment_count: i32,
    pub athlete_count: i32,
    pub photo_count: i32,
    //pub map: Map,
    pub trainer: bool,
    pub commute: bool,
    pub manual: bool,
    pub private: bool,
    pub flagged: bool,
    pub workout_type: Option<WorkoutType>,
    pub gear_id: String,
    pub average_speed: f32,
    pub max_speed: f32,
    pub average_cadence: f32,
    pub average_temp: Option<f32>,
    pub average_watts: Option<f32>,
    pub weighted_average_watts: Option<i32>,
    pub kilojoules: Option<f32>,
    pub device_watts: Option<bool>,
    pub max_heartrate: f32,
    pub truncated: Option<i32>,
    pub has_kudoed: bool,

    // Detail represenation
    pub calories: Option<f32>,
    pub description: Option<String>,
    // TODO pub gear: Gear,
    pub segment_efforts: Option<Vec<SegmentEffort>>,
    pub splits_metric: Vec<Split>,
    pub splits_standard: Vec<Split>,
    pub best_efforts: Vec<SegmentEffort>,
    // TODO pub photos: Photos
}

#[derive(Debug, Deserialize)]
pub struct SummaryActivity {
    pub id: u64,
    pub external_id: String,
    pub upload_id: u64,
    pub athlete: Athlete,
    pub name: String,
    pub distance: f32,
    pub moving_time: u32,
    pub elapsed_time: u32,
    pub total_elevation_gain: f32,
    pub elev_high: Option<f32>,
    pub elev_low: Option<f32>,
    pub r#type: ActivityType,
    pub start_date: String,
    pub start_date_local: String,
    pub timezone: String,
    pub start_latlng: Vec<f32>,
    pub end_latlng: Vec<f32>,
    pub achievement_count: u32,
    pub kudos_count: u32,
    pub comment_count: u32,
    pub athlete_count: u32,
    pub photo_count: u32,
    pub total_photo_count: u32,
    //pub map: Map
    pub trainer: bool,
    pub commute: bool,
    pub manual: bool,
    pub private: bool,
    pub flagged: bool,
    //pub workout_type:  u32
    pub upload_id_str: String,
    pub average_speed: f32,
    pub max_speed: f32,
    pub has_kudoed: bool,
    pub gear_id: Option<String>,
    pub kilojoules: Option<f32>,
    pub average_watts: Option<f32>,
    pub device_watts: Option<bool>,
    pub max_watts: Option<u32>,
    pub weighted_average_watts: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Cords {
    x: f32,
    y: f32,
}

impl SummaryActivity {
    pub async fn get(token: &AccessToken, id: String) -> Result<SummaryActivity> {
        let url = v3(Some(token), format!("activities/{}", id));
        Ok(get::<SummaryActivity>(&url[..]).await?)
    }

    pub async fn athlete_activities(token: &AccessToken) -> Result<Vec<SummaryActivity>> {
        let url = v3(Some(token), "athlete/activities".to_string());
        Ok(get::<Vec<SummaryActivity>>(&url[..]).await?)
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Split;

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use super::Activity;
    use api::AccessToken;
    #[test]
    fn get_activity() {
        let id = "321934".to_string();
        let token = AccessToken::new_from_env().unwrap();
        let activity = Activity::get(&token, id);
        println!("{:?}", activity);
    }

    #[test]
    fn get_athlete_activities() {
        let token = AccessToken::new_from_env().unwrap();
        let activities = Activity::athlete_activities(&token);
        println!("{:?}", activities);
    }
}
