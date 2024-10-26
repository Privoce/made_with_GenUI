// use makepad_widgets::*;

// use crate::state::DATA;

// live_design! {
//     import makepad_widgets::base::*;
//     import makepad_widgets::theme_desktop_dark::*;
//     import gen_components::components::*;
//     import crate::page_a::*;

//     App = {{App}}{
//         root: <Root>{
//             main_window = <GWindow>{
//                 os_type: Linux,
//                 show_icon: false,
//                 window_bar = {
//                     window_title = {
//                         align: {x: 0.5},
//                     }
//                 },
//                 width: Fill,
//                 height: Fill,
//                 window: {inner_size: vec2(820, 820)},
//                 background_color: #161616,
//                 background_visible: true,
//                 clip_x: true,
//                 clip_y: true,
//                 body = <View>{
//                     spacing: 16.0,
//                     align: {x: 0.5, y: 0.5},
//                     img = <Image>{
//                         source: dep("crate://self/resource/logo.png")
//                         height: 400.0,
//                         width: 400.0,
//                     }
//                     lb_view = <View>{
//                         height: Fit,
//                         width: Fit,
//                         <Label>{
//                             text: "Hello World!!!",   
//                         }
//                     }
//                     // <Icon>{
//                     //     draw_icon: {
//                     //         svg_file: dep("crate://self/resource/home.svg"),
//                     //     }
//                     //     icon_walk: {width: 32.0, height: 32.0}
//                     // }
//                     // <Icon>{
//                     //     draw_icon: {
//                     //         svg_file: dep("crate://self/resource/home2.svg"),
//                     //     }
//                     //     icon_walk: {width: 32.0, height: 32.0}
//                     // }
//                     // <Icon>{
//                     //     draw_icon: {
//                     //         svg_file: dep("crate://self/resource/sofa2.svg"),
//                     //     }
//                     //     icon_walk: {width: 32.0, height: 32.0}
//                     // }
//                     // <Icon>{
//                     //     draw_icon: {
//                     //         svg_file: dep("crate://self/resource/aim.svg"),
//                     //     }
//                     //     icon_walk: {width: 32.0, height: 32.0}
//                     // }
                    
//                     // txt_view = <View>{
//                     //     height: Fit,
//                     //     width: Fit,
//                     //     <Label>{
//                     //         text: "Hello World",
//                     //     }
//                     // }
//                 }
//             }
//         }
//     }
// }

// #[derive(Live, LiveHook)]
// pub struct App {
//     #[live]
//     root: WidgetRef,
//     #[rust]
//     timer: Timer,
// }

// impl LiveRegister for App {
//     fn live_register(cx: &mut Cx) {
//         crate::makepad_widgets::live_design(cx);
//         crate::gen_components::live_design(cx);
//         crate::page_a::live_design(cx);
//     }
// }

// impl MatchEvent for App {
//     fn handle_startup(&mut self, cx: &mut Cx) {
//         // dbg!("start");
//         // let mut data = DATA.lock().unwrap();
//         // data.data1 = "Hello".to_string();
//         // data.data2 = 16;
//         let img = self.root.image(id!(img));
//         self.root.view(id!(lb_view)).borrow_mut().map(|mut x|{
//             let img_pos = img.area().rect(cx).pos;
            
//             x.apply_over(cx, live!{
//                 abs_pos: (img_pos)
//             });
//         });

//     }
// }

// impl AppMain for App {
//     fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
//         self.match_event(cx, event);
//         self.root.handle_event(cx, event, &mut Scope::empty());
//         if self.timer.is_empty() {
//             self.timer = cx.start_interval(0.5);
//             dbg!("timer started");
//         } else {
//             if let Event::Timer(t) = event {
//                 if let Some(mut lb_view) = self.root.view(id!(lb_view)).borrow_mut() {
//                     let mut walk = lb_view.walk(cx);
//                     if walk.abs_pos.is_some_and(|pos| pos.x > 800.0) {
//                         dbg!("timer stopped");
//                         cx.stop_timer(self.timer);
//                     }
//                     walk.abs_pos = Some(walk.abs_pos.unwrap_or(dvec2(0.0, 0.0)) + dvec2(4.0, 0.0));
//                     // txt.draw_walk(cx, &mut Scope::empty(), new_walk);
//                     lb_view.apply_over(
//                         cx,
//                         live! {
//                             abs_pos: (walk.abs_pos.unwrap())
//                         },
//                     );
//                     lb_view.redraw(cx);
//                 }
//             }
//         }
//     }
// }

// app_main!(App);
use makepad_widgets::*;
        
live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                body = <ScrollXYView>{
                    flow: Down,
                    spacing:10,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    <Label> {
                        text: "Hello world"
                    }
                }
            }
        }
        win2: <Window>{
            body = {
                <Label> {
                    text: "Win2"
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[live] win2: WidgetRef,
 }

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App{
    fn handle_startup(&mut self, cx: &mut Cx) {
       
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
       
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
        self.win2.handle_event(cx, event, &mut Scope::empty());
    }
}