use crate::{Context, Error};
#[poise::command(prefix_command, slash_command, ephemeral)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), Error> {
    let mut allow_vote = true;
    let num_votes = {
        let mut hash_map = ctx.data().votes.lock().unwrap();
        let people = hash_map.entry(choice.clone()).or_default();
        let name = ctx.author().clone().name;
        for x in people.0.clone() {
            if x == name {
                allow_vote = false;
                break;
            }
        }
        if allow_vote {
            people.0.push(ctx.author().clone().name);
            people.1 += 1;
        }
        people.1
    };
    let response = if allow_vote {
        format!("Successfully voted for {choice}. {choice}, now has {num_votes} votes!")
    } else {
        format!("Unsuccessfully voted for {choice}. {choice} still has {num_votes} votes!")

    };
    ctx.say(response).await?;
    Ok(())
}
