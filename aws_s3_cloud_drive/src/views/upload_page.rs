use std::{path::PathBuf, str::FromStr};

use chrono::{Datelike, Local, Timelike};
use gen_components::{
    components::{
        breadcrumb::GBreadCrumbWidgetExt,
        button::GButtonWidgetExt,
        drop_down::{GDropDownWidgetExt, GDropDownWidgetRefExt},
        file_upload::new_file_dialog,
        image::GImageWidgetExt,
        input::GInputWidgetExt,
        label::GLabelWidgetExt,
        view::{GView, GViewWidgetExt, GViewWidgetRefExt},
    },
    utils::lifetime::{Executor, Lifetime},
};
use makepad_widgets::*;

use crate::utils::{
    cp, format_s3_path, format_size, rm, share, CpId, CpState, LsResult, Req, ShareItem, State,
    APP_STATE, LOAD_LIST, THREAD_POOL,
};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    BOLD_FONT2 = dep("crate://self/resources/FiraCode-Bold.ttf");
    UploadPage = {{UploadPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #1F1616,
        upload_item: <GView>{
            event_order: Down,
            flow: Right,
            animation_key: true,
            event_key: true,
            hover_color: #37181F,
            background_color: #251619,
            focus_color: #67301B,
            height: 54.0,
            width: Fill,
            align: {
                y: 0.5
            },
            padding:{left: 4.0, right: 12.0},
            spacing: 8.0,
            file_icon = <GImage>{
                src: dep("crate://self/resources/file2.png"),
            }
            folder_icon = <GImage>{
                src: dep("crate://self/resources/folder2.png"),
            }
            item_wrap = <GVLayout>{
                event_key: true,
                height: Fill,
                width: Fill,
                align: {
                    y: 0.5
                },
                f_name = <GLabel>{
                    font_size: 9.33,
                    font_family: (BOLD_FONT2),
                    text: "Personal",
                }
                sd_box = <GHLayout>{
                    height: Fit,
                    width: Fit,
                    f_size = <GLabel>{
                        color: #656F7C,
                        font_size: 8.0,
                        font_family: (BOLD_FONT2),
                        text: "",
                        margin: {right: 12.0, top: 4.0}
                    }
                    f_date = <GLabel>{
                        margin: {top: 4.0}
                        color: #656F7C,
                        font_size: 8.0,
                        font_family: (BOLD_FONT2),
                        text: ""
                    }
                }
            }
            upload_drop_down = <GDropDown>{
                height: Fit,
                width: Fit,
                position: Bottom,
                proportion: 0.18,
                trigger_more = <GIcon>{
                    cursor: Hand,
                    theme: Info,
                    icon_type: More,
                    height: 16.0,
                    width: 16.0
                }
                popup : <GDrawer> {
                    container: {
                        flow: Down,
                        padding: 8.0,
                        spacing: 4.0,
                        background_visible: true,
                        background_color: #22262F,
                        <GVLayout>{
                            height: Fit,
                            share_wrap = <GHLayout>{
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 10.0,
                                spacing: 12.0,
                                animation_key: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                focus_color: #1D2028,
                                background_visible: true,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/share.png"),
                                }
                                <GLabel>{
                                    font_size: 9.33,
                                    font_family: (BOLD_FONT2),
                                    text: "Share"
                                }
                            }
                            delete_wrap = <GHLayout>{
                                animation_key: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                focus_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 10.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/delete.png"),
                                }
                                <GLabel>{
                                    font_size: 9.33,
                                    font_family: (BOLD_FONT2),
                                    text: "Delete"
                                }
                            }
                            download_wrap = <GHLayout>{
                                animation_key: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                focus_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 10.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/download.png"),
                                }
                                <GLabel>{
                                    font_size: 9.33,
                                    font_family: (BOLD_FONT2),
                                    text: "Download"
                                }
                            }
                        }
                    }
                }
            }
        },
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 8.0,
            align: {y:0.5},
            padding: 12.0,
            <GLabel>{
                width: Fill,
                font_size: 12.0,
                font_family: (BOLD_FONT2),
                text: "GenUI Cloud Drive",
            }
            tool_drop = <GDropDown>{
                height: Fit,
                width: Fit,
                position: Bottom,
                proportion: 0.32,
                trigger = <GIcon>{
                    cursor: Hand,
                    theme: Info,
                    icon_type: Add,
                    height: 16.0,
                    width: 16.0
                }
                popup :<GDrawer> {
                    container: {
                        flow: Down,
                        padding: 8.0,
                        spacing: 4.0,
                        background_visible: true,
                        background_color: #22262F,
                        <GLabel>{
                            margin: {
                                top: 4.0,
                                bottom: 4.0,
                            },
                            font_size: 10.67,
                            font_family: (BOLD_FONT2),
                            text: "Add To AWS S3"
                        }
                        <GVLayout>{
                            height: Fit,
                            upload_file_view = <GHLayout>{
                                animation_key: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                focus_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 10.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/upload_file.png"),
                                }
                                <GLabel>{
                                    font_size: 9.33,
                                    font_family: (BOLD_FONT2),
                                    text: "Upload File"
                                }
                            }
                            <GHLayout>{
                                animation_key: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                focus_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 10.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/upload_folder.png"),
                                }
                                <GLabel>{
                                    font_size: 9.33,
                                    font_family: (BOLD_FONT2),
                                    text: "Upload Folder"
                                }
                            }
                        }
                        <GDivider>{
                            height: 4.0,
                            theme: Dark
                        }
                        <GVLayout>{
                            height: Fit,
                            <GHLayout>{
                                animation_key: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                focus_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 10.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/add_file.png"),
                                }
                                <GLabel>{
                                    font_size: 9.33,
                                    font_family: (BOLD_FONT2),
                                    text: "Add File"
                                }
                            }
                            <GHLayout>{
                                animation_key: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                focus_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 10.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/add_folder.png"),
                                }
                                <GLabel>{
                                    font_size: 9.33,
                                    font_family: (BOLD_FONT2),
                                    text: "Add Folder"
                                }
                            }
                        }
                    }
                }
            }
            url_popup = <GDropDown>{
                height: Fit,
                width: Fit,
                popup :<GDialog> {
                    container: {
                        height: 380.0,
                        width: 300.0,
                        flow: Down,
                        <GView>{
                            height: Fill,
                            width: Fill,
                            padding: 8.0,
                            clip_x: true,
                            clip_y: true,
                            flow: Down,
                            align: {
                                x: 0.5,
                                y: 0.5
                            },
                            spacing: 16.0,
                            background_color: #191111,
                            <GLabel>{
                                text: " Shareable Link ",
                                color: #FF7043,
                                font_family: (BOLD_FONT2),
                                font_size: 12.0,
                            }
                            url_input = <GInput>{
                                theme: Dark,
                                width: Fill,
                                text:"",
                                font_family: (BOLD_FONT2),
                                font_size: 9.33,
                                height: 260.0,
                            }
                            url_copy_btn = <GButton>{
                                background_color: #EC4925,
                                hover_color: #Fa4319,
                                focus_color: #E36741,
                                slot: {
                                    font_family: (BOLD_FONT2),
                                    text: "Copy Link"
                                }
                            }
                        }
                    }
                }
            }
            notice_popup = <GDropDown>{
                height: Fit,
                width: Fit,
                notice_icon = <GIcon>{
                    cursor: Hand,
                    theme: Info,
                    icon_type: Notice,
                    height: 16.0,
                    width: 16.0
                },
                popup :<GDialog> {
                    container: {
                        height: 200.0,
                        width: 300.0,
                        flow: Down,
                        <GView>{
                            height: Fill,
                            width: Fill,
                            padding: 12.0,
                            clip_x: true,
                            clip_y: true,
                            flow: Down,
                            align: {
                                x: 0.5,
                                y: 0.5
                            },
                            spacing: 8.0,
                            background_color: #191111,
                            // <GLabel>{
                            //     text: "[ Note ]",
                            //     color: #FF7043,
                            //     font_family: (BOLD_FONT2),
                            //     font_size: 12.0,
                            // }
                            <GLoading>{
                                theme: Info,
                            }
                            note_label = <GLabel>{
                                margin: 6.0,
                                width: Fill,
                                text:"",
                                font_family: (BOLD_FONT2),
                                font_size: 9.33,
                            }
                        }
                    }
                }
            }
            notice_dialog = <GDropDown>{
                height: Fit,
                width: Fit,
                notice_icon = <GIcon>{
                    visible: false,
                    cursor: Hand,
                    theme: Info,
                    icon_type: Notice,
                    height: 16.0,
                    width: 16.0
                },
                popup :<GDialog> {
                    container: {
                        height: 200.0,
                        width: 300.0,
                        flow: Down,
                        <GView>{
                            height: Fill,
                            width: Fill,
                            padding: 12.0,
                            clip_x: true,
                            clip_y: true,
                            flow: Down,
                            align: {
                                x: 0.5,
                                y: 0.5
                            },
                            spacing: 8.0,
                            background_color: #191111,
                            <GLabel>{
                                text: "[ Note ]",
                                color: #FF7043,
                                font_family: (BOLD_FONT2),
                                font_size: 12.0,
                            }
                            note_label = <GLabel>{
                                margin: 6.0,
                                width: Fill,
                                text:"",
                                font_family: (BOLD_FONT2),
                                font_size: 9.33,
                            }
                        }
                    }
                }
            }
        }
        <GHLayout>{
            height: Fit,
            padding: 8.0,
            spacing: 12.0,
            upload_file_btn = <GButton>{
                grab_key_focus: false,
                theme: Warning,
                background_color: #EC4925,
                hover_color: #E97056,
                focus_color: #C73514,
                slot: <GHLayout>{
                    height: Fit,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    spacing: 6.0,
                    background_color: #EC4925,
                    hover_color: #E97056,
                    focus_color: #C73514,
                    <GImage>{
                        height: 16.0,
                        width: 16.0,
                        src: dep("crate://self/resources/upload_file2.png"),
                    }
                    <GLabel>{
                        font_size: 9.33,
                        font_family: (BOLD_FONT2),
                        text: "Upload"
                    }
                }
            }
            <GButton>{
                theme: Warning,
                background_color: #EC4925,
                hover_color: #E97056,
                focus_color: #C73514,
                slot: <GHLayout>{
                    height: Fit,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    spacing: 6.0,
                    <GImage>{
                        height: 16.0,
                        width: 16.0,
                        src: dep("crate://self/resources/add_file2.png"),
                    }
                    <GLabel>{
                        font_size: 9.33,
                        font_family: (BOLD_FONT2),
                        text: "Add"
                    }
                }
            }
        }
        <GVLayout>{
            height: Fit,
            padding: 12.0,
            spacing: 8.0,
            <GHLayout>{
                height: Fit,
                <GLabel>{
                    font_size: 10.67,
                    font_family: (BOLD_FONT2),
                    text: "MY AWS S3"
                }
            }
            <GDivider>{
                theme: Dark,
                background_color: #EC4925,
                height: 4.0,
                margin: {
                    top: 2.0,
                    bottom: 2.0,
                }
            }
            path_header = <GBreadCrumb>{
                theme: Dark,
                // icon: {
                //     color: #EC4925,
                // }
                background_visible: false,
                // stroke_color: #EC4925,
                // stroke_hover_color: #FF7043,
                item: {
                    font_family: (BOLD_FONT2),
                    color: #ED4A26,
                    text_hover_color: #ED4A26,
                }
                path: []
            }
            s3_list = <GVLayout>{
                height: 600.0,
                width: Fill,
                clip_x: true,
                clip_y: true,
                scroll_bars: <GScrollBars>{}
            }
        }
        update_loading = <GHLayout>{
            visible: false,
            height: 32.0,
            align:{
                x: 0.5,
                y: 0.5
            },
            <GLabel>{
                font_size: 8.0,
                font_family: (BOLD_FONT2),
                text: "Loading..."
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct UploadPage {
    #[deref]
    pub super_widget: GView,
    #[live]
    pub upload_item: Option<LivePtr>,
    #[rust(true)]
    pub is_bucket: bool,
    #[rust]
    pub lifetime: Lifetime,
    #[rust]
    share_url: String,
}

impl LiveHook for UploadPage {
    fn after_apply(
        &mut self,
        _cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if !self.visible {
            return;
        }
    }
}

impl Widget for UploadPage {
    fn is_visible(&self) -> bool {
        self.visible
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.visible {
            let _ = self.super_widget.draw_walk(cx, scope, walk);
            self.lifetime.init().execute(|| self.init(cx)).map(|_| {
                self.lifetime.next();
            });
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));

        self.update_list(cx, &actions).map(|_| {
            return;
        });
        self.handle_path_header(cx, &actions);
        self.click_upload_file_btn(cx, &actions);
        self.click_upload_file_view(cx, &actions);
        self.handle_req(cx, &actions);

        if let Event::Signal = event {
            let mut update = false;
            let mut state = APP_STATE.lock().unwrap();
            match &state.req {
                Req::None => (),
                Req::Upload => {
                    self.close_load_note(cx);
                    update = true;
                }
                Req::Rm => {
                    self.set_note(cx, "Delete success!");
                    update = true;
                }
                Req::Cp => {
                    self.set_note(cx, "Download success!");
                    update = true;
                }
                Req::Mv => (),
                Req::Share => {
                    dbg!("share");
                }
                Req::Error(e) => {
                    self.set_note(cx, e);
                }
            }
            state.req = Req::None;
            if update {
                let _ = state.ls();
                state.current.as_ref().map(|res| {
                    self.set_dir_file(cx, res);
                });
            }
        }
        self.gdrop_down(id!(url_popup))
            .borrow_mut()
            .map(|mut popup| {
                popup.get_mut(cx, |_, _, popup| {
                    popup.gbutton(id!(url_copy_btn)).borrow().map(|btn| {
                        if btn.clicked(&actions).is_some() {
                            let _ = copy_to_clipboard(&self.share_url);
                        }
                    });
                });
            });
    }
}

impl UploadPage {
    pub fn handle_req(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut flag = Req::None;
        let mut target = String::new();
        let mut download_path = None;
        self.gview(id!(s3_list)).borrow_mut().map(|list| {
            for (index, (_, child)) in list.children.iter().enumerate() {
                child
                    .gdrop_down(id!(upload_drop_down))
                    .borrow_mut()
                    .map(|mut dp| {
                        dp.get_mut(cx, |cx, dp, container| {
                            if dp.opened {
                                if container.gview(id!(share_wrap)).clicked(actions).is_some() {
                                    flag = Req::Share;
                                    let mut state = APP_STATE.lock().unwrap();
                                    target = format!(
                                        "s3://{}/{}",
                                        state.s3_path.join("/"),
                                        state.current.as_ref().unwrap()[index].name
                                    );
                                    state.req = Req::Share;
                                    dp.close(cx);
                                }

                                if container.gview(id!(delete_wrap)).clicked(actions).is_some() {
                                    flag = Req::Rm;
                                    let mut state = APP_STATE.lock().unwrap();
                                    target = format!(
                                        "s3://{}/{}",
                                        state.s3_path.join("/"),
                                        state.current.as_ref().unwrap()[index].name
                                    );
                                    state.req = Req::Rm;
                                    dp.close(cx);
                                }

                                if container
                                    .gview(id!(download_wrap))
                                    .clicked(actions)
                                    .is_some()
                                {
                                    flag = Req::Cp;
                                    let mut state = APP_STATE.lock().unwrap();
                                    target = format!(
                                        "s3://{}/{}",
                                        state.s3_path.join("/"),
                                        state.current.as_ref().unwrap()[index].name
                                    );
                                    state.req = Req::Cp;
                                    let f = new_file_dialog();
                                    download_path = f.pick_folder();
                                    dp.close(cx);
                                }
                            }
                        });
                    });
                if flag != Req::None {
                    break;
                }
            }
        });
        let (sender, recv) = std::sync::mpsc::channel();
        // handle req
        match flag {
            Req::None | Req::Upload | Req::Mv => (),
            Req::Rm => {
                THREAD_POOL.spawn(async move {
                    match rm(&target).await {
                        Ok(_) => {
                            let _ = sender.send(Req::Rm);
                        }
                        Err(_) => {
                            let _ = sender.send(Req::Error(format!("Delete failed: {}", target)));
                        }
                    }
                });
            }
            Req::Cp => {
                download_path.map(|to| {
                    THREAD_POOL.spawn(async move {
                        let to = to.to_str().unwrap().to_string();
                        let id = CpId::new(&target, &to, false);
                        match cp(&target, &to, &id).await {
                            Ok(_) => {
                                let _ = sender.send(Req::Cp);
                            }
                            Err(_) => {
                                let _ = sender.send(Req::Error(format!(
                                    "Download {} to {} fail",
                                    target, to
                                )));
                            }
                        }
                    });
                });
            }
            Req::Share => match share(&target, 3600.0) {
                Ok(url) => {
                    self.set_url_note(cx, &url, &target);
                }
                Err(e) => {
                    self.set_load_note(cx, &e);
                }
            },
            Req::Error(e) => self.set_note(cx, &e),
        }

        std::thread::spawn(move || {
            if let Ok(req) = recv.recv() {
                match req {
                    Req::Error(e) => {
                        let mut state = APP_STATE.lock().unwrap();
                        state.req = Req::Error(e);
                    }
                    _ => {
                        SignalToUI::set_ui_signal();
                    }
                }
            }
        });
    }
    pub fn click_upload_file_view(&mut self, cx: &mut Cx, actions: &Actions) {
        self.gdrop_down(id!(tool_drop))
            .get_mut(cx, |cx, drop, container| {
                if container
                    .gview(id!(upload_file_view))
                    .clicked(actions)
                    .is_some()
                {
                    self.upload_file(cx);
                    drop.close(cx);
                }
            });
    }
    pub fn click_upload_file_btn(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .gbutton(id!(upload_file_btn))
            .clicked(actions)
            .is_some()
        {
            self.upload_file(cx);
        }
    }
    fn upload_file(&mut self, cx: &mut Cx) {
        let mut state = APP_STATE.lock().unwrap();
        if state.s3_path.is_empty() {
            self.set_note(cx, "Currently, you have not selected any buckets. Please select a bucket from the bucket list before uploading!");
        } else {
            // open fs system
            let f = new_file_dialog()
                .add_filter("allows", &["zip", "png", "jpg", "svg", "txt", "md"])
                .set_directory(PathBuf::from_str("/").unwrap());
            let (sender, recv) = std::sync::mpsc::channel();
            f.pick_file().map(|p| {
                let from_path = p.to_str().unwrap().to_string();
                // has space or not
                if from_path.contains(' ') {
                    self.set_note(
                        cx,
                        "The file path contains spaces, please rename the file and try again!",
                    );
                    return;
                }
                let to_path = format_s3_path(&state.s3_path);
                let id = CpId::new(&from_path, &to_path, true);
                {
                    let mut list = LOAD_LIST.lock().unwrap();
                    list.insert(id.clone(), CpState::default());
                }
                self.set_load_note(cx, &format!("{} is uploading.", &from_path));
                state.req = Req::Upload;
                // do cp
                THREAD_POOL.spawn(async move {
                    match cp(&from_path, &to_path, &id).await {
                        Ok(id) => {
                            let _ = sender.send(id);
                        }
                        Err(_) => {
                            let mut list = LOAD_LIST.lock().unwrap();
                            list.insert(id.clone(), CpState::Failed);
                        }
                    }
                });
            });

            // let uid = self.widget_uid();
            std::thread::spawn(move || {
                if let Ok(_) = recv.recv() {
                    // close load note
                    SignalToUI::set_ui_signal();
                }
            });
        }
    }
    // init bucket
    pub fn init(&mut self, cx: &mut Cx) {
        let mut state = APP_STATE.lock().unwrap();
        if !state.req.is_none() {
            state.ls();
            state.req = Req::None;
        }
        state.current.as_ref().map(|res| {
            self.set_dir_file(cx, res);
        });
        self.gbread_crumb(id!(path_header))
            .set_path(state.s3_path.clone());
    }
    pub fn update_path_header(&mut self, labels: Vec<String>) {
        self.gbread_crumb(id!(path_header))
            .borrow_mut()
            .map(|mut x| {
                x.path = labels;
            });
    }
    pub fn handle_path_header(&mut self, cx: &mut Cx, actions: &Actions) {
        self.gbread_crumb(id!(path_header))
            .borrow_mut()
            .map(|mut x| {
                let mut flag = true;
                let mut state = APP_STATE.lock().unwrap();
                if let Some(e) = x.changed(actions) {
                    if e.index < state.s3_path.len() - 1 {
                        state.s3_path.truncate(e.index + 1);
                    } else {
                        flag = false;
                    }
                } else if x.home(actions).is_some() {
                    state.s3_path.clear();
                } else {
                    flag = false;
                }
                if flag {
                    x.path = state.s3_path.clone();
                    // update list
                    let _ = state.ls();
                    state.current.as_ref().map(|res| {
                        self.set_dir_file(cx, res);
                    });
                }
            });
    }
    pub fn update_list(&mut self, cx: &mut Cx, actions: &Actions) -> Option<()> {
        // let mut target_name: Option<String> = None;
        let mut flag = false;
        self.gview(id!(s3_list)).borrow().map(|list| {
            for (_, (_, child)) in list.children.iter().enumerate() {
                // actions.find
                child.as_gview().borrow().map(|wrap| {
                    if wrap.gview(id!(item_wrap)).clicked(&actions).is_some()
                        && !wrap.glabel(id!(f_size)).is_visible()
                    {
                        let mut state = APP_STATE.lock().unwrap();
                        state.s3_path.push(wrap.glabel(id!(f_name)).text());

                        flag = true;
                    }
                });
                if flag {
                    break;
                }
            }
        });

        if flag {
            let mut state = APP_STATE.lock().unwrap();
            self.update_path_header(state.s3_path.clone());
            let _ = state.ls();
            state.current.as_ref().map(|res| {
                self.set_dir_file(cx, res);
            });

            self.gview(id!(update_loading)).borrow_mut().map(|mut x| {
                x.visible = false;
                x.redraw(cx);
            });
            Some(())
        } else {
            None
        }
    }
    pub fn set_load_note(&mut self, cx: &mut Cx, note: &str) {
        self.gdrop_down(id!(notice_popup))
            .borrow_mut()
            .map(|mut x| {
                x.open(cx);
                x.get_mut(cx, |cx, _, popup| {
                    popup.glabel(id!(note_label)).set_text_and_redraw(cx, note);
                });
            });
    }
    pub fn close_load_note(&mut self, cx: &mut Cx) {
        self.gdrop_down(id!(notice_popup))
            .borrow_mut()
            .map(|mut x| {
                x.close(cx);
            });
    }
    pub fn set_url_note(&mut self, cx: &mut Cx, url: &str, name: &str) {
        self.share_url = url.to_string();
        let mut state = APP_STATE.lock().unwrap();
        let date = Local::now().naive_local();
        state.push_share(ShareItem {
            url: url.trim().to_string(),
            name: name.to_string(),
            date: (
                date.year() as usize,
                (date.month0() + 1) as u8,
                (date.day0() + 1) as u8,
                (date.hour()) as u8,
                (date.minute()) as u8,
            ),
            during: 3600.0,
        });
        State::sync_shares(false);
        self.gdrop_down(id!(url_popup)).borrow_mut().map(|mut x| {
            x.open(cx);
            x.get_mut(cx, |cx, _, popup| {
                popup.ginput(id!(url_input)).set_text_and_redraw(cx, url);
            });
        });
    }
    pub fn set_note(&mut self, cx: &mut Cx, note: &str) -> () {
        self.gdrop_down(id!(notice_dialog))
            .borrow_mut()
            .map(|mut x| {
                x.open(cx);
                x.get_mut(cx, |cx, _, popup| {
                    popup.glabel(id!(note_label)).set_text_and_redraw(cx, note);
                });
            });
    }
    pub fn set_dir_file(&mut self, cx: &mut Cx, res: &Vec<LsResult>) {
        self.gview(id!(s3_list)).borrow_mut().map(|mut s3_list| {
            s3_list.children.clear();
            let mut list: Vec<(LiveId, WidgetRef)> = Vec::new();
            for (index, f) in res.iter().enumerate() {
                list.push((
                    LiveId(index as u64),
                    WidgetRef::new_from_ptr(cx, self.upload_item),
                ));

                if let Some((_, target)) = list.last_mut() {
                    target.as_gview().borrow().map(|t_view| {
                        t_view.glabel(id!(f_name)).set_text_and_redraw(cx, &f.name);
                        f.date.as_ref().map(|s| {
                            t_view.glabel(id!(f_date)).set_text_and_redraw(cx, s);
                        });

                        if f.size.is_some() {
                            // means file
                            t_view.glabel(id!(f_size)).borrow_mut().map(|mut x| {
                                x.set_text(&format_size(f.size.unwrap()));
                                x.visible = true;
                                x.redraw(cx);
                            });

                            t_view.gimage(id!(file_icon)).borrow_mut().map(|mut x| {
                                x.visible = true;
                            });
                            t_view.gimage(id!(folder_icon)).borrow_mut().map(|mut x| {
                                x.visible = false;
                            });
                        } else {
                            // fold
                            t_view.glabel(id!(f_size)).borrow_mut().map(|mut x| {
                                x.visible = false;
                            });
                            t_view.glabel(id!(f_date)).borrow_mut().map(|mut x| {
                                x.visible = false;
                            });

                            t_view.gimage(id!(file_icon)).borrow_mut().map(|mut x| {
                                x.visible = false;
                            });
                            t_view.gimage(id!(folder_icon)).borrow_mut().map(|mut x| {
                                x.visible = true;
                            });
                        }
                    });
                }
            }

            s3_list.children.extend(list.into_iter());
            s3_list.redraw(cx);
        });
    }
}

pub fn copy_to_clipboard(value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clip_board = arboard::Clipboard::new()?;
    clip_board.set_text(value).map_err(|e| e.into())
}
