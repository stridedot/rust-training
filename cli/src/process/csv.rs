use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::opts::csv::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub struct Player {
    #[serde(alias = "Name")]
    pub name: String,

    #[serde(alias = "Position")]
    pub position: String,

    #[serde(alias = "DOB")]
    pub dob: String,

    #[serde(alias = "Nationality")]
    pub nationality: String,

    #[serde(alias = "Kit Number")]
    pub kit_number: String,
}

pub async fn process_csv(input: &str, output: &str, format: &OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;

    // 这种方式无法改变 key 的大小写
    // let headers = reader.headers()?.clone();
    // let mut players = Vec::new();

    // for record in reader.records() {
    //     let record = record?;
    //     let mut m = HashMap::new();

    //     for (header, value) in headers.iter().zip(record.iter()) {
    //         m.insert(header.to_string(), value.to_string());
    //     }

    //     players.push(m);
    // }

    let mut players: Vec<Player> = Vec::new();
    for record in reader.deserialize() {
        let player: Player = record?;
        players.push(player);
    }

    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&players)?;
            std::fs::write(output, json)?;
        }
        OutputFormat::Toml => {
            let wrapped = toml::Value::try_from(std::collections::BTreeMap::from([(
                "players".to_string(),
                toml::Value::try_from(players)?,
            )]))?;
            let toml = toml::to_string_pretty(&wrapped)?;
            std::fs::write(output, toml)?;
        }
        OutputFormat::Yaml => {
            let yaml = serde_yaml::to_string(&players)?;
            std::fs::write(output, yaml)?;
        }
    }

    Ok(())
}
