use crate::{Context, Error, Data};
use poise::serenity_prelude as serenity;

#[derive(Debug, poise::Modal)]
struct RegisterModal {
    first_name: String,
    last_initial: Option<String>,
    student_email: String
}
#[poise::command(slash_command, ephemeral)]
pub async fn register(
    ctx: poise::ApplicationContext<'_, Data, Error>,
) -> Result<(), Error> {
    use poise::Modal as _;
    let data = RegisterModal::execute(ctx).await?;
    println!("Got data; {:?}", data);
    ctx.say("Register command reached").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn component_modal(ctx: Context<'_>) -> Result<(), Error> {
    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("open_modal")
                .label("Open modal")
                .style(serenity::ButtonStyle::Success),
        ])];

        poise::CreateReply::default()
            .content("Click the button below to open the modal")
            .components(components)
    };

    ctx.send(reply).await?;

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "open_modal")
        .await
    {
        let data =
            poise::execute_modal_on_component_interaction::<RegisterModal>(ctx, mci, None, None).await?;
        println!("Got data: {:?}", data);
    }
    Ok(())
}