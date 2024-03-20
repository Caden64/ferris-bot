use crate::{Context, Error};
#[poise::command(slash_command, ephemeral)]
pub async fn register(
    ctx: Context<'_>,
    name: String,
    s_number: String,
    student_email: String,
) -> Result<(), Error> {
    ctx.say("Register command reached").await?;
    ctx.say(format!("Name:{name}\nSNumber{s_number}\nStudentEmail\n{student_email}")).await?;
    Ok(())
}
