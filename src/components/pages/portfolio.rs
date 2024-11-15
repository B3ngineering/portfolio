use yew::prelude::*;

#[function_component(Portfolio)]
pub fn about() -> Html {
    html! {
        <div>
        <div style="padding-left: 20px">
            <h1>{"What Have I Made?"}</h1>
        </div>
        <hr style="border: 1px solid #0824c4; width: 100%;" />
        <div class="project" style="padding : 50px">
            <h2>{"Project Title 1"}</h2>
            <img src="static/project1.jpg" alt="Project 1 Image" style="max-width: 100%; height: auto; border: 2px solid #0824c4; margin-bottom: 20px;" />
            <p>{"Description of project 1. This project is about..."}</p>
        </div>
        <hr style="border: 1px solid #0824c4; width: 100%;" />
        <div class="project" style="padding : 50px">
            <h2>{"Project Title 2"}</h2>
            <img src="static/project2.jpg" alt="Project 2 Image" style="max-width: 100%; height: auto; border: 2px solid #0824c4; margin-bottom: 20px;" />
            <p>{"Description of project 2. This project is about..."}</p>
        </div>
        <hr style="border: 1px solid #0824c4; width: 100%;" />
        <div class="project" style="padding : 50px">
            <h2>{"Project Title 3"}</h2>
            <img src="static/project3.jpg" alt="Project 3 Image" style="max-width: 100%; height: auto; border: 2px solid #0824c4; margin-bottom: 20px;" />
            <p>{"Description of project 3. This project is about..."}</p>
        </div>
        </div>
    }
}