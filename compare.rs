use gen_components::*;
use makepad_widgets::*;
live_design! { 
    use link :: widgets :: * ; 
    use link :: gen_components :: * ; 
    use link :: shaders :: * ; 
    pub EasyLabel = { { EasyLabel } } { 
        item_ptr0 : < GLabel > { font_size : 16.0 , color : vec4 (1.0 , 0.4392157 , 0.2627451 , 1.0) , } , 
        item_ptr1 : < GView > { flow : Down , } , 
        height : 100.0 , 
        my_view = < GView > { height : Fit , align : { x : 0.5 , y : 0.0 } , } 
    } 
}
#[derive(Live, Widget)]
pub struct EasyLabel {
    #[deref]
    pub deref_widget: GView,
    #[live]
    lbs: Vec<Vec<String>>,
    #[live]
    item_ptr0: Option<LivePtr>,
    #[live]
    item_ptr1: Option<LivePtr>,
}
impl EasyLabel {
    fn get_lbs(&self) -> Vec<Vec<String>> {
        self.lbs.clone()
    }
    fn set_lbs(&mut self, cx: &mut Cx, value: Vec<Vec<String>>) -> () {
        self.sugar_for_lbs(cx, &value);
        self.lbs = value.clone();
    }
    fn sugar_for_lbs(&mut self, cx: &mut Cx, value: &Vec<Vec<String>>) -> () {
        let len_lbs = self.lbs.len();
        if let Some(mut father) = self.gview(id!(my_view)).borrow_mut() {
            if len_lbs > 0 && father.children.len() > 0usize {
                for _ in 0usize..(0usize + len_lbs) {
                    father.children.remove(0usize);
                }
            }
            for (index, item) in value.iter().enumerate() {
                let item = item.clone();
                let widget_ref = WidgetRef::new_from_ptr(cx, self.item_ptr1);
                let widget_target = widget_ref.as_gview();
                let len_item = self.lbs.get(index).map_or(0, |v| v.len());
                if let Some(mut father) = widget_target.borrow_mut() {
                    if len_item > 0 && father.children.len() > 0usize {
                        for _ in 0usize..(0usize + len_item) {
                            father.children.remove(0usize);
                        }
                    }
                    for (index1, item1) in item.iter().enumerate() {
                        let item1 = item1.clone();
                        let widget_ref = WidgetRef::new_from_ptr(cx, self.item_ptr0);
                        let widget_target = widget_ref.as_glabel();
                        widget_target.set_text(cx, item1);
                        father
                            .children
                            .insert(0usize + index1, (LiveId(index1 as u64), widget_ref));
                    }
                }
                father
                    .children
                    .insert(0usize + index, (LiveId(index as u64), widget_ref));
            }
            father.redraw(cx);
        }
    }
}
#[allow(unused)]
impl EasyLabelRef {
    pub fn get_lbs(&self) -> Vec<Vec<String>> {
        self.getter(|c_ref| c_ref.lbs.clone())
    }
    pub fn set_lbs(&self, cx: &mut Cx, value: Vec<Vec<String>>) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_lbs(cx, value);
        });
    }
    fn setter<F>(&self, cx: &mut Cx, f: F) -> ()
    where
        F: FnOnce(&mut std::cell::RefMut<'_, EasyLabel>, &mut Cx),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            f(&mut c_ref, cx);
        }
    }
    fn getter<T, F>(&self, f: F) -> T
    where
        F: Fn(&std::cell::Ref<'_, EasyLabel>) -> T,
        T: Default,
    {
        if let Some(c_ref) = self.borrow() {
            f(&c_ref)
        } else {
            T::default()
        }
    }
}
impl Widget for EasyLabel {
    #[allow(unused_variables)]
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    #[allow(unused_variables)]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
    #[allow(unused_variables)]
    fn is_visible(&self) -> bool {
        self.visible
    }
}
impl LiveHook for EasyLabel {
    #[allow(unused_variables)]
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        let deref_prop = EasyLabelDeref::default();
        self.set_lbs(cx, deref_prop.lbs);
    }
}
impl Default for EasyLabelDeref {
    fn default() -> Self {
        Self {
            lbs: vec![
                vec!["Hello1".to_string()],
                vec!["Hello2".to_string(), "World".to_string()],
            ],
        }
    }
}
pub struct EasyLabelDeref {
    lbs: Vec<Vec<String>>,
}
