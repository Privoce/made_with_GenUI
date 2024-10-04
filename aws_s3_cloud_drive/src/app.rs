use gen_components::components::{
    router::{GRouterWidgetExt, GRouterWidgetRefExt},
    view::GViewWidgetRefExt,
};
use makepad_widgets::*;

use crate::{utils::APP_STATE, views::main_page::AppMainPageWidgetRefExt};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    import crate::views::main_page::*;


    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");

    App = {{App}}{
        root: <Root>{
            main_window = <GWindow>{
                os_type: Linux,
                show_icon: false,
                window_bar = {
                    window_title = {
                        align: {x: 0.5},
                    }
                },
                width: Fill,
                height: Fill,
                window: {inner_size: vec2(420, 820)},
                background_color: #161616,
                background_visible: true,
                clip_x: true,
                clip_y: true,
                body = <GVLayout>{
                    app_main_page = <AppMainPage>{}
                    // <StartPage>{}
                    // height: Fill,
                    // <SettingsPage>{}
                    // margin: {
                    //     top: 32.0
                    // }
                    // <SiginPage>{}
                    // <MainPage>{}
                    // <BucketPage>{}
                    // <UploadPage>{}



                    // navigation = <StackNavigation>{
                    //     root_view = {
                    //         width: Fill
                    //         height: Fill
                    //         padding: 0
                    //         flow: Down
                    //         align: {x: 0.0, y: 0.0}
                    //         spacing: 0
                    //         application_pages = <View> {
                    //             margin: 0.0,
                    //             padding: 0.0
                    //             flow: Overlay
                    //             bucket_frame = <BucketPage>{visible: true}
                    //             upload_frame = <UploadPage>{visible: false}
                    //             setting_frame = <SettingsPage> {visible: false}
                    //         }
                    //         menu = <GView>{
                    //             border_radius: 0.0,
                    //             theme: Dark,
                    //             height: 46.0,
                    //             width: Fill,
                    //             modes = <GHLayout>{
                    //                 height: Fill,
                    //                 width: Fill,
                    //                 align: {x: 0.5, y: 0.5},
                    //                 spacing: 16.0,
                    //                 tab1 = <AppTab>{
                    //                     animator: {selected = {default: on}}
                    //                     text: "Home"
                    //                     draw_icon: {
                    //                         svg_file: dep("crate://self/resources/home.svg"),
                    //                     }
                    //                 }
                    //                 tab2 = <AppTab>{
                    //                     animator: {selected = {default: off}}
                    //                     text: "Upload"
                    //                     draw_icon: {
                    //                         svg_file: dep("crate://self/resources/upload.svg"),
                    //                     }
                    //                 }
                    //                 tab3 = <AppTab>{
                    //                     animator: {selected = {default: off}}
                    //                     text: "Settings"
                    //                     draw_icon: {
                    //                         svg_file: dep("crate://self/resources/setting.svg"),
                    //                     }
                    //                 }
                    //             }
                    //         }
                    //     }

                    //     start_page = <StackNavigationView> {
                    //         height: Fill,
                    //         draw_bg: {
                    //             color: #F5F5F500,
                    //         }
                    //         header = <View>{
                    //             height: 0.0,
                    //             visible: false,
                    //         }
                    //         body = {
                    //             margin: {top: 32.0},
                    //             start_screen = <StartPage>{}
                    //         }
                    //     }
                    //     sigin_page_view = <StackNavigationView> {
                    //         draw_bg: {
                    //             color: #F5F5F500,
                    //         }
                    //         header = <View>{
                    //             height: 0.0,
                    //             visible: false,
                    //         }
                    //         body = {
                    //             margin: {top: 32.0},
                    //             sigin_screen = <SiginPage>{

                    //             }
                    //         }
                    //     }
                    // }
                }
            }
        }
    }
}

#[derive(Live)]
pub struct App {
    #[live]
    root: WidgetRef,
    #[rust]
    pub timer: Timer,
}

impl LiveHook for App {
    fn after_apply(
        &mut self,
        _cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        // get configs
        let mut state = APP_STATE.lock().unwrap();
        let _ = state.get_confih_credentials();
        let _ = state.ls();
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::gen_components::live_design(cx);
        crate::views::start_page::live_design(cx);
        crate::views::sigin_page::live_design(cx);
        crate::views::settings_page::live_design(cx);
        crate::views::bucket_page::live_design(cx);
        crate::views::upload_page::live_design(cx);
        crate::views::main_page::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_timer(&mut self, cx: &mut Cx, _e: &TimerEvent) {
        // self.root
        //     .app_main_page(id!(app_main_page))
        //     .borrow()
        //     .map(|page| {
        //         page.grouter(id!(app_router)).borrow_mut().map(|mut route| {
        //             route.nav_to(cx, id!(bucket_frame));
        //         });
        //     });

        self.nav_to(cx, id!(bucket_frame));

        cx.stop_timer(self.timer);
    }
    fn handle_startup(&mut self, cx: &mut Cx) {
        // dbg!("start up");
        self.timer = cx.start_timeout(12.0);
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // self.root
        //     .radio_button_set(ids!(modes.tab1, modes.tab2, modes.tab3))
        //     .selected_to_visible(
        //         cx,
        //         &self.root,
        //         &actions,
        //         ids!(
        //             application_pages.bucket_frame,
        //             application_pages.upload_frame,
        //             application_pages.setting_frame
        //         ),
        //     );

        // let mut navigation = self.root.stack_navigation(id!(navigation));
        // navigation.handle_stack_view_actions(cx, &actions);

        // self.root.
        // router.handle_event(cx, event, scope);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.root.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    pub fn nav_to(&mut self, cx: &mut Cx, path: &[LiveId]) {
        self.root
            .app_main_page(id!(app_main_page))
            .borrow()
            .map(|page| {
                page.grouter(id!(app_router)).borrow_mut().map(|mut route| {
                    route.nav_to(cx, path);
                });
            });
    }
}

app_main!(App);
