# Confluence Content Fetcher

A Rust-based command-line tool designed to interact with the Confluence API, fetches content pages along with their attachments based on specified media types. This tool processes content incrementally, handling pagination seamlessly to ensure efficient data retrieval.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Fetch Content Pages:** Retrieve pages from a specified Confluence instance.
- **Attachment Retrieval:** For each content page, fetch attachments based on defined media types (e.g., `video/mp4`, `audio/m4a`).
- **Pagination Handling:** Automatically handles API pagination to process large volumes of data efficiently.
- **Incremental Processing:** Processes each content and its attachments one by one, optimizing memory usage.
- **Simplified Error Handling:** Uses `unwrap()` for error handling, suitable for initial development and testing phases.

## Prerequisites

- **Rust:** Ensure that Rust is installed on your system. If not, download it from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Confluence Access:** Access to a Confluence instance with the necessary permissions to fetch content and attachments.

## Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/confluence-content-fetcher.git
   cd confluence-content-fetcher
   ```

2. **Build the Project:**

   ```bash
   cargo build --release
   ```

3. **Run the Application:**

   ```bash
   cargo run --release
   ```

   Alternatively, after building, you can run the executable directly:

   ```bash
   ./target/release/confluence-content-fetcher
   ```

## Configuration

The application currently uses hardcoded values for the Confluence base URL, REST API endpoint, and media types. To customize these settings:

1. **Edit the Source Code:**

   Open `src/main.rs` and modify the following sections as needed:

   ```rust
   // Initialize Confluence instance
   let wiki = Confluence::new("https://your-confluence-instance.com", "rest/api");

   // Define media types to fetch
   let media_types = ["video/mp4", "audio/m4a"];
   ```

2. **Media Types:**

   Adjust the `media_types` array to include the MIME types of attachments you wish to fetch.

## Usage

Upon running the application, it will:

1. Connect to the specified Confluence instance.
2. Fetch content pages of type `page`, expanding space information.
3. For each content page, fetch attachments matching the defined media types.
4. Print the details of each attachment, including URLs for direct access.

**Sample Output:**

```
"/Space/Introduction to Project/Project Overview" https://your-confluence-instance.com/download/attachments/project_overview.mp4/data https://your-confluence-instance.com/rest/api/content/12345/child/attachment/67890/data
Fetching attachments for media type: audio/m4a
"/Space/Introduction to Project/Voice Overview" https://your-confluence-instance.com/download/attachments/voice_overview.m4a/data https://your-confluence-instance.com/rest/api/content/12345/child/attachment/67891/data
"/Space/Project Components/Component A" https://your-confluence-instance.com/download/attachments/component_a.mp4/data https://your-confluence-instance.com/rest/api/content/12346/child/attachment/67892/data
"/Space/Project Components/Component B" https://your-confluence-instance.com/download/attachments/component_b.m4a/data https://your-confluence-instance.com/rest/api/content/12346/child/attachment/67893/data

```

## Project Structure

```
confluence-content-fetcher/
│
├── src/
│   └── main.rs        # Main application source code
│
├── Cargo.toml         # Project configuration and dependencies
│
└── README.md          # Project documentation
```

## Contributing

Contributions are welcome! To contribute:

1. **Fork the Repository:**

   Click the "Fork" button on the repository page to create a personal copy.

2. **Clone Your Fork:**

   ```bash
   git clone https://github.com/yourusername/confluence-content-fetcher.git
   cd confluence-content-fetcher
   ```

3. **Create a New Branch:**

   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make Your Changes:**

   Implement your feature or bug fix.

5. **Commit Your Changes:**

   ```bash
   git commit -m "Add your descriptive commit message"
   ```

6. **Push to Your Fork:**

   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request:**

   Navigate to the original repository and create a pull request from your fork.

## License

This project is licensed under the [MIT License](LICENSE).

---

> **Note:** The current implementation uses `unwrap()` for error handling, which will cause the application to panic on encountering any errors. This is suitable for development and testing purposes only. For a production-ready version, consider implementing robust error handling mechanisms.

