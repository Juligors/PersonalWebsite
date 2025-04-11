use intro::Intro;
use leptos::prelude::*;
use projects::Projects;
use qna::QnA;

mod intro;
mod projects;
mod qna;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Intro/>
        <Projects/>
        <QnA/>
    }
}
