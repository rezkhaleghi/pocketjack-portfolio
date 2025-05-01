use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-blue-600 text-white py-4">
            <nav class="container mx-auto px-4 flex justify-between items-center">
                <h1 class="text-2xl font-bold">{"My Portfolio"}</h1>
                <ul class="flex space-x-4">
                    <li><a href="#home" class="hover:underline">{"Home"}</a></li>
                    <li><a href="#projects" class="hover:underline">{"Projects"}</a></li>
                    <li><a href="#contact" class="hover:underline">{"Contact"}</a></li>
                </ul>
            </nav>
        </header>
    }
}