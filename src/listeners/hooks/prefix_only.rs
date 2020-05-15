use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::hook;

#[hook]
pub async fn prefix_only(context: &mut Context, message: &Message) -> () {
    let _ = message.channel_id.send_message(&context, |message| {
        message.content(
            "Hello! I noticed that you provided my prefix but didn't send a \
            command. If you would like to get help on how to use my functionality, \
            please run the help command."
        )
    }).await;
}
