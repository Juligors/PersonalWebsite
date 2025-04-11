use leptos::prelude::*;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <h1>"Hi 👋"</h1>
        <p class="section">
            "I'm Julian Górski, a student of"
            <a href="https://www.agh.edu.pl/en" target="_blank">" AGH"</a>
            " in"
            <a href="https://www.eaiib.agh.edu.pl/informatyka-i-systemy-inteligentne/" target="_blank">
                " IT and Intellighent Systems, specialization Artificial Intelligence and Data Analysis."
            </a>
            " I'm passionate about programming and I'd like to share some of my projects and skills here."
        </p>
    }
}
