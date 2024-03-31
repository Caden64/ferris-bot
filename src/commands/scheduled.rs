use std::fmt;

use chrono::{NaiveTime, Utc};
use chrono_tz::America::Denver;
use poise::serenity_prelude::CacheHttp;

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
    #[choices("Monday", "Tuesday", "Wednesday", "Thursday", "Friday")]
    weekdays: &'static str,
    am_or_pm: Option<AmOrPm>
) -> Result<(), Error> {
    println!("{:?}", weekdays);
    let dt = Utc::now().with_timezone(&Denver);
    ctx.say(format!("{}", dt)).await?;
    let time = if am_or_pm.is_some() {
        NaiveTime::parse_from_str(format!("{hour}:{minute}{}", am_or_pm.unwrap()).as_str(), "%I:%M %p")
    } else {
        NaiveTime::parse_from_str(format!("{hour}:{minute}").as_str(), "%H:%M")
    };
    if time.is_err() {
        ctx.say("Invalid time given").await?;
        return Ok(())
    }
    let time = time.unwrap();
    println!("Going in the loop");
    ctx.say(format!("{}", time)).await?;
    let cache = poise::serenity_prelude::CacheHttp::cache(&ctx).unwrap().clone();
    let http = {ctx.http().clone().http()};
    let guild_channel = ctx.guild_channel().await.unwrap();
    if time > dt.time() {
        tokio::spawn(async move {
            loop {
                let dt = Utc::now().with_timezone(&Denver);
                if dt.time() >= time {
                    guild_channel.say((&cache, http.clone()), "It's time!").await.unwrap();
                    // ctx.channel_id().say(ctx.http(), "It's time!").await.unwrap();
                    break;
                }
            }
        });
    }
    Ok(())
}
