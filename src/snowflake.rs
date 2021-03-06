use crate::random;
use yew::{html, Component, Context, Html, Properties};

# [derive(Clone, Debug, Eq, PartialEq)]
pub struct SnowflakeInfo {
    pub start_x_pos: u8,
    pub id: usize,
}
impl SnowflakeInfo {
    pub fn new(id: usize) -> Self {
        SnowflakeInfo {
            start_x_pos: random::range_exclusive(0,100) as u8,
            id,
        }
    }

    fn x_pos(&self) -> String {
        format!("position: absolute;top:0px;left:{}vw;", self.start_x_pos).to_string()
    }

    fn render(&self) -> Html {
        html! {
            <div class="icon-snowflake-full">
            // https://codepen.io/Elyon/pen/nOdXBR
                <svg version="1.1" id="snowflake2" xmlns="http://www.w3.org/2000/svg"  x="0px" y="-100px" width="100px" height="100px" viewBox="0 0 100 100" style="enable-background:new 0 0 100 100;" >
                    <g>
	                    <polygon style="fill:#ffffff;" points="38.779,47.963 32.298,50 38.779,52.037 45.26,50 	"/>
	                    <polygon style="fill:#ffffff;" points="60.674,52.037 67.158,50 60.674,47.963 54.195,50 	"/>
            <path style="fill:#ffffff;" d="M78.006,65.943l1.779,0.564l-4.027-3.443l-0.504-0.158l-0.387-0.428l4.141-0.672l-4.117-1.016
		l-1.033,0.17l5.553-2.035l-5.488-0.74l-4.42,1.617l-1.725-0.955l5.381-1.541l-0.387-1.352l-6.732,1.932l-1.641-0.91l3.619-1.328
		l-0.943-0.127l0.734-0.211l-0.387-1.35l-5.578,1.6l-2.457-1.361l8.918-2.561l-0.387-1.348l-9.857,2.829l-1.445-1.596l-3.52-0.81
		l-1.326-0.736l1.029-0.605l3.824-0.863l1.613-1.768l9.668,2.507l0.352-1.36l-8.949-2.319l0.055-0.06l2.334-1.374l5.658,1.465
		l0.289-1.119l0.398-0.053l-0.35-0.13l0.014-0.058l-0.564-0.146l-2.723-1.011l-0.785,0.101l-0.225-0.058l0.342-0.201l1.34-0.302
		l0.822-0.901l6.855,1.775l0.352-1.358l-5.578-1.445l1.203-0.708l5.211,1.934l5.492-0.721l-6.133-2.275l-3.557,0.467l0.506-0.298
		l3.088-0.699l1.248,0.172l4.09-1.118l-3.203-0.437l3.543-3.884l-0.172,0.04l0.488-0.438l-2.797,0.96L73.6,34.354l1.303-3.342
		l-2.963,3.032l-0.74,1.896l-1.109,1.217l-0.699,0.412l1.773-2.345l1.059-6.456l-3.344,4.417l-0.848,5.185l-1.381,0.812l1.402-5.421
		l-1.357-0.352l-1.756,6.779l-0.309,0.182l-0.752,0.169l-0.539,0.59l-0.668,0.393l0.477-0.627l0.623-3.816l-0.199,0.265l-1.312-0.34
		l-1.453,5.617l-2.424,1.426l2.326-8.982l-1.359-0.352l-2.527,9.765L55.5,44.776l1.689-4.038l-5.051,4.158l-1.261,3.015v-0.693
		l1.16-3.694l-0.703-2.237l7.098-7.099l-0.994-0.993l-6.56,6.562v-2.812l4.13-4.131l-0.891-0.892l0.131-0.318l-2.984,2.457
		l-0.369,0.883l-0.018,0.018v-0.672l0.328-1.04l-0.328-1.041v-0.059l5.068-5.067l-0.994-0.993l-4.074,4.076v-1.497l4.175-3.438
		l2.137-5.11l-5.051,4.159l-1.261,3.015v-0.694l0.722-2.302l1.023-1.298l1.111-4.09l-2.111,2.678l-1.35-4.296l-0.274-1.342
		l-0.277,1.361l-1.345,4.279l-2.11-2.682l1.108,4.091l1.027,1.303l0.719,2.291v0.697l-1.26-3.012l-5.051-4.159l2.138,5.109
		l4.173,3.436v1.5l-3.958-3.961l-0.993,0.995l4.951,4.95v0.062l-0.325,1.037l0.325,1.037v0.675l-0.016-0.017l-0.37-0.884L45.75,31.6
		l0.151,0.36l-0.883,0.883l4.103,4.1v2.812l-6.561-6.562l-0.992,0.993l7.098,7.1l-0.703,2.236l1.158,3.687v0.699l-1.26-3.013
		l-5.051-4.158l1.677,4.006l-1.527-0.353l-2.706-9.43l-1.35,0.386l2.491,8.685l-0.609-0.141l-1.782-0.986l-0.534-1.864l-0.223-1.399
		l-0.289-0.385l-0.565-1.968l-0.751,0.216l-0.36-0.481l0.09,0.558l-0.328,0.095l0.771,2.69l0.075,0.475l0.1,0.132l0.179,0.624
		l-1.008-1.114l-1.478-0.34l-1.85-6.451l-1.349,0.388l1.589,5.539l-0.925-0.512l-0.954-6.002l-3.325-4.43l1.029,6.46l2.785,3.714
		l-0.223-0.124l-3.797-4.196l-0.138-0.032l-2.914-2.832l1.005,2.393l-4.575-1.053l0.952,1.053l-1.779-0.562l4.025,3.44l0.505,0.159
		l0.388,0.429l-4.141,0.672l4.119,1.013l1.032-0.168l-5.554,2.035l5.489,0.741l4.422-1.618l1.724,0.955l-5.383,1.543l0.389,1.349
		l6.729-1.931l1.64,0.91l-3.619,1.327l0.945,0.127L32.19,44.69l0.387,1.35l5.578-1.601l2.459,1.362l-8.919,2.56l0.388,1.35
		l9.857-2.828l1.444,1.595l3.52,0.812l1.327,0.733l-1.029,0.607l-3.827,0.862l-1.611,1.768l-9.667-2.503l-0.353,1.359l8.951,2.316
		l-0.054,0.061l-2.338,1.375l-5.657-1.465l-0.29,1.117l-0.398,0.053l0.351,0.131l-0.014,0.059l0.562,0.145l2.726,1.014l0.784-0.104
		l0.226,0.057l-0.344,0.203l-1.338,0.303l-0.822,0.9l-6.859-1.775l-0.351,1.359l5.579,1.443l-1.203,0.709l-5.211-1.936l-5.491,0.721
		l6.133,2.277l3.556-0.467l-0.504,0.297l-3.092,0.697l-1.246-0.168l-4.089,1.117l3.201,0.438l-3.541,3.881l0.171-0.039l-0.489,0.438
		l2.798-0.961l2.95-0.664l-1.304,3.342l2.963-3.031l0.739-1.898l1.112-1.217l0.697-0.41l-1.772,2.342l-1.058,6.457l3.342-4.416
		l0.85-5.184l1.38-0.811l-1.404,5.418l1.358,0.352l1.754-6.777l0.312-0.182l0.75-0.172l0.535-0.586l0.67-0.395l-0.474,0.625
		l-0.624,3.816l0.198-0.264l1.312,0.34l1.455-5.617l2.423-1.424l-2.327,8.98l1.359,0.352l2.529-9.764l1.323-0.301l-1.69,4.041
		l5.05-4.158l1.261-3.014v0.691l-1.16,3.695l0.704,2.238l-7.097,7.096l0.992,0.992l6.561-6.559v2.811l-4.131,4.131l0.894,0.893
		l-0.134,0.318l2.985-2.455l0.368-0.885l0.018-0.018v0.67l-0.327,1.041l0.327,1.043v0.057l-5.067,5.068l0.993,0.992l4.074-4.074
		v1.498l-4.174,3.436l-2.137,5.109l5.05-4.158l1.261-3.014v0.693l-0.723,2.301L47.375,77l-1.108,4.092l2.109-2.678l1.349,4.291
		l0.273,1.346l0.278-1.361l1.345-4.279l2.113,2.684L52.623,77l-1.025-1.301l-0.72-2.289v-0.697l1.259,3.01l5.053,4.158l-2.141-5.109
		l-4.171-3.436v-1.498l3.957,3.957l0.994-0.992l-4.951-4.951v-0.064l0.326-1.035l-0.326-1.037v-0.674l0.015,0.014l0.372,0.889
		l2.984,2.455L54.1,68.039l0.879-0.881l-4.101-4.102v-2.811l6.56,6.559l0.994-0.992l-7.1-7.098l0.703-2.236l-1.158-3.688v-0.695
		l1.259,3.01l5.053,4.158l-1.678-4.008l1.527,0.354l2.707,9.43l1.348-0.387l-2.492-8.684l0.609,0.139l1.785,0.988l0.533,1.867
		l0.225,1.396l0.287,0.385l0.564,1.969l0.75-0.215l0.361,0.479l-0.09-0.557l0.328-0.094l-0.771-2.686l-0.076-0.482l-0.1-0.131
		l-0.178-0.621l1.008,1.115l1.479,0.338l1.85,6.449l1.35-0.385l-1.59-5.541l0.924,0.512l0.955,6l3.326,4.432L71.1,64.865
		l-2.785-3.713l0.223,0.123l3.799,4.197l0.137,0.031l2.914,2.832l-1.004-2.393l4.574,1.053L78.006,65.943z M72.861,61.123
		l-0.756,0.123l-0.539-0.299L72.861,61.123z M27.136,38.878l0.759-0.123l0.539,0.299L27.136,38.878z"/>
                    </g>
                </svg>
            </div>
        }
    }
}

#[derive(Debug, Eq, PartialEq, Properties)]
pub struct SnowflakeProps {
    info: SnowflakeInfo,
}

pub struct SnowflakeComponent {
    pub visible: bool,
    pub clicked: bool,
}

impl SnowflakeComponent {
    pub fn hide(&mut self) {
        self.visible = false;
    }
    pub fn show(&mut self) {
        self.visible = true;
    }
}

pub enum SnowflakeMsg {
    Hide,
    Show,
    Clicked,
}

impl Component for SnowflakeComponent {
    type Message = SnowflakeMsg;
    type Properties = SnowflakeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            visible: true,
            clicked: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SnowflakeMsg::Hide => {
                self.hide();
            },
            SnowflakeMsg::Show => {
                self.show();
            },
            SnowflakeMsg::Clicked => {
                self.clicked = true;
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div
                class={if self.visible { if self.clicked { "snowflake snowflake-clicked" } else { "snowflake" } } else { "snowflake hidden" }}
                id={format!{"{}h", ctx.props().info.id}.to_string()}
                style={ctx.props().info.x_pos()}
                onclick={link.callback(|_| SnowflakeMsg::Clicked)}
            >
                { ctx.props().info.render() }
            </div>
        }
    }
}

pub enum SnowflakeType {
    Component(SnowflakeInfo),
}

impl SnowflakeType {
    pub fn info(&self) -> &SnowflakeInfo {
        match self {
            Self::Component(info) => info,
        }
    }

    pub fn new(id: usize) -> Self {
        let info = SnowflakeInfo::new(id as usize);
        Self::Component(info)
    }

    pub fn render(&self) -> Html {
        let info = self.info();
        html! { <SnowflakeComponent key={info.id.to_string()} info={info.clone()} /> }
    }
}
