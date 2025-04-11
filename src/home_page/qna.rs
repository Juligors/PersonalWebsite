use leptos::prelude::*;

#[component]
pub fn QnA() -> impl IntoView {
    view! {
        <h1>"About me"</h1>
        <div class="section">
            <div class="question-and-answer">
                <div class="question">"Favourite language"</div>
                <div class="answer">"Rust, but I'm fine using anything"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Favourite editor"</div>
                <div class="answer">"VS Code with Vim Motions"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Cats or dogs"</div>
                <div class="answer">"Cats"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Favourite operating system"</div>
                <div class="answer">"Linux"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Favourite book"</div>
                <div class="answer">"Twilight"</div>
            </div>
        </div>
    }
}
