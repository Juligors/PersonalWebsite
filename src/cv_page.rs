use leptos::prelude::*;

#[component]
pub fn CvPage() -> impl IntoView {
    view! {
        <div class="cv-container">
            <iframe src="/CV.pdf" width="100%" height="800px" style="border: none;" />
        </div>
    }
}
