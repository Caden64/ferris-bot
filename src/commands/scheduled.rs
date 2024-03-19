use std::fmt;

use chrono::{NaiveTime, Utc};
use chrono_tz::America::Denver;

use crate::{Context, Error};

#[derive(Debug, poise::ChoiceParameter)]
pub enum AmOrPm {
    AM,
    PM
}

impl fmt::Display for AmOrPm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AmOrPm::AM => write!(f, "AM"),
            AmOrPm::PM => write!(f, "PM"),
        }
    }
}
#[poise::command(slash_command)]
pub async fn scheduled(
    ctx: Context<'_>,
    hour: u8,
    minute: u8,
    second: u8,
    am_or_pm: Option<AmOrPm>
) -> Result<(), Error> {
    let dt = Utc::now().with_timezone(&Denver);
    ctx.say(format!("{}", dt)).await?;
    let time = if am_or_pm.is_some() {
        NaiveTime::parse_from_str(format!("{hour}:{minute}:{second} {}", am_or_pm.unwrap()).as_str(), "%I:%M:%S %p")
    } else {
        NaiveTime::parse_from_str(format!("{hour}:{minute}:{second}").as_str(), "%H:%M:%S")
    };
    if time.is_err() {
        ctx.say("Invalid time given").await?;
        return Ok(())
    }
    ctx.say(format!("{}", time.unwrap())).await?;
    Ok(())
}
