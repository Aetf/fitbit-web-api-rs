use chrono::NaiveDate;
use strum_macros::*;
use url::Url;

use crate::UserId;

pub mod fat;
pub mod weight;

/// Possible body log types
#[derive(Debug, PartialEq, Copy, Clone, ToString)]
#[strum(serialize_all = "snake_case")]
pub enum Resource {
    Fat,
    Weight,
}

/// Possible period ranges for log
#[derive(Debug, PartialEq, Copy, Clone, ToString)]
#[strum(serialize_all = "snake_case")]
pub enum Period {
    #[strum(serialize = "1d")]
    OneDay,
    #[strum(serialize = "7d")]
    SevenDays,
    #[strum(serialize = "1w")]
    OneWeek,
    #[strum(serialize = "1m")]
    OneMonth,
}

/// Generate the request URL from a resource, and date.
pub fn url_from_date(
    user_id: &UserId,
    resource: Resource,
    date: NaiveDate
) -> Url {
    Url::parse(&format!(
        "https://api.fitbit.com/1/user/{}/body/log/{}/date/{}.json",
        user_id.to_string(),
        resource.to_string(),
        date,
    ))
        .unwrap()
}

/// Generate the request URL from a resource, and date period.
pub fn url_from_date_period(
    user_id: &UserId,
    resource: Resource,
    start: NaiveDate,
    period: Period,
) -> Url {
    Url::parse(&format!(
        "https://api.fitbit.com/1/user/{}/body/log/{}/date/{}/{}.json",
        user_id.to_string(),
        resource.to_string(),
        start,
        period.to_string(),
    ))
        .unwrap()
}

pub fn url_from_date_range(
    user_id: &UserId,
    resource: Resource,
    min: NaiveDate,
    max: NaiveDate,
) -> Url {
Url::parse(&format!(
    "https://api.fitbit.com/1/user/{}/body/log/{}/date/{}/{}.json",
        user_id.to_string(),
        resource.to_string(),
        min,
        max,
    ))
    .unwrap()
}
