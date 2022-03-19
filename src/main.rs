use yew::{Component, Context, html, Html};
use snowflake::SnowflakeType;
use gloo::{
    timers::callback::{Interval},
};

mod random;
mod snowflake;
mod message;

enum Msg {
    ClearSnowflakes,
    UpdateTime,
}

struct App {
    snowflakes: Vec<SnowflakeType>,
    num_snowflakes: usize,
    _timer: Interval,
}

impl App {
    fn create_interval(ms: u32, ctx: &Context<Self>) -> Interval {
        let link = ctx.link().clone();
        Interval::new(ms, move || link.send_message(Msg::UpdateTime))
    }
    fn create_snowflake(&mut self, _ctx: &Context<Self>) {
        self.num_snowflakes += 1;
        if self.num_snowflakes < 52 {
            log::info!("Number of snowflakes: {}", self.num_snowflakes);
            self.snowflakes.push(SnowflakeType::new(self.num_snowflakes));
        }
    }
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            snowflakes: vec![],
            num_snowflakes: 0,
            _timer: App::create_interval(500, ctx),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClearSnowflakes => {
                log::info!{"Clearing snowflakes"};
                self.num_snowflakes = 0;
                self.snowflakes.clear();
            },
            Msg::UpdateTime => {
                log::info!("Timed Event Triggered");
                self.create_snowflake(ctx);
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!{
            <>
            <div class="background lower" style="position: fixed;z-index:0;">
                { for self.snowflakes.iter().map(|h| h.render()) }
            </div>
            <div id="counterheader">
                    <p class="higher" id="counter"> { self.num_snowflakes } { " snowflakes have fallen"} </p>
            </div>
            <div id="buttons" class="higher">
                <button onclick={link.callback(|_| Msg::ClearSnowflakes) } >{ "Clear Snowflakes" }</button>
            </div>

            < message::MessageText />
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
