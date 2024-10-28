```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use gen_components::components::view::GView;
use gen_macros::prop;
use makepad_widgets::*;
pub fn live_design(cx: &mut Cx) {
    let live_body = LiveBody {
        cargo_manifest_path: "/Users/shengyifei/projects/gen_ui/made_with_GenUI/macro_test"
            .to_string(),
        module_path: "test1".to_string(),
        file: "src/test1.rs".to_string().replace("\\", "/"),
        line: 5u32 as usize,
        column: 1u32 as usize,
        live_type_infos: {
            let mut v = Vec::new();
            Tw::live_design_with(cx);
            v.push(Tw::live_type_info(cx));
            v
        },
        code: "import makepad_widgets ::base ::*;import makepad_widgets ::theme_desktop_dark ::*;import gen_components ::components ::*;Tw ={{dummy}}{height :Fill ,width :Fill ,flow :Down ,border_radius :0.0 ,background_visible :true ,background_color :#FFFFFF ,align :{x :0.5 ,y :0.4},spacing :24.0 ,}"
            .to_string(),
    };
    cx.register_live_body(live_body);
}
pub struct Tw {
    #[deref]
    pub deref_widget: GView,
}
impl std::ops::Deref for Tw {
    type Target = GView;
    fn deref(&self) -> &Self::Target {
        &self.deref_widget
    }
}
impl std::ops::DerefMut for Tw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_widget
    }
}
impl LiveApplyReset for Tw {
    fn apply_reset(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        start_index: usize,
        nodes: &[LiveNode],
    ) {
        self.deref_widget.apply_reset(cx, apply, start_index, nodes);
        if !nodes[start_index].value.is_structy_type() {
            return;
        }
        let mut index = start_index + 1;
        while !nodes[index].is_close() {
            if nodes[index].origin.has_prop_type(LivePropType::Field) {
                match nodes[index].id {
                    _ => {}
                }
            }
            index = nodes.skip_node(index);
        }
    }
}
impl LiveApplyValue for Tw {
    fn apply_value(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        if nodes[index].origin.has_prop_type(LivePropType::Field) {
            match nodes[index].id {
                _ => self.deref_widget.apply_value(cx, apply, index, nodes),
            }
        } else {
            self.deref_widget.apply_value(cx, apply, index, nodes)
        }
    }
}
impl LiveHookDeref for Tw {
    fn deref_before_apply(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) {
        <Self as LiveHook>::before_apply(self, cx, apply, index, nodes);
        self.deref_widget.deref_before_apply(cx, apply, index, nodes);
    }
    fn deref_after_apply(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) {
        <Self as LiveHook>::after_apply(self, cx, apply, index, nodes);
        self.deref_widget.deref_after_apply(cx, apply, index, nodes);
        <Self as LiveHook>::after_apply_from(self, cx, apply);
    }
}
impl LiveApply for Tw {
    fn apply(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        start_index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        self.deref_before_apply(cx, apply, start_index, nodes);
        let index = if let Some(index) = <Self as LiveHook>::skip_apply(
            self,
            cx,
            apply,
            start_index,
            nodes,
        ) {
            index
        } else {
            let struct_id = LiveId(17674443142055714164u64);
            if !nodes[start_index].value.is_structy_type() {
                cx.apply_error_wrong_type_for_struct(
                    LiveErrorOrigin {
                        filename: "src/test1.rs".to_string(),
                        line: 32u32 as usize,
                    },
                    start_index,
                    nodes,
                    struct_id,
                );
                <Self as LiveHook>::after_apply(self, cx, apply, start_index, nodes);
                return nodes.skip_node(start_index);
            }
            let mut index = start_index + 1;
            loop {
                if nodes[index].value.is_close() {
                    index += 1;
                    break;
                }
                index = self.apply_value(cx, apply, index, nodes);
            }
            index
        };
        if apply.from.should_apply_reset() {
            <Self as LiveApplyReset>::apply_reset(self, cx, apply, start_index, nodes);
        }
        self.deref_after_apply(cx, apply, start_index, nodes);
        return index;
    }
}
impl LiveNew for Tw {
    fn live_type_info(cx: &mut Cx) -> LiveTypeInfo {
        let mut fields = Vec::new();
        fields
            .push(LiveTypeField {
                id: LiveId::from_str_with_lut("deref_widget").unwrap(),
                live_type_info: <GView as LiveNew>::live_type_info(cx),
                live_field_kind: LiveFieldKind::Deref,
            });
        LiveTypeInfo {
            module_id: LiveModuleId::from_str(&"test1").unwrap(),
            live_type: LiveType::of::<Self>(),
            live_ignore: false,
            fields,
            type_name: LiveId::from_str_with_lut("Tw").unwrap(),
        }
    }
    fn live_design_with(cx: &mut Cx) {
        <Self as LiveRegister>::live_register(cx);
        <GView as LiveNew>::live_design_with(cx);
    }
    fn new(cx: &mut Cx) -> Self {
        let mut ret = Self {
            deref_widget: LiveNew::new(cx),
        };
        <Self as LiveHook>::after_new_before_apply(&mut ret, cx);
        ret
    }
}
impl WidgetNode for Tw {
    fn area(&self) -> Area {
        self.deref_widget.area()
    }
    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.deref_widget.walk(cx)
    }
    fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx)
    }
    fn find_widgets(
        &self,
        path: &[LiveId],
        cached: WidgetCache,
        results: &mut WidgetSet,
    ) {
        self.deref_widget.find_widgets(path, cached, results)
    }
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        self.deref_widget.uid_to_widget(uid)
    }
}
impl LiveRegister for Tw {
    fn live_register(cx: &mut Cx) {
        {
            struct Factory();
            impl WidgetFactory for Factory {
                fn new(&self, cx: &mut Cx) -> Box<dyn Widget> {
                    Box::new(<Tw>::new(cx))
                }
            }
            let module_id = LiveModuleId::from_str(&"test1").unwrap();
            if let Some((reg, _)) = cx
                .live_registry
                .borrow()
                .components
                .get_or_create::<WidgetRegistry>()
                .map
                .get(&LiveType::of::<Tw>())
            {
                if reg.module_id != module_id {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Component already registered {0} {1}",
                                "Tw",
                                reg.module_id,
                            ),
                        );
                    };
                }
            }
            cx.live_registry
                .borrow()
                .components
                .get_or_create::<WidgetRegistry>()
                .map
                .insert(
                    LiveType::of::<Tw>(),
                    (
                        LiveComponentInfo {
                            name: LiveId::from_str_with_lut("Tw").unwrap(),
                            module_id,
                        },
                        Box::new(Factory()),
                    ),
                );
        };
    }
}
pub struct TwRef(WidgetRef);
#[automatically_derived]
impl ::core::clone::Clone for TwRef {
    #[inline]
    fn clone(&self) -> TwRef {
        TwRef(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TwRef {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "TwRef", &&self.0)
    }
}
impl std::ops::Deref for TwRef {
    type Target = WidgetRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for TwRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TwRef {
    pub fn has_widget(&self, widget: &WidgetRef) -> TwRef {
        if self.0 == *widget {
            TwRef(widget.clone())
        } else {
            TwRef(WidgetRef::default())
        }
    }
    pub fn borrow(&self) -> Option<std::cell::Ref<'_, Tw>> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> Option<std::cell::RefMut<'_, Tw>> {
        self.0.borrow_mut()
    }
    pub fn borrow_if_eq(&self, widget: &WidgetRef) -> Option<std::cell::Ref<'_, Tw>> {
        if self.0 == *widget { self.0.borrow() } else { None }
    }
    pub fn borrow_mut_if_eq(
        &self,
        widget: &WidgetRef,
    ) -> Option<std::cell::RefMut<'_, Tw>> {
        if self.0 == *widget { self.0.borrow_mut() } else { None }
    }
}
pub trait TwWidgetRefExt {
    fn tw(&self, path: &[LiveId]) -> TwRef;
    fn as_tw(&self) -> TwRef;
}
pub trait TwWidgetActionsExt {
    fn tw(&self, path: &[LiveId]) -> TwRef;
}
impl TwWidgetRefExt for WidgetRef {
    fn tw(&self, path: &[LiveId]) -> TwRef {
        TwRef(self.widget(path))
    }
    fn as_tw(&self) -> TwRef {
        TwRef(self.clone())
    }
}
impl TwWidgetActionsExt for Actions {
    fn tw(&self, path: &[LiveId]) -> TwRef {
        TwRef(self.widget(path))
    }
}
impl Default for TwRef {
    fn default() -> Self {
        Self(Default::default())
    }
}
pub trait TwWidgetExt {
    fn tw(&self, path: &[LiveId]) -> TwRef;
}
impl<T> TwWidgetExt for T
where
    T: Widget,
{
    fn tw(&self, path: &[LiveId]) -> TwRef {
        TwRef(self.widget(path))
    }
}
pub struct TwSet(WidgetSet);
#[automatically_derived]
impl ::core::clone::Clone for TwSet {
    #[inline]
    fn clone(&self) -> TwSet {
        TwSet(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TwSet {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "TwSet", &&self.0)
    }
}
impl std::ops::Deref for TwSet {
    type Target = WidgetSet;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for TwSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
pub trait TwSetWidgetSetExt {
    fn tw_set(&self, paths: &[&[LiveId]]) -> TwSet;
    fn as_tw_set(&self) -> TwSet;
}
impl TwSet {
    pub fn has_widget(&self, widget: &WidgetRef) -> TwRef {
        if self.contains(widget) {
            TwRef(widget.clone())
        } else {
            TwRef(WidgetRef::default())
        }
    }
}
impl TwSetWidgetSetExt for WidgetSet {
    fn tw_set(&self, paths: &[&[LiveId]]) -> TwSet {
        TwSet(self.widgets(paths))
    }
    fn as_tw_set(&self) -> TwSet {
        TwSet(self.clone())
    }
}
pub trait TwSetWidgetRefExt {
    fn tw_set(&self, paths: &[&[LiveId]]) -> TwSet;
}
impl TwSetWidgetRefExt for WidgetRef {
    fn tw_set(&self, paths: &[&[LiveId]]) -> TwSet {
        TwSet(self.widgets(paths))
    }
}
pub trait TwSetWidgetExt {
    fn tw_set(&mut self, paths: &[&[LiveId]]) -> TwSet;
}
impl<T> TwSetWidgetExt for T
where
    T: Widget,
{
    fn tw_set(&mut self, paths: &[&[LiveId]]) -> TwSet {
        TwSet(self.widgets(paths))
    }
}
impl TwSet {
    pub fn iter(&self) -> TwSetIterator {
        TwSetIterator {
            iter: self.0.iter(),
        }
    }
}
pub struct TwSetIterator<'a> {
    iter: WidgetSetIterator<'a>,
}
impl<'a> Iterator for TwSetIterator<'a> {
    type Item = TwRef;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.iter.next() {
            return Some(TwRef(next.clone()));
        }
        None
    }
}
impl LiveHook for Tw {
    fn before_apply(
        &mut self,
        cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        match "before" {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src/test1.rs",
                            45u32,
                            9u32,
                            "\"before\"",
                            &tmp,
                        ),
                    );
                };
                tmp
            }
        };
    }
    fn after_apply(
        &mut self,
        _cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        match "after apply" {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src/test1.rs",
                            55u32,
                            9u32,
                            "\"after apply\"",
                            &tmp,
                        ),
                    );
                };
                tmp
            }
        };
    }
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        match "new from doc" {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src/test1.rs",
                            58u32,
                            9u32,
                            "\"new from doc\"",
                            &tmp,
                        ),
                    );
                };
                tmp
            }
        };
    }
    fn after_update_from_doc(&mut self, _cx: &mut Cx) {
        match "upadte from doc" {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src/test1.rs",
                            62u32,
                            9u32,
                            "\"upadte from doc\"",
                            &tmp,
                        ),
                    );
                };
                tmp
            }
        };
    }
    fn after_apply_from_doc(&mut self, _cx: &mut Cx) {
        match "apply from doc" {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src/test1.rs",
                            66u32,
                            9u32,
                            "\"apply from doc\"",
                            &tmp,
                        ),
                    );
                };
                tmp
            }
        };
    }
    fn after_new_before_apply(&mut self, _cx: &mut Cx) {
        match "new before apply" {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src/test1.rs",
                            70u32,
                            9u32,
                            "\"new before apply\"",
                            &tmp,
                        ),
                    );
                };
                tmp
            }
        };
    }
}
impl Widget for Tw {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        match "draw walk" {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src/test1.rs",
                            77u32,
                            9u32,
                            "\"draw walk\"",
                            &tmp,
                        ),
                    );
                };
                tmp
            }
        };
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event.hits(cx, self.area()) {
            Hit::FingerUp(_) => {
                match "finger up, do apply over" {
                    tmp => {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "[{0}:{1}:{2}] {3} = {4:#?}\n",
                                    "src/test1.rs",
                                    83u32,
                                    17u32,
                                    "\"finger up, do apply over\"",
                                    &tmp,
                                ),
                            );
                        };
                        tmp
                    }
                };
                self.apply_over(
                    cx,
                    &[
                        LiveNode {
                            origin: LiveNodeOrigin::empty(),
                            id: LiveId(0),
                            value: LiveValue::Object,
                        },
                        LiveNode {
                            origin: LiveNodeOrigin::field(),
                            id: LiveId(14599123999131152264u64),
                            value: LiveValue::Color(4278190335u32),
                        },
                        LiveNode {
                            origin: LiveNodeOrigin::empty(),
                            id: LiveId(0),
                            value: LiveValue::Close,
                        },
                    ],
                );
            }
            _ => {}
        }
    }
}
```