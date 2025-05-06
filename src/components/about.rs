use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" class="section">
            <div class="container">
                <h2 class="section-title">{ "About Me" }</h2>
                <div style="max-width: 800px; margin: 0 auto; text-align: center;">
                    <p style="margin-bottom: 1.5rem;">
                        // { "Hello! I'm Reza, also known as PocketJack. I'm a passionate software engineer and musician with a love for creating both functional applications and expressive music." }
                        { "Hey, I'm Reza Khaleghi, but you might know me as Pocketjack. Born on July 25, 2000, in Iran, I’m currently living in Turkey." }
                    </p>
                    <p style="margin-bottom: 1.5rem;">
                        // { "My technical journey has led me through various technologies including Rust, which I'm currently using with the Yew framework. I believe in clean, efficient code that solves real problems." }
                        { " My journey into tech started back in 2011 when I began learning computer programming. I spent a couple of years dabbling in different areas like networking and security, but it wasn’t until I found backend development that everything clicked. I started out as a freelancer and eventually moved into leading backend teams, focusing on building scalable and efficient systems." }
                    </p>
                    <p style="margin-bottom: 1.5rem;">
                        { "Outside of tech, I’m all about music. I picked up the bass guitar in 2013, then moved on to the acoustic guitar, and got hooked on rock music. In 2020, I dove into trap and hip-hop by making loops and beats, and it's become a hobby I still enjoy today." }
                    </p>
                    <p style="margin-bottom: 1.5rem;">
                        { "Right now, I’m totally fascinated by two things: jazz music and Rust. They’ve both completely changed how I see the world—jazz opened my ears to a whole new dimension of sound, while Rust has shifted how I think about programming, making it even more enjoyable and powerful." }
                    </p>
                    <p style="margin-bottom: 1.5rem;">
                        { " When I'm not coding or making music, you'll find me out in nature—whether it’s camping, hiking, or traveling. These have been passions of mine since I was a kid. If I’m back in Iran, you’ll probably catch me offroading in my old Land Rover, chasing that next adventure." }
                    </p>
//             <p>
//                         { "Hey, I'm Reza Khaleghi, but you might know me as Pocketjack. Born on July 25, 2000, in Iran, I’m currently living in Turkey. My journey into tech started back in 2011 when I began learning computer programming. I spent a couple of years dabbling in different areas like networking and security, but it wasn’t until I found backend development that everything clicked. I started out as a freelancer and eventually moved into leading backend teams, focusing on building scalable and efficient systems.

// Outside of tech, I’m all about music. I picked up the bass guitar in 2013-14, then moved on to the acoustic guitar, and got hooked on rock music. In 2020, I dove into trap and hip-hop by making loops and beats, and it's become a hobby I still enjoy today.

// Right now, I’m totally fascinated by two things: jazz music and Rust. They’ve both completely changed how I see the world—jazz opened my ears to a whole new dimension of sound, while Rust has shifted how I think about programming, making it even more enjoyable and powerful.

// When I'm not coding or making music, you'll find me out in nature—whether it’s camping, hiking, or traveling. These have been passions of mine since I was a kid. If I’m back in Iran, you’ll probably catch me offroading in my old Land Rover, chasing that next adventure." }
//                     </p>
        
                </div>
            </div>
        </section>
    }
}
