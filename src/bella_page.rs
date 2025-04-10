use leptos::prelude::*;

#[component]
pub fn BellaPage() -> impl IntoView {
    view! {
        <script type="module">{include_str!("bella_load_script.js")}</script>

        <div class="section">
            <h1>"Bella"</h1>
            <div>
                <p>
                    "This is my Master's Thesis"
                    <i>" \"Evolution in multi-agent based systems. Theoretic and application-based aspects\"."</i>

                    " project that I've been working on for some time now. It's meant to be an agent simulation of environment with plants as basis of a food chain, followed by herbivorous, carnivorous and omnivorous animals. Like any environment model, it has to be a significant simplification in relation to th real world, especially since I'm working on it on my own. Nonetheless, I've managed to implement those elements so far:
                    "
                    <ul>
                        <li>
                            "Procedurally generated terrain with"
                            <ul>
                                <li>"different biomes with different parameters"</li>
                                <li>"humidity and temperature propagation"</li>
                                <li>"nutrients levels"</li>
                            </ul>
                        </li>
                        <li>"Reproducing and aging organisms, such as"
                            <ul>
                                <li>"plants"</li>
                                <li>"animals with AI, capable of moving and attacking"</li>
                            </ul>
                        </li>
                        <li>"Night and day cycle"</li>
                        <li>"Carcass system for dead organisms"</li>
                        <li>"Dominant and recessive genes"</li>
                        <li>"Data collection system"</li>
                        <li>"Ability to pause or restart simulation"</li>
                        <li>"Debugging tools and gizmos"</li>
                        <li>
                            "Multiple builds"
                            <ul>
                                <li>"Native windowed"</li>
                                <li>"WASM-based windowed build for browser you're seeing now"</li>
                                <li>"Native headless, for maximum performance for experiments"</li>
                            </ul>
                        </li>
                    </ul>
                    <h3>"If you'd like to take a closer look at Bella you can:"</h3>
                    <ul>
                        <li>"Move around with" <b>" right mouse button"</b></li>
                        <li><b>"Scroll"</b>" to zoom in/out"</li>
                        <li>"Reset with" <b>" 'R'"</b></li>
                        <li>"Pause with" <b>" 'P'"</b></li>
                        <li>"Inspect any entity and changing its components by choosing it with" <b>" left mouse button"</b></li>
                    </ul>
                    "I update Bella version on this website whenever I make enough progress to warrant it."
                </p>
            </div>
            <div class="simulation-container">
                <div id="loading-message">"Loading the simulation 😎 Please wait"</div>
                <canvas id="bevy-bella">
                    "Your browser does not support the canvas element :("
                </canvas>
            </div>
        </div>
    }
}
