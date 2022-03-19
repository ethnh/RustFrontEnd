use yew::{html, Component, Context, Html};

pub static TEXT: &'static [&'static str] = &[
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec molestie sem augue, sed feugiat erat maximus id. Duis commodo dictum risus eu fringilla. Quisque ipsum mi, laoreet non varius pellentesque, condimentum sit amet nibh. Aliquam erat volutpat. Quisque semper quam sed lacus tristique, id pellentesque leo aliquet. Morbi tempor erat justo, et tristique nunc placerat non. Curabitur libero quam, tempor vel volutpat non, varius nec quam. Vivamus elit arcu, rhoncus eu est vel, ultricies sagittis nulla. Aliquam a convallis turpis. Etiam tincidunt ultricies ullamcorper. Mauris eget erat eros. Curabitur ut urna hendrerit, maximus est vitae, semper lectus. Quisque ultricies porttitor tempor. Curabitur pulvinar et justo non tempus.",
    "Mauris eu lorem et quam dictum auctor. Maecenas nec sagittis lectus. Donec vel quam quis leo consectetur consequat. Mauris varius justo mauris, consequat bibendum libero accumsan et. Pellentesque at suscipit felis. Phasellus quis auctor tellus. Suspendisse pulvinar suscipit pretium. Morbi sit amet sem feugiat, elementum arcu eget, dignissim purus. Vestibulum at elit eu magna hendrerit ultrices ac nec sapien. In a nunc euismod, rutrum mi non, sagittis dui. Vestibulum justo magna, commodo blandit fringilla non, vestibulum non tortor. Mauris metus est, hendrerit non rutrum ut, tristique et lacus. Donec dignissim elit et rat laoreet, vulputate egestas neque eleifend. Etiam consequat lacinia eros, in consequat quam elementum quis. Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    "Vestibulum congue lacus eu nunc laoreet, sit amet consectetur tellus vulputate. Donec placerat mattis nisi non pretium. Nulla eleifend molestie quam, et dignissim ante faucibus fringilla. Duis et sem sed augue convallis ornare. Quisque blandit neque a ex vulputate, nec hendrerit purus cursus. Nulla aliquet, ante eget pharetra dignissim, metus augue ultrices dolor, vel posuere velit mi iaculis nisl. Aliquam finibus orci tellus, quis vestibulum orci aliquam a. Phasellus ut enim sit amet libero faucibus sodales et vel leo. Mauris porttitor condimentum orci. Quisque quis lectus at nisi rhoncus accumsan. Etiam sed ipsum ex.",
    "Ut vehicula enim sit amet ullamcorper blandit. Vivamus ac purus id diam tincidunt elementum vitae vel mauris. Maecenas fringilla euismod ex interdum mollis. Quisque sagittis, quam in iaculis cursus, quam risus ornare libero, egestas bibendum dui sapien laoreet ligula. Mauris eget neque tincidunt, condimentum massa sed, maximus ex. Vestibulum rutrum dui tortor, nec varius enim eleifend dignissim. Fusce ullamcorper elit non vehicula scelerisque. Quisque luctus enim a dolor porttitor imperdiet. Mauris vel ultricies magna. Sed ut lacinia turpis. Mauris sit amet consectetur felis.",
    "Aenean congue, odio nec interdum efficitur, tortor lectus viverra est, eu lacinia purus quam non ante. Etiam et enim consequat, lacinia augue ut, vestibulum elit. Pellentesque a nibh a purus mattis fermentum. Morbi sit amet quam ultrices, fringilla diam eget, pharetra felis. In hac habitasse platea dictumst. Aliquam vehicula commodo luctus. Phasellus feugiat, ipsum sit amet tempus commodo, lorem est dictum leo, at cursus elit nibh sed risus. Donec vel massa eros. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam condimentum, ligula scelerisque malesuada aliquam, risus urna sollicitudin nibh, sit amet condimentum quam turpis vitae nunc. Maecenas consequat dolor vel arcu placerat fringilla.",
    "Aliquam sagittis, ex eu tristique vulputate, ex odio tristique lorem, ut fermentum mi arcu id leo. Duis sed varius massa. Aenean hendrerit sed risus nec placerat. Praesent iaculis non magna ut ultricies. Pellentesque tempus ultrices semper. Maecenas a eros aliquam, interdum mi in, congue enim. Aenean at tellus aliquet lorem sagittis sagittis eget vel erat. Phasellus quis lobortis dui. Suspendisse aliquet efficitur mauris, eu dignissim libero imperdiet at. Praesent et enim viverra, egestas sapien sed, convallis mauris. Nulla tempor nibh justo, non pharetra magna egestas vitae. Fusce quis massa tincidunt, luctus nisi eu, porttitor orci. Nam facilisis lacus ante, id vestibulum mi rutrum nec. Vivamus interdum finibus nisi, sed tincidunt enim efficitur a. Proin tristique purus orci, a finibus urna posuere vitae.",
    "Etiam laoreet sagittis libero in ultrices. Suspendisse varius tellus nec justo hendrerit, a tincidunt est fermentum. Ut sed consectetur urna, quis tincidunt ligula. Proin quis justo tortor. Ut scelerisque, ex ac commodo varius, tellus nisi vehicula nibh, nec viverra eros sem sed leo. Integer lacus libero, dignissim at mauris at, efficitur fermentum est. Vivamus sit amet est imperdiet, tincidunt sapien quis, placerat lectus.",
    "Aenean elit tellus, maximus eget justo ac, pulvinar molestie purus. Etiam vitae dapibus ligula, in pellentesque lacus. Ut non mollis risus, nec laoreet nunc. Mauris feugiat interdum tristique. Aliquam mattis odio eget aliquam rhoncus. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Maecenas lacus turpis, faucibus in accumsan eu, rutrum eu nulla. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam lobortis in arcu et luctus. Maecenas varius neque ac gravida porta.",
    "Sed euismod arcu sed neque lacinia, vitae maximus orci malesuada. Nullam laoreet enim id mauris lacinia ultricies. Sed imperdiet lacus dui, nec aliquet nisl ornare scelerisque. Nulla convallis turpis quam, quis dictum justo elementum at. Donec vel massa posuere, posuere metus in, posuere augue. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Donec suscipit est massa, vel cursus neque ultrices vitae. Morbi sed ante auctor, interdum quam id, mollis nibh. Morbi rutrum lacus auctor risus aliquet scelerisque. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.",
    "Mauris fermentum mattis consequat. In risus elit, gravida eget tincidunt ut, ornare quis lectus. Duis elementum dui eget nisl ornare, vel semper tellus porttitor. Aenean imperdiet hendrerit scelerisque. Suspendisse dictum lacus ac faucibus feugiat. Aenean luctus diam velit, ac condimentum leo gravida in.Nunc posuere augue nec purus aliquam, sed tempus augue faucibus. Aenean eu lorem eget sapien tincidunt pharetra. Proin et mauris fermentum, porttitor nibh pretium, imperdiet magna. Ut sollicitudin elit at tellus iaculis, ut laoreet lorem tincidunt. Nullam sed congue erat. Morbi porta ex quis quam eleifend, non hendrerit est mattis. Etiam euismod porta pulvinar. Nullam elementum condimentum porttitor. Vestibulum vel consequat purus, ut venenatis arcu.",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageText {
    pub visible: bool,
    pub current_id: usize,
    pub interacted_yet: bool,
}

impl MessageText {
    pub fn new(current_id: usize) -> Self {
        MessageText {
            visible: true,
            current_id,
            interacted_yet: false,
        }
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }
    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn set_interacted(&mut self) {
        self.interacted_yet = true;
    }

    pub fn not_yet_interacted_hint(&self) -> Html {
        if !self.interacted_yet {
            html! {
                <div class="message-hint">
                    <p class="mini">{"Hint: You can tap on the text to continue."}</p>
                </div>
            }
        } else {
            html! {}
        }
    }

    pub fn next_id(&mut self) {
        if self.current_id < TEXT.len() - 1 {
            self.current_id += 1;
        } else {
            self.current_id = 0;
        }
    }

    pub fn last_id(&mut self) {
        if self.current_id > 0 {
            self.current_id -= 1;
        } else {
            self.current_id = TEXT.len() - 1;
        }
    }

    pub fn render_message(&self) -> Html {
        html! {
            <div class="message">
                { self.not_yet_interacted_hint() }
                <p>{ TEXT[self.current_id] }</p>
            </div>
        }
    }
}

pub enum Msg {
    Hide,
    Show,
    Next,
    Back,
}

impl Component for MessageText {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MessageText::new(0)
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Hide => {
                self.hide();
                self.set_interacted();
                true
            },
            Msg::Show => {
                self.show();
                self.set_interacted();
                true
            },
            Msg::Next => {
                self.next_id();
                self.set_interacted();
                true
            },
            Msg::Back => {
                self.last_id();
                self.set_interacted();
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let link = ctx.link();

        let class = if self.visible {
            "message-text"
        } else {
            "message-text hidden"
        };
        html! {
            <div class={class}>
                <div class="message-text-container" onclick={link.callback(|_| Msg::Next)}>
                    { self.render_message() }
                </div>
            <div class="message-text-buttons">
                <button onclick={link.callback(|_| Msg::Back)}> {"Back"} </button>
                <button onclick={link.callback(|_| Msg::Next)}> {"Next"} </button>
            </div>
            </div>
        }
    }
}
