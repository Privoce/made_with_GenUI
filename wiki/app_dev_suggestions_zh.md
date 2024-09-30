# Suggestions for Makepad App development

Makepad 的执行机制上没有问题，但在实际开发页面或 App 时，一些不合理的操作可能会显著降低响应速度并带来性能开销。以下是需要避免的几种情况及对应的解决方案：

## 1. 不合理的数据渲染位置
将需要进行网络请求或同步阻塞的代码放置在 `after_apply` 中会引发性能问题。因为子组件的 `after_apply` 方法优先于 `start_up` 执行，且没有使用 `visible` 属性阻止页面在未显示时执行，导致以下问题：
- **App 启动过慢**：阻塞操作延迟了启动过程。
- **页面切换阻塞**：每次页面切换都被同步操作拖慢。

**解决方案**：避免将阻塞操作放置在生命周期方法中，尤其是 `after_apply` 阶段，而是应在适当时机通过更灵活的控制来延迟或异步执行这些操作。

## 2. 不合理的 `draw_walk`
在 `draw_walk` 中执行更新页面的代码会导致性能问题。`draw_walk` 方法经常被刷新调用，若不限制其中的更新逻辑，将导致频繁的页面重绘，增加大量的计算和渲染开销。

**解决方案**：应限制更新逻辑的执行频率，避免每次绘制时都触发不必要的更新。可以使用标记或状态变量来控制更新的发生，仅在必要时重新绘制页面。

## 3. 不合理的 `handle_event`
和 `draw_walk` 类似，事件处理逻辑也需要得到适当限制。处理事件后应尽快返回，不应继续执行不相关的逻辑，以避免事件污染和意外的错误行为。

**解决方案**：确保 `handle_event` 中的处理逻辑简洁明了，执行后及时返回，减少事件处理的复杂性。

## 4. 不合理的数据同步
常见的数据同步问题有两种：
1. **父子组件数据传递**：通过组件之间的数据传递进行同步，存在父组件无法确定子组件是否需要更新数据的问题，可能会导致意外的状态变更。
2. **持久化层数据同步**：虽然使用持久化层同步数据相对更稳妥，但它涉及频繁的读写操作，可能会引入性能瓶颈（如锁机制）。

**解决方案**：对于父子组件传递数据，推荐使用事件驱动方式，由子组件发出请求，父组件处理后返回更新，减少不必要的状态同步。使用持久化层时，注意优化读写操作的频率。

## 解决方案的总结

对于问题 1、2、3，我通过 **生命周期管理** 来优化：
- 在封装页面时，为避免将事件传递到 App 主页面，我通常将事件和数据处理放置在 `handle_event` 或 `draw_walk` 中，但由于这两个地方会频繁刷新或调用，所以必须限制执行频率。
- 使用生命周期管理，控制执行器的执行，执行器再控制具体函数的执行，最终函数的执行结果会反馈并更新生命周期状态。这一套方法显著提升了页面的响应速度。

对于问题 4，我采用了 `lazy_static` 进行数据管理：
- 虽然 `lazy_static` 可能引入类似持久化层的同步问题，但相较于频繁的父子组件数据传递和读写操作，`lazy_static` 更加高效，减少了同步的复杂性和开销。

---

## lifetime

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
            //back next
            Some(self.current.next())
        } else {
            None
        }
    }
}

```

## 使用Lifetime

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