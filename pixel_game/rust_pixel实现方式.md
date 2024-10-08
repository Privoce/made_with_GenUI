# Rust_pixel 与 Makepad交互实现方式

使用 JavaScript 作为桥梁让 `Makepad` 和 `rust_pixel` 两个独立构建为 `WASM` 的部分通信，这样可以避免跨线程的数据传输复杂性，并且能够灵活地处理 UI 和渲染的交互。

在这个设计中，可以通过 `WASM` 模块和 `JS` 之间的通信机制来处理 `Makepad` 的 UI 更新和 `rust_pixel` 的游戏逻辑。具体思路如下：

1. **独立构建 `WASM` 模块**：
   - 将 `Makepad` 和 `rust_pixel` 分别编译为独立的 `WASM` 模块。
   - 使用 `wasm-bindgen` 或者 `stdweb` 来与 JavaScript 进行交互。

2. **JavaScript 作为控制器**：
   - 在 JavaScript 中加载两个 `WASM` 模块。
   - 使用 `JS` 控制 `Makepad` 的 UI 逻辑，响应用户的输入。
   - 使用 `JS` 作为中介，将用户输入或 UI 事件传递给 `rust_pixel` 来更新游戏逻辑。

3. **事件传递与定时更新**：
   - 可以使用 JavaScript 的 `setInterval` 或 `requestAnimationFrame` 实现游戏主循环来处理渲染时机。
   - JavaScript 捕获用户输入事件（如键盘、鼠标等），并将事件信息通过 `WASM` 接口传递给 `rust_pixel`，同时控制 `Makepad` UI 组件更新。

4. **渲染与数据共享**：
   - `Makepad` 和 `rust_pixel` 分别处理自己的渲染任务，并通过 `JS` 传递必要的共享数据（例如游戏状态或界面数据）。
   - `JS` 中可以实现一个统一的调度器，管理 `Makepad` 和 `rust_pixel` 的运行时，并通过接口更新它们的状态。

这种架构可以保持两个引擎的独立性，同时利用 `JS` 简化它们之间的交互逻辑。