use cucumber_rust::{async_trait, Context, Cucumber, World};
use expenses::bot::Bot;
use std::convert::Infallible;
use std::env;

pub enum BotWorld {
    Nothing,
    Response(String),
}

#[async_trait(?Send)]
impl World for BotWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::Nothing)
    }
}

mod addition_steps {
    use cucumber_rust::{t, Steps};
    use expenses::bot::Bot;

    use crate::BotWorld;

    pub fn steps() -> Steps<BotWorld> {
        let mut steps: Steps<BotWorld> = Steps::new();

        steps.given_async("a bot", t!(|world, ctx| BotWorld::Nothing));

        steps.when_regex_async(
            r#"^user sends "(.*)" to bot$"#,
            t!(|world, ctx| {
                let bot = ctx.get::<Bot>().unwrap();
                let command = &ctx.matches[1];
                assert_eq!(command, "/help");
                let response = bot
                    .handle_message(command.to_string(), "54981987")
                    .await
                    .unwrap();
                BotWorld::Response(response)
            }),
        );

        steps.then_regex_async(
            r#"^I receive text equals to "(.*)"$"#,
            t!(|world, ctx| {
                let expected_response = &ctx.matches[1];
                match world {
                    BotWorld::Response(val) => assert_eq!(val, expected_response.to_string()),
                    _ => panic!("Invalid ze warudo state"),
                }
                BotWorld::Nothing
            }),
        );

        steps
    }
}

#[tokio::main]
async fn main() {
    let bot = configure_bot();
    Cucumber::<BotWorld>::new()
        .features(&["./tests/features"])
        .steps(addition_steps::steps())
        .context(Context::new().add(bot))
        .run_and_exit()
        .await
}

fn configure_bot() -> Bot {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    Bot::new(
        &env::var("TELEGRAM_BOT_TOKEN").expect("Set TELEGRAM_BOT_TOKEN environment variable"),
        &env::var("DATABASE_URL")
            .expect("Set DATABASE_URL environment variable or configure it at test.env file"),
    )
}
