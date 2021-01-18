//! Body fat logs

use chrono::naive::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogFatResponse {
    pub weight_log: FatLogWithBmi,
}

#[derive(Debug, Deserialize)]
pub struct GetFatLogsResponse {
    pub fat: Vec<FatLog>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FatLog {
    pub date: NaiveDate,
    pub fat: f32,
    pub log_id: usize,
    pub time: NaiveTime,
    pub source: String, // TODO: Device enum
}

#[derive(Debug, Serialize)]
pub struct FatLogWithBmi {
    /// Body mass index
    pub bmi: f32,
    #[serde(flatten)]
    pub entry: FatLog,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let expected = r#"
{
    "weightLog": {
        "bmi": 23.57,
        "date": "2012-03-05",
        "fat": 14.5,
        "logId": 1330991999000,
        "time": "23:59:59",
        "source": "API"
    }
}
        "#;
        let expected = expected.replace(" ", "").replace("\n", "");

        let response = LogFatResponse {
            weight_log: FatLogWithBmi {
                bmi: 23.57,
                entry: FatLog {
                    date: NaiveDate::from_ymd(2012, 3, 5),
                    fat: 14.5,
                    log_id: 1330991999000,
                    time: NaiveTime::from_hms(23, 59, 59),
                    source: "API".to_string(),
                },
            },
        };

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize() {
        let data = r#"
{
    "fat":[
        {
            "date":"2012-03-05",
            "fat":14,
            "logId":1330991999000,
            "time":"23:59:59",
            "source": "API"
        },
        {
            "date":"2012-03-05",
            "fat":13.5,
            "logId":1330991999000,
            "time":"21:20:59",
            "source":"Aria"
        }
    ]
}
        "#;

        let _res: GetFatLogsResponse = serde_json::from_str(data).unwrap();
    }
}
