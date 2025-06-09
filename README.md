# PocketJack Portfolio🦀

## Overview

This is a personal portfolio website for Reza Khaleghi, also known as **PocketJack**, a software engineer and musician. The site showcases Reza's technical projects, client work, music, skills, and professional experience. Built using the **Rust** programming language and the **Yew** framework, this project leverages WebAssembly to deliver a fast, modern, and interactive web experience.

## Features

- **Hero Section**: Introduces Reza with a brief bio, social media links (including Medium), and a profile image.
- **About Section**: Details Reza's background as a software engineer and musician, with a focus on his passions and interests.
- **Skills Section**: Displays technical skills and hobbies in a responsive grid layout with glowing hover effects, using data from `data.rs`.
- **Languages Section**: Showcases language proficiencies with circular progress rings indicating skill levels.
- **Personal Projects**: Highlights projects developed under Reza's company, FalseFoundation, with a carousel for browsing.
- **Client Projects**: Showcases freelance and contract-based work with detailed descriptions and technologies used.
- **Music Section**: Features music tracks with embedded YouTube players and links to platforms like Spotify and SoundCloud.
- **Experience Section**: Presents a timeline of Reza's professional experience, including roles, companies, and technologies.
- **Contact Section**: Provides contact information.
- **Footer Section**: Includes license and copyright details.

## Project Structure

```
pocketjack-portfolio/
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── index.html
├── Trunk.toml
├── src/
│   ├── main.rs
│   ├── components/
│   │   ├── about.rs
│   │   ├── client_projects.rs
│   │   ├── experience.rs
│   │   ├── footer.rs
│   │   ├── header.rs
│   │   ├── hero.rs
│   │   ├── mod.rs
│   │   ├── music.rs
│   │   ├── my_projects.rs
│   │   ├── skills.rs
│   │   ├── contact.rs
│   │   └── lang.rs
│   ├── pages/
│   │   ├── home.rs          # Integrates all components
│   │   ├── mod.rs
│   ├── data.rs
│   ├── models.rs            # Defines data structures for Every parts
├── static/
```

## Technologies Used

- **Rust**: Core programming language for the project.
- **Yew**: A Rust framework for building front-end web applications using WebAssembly.
- **WebAssembly (WASM)**: Enables high-performance web applications.
- **web-sys**: Provides bindings to interact with browser APIs.
- **wasm-bindgen**: Facilitates communication between Rust and JavaScript.
- **Trunk**: A build tool for bundling and serving the Yew application.

## Setup and Installation

### Prerequisites

- **Rust**: Install Rust and Cargo via [rustup](https://rustup.rs/).
- **Trunk**: Install Trunk, a WASM build tool, by running:
  ```bash
  cargo install trunk
  ```
- **WASM Target**: Add the WebAssembly target for Rust:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Running the Project

1. **Clone the Repository**:

   ```bash
   git clone <repository-url>
   cd pocketjack-portfolio
   ```

2. **Build and Serve**:
   Use Trunk to build and serve the application locally:

   ```bash
   trunk serve
   ```

   This will start a local development server (typically at `http://localhost:8080`).

3. **Build for Production**:
   To create a production build:
   ```bash
   trunk build --release
   ```
   The output will be in the `dist/` directory.

## Usage

- Navigate through the sections using the browser.
- The **Projects** and **Client Projects** sections feature a carousel for browsing multiple items (auto-advances every 5 seconds, pauses on hover).
- The **Music** section includes embedded YouTube players for tracks and links to external platforms.
- The **Skills** section displays technical skills and hobbies in a responsive grid, with glowing hover effects on cards, sourced from `data.rs`.
- The **Languages** section features circular progress rings indicating proficiency levels for English, German, and Persian, driven by `LANGUAGE_DATA` in `data.rs` and rendered by `lang.rs`.
- The **Experience** section presents a timeline of professional roles.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For inquiries, reach out via:

- **LinkedIn**: [linkedin.com/in/rezaxkhaleghi](https://linkedin.com/in/rezaxkhaleghi)
- **GitHub**: [github.com/rezkhaleghi](https://github.com/rezkhaleghi)
- **Medium**: [medium.com/@rezaxkhaleghi](https://medium.com/@rezaxkhaleghi)
- **Email**: (Rezaxkhaleghi@gmail.com)

---

Built with 🦀 by PocketJack
