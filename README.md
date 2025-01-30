# Ungeckit: A Naive WebDriver Implementation 🦊

<p align="center">
	<img src="docs/images/ungeckit.png" alt="Ungeckit Logo" width="400"/>
</p>

<p align="center">
	<img src="https://img.shields.io/badge/stability-wip-lightgrey.svg" alt="Work in Progress"/>
	<img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"/>
	<img src="https://github.com/yyunon/ungeckit/actions/workflows/rust.yml/badge.svg" alt="Build Status"/>
</p>

Ungeckit is a **simple yet powerful** WebDriver implementation that lets you retrieve web pages both **asynchronously** and **synchronously**. While it’s currently in its early stages, you can already fetch web pages and save screenshots. More features are on the way, so stay tuned! 🚀

---

## 🛠️ Prerequisites

Before diving in, make sure you have **geckodriver** installed. It’s a must-have for Ungeckit to work. You can download it from the official [geckodriver releases page](https://github.com/mozilla/geckodriver/releases). For convenience, add it to your system’s PATH.

Test your installation with:
```bash
geckodriver -h
```

---

## 🏗️ Architecture

Ungeckit is designed to be intuitive and easy to use. Here’s how it works:

1. **DriverBuilder**: Create a `DriverBuilder` object, which sets up a Firefox driver when you call `build()`.
2. **Headless Mode**: When you call `get()` on a webpage, it initiates a session in Firefox (headless mode) and retrieves the page.
3. **Extensible**: The architecture is built to grow, with more features and capabilities planned for the future.

Here’s a visual representation of the architecture:
<p align="center">
	<img src="docs/images/arch.png" alt="Ungeckit Architecture"/>
</p>

---

## 🚀 What’s Next?

I’m currently working on integrating **CDP (Chrome DevTools Protocol)** support, which will allow you to run commands directly on DOM objects using WebSockets. This will unlock even more possibilities for automation and debugging. You can keep track of that  [here](https://github.com/yyunon/cdpgen)

---

## 📚 Examples

Check out the examples directory to see Ungeckit in action. Here’s a sneak peek:

```rust
// Example: Fetch a webpage and save a screenshot
let driver = DriverBuilder::new().build().unwrap();
driver.get("https://example.com").unwrap();
driver.save_screenshot("example.png").unwrap();
```

---

## 🌟 Stay Tuned!

Ungeckit is still evolving, and there’s a lot more to come. Your feedback and contributions are highly appreciated as we work to make this tool even better. Let’s build something amazing together! 🎉
