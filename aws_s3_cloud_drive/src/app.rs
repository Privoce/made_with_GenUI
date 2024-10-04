use gen_components::components::router::GRouterWidgetExt;
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
        self.nav_to(cx, id!(bucket_frame));

        cx.stop_timer(self.timer);
    }
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.timer = cx.start_timeout(12.0);
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
