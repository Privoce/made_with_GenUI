# GenUI Project

Welcome to the GenUI Project! This project is built with the Ract tool and serves as a standardized GenUI project template. Below is the version information for the main frameworks and tools used:

| Framework/Tool                | Version/Branch        |
|-------------------------------|-----------------------|
| **GenUI**                     | v0.1.1                |
| **Makepad**                   | gen_ui (branch)       |
| **Ract**                      | v0.1.3                |
| **GenUI Built-in Component**  | v0.2.2                |

---

## Project Structure

The GenUI project adopts a typical Rust Workspace structure, consisting of multiple sub-projects. Below is a description of the project directory and its functions:

> [!TIP]
> The following content symbols are explained:
> - `#`: Descriptor, the specific name is unknown, for example, `#workspace` means the project name of a workspace project created by the user
> - `[]`: means optional

```
#workspace
│
├── source_project             // Source code package
│   ├── src/                   // Rust source code files
│   │   └── main.rs            // Project entry file (usually empty)
│   ├── resources/             // Static resources
│   ├── views/                 // Main page files
│   │   ├── root.gen           // UI entry file
│   │   ├── home.gen           // Home page
│   │   └── mod.gen            // Page export mod file
│   ├── components/            // Component files
│   │   ├── hello.gen          // Hello component
│   │   ├── easy.gen           // Easy component
│   │   └── mod.gen            // Component export mod file
│   ├── .gen_ui_cache/         // Cache files
│   ├── Cargo.toml             // Rust package configuration file
│   └── gen_ui.toml            // GenUI project configuration file
│
├── compiled_project           // Compiled result package
│
├── .ract                      // Ract configuration file
├── Cargo.toml                 // Workspace configuration file
├── Cargo.lock                 // Dependency lock file
├── [.gitignore]               // gitignore (optional)
└── [LICENSE]                  // Project license file (optional)
```

---

## Launch and Compilation

Managed by the Ract tool, you can easily compile and launch the project:

1. Navigate to the project root directory in the terminal:
```bash
cd /path/to/workspace
```
2. Run the following command to start the project:
```bash
ract run
```
Ract will automatically locate the source code directory and start the project. Ensure that the .ract and gen_ui.toml configuration files are correctly set up. Upon successful launch, you will see the following output:

```
🥳 Welcome to use ract project runner!

🔸 Now you can run makepad and gen_ui (Comming Soon) projects
❗️ Please make sure your project root has a `.ract` file to indicate the project type
🔸 If you are unfamiliar with the `.ract` file, please run `ract book` to search (Coming Soon)


                                                      
     _/_/_/  _/_/_/_/  _/      _/  _/    _/  _/_/_/   
  _/        _/        _/_/    _/  _/    _/    _/      
 _/  _/_/  _/_/_/    _/  _/  _/  _/    _/    _/       
_/    _/  _/        _/    _/_/  _/    _/    _/        
 _/_/_/  _/_/_/_/  _/      _/    _/_/    _/_/_/       
                                                      

GenUI-Compiler :: [2025-01-20 06:17:31] :: INFO >>> 🔧 Log Service is starting... Log entries will be available after the `app event::Change` occurs!
    Creating binary (application) `src_gen_0` package
note: see more `Cargo.toml` keys and their definitions at <https://doc.rust-lang.org/cargo/reference/manifest.html> 
GenUI-Compiler :: [2025-01-20 06:17:31] :: INFO >>> 🔧 Watcher Service started successfully! Ract is watching: `/User/GenUI/Source/Project/Path`
```
## Learning GenUI
For more information, please see: [GenUI Official Documentation](https://privoce.github.io/GenUI.github.io/).
## Collaboration and Feedback
> [!IMPORTANT]
> GenUI is currently in the early stages of development, with many features still being planned and implemented. We welcome community feedback and collaboration! If you have any suggestions for the framework, need to report an issue, or would like to add features, please contact us through the following channels:

- **GitHub**: [https://github.com/Privoce/GenUI](https://github.com/Privoce/GenUI)
- **Discord**: [https://discord.gg/jVEJDhE75Y](https://discord.gg/jVEJDhE75Y)
- **Email**: [syf20020816@outlook.com](mailto:syf20020816@outlook.com)
- **Collaboration Email**: [han@privoce.com](mailto:han@privoce.com)


Thank you for your support, and we look forward to building a better GenUI with you!
