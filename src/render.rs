use minijinja::{Environment, Value, context};
use serde::Serialize;
use thiserror::Error;

const TEMPLATE: &str = include_str!("../templates/index.html.jinja");

#[derive(Debug, Clone, Serialize)]
pub struct AircraftStatus {
    pub callsign: String,
    pub aircraft_type: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClubStatus {
    pub name: String,
    pub aircraft: Vec<AircraftStatus>,
}

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("Template error: {0}")]
    Template(#[from] minijinja::Error),
}

fn aircraft_value(aircraft: &AircraftStatus) -> Value {
    context! {
        callsign => aircraft.callsign.as_str(),
        aircraft_type => aircraft.aircraft_type.as_str(),
        status => aircraft.status.as_str(),
    }
}

fn club_value(club: &ClubStatus) -> Value {
    let aircraft: Vec<Value> = club.aircraft.iter().map(aircraft_value).collect();
    context! {
        name => club.name.as_str(),
        aircraft => aircraft,
    }
}

pub fn render_html(clubs: &[ClubStatus], rendered_at: &str) -> Result<String, RenderError> {
    let mut env = Environment::new();
    env.add_template("index.html", TEMPLATE)?;

    let template = env.get_template("index.html")?;
    let clubs: Vec<Value> = clubs.iter().map(club_value).collect();
    let html = template.render(context! {
        clubs => clubs,
        rendered_at => rendered_at,
    })?;

    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_html() {
        let clubs = vec![
            ClubStatus {
                name: "LVA".to_string(),
                aircraft: vec![
                    AircraftStatus {
                        callsign: "D-3886".to_string(),
                        aircraft_type: "Astir".to_string(),
                        status: "airworthy".to_string(),
                    },
                    AircraftStatus {
                        callsign: "D-1108".to_string(),
                        aircraft_type: "LS 4".to_string(),
                        status: "grounded".to_string(),
                    },
                ],
            },
            ClubStatus {
                name: "Nordstern".to_string(),
                aircraft: vec![AircraftStatus {
                    callsign: "D-5074".to_string(),
                    aircraft_type: "ASK 21".to_string(),
                    status: "prewarning".to_string(),
                }],
            },
        ];

        let html = render_html(&clubs, "2025-01-01 12:00").unwrap();
        assert_snapshot!(html);
    }
}
