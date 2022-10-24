use yew::prelude::*;

use crate::positions::{get_positions, write_report};
use crate::text_input::{get_value_from_select_event, TextInput, UsernameInput};

pub enum Msg {
    SetUsername(String),
    SetReport(String),
    SetDiscussion(String),
    SetPosition(String),
    GenerateReport,
}

#[derive(Debug, Default)]
pub struct App {
    username: String,
    report: String,
    discussion: String,
    position: String,
    positions: Vec<String>,
    report_submitted: bool,
}

impl App {
    fn generate_report(&mut self) {
        let report = format!(
            "{} ({})\nReport\n{}\nDiscussion Topics\n{}\n\n",
            self.position, self.username, self.report, self.discussion,
        );

        let _ = write_report(self.position.clone(), report);
        self.report_submitted = true;
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut sel = Self::default();
        sel.positions = get_positions();
        sel
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetUsername(username) => self.username = username,
            Msg::SetReport(report) => self.report = report,
            Msg::GenerateReport => self.generate_report(),
            Msg::SetDiscussion(topics) => self.discussion = topics,
            Msg::SetPosition(position) => self.position = position,
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_username_change = ctx.link().callback(Msg::SetUsername);
        let on_discussion_change = ctx.link().callback(Msg::SetDiscussion);
        let on_position_change = ctx.link().callback(|e: Event| {
            let value = get_value_from_select_event(e);
            Msg::SetPosition(value)
        });
        let on_change = ctx.link().callback(Msg::SetReport);
        let onclick = ctx.link().callback(|_| Msg::GenerateReport);
        if !self.report_submitted {
            html! {
                <main>
                    <h1>{"TKE:XU Report Submission"}</h1>
                    <div class="name">
                        <div>{"Enter your name below:"}</div>
                        <div>
                            <UsernameInput on_change = {on_username_change} value={self.username.clone()} />
                        </div>
                    </div>
                    <div class="entry">
                        <select class="custom-select" onchange={on_position_change} name="report-selections" id="report-selections">
                        {
                            self.positions.clone().into_iter().map(|position| {
                                html!{<option class="option">{ format!("{}", position) }</option>}
                            }).collect::<Html>()
                        }
                        </select>
                    </div>
                    <div class="entry">
                        <div>
                            {"Enter your report below:"}
                            <div class="footnote">
                                {"(Uses markdown formatting, if you want sub bullets, add a - in-front of your text)"}
                            </div>
                        </div>
                        <div class="white-text">
                            <TextInput {on_change} value={self.report.clone()} />
                        </div>
                    </div>
                    <div class="entry">
                        <div>
                            {"Enter any discussion topics below:"}
                            <div class="footnote">
                                {"(Uses markdown formatting, if you want sub bullets, add a - in-front of your text)"}
                            </div>
                        </div>
                        <div class="white-text">
                            <TextInput on_change={on_discussion_change} value={self.discussion.clone()} />
                        </div>
                    </div>
                    <footer>{"747"}</footer>
                    <header>
                        <button {onclick}>
                            {"Submit report"}
                        </button>
                    </header>
                </main>
            }
        } else {
            html! {
                <main>
                    <h1>{"TKE:XU Report Submission"}</h1>
                </main>
            }
        }
    }
}
