use async_trait::async_trait;
use std::convert::Infallible;

struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

pub struct BotTestTheWorld {
    bot: Bot,
}

#[async_trait(?Send)]
impl cucumber::World for AnimalWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            cat: Cat { hungry: false },
        })
    }
}

mod addition_steps {
    use cucumber::Steps;

    pub fn steps() -> Steps<crate::AnimalWorld> {
        let mut builder: Steps<crate::AnimalWorld> = Steps::new();

        builder
            .given("A hungry cat", |mut world, _step| {
                world.cat.hungry = true;
                world
            })
            .when("I feed the cat", |mut world, _step| {
                world.cat.feed();
                world
            })
            .then("The cat is not hungry", |world, _step| {
                assert_eq!(world.cat.hungry, false);
                world
            });

        builder
    }
}

fn main() {
    let runner = cucumber::Cucumber::<AnimalWorld>::new()
        .features(&["./tests/features"])
        .steps(addition_steps::steps());
    futures::executor::block_on(runner.run());
}
