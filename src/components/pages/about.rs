use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
        <div style="padding-left: 20px">
            <h1>{"Who Am I?"}</h1>
        </div>
        <hr style="border: 1px solid #0824c4; width: 100%;" />
            <div style="padding: 20px;">
                <img src="static/me.jpg" alt="About Image" style="display: block; margin: 0 auto;"/>
            </div>
            <div style="text-align: center;">
                <p>{"Developing telescope alignment software @ QEYNet."}</p>
            </div>
        <div style="padding: 50px;">
            <ul>
                <li>{"I'm a 4th year Systems Design Engineering student at the University of Waterloo."}</li>
                <li>{"I have 3 years of technical experience, working on software teams to build drones, batteries, and communication networks."}</li>
                <li>{"I'm a member of the University of Waterloo RoboSub team and an engineering orientation leader."}</li>
                <li>{"I believe in solving complex problems as simply as possible."}</li>
                <li>{"I'm interested in using my experience with autonomous vehicles and machine learning applications to solve real problems"}</li>
            </ul>
        </div>
        </div>
    }
}