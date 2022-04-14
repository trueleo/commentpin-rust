use commentpin::command_handler;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.command("cpin", command_handler);

    bot.polling().start().await.unwrap();
}
