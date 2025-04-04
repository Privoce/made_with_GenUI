use gen_components::{themes::*, utils::*, *};
use makepad_widgets::*;
live_design! { use link :: widgets :: * ; use link :: gen_components :: * ; use link :: shaders :: * ; pub MultiIf = { { MultiIf } } { spacing : 12.0 , fm8_tbj2_hv6_a2 = < GLabel > { text : "if1" , } em8_tbj2_hv62_l = < GLabel > { text : "if2" , } dm8_tbj2_hvc84 = < GLabel > { text : "if3" , } bm8_tbj2_hv3_af = < GButton > { slot : < GLabel > { text : "show if1" , } } em8_tbj2_hvcjg = < GButton > { slot : < GLabel > { text : "show if2" , } } } }
#[derive(Live, Widget)]
pub struct MultiIf {
    #[deref]
    pub deref_widget: GView,
    #[live]
    vis1: bool,
    #[live]
    vis2: bool,
    #[rust]
    twb_poll: TwoWayBindingPoll,
}
impl MultiIf {
    setter! { MultiIf { set_theme (theme : Themes) { | c , cx | { c . theme = theme ; c . render (cx) } } , set_background_color (color : String) { | c , _cx | { let color = hex_to_vec4 (& color) ? ; c . background_color . replace (color) ; c . draw_view . background_color = color ; Ok (()) } } , set_shadow_color (color : String) { | c , _cx | { let color = hex_to_vec4 (& color) ? ; c . shadow_color . replace (color) ; c . draw_view . shadow_color = color ; Ok (()) } } , set_hover_color (color : String) { | c , _cx | { let color = hex_to_vec4 (& color) ? ; c . hover_color . replace (color) ; c . draw_view . hover_color = color ; Ok (()) } } , set_focus_color (color : String) { | c , _cx | { let color = hex_to_vec4 (& color) ? ; c . focus_color . replace (color) ; c . draw_view . focus_color = color ; Ok (()) } } , set_border_color (color : String) { | c , _cx | { let color = hex_to_vec4 (& color) ? ; c . border_color . replace (color) ; c . draw_view . border_color = color ; Ok (()) } } , set_border_width (width : f32) { | c , _cx | { c . border_width = width ; c . draw_view . border_width = width ; Ok (()) } } , set_border_radius (radius : f32) { | c , _cx | { c . border_radius = radius ; c . draw_view . border_radius = radius ; Ok (()) } } , set_shadow_offset (offset : Vec2) { | c , _cx | { c . shadow_offset = offset ; c . draw_view . shadow_offset = offset ; Ok (()) } } , set_spread_radius (radius : f32) { | c , _cx | { c . spread_radius = radius ; c . draw_view . spread_radius = radius ; Ok (()) } } , set_blur_radius (radius : f32) { | c , _cx | { c . blur_radius = radius ; c . draw_view . blur_radius = radius ; Ok (()) } } , set_background_visible (visible : bool) { | c , _cx | { c . background_visible = visible ; c . draw_view . background_visible = visible . to_f32 () ; Ok (()) } } , set_visible (visible : bool) { | c , _cx | { c . visible = visible ; Ok (()) } } , set_cursor (cursor : MouseCursor) { | c , _cx | { c . cursor = Some (cursor) ; Ok (()) } } , set_grab_key_focus (grab : bool) { | c , _cx | { c . grab_key_focus = grab ; Ok (()) } } , set_block_signal_event (block : bool) { | c , _cx | { c . block_signal_event = block ; Ok (()) } } , set_abs_pos (pos : Option < DVec2 >) { | c , _cx | { c . walk . abs_pos = pos ; Ok (()) } } , set_margin (margin : Margin) { | c , _cx | { c . walk . margin = margin ; Ok (()) } } , set_height (height : Size) { | c , _cx | { c . walk . height = height ; Ok (()) } } , set_width (width : Size) { | c , _cx | { c . walk . width = width ; Ok (()) } } , set_scroll (scroll : DVec2) { | c , _cx | { c . layout . scroll = scroll ; Ok (()) } } , set_clip_x (clip : bool) { | c , _cx | { c . layout . clip_x = clip ; Ok (()) } } , set_clip_y (clip : bool) { | c , _cx | { c . layout . clip_y = clip ; Ok (()) } } , set_padding (padding : Padding) { | c , _cx | { c . layout . padding = padding ; Ok (()) } } , set_align (align : Align) { | c , _cx | { c . layout . align = align ; Ok (()) } } , set_flow (flow : Flow) { | c , _cx | { c . layout . flow = flow ; Ok (()) } } , set_spacing (spacing : f64) { | c , _cx | { c . layout . spacing = spacing ; Ok (()) } } , set_dpi_factor (factor : f64) { | c , _cx | { c . dpi_factor . replace (factor) ; Ok (()) } } , set_optimize (optimize : ViewOptimize) { | c , _cx | { c . optimize = optimize ; Ok (()) } } , set_capture_overload (overload : bool) { | c , _cx | { c . capture_overload = overload ; Ok (()) } } , set_event_key (event_key : bool) { | c , _cx | { c . event_key = event_key ; Ok (()) } } } }
    getter! { MultiIf { get_theme (Themes) { | c | { c . theme } } , get_background_color (String) { | c | { vec4_to_hex (& c . draw_view . background_color) } } , get_shadow_color (String) { | c | { vec4_to_hex (& c . draw_view . shadow_color) } } , get_hover_color (String) { | c | { vec4_to_hex (& c . draw_view . hover_color) } } , get_focus_color (String) { | c | { vec4_to_hex (& c . draw_view . focus_color) } } , get_border_color (String) { | c | { vec4_to_hex (& c . draw_view . border_color) } } , get_border_width (f32) { | c | { c . draw_view . border_width } } , get_border_radius (f32) { | c | { c . draw_view . border_radius } } , get_shadow_offset (Vec2) { | c | { c . draw_view . shadow_offset } } , get_spread_radius (f32) { | c | { c . draw_view . spread_radius } } , get_blur_radius (f32) { | c | { c . draw_view . blur_radius } } , get_background_visible (bool) { | c | { c . draw_view . background_visible . to_bool () } } , get_visible (bool) { | c | { c . visible } } , get_cursor (MouseCursor) { | c | { c . cursor . unwrap_or_default () } } , get_grab_key_focus (bool) { | c | { c . grab_key_focus } } , get_block_signal_event (bool) { | c | { c . block_signal_event } } , get_abs_pos (Option < DVec2 >) { | c | { c . walk . abs_pos . clone () } } , get_margin (Margin) { | c | { c . walk . margin } } , get_height (Size) { | c | { c . walk . height } } , get_width (Size) { | c | { c . walk . width } } , get_scroll (DVec2) { | c | { c . layout . scroll } } , get_clip_x (bool) { | c | { c . layout . clip_x } } , get_clip_y (bool) { | c | { c . layout . clip_y } } , get_padding (Padding) { | c | { c . layout . padding } } , get_align (Align) { | c | { c . layout . align } } , get_flow (Flow) { | c | { c . layout . flow } } , get_spacing (f64) { | c | { c . layout . spacing } } , get_dpi_factor (f64) { | c | { c . dpi_factor . unwrap_or_default () } } , get_optimize (ViewOptimize) { | c | { c . optimize } } , get_capture_overload (bool) { | c | { c . capture_overload } } , get_event_key (bool) { | c | { c . event_key } } } }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
    fn get_vis1(&self) -> bool {
        self.vis1.clone()
    }
    fn set_vis1(&mut self, cx: &mut Cx, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        let widget = self.glabel(id!(fm8_tbj2_hv6_a2));
        widget.set_visible(cx, value.clone())?;
        self.vis1 = value.clone();
        self.update_un_vis(cx)?;
        Ok(())
    }
    fn get_vis2(&self) -> bool {
        self.vis2.clone()
    }
    fn set_vis2(&mut self, cx: &mut Cx, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        let widget = self.glabel(id!(em8_tbj2_hv62_l));
        widget.set_visible(cx, value.clone())?;
        self.vis2 = value.clone();
        Ok(())
    }
    fn update_un_vis(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        let new_value = self.un_vis(cx);
        let widget = self.glabel(id!(dm8_tbj2_hvc84));
        widget.set_visible(cx, new_value)?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn show_vis1(&mut self, cx: &mut Cx) {
        self.set_vis1(cx, !self.vis1);
        self.redraw(cx);
    }
    #[allow(unused_variables)]
    fn show_vis2(&mut self, cx: &mut Cx) {
        self.set_vis2(cx, !self.vis2);
        self.redraw(cx);
    }
    fn un_vis(&self, cx: &mut Cx) -> bool {
        return !self.vis1;
    }
}
#[allow(unused)]
impl MultiIfRef {
    ref_getter_setter! { get_theme , set_theme -> Themes , get_background_color , set_background_color -> String , get_shadow_color , set_shadow_color -> String , get_hover_color , set_hover_color -> String , get_focus_color , set_focus_color -> String , get_border_color , set_border_color -> String , get_border_width , set_border_width -> f32 , get_border_radius , set_border_radius -> f32 , get_shadow_offset , set_shadow_offset -> Vec2 , get_spread_radius , set_spread_radius -> f32 , get_blur_radius , set_blur_radius -> f32 , get_background_visible , set_background_visible -> bool , get_visible , set_visible -> bool , get_cursor , set_cursor -> MouseCursor , get_grab_key_focus , set_grab_key_focus -> bool , get_block_signal_event , set_block_signal_event -> bool , get_abs_pos , set_abs_pos -> Option < DVec2 > , get_margin , set_margin -> Margin , get_height , set_height -> Size , get_width , set_width -> Size , get_scroll , set_scroll -> DVec2 , get_clip_x , set_clip_x -> bool , get_clip_y , set_clip_y -> bool , get_padding , set_padding -> Padding , get_align , set_align -> Align , get_flow , set_flow -> Flow , get_spacing , set_spacing -> f64 , get_dpi_factor , set_dpi_factor -> f64 , get_optimize , set_optimize -> ViewOptimize , get_capture_overload , set_capture_overload -> bool , get_event_key , set_event_key -> bool }
    ref_render!();
    ref_redraw_mut!();
    pub fn get_vis1(&self) -> bool {
        self.getter(|c_ref| c_ref.vis1.clone())
    }
    pub fn set_vis1(&self, cx: &mut Cx, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.setter(cx, |c_ref, cx| c_ref.set_vis1(cx, value))
    }
    pub fn get_vis2(&self) -> bool {
        self.getter(|c_ref| c_ref.vis2.clone())
    }
    pub fn set_vis2(&self, cx: &mut Cx, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.setter(cx, |c_ref, cx| c_ref.set_vis2(cx, value))
    }
    fn setter<F>(&self, cx: &mut Cx, f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(
            &mut std::cell::RefMut<'_, MultiIf>,
            &mut Cx,
        ) -> Result<(), Box<dyn std::error::Error>>,
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            return f(&mut c_ref, cx);
        }
        Ok(())
    }
    fn getter<T, F>(&self, f: F) -> T
    where
        F: Fn(&std::cell::Ref<'_, MultiIf>) -> T,
        T: Default,
    {
        if let Some(c_ref) = self.borrow() {
            f(&c_ref)
        } else {
            T::default()
        }
    }
}
impl Widget for MultiIf {
    #[allow(unused_variables)]
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    #[allow(unused_variables)]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let fm8_tbj2_hv6_a2 = self.glabel(id!(fm8_tbj2_hv6_a2));
        let em8_tbj2_hv62_l = self.glabel(id!(em8_tbj2_hv62_l));
        let em8_tbj2_hvcjg = self.gbutton(id!(em8_tbj2_hvcjg));
        let bm8_tbj2_hv3_af = self.gbutton(id!(bm8_tbj2_hv3_af));
        if let Some(_) = bm8_tbj2_hv3_af.clicked(&actions) {
            self.show_vis1(cx);
        }
        if let Some(_) = em8_tbj2_hvcjg.clicked(&actions) {
            self.show_vis2(cx);
        }
    }
    #[allow(unused_variables)]
    fn is_visible(&self) -> bool {
        self.visible
    }
}
impl LiveHook for MultiIf {
    #[allow(unused_variables)]
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        let deref_prop = MultiIfDeref::default();
        self.set_vis1(cx, deref_prop.vis1);
        self.set_vis2(cx, deref_prop.vis2);
    }
    #[allow(unused_variables)]
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        let c_ptr = self as *mut MultiIf;
        self.twb_poll.on_vis1_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).vis1 = new_state;
        }));
        self.twb_poll.on_vis2_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).vis2 = new_state;
        }));
    }
}
#[derive(Default)]
struct TwoWayBindingPoll {
    pub on_vis1_change: Option<Box<dyn Fn(&mut Cx, bool)>>,
    pub on_vis2_change: Option<Box<dyn Fn(&mut Cx, bool)>>,
}
impl Default for MultiIfDeref {
    fn default() -> Self {
        Self {
            vis1: true,
            vis2: false,
        }
    }
}
pub struct MultiIfDeref {
    vis1: bool,
    vis2: bool,
}
