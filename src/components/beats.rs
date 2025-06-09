use std::rc::Rc;
use yew::prelude::*;
use web_sys::HtmlAudioElement;
use crate::data::BEATS_DATA;

#[function_component(Beats)]
pub fn beats() -> Html {
    // 2. Add explicit type annotation.
    // 3. Fix the argument order in `use_memo`. It's `use_memo(dependencies, closure)`.
    let audio_refs: Rc<Vec<NodeRef>> = use_memo(
        (), // Dependencies go first. Empty tuple `()` means it runs only once.
        |_| (0..BEATS_DATA.len()).map(|_| NodeRef::default()).collect::<Vec<_>>()
    );

    let playing_states = use_state(|| vec![false; BEATS_DATA.len()]);

    html! {
        <section id="beats" class="section">
            <div class="container">
                <h2 class="section-title">{ "Beats" }</h2>
                <div class="beats-grid">
                    {
                        BEATS_DATA.iter().enumerate().map(|(index, beat)| {
                            let playing = playing_states.get(index).cloned().unwrap_or(false);
                            // `audio_refs` is an `Rc`, so we dereference it to access the Vec.
                            let audio_ref = audio_refs.get(index).cloned().unwrap();

                            let onclick = {
                                let playing_states = playing_states.clone();
                                let audio_ref = audio_ref.clone();
                                Callback::from(move |_| {
                                    if let Some(audio) = audio_ref.cast::<HtmlAudioElement>() {
                                        let mut states = (*playing_states).clone();
                                        if states[index] {
                                            let _ = audio.pause();
                                        } else {
                                            let _ = audio.play();
                                        }
                                        states[index] = !states[index];
                                        playing_states.set(states);
                                    }
                                })
                            };

                            html! {
                                <div class="beat-card" key={beat.title}>
                                    <div class="beat-content">
                                        <h3 class="beat-title">{ beat.title }</h3>
                                        <p class="beat-details">{ format!("Key: {}, BPM: {}", beat.key, beat.bpm) }</p>
                                        <button class={classes!("beat-play", playing.then_some("playing"))} {onclick}>
                                            <span>{ if playing { "Pause" } else { "Play" } }</span>
                                        </button>
                                    </div>
                                    <audio ref={audio_ref} src={format!("static/{}", beat.audio_path)}></audio>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </section>
    }
}