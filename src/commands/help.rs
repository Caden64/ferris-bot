use crate::{Context, Error};

#[poise::command(slash_command, ephemeral)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Awesome discord bot for learning poise",
            ..Default::default()
        },
    )
        .await?;
    Ok(())
}
