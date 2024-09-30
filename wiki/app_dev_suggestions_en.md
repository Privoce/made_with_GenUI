# Suggestions for Makepad App Development

Makepad's execution mechanism functions correctly, but during page or app development, certain inefficient practices can significantly reduce response speed and increase performance overhead. The following are issues to avoid and their corresponding solutions:

## 1. Inefficient Data Rendering Placement
Placing network requests or synchronous blocking code inside `after_apply` can lead to performance issues. Since the `after_apply` method of child components executes before `start_up`, and there is no use of the `visible` property to prevent execution when the page is not displayed, this causes two major problems:
- **Slow App Startup**: Blocking operations delay the startup process.
- **Page Switching Blockage**: Each time a page is switched, the app is delayed by synchronous operations.

**Solution**: Avoid placing blocking operations inside lifecycle methods, especially `after_apply`. Instead, use a more flexible approach to delay or asynchronously execute these operations at the appropriate time.

## 2. Inefficient `draw_walk` Usage
Executing code that updates the page within `draw_walk` leads to performance problems. The `draw_walk` method is frequently refreshed, and without restricting the update logic inside, it can trigger constant page redraws, increasing computational and rendering overhead.

**Solution**: Limit the frequency of updates in `draw_walk`. Only trigger updates when necessary by using markers or state variables to control when a page needs to be redrawn.

## 3. Inefficient `handle_event` Usage
Like `draw_walk`, event handling logic also needs restrictions. After processing an event, the function should return promptly without continuing unnecessary execution, as this can lead to event pollution and unexpected errors.

**Solution**: Ensure that the logic within `handle_event` is simple and clean, and returns quickly after processing, reducing complexity in event handling.

## 4. Inefficient Data Synchronization
There are two common problems related to data synchronization:
1. **Parent-Child Data Passing**: Synchronizing data through parent-child component data passing can be problematic because the parent cannot always know whether the child needs to update its data, leading to unintended state changes.
2. **Persistent Layer Data Synchronization**: Although synchronizing data through persistent storage is more stable, frequent read/write operations can introduce performance bottlenecks (e.g., through locking mechanisms).

**Solution**: For parent-child data passing, use an event-driven approach where the child requests an update from the parent, and the parent returns the updated data, reducing unnecessary state synchronization. When using persistent storage, be mindful of optimizing the read/write frequency.

## Summary of Solutions

For issues 1, 2, and 3, I adopted **lifecycle management** as the solution:
- When encapsulating pages, I prefer not to pass events back to the main App page. Instead, I place event and data handling within `handle_event` or `draw_walk`, but since both are frequently refreshed or invoked, it is necessary to limit their execution frequency.
- By using lifecycle management, I control the executor, which in turn controls the function execution. The result of the function then updates the lifecycle state. This approach has significantly improved page response times.

For issue 4, I used `lazy_static` for data management:
- While `lazy_static` might introduce synchronization issues similar to persistent storage, it is more efficient compared to frequent parent-child data passing and file read/write operations, reducing the complexity and overhead of synchronization.

---

### Lifetime Example

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lifetime {
    Init,
    InProcess,
    Destroy,
}

impl Default for Lifetime {
    fn default() -> Self {
        Lifetime::Init
    }
}

impl Lifetime {
    pub fn next(&self) -> Self {
        match self {
            Lifetime::Init => Lifetime::InProcess,
            Lifetime::InProcess => Lifetime::Destroy,
            Lifetime::Destroy => Lifetime::Init,
        }
    }
    pub fn init(&self) -> LifetimeExecutor {
        LifetimeExecutor {
            current: *self,
            target: Lifetime::Init,
        }
    }
    pub fn in_process(&self) -> LifetimeExecutor {
        LifetimeExecutor {
            current: *self,
            target: Lifetime::InProcess,
        }
    }
    pub fn destroy(&self) -> LifetimeExecutor {
        LifetimeExecutor {
            current: *self,
            target: Lifetime::Destroy,
        }
    }
}

pub trait Executor {
    type Item;
    fn execute<F>(&self, f: F) -> Option<Self::Item>
    where
        F: FnOnce();
}

pub struct LifetimeExecutor {
    current: Lifetime,
    target: Lifetime,
}

impl Executor for LifetimeExecutor {
    type Item = Lifetime;

    fn execute<F>(&self, f: F) -> Option<Self::Item>
    where
        F: FnOnce(),
    {
        if self.current == self.target {
            f();
            Some(self.current.next())
        } else {
            None
        }
    }
}
```

### Usage of Lifetime

```rust
// ...omit import
use makepad_widgets::*;

use crate::utils::{
    lifetime::{Executor, Lifetime},
    APP_STATE,
};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    SiginPage = {{SiginPage}}{
        // omit page code
    }
}

#[derive(Live, Widget)]
pub struct SiginPage {
    #[deref]
    pub super_widget: GCard,
    #[rust]
    pub lifetime: Lifetime,
}

impl LiveHook for SiginPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let _ = self.super_widget.after_apply(cx, apply, index, nodes);
        let _ = self.gselect(id!(region_select)).borrow_mut().map(|mut x| {
            x.options = vec![
                ("美国东部(俄亥俄) us-east-2", "us-east-2").into(),
                ("美国东部(弗吉尼亚州北部) us-east-1", "us-east-1").into(),
            ]
        });
    }
}

impl Widget for SiginPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.super_widget.draw_walk(cx, scope, walk);
        self.lifetime
            .init()
            .execute(|| self.get(cx))
            .map(|lifetime| {
                self.lifetime = lifetime;
            });

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));

        if self.gbutton(id!(auto_connect)).clicked(&actions).is_some() {
            // get state and call
            // check if the toolkit is available
            self.check(cx);
            self.get(cx);
            return;
        }

        if self.gbutton(id!(try_connect)).clicked(&actions).is_some() {
            self.update();
            return;
        }
        if self.gbutton(id!(back_setting)).clicked(&actions).is_some() {
            self.lifetime = Lifetime::Destroy;
            // todo nav to main page
            self.lifetime
                .destroy()
                .execute(|| {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        StackNavigationAction::NavigateTo(live_id!(root_view)),
                    )
                })
                .map(|lifetime| {
                    self.lifetime = lifetime;
                });
        }
    }
}

impl SiginPage {
    pub fn check(&mut self, cx: &mut Cx) {
        let mut state = APP_STATE.lock().unwrap();
        if !state.check_toolkit() {
            let _ = self.gcard(id!(download_btn)).borrow_mut().map(|mut x| {
                x.visible = true;
            });
            self.redraw(cx);
            self.glabel(id!(res_str))
                .set_text_and_redraw(cx, &state.msg);
        }
    }
    pub fn get(&mut self, cx: &mut Cx) {
        let state = APP_STATE.lock().unwrap();
        if state.check && state.login {
            self.ginput(id!(accsee_key_input))
                .borrow_mut()
                .map(|mut x| {
                    x.text = state.accsee_key.to_string();
                });
            self.ginput(id!(secret_key_input))
                .borrow_mut()
                .map(|mut x| {
                    x.text = state.secret_key.to_string();
                });
            self.gselect(id!(region_select)).borrow_mut().map(|mut x| {
                for (index, option) in x.options.clone().iter().enumerate() {
                    if option.value == state.region {
                        x.selected = index;
                    }
                }
            });
            self.gradio_group(id!(output_group))
                .borrow_mut()
                .map(|mut x| {
                    x.set_selected(
                        cx,
                        match state.output.as_str() {
                            "json" => 0,
                            "yaml" => 1,
                            "text" => 2,
                            _ => 0,
                        },
                    );
                });
            self.glabel(id!(res_str))
                .set_text_and_redraw(cx, &state.msg);
        }
    }
    pub fn update(&mut self) {
        let mut state = APP_STATE.lock().unwrap();
        self.ginput(id!(accsee_key_input)).borrow().map(|x| {
            state.accsee_key = x.text.to_string();
        });
        self.ginput(id!(secret_key_input)).borrow().map(|x| {
            state.secret_key = x.text.to_string();
        });
        self.gselect(id!(region_select)).borrow().map(|x| {
            state.region = x.options[x.selected].value.to_string();
        });
        self.gradio_group(id!(output_group)).borrow().map(|x| {
            state.output = ["json", "yaml", "text"][x.selected as usize].to_string();
        });
    }
}

```