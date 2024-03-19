use tokio::time::Duration;
use tokio::time::sleep;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn timer(
    ctx: Context<'_>,
    duration: Option<u64>
) -> Result<(), Error> {
    let duration = duration.unwrap_or(5);
    ctx.defer_ephemeral().await?;
    ctx.say(format!("setting up {duration} second timer")).await?;
    sleep(Duration::from_secs(duration)).await;
    ctx.channel_id().say(ctx.http(), "TIME UP!!!").await?;

    Ok(())
}
