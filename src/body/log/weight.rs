//! Weight logging API

use chrono::naive::{NaiveDate, NaiveTime};
use serde::Deserialize;

use crate::UserId;

#[derive(Debug, Deserialize)]
pub struct GetResponse {
    pub weight: Vec<WeightLog>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutResponse {
    pub weight_log: Vec<WeightLog>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightLog {
    pub bmi: f32,
    pub date: NaiveDate,
    pub log_id: usize,
    pub time: NaiveTime,
    pub weight: f32,
    pub source: String, // TODO: Device enum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_get_weight_logs() {
        let data = r#"
{
    "weight":[
    {
        "bmi":23.57,
        "date":"2015-03-05",
        "logId":1330991999000,
        "time":"23:59:59",
        "weight":73,
        "source": "API"
    },
    {
        "bmi":22.57,
        "date":"2015-03-05",
        "logId":1330991999000,
        "time":"21:10:59",
        "weight":72.5,
        "source": "Aria"
    }
    ]
}
        "#;

        let _res: GetResponse = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn deserialize_log_weight() {
        let data = r#"
{
    "weightLog": [
    {
        "bmi": 23.57,
        "date": "2012-03-05",
        "logId": 1330991999000,
        "time": "23:59:59",
        "weight": 73,
        "source": "API"
    }
    ]
}
        "#;

        let _res: PutResponse = serde_json::from_str(data).unwrap();
    }
}
