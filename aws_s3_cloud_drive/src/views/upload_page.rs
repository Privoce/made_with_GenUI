use std::{path::PathBuf, str::FromStr};

use chrono::{Datelike, Local, Timelike};
use gen_components::{
    components::{
        breadcrumb::GBreadCrumbWidgetExt,
        button::GButtonWidgetExt,
        drop_down::{GDropDownWidgetExt, GDropDownWidgetRefExt},
        file_upload::event::new_file_dialog,
        image::GImageWidgetExt,
        input::GInputWidgetExt,
        label::{GLabelWidgetExt, GLabelWidgetRefExt},
        view::{GView, GViewWidgetExt, GViewWidgetRefExt},
    },
    utils::{
        copy_to_clipboard,
        lifetime::{Executor, Lifetime},
    },
};
use makepad_widgets::*;

use crate::utils::{
    cp, format_s3_path, format_size, share, CpId, CpState, LsResult, ShareItem, State, APP_STATE,
    LOAD_LIST, THREAD_POOL,
};

use super::bucket_page::BucketPageEvent;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    BOLD_FONT2 = dep("crate://self/resources/FiraCode-Bold.ttf");
    UploadPage = {{UploadPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #16191F,
        upload_item: <GView>{
            event_order: Down,
            flow: Right,
            animation_open: true,
            hover_color: #21252C,
            background_color: #21252CA0,
            pressed_color: #21252C,
            height: 48.0,
            width: Fill,
            align: {
                y: 0.5
            },
            padding:{left: 4.0, right: 12.0},
            spacing: 8.0,
            file_icon = <GImage>{
                src: dep("crate://self/resources/file.png"),
            }
            folder_icon = <GImage>{
                src: dep("crate://self/resources/folder.png"),
            }
            item_wrap = <GVLayout>{
                height: Fill,
                width: Fill,
                align: {
                    y: 0.5
                },
                spacing: 4.0,
                f_name = <GLabel>{
                    font_size: 9.0,
                    font_family: (BOLD_FONT),
                    text: "Personal",
                }
                <GHLayout>{
                    height: Fit,
                    spacing: 12.0,
                    f_size = <GLabel>{
                        color: #656F7C,
                        font_size: 8.0,
                        font_family: (BOLD_FONT2),
                        text: ""
                    }
                    f_date = <GLabel>{
                        color: #656F7C,
                        font_size: 8.0,
                        font_family: (BOLD_FONT2),
                        text: ""
                    }
                }
            }
            upload_drop_down = <GDropDown>{
                mode: Drawer,
                height: Fit,
                width: Fit,
                position: Bottom,
                proportion: 0.2,
                trigger_more = <GIcon>{
                    cursor: Hand,
                    theme: Dark,
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
                                padding: 8.0,
                                spacing: 12.0,
                                animation_open: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                pressed_color: #1D2028,
                                background_visible: true,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/share.png"),
                                }
                                <GLabel>{
                                    font_size: 9.0,
                                    font_family: (BOLD_FONT2),
                                    text: "Share"
                                }
                            }
                            delete_wrap = <GHLayout>{
                                animation_open: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                pressed_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 8.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/delete.png"),
                                }
                                <GLabel>{
                                    font_size: 9.0,
                                    font_family: (BOLD_FONT2),
                                    text: "Delete"
                                }
                            }
                            download_wrap = <GHLayout>{
                                animation_open: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                pressed_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 8.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/download.png"),
                                }
                                <GLabel>{
                                    font_size: 9.0,
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
                font_family: (BOLD_FONT),
                text: "AWS S3",
            }
            <GDropDown>{
                mode: Drawer,
                height: Fit,
                width: Fit,
                position: Bottom,
                proportion: 0.32,
                trigger = <GIcon>{
                    cursor: Hand,
                    theme: Dark,
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
                            font_size: 9.0,
                            font_family: (BOLD_FONT2),
                            text: "Add To AWS S3"
                        }
                        <GVLayout>{
                            height: Fit,
                            <GHLayout>{
                                animation_open: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                pressed_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 8.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/upload_file.png"),
                                }
                                <GLabel>{
                                    font_size: 8.0,
                                    font_family: (BOLD_FONT2),
                                    text: "Upload File"
                                }
                            }
                            <GHLayout>{
                                animation_open: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                pressed_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 8.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/upload_folder.png"),
                                }
                                <GLabel>{
                                    font_size: 8.0,
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
                                animation_open: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                pressed_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 8.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/add_file.png"),
                                }
                                <GLabel>{
                                    font_size: 8.0,
                                    font_family: (BOLD_FONT2),
                                    text: "Add File"
                                }
                            }
                            <GHLayout>{
                                animation_open: true,
                                hover_color: #1D2028,
                                background_color: #22262F,
                                pressed_color: #1D2028,
                                background_visible: true,
                                height: Fit,
                                align: {
                                    x: 0.0,
                                    y: 0.5,
                                },
                                padding: 8.0,
                                spacing: 12.0,
                                <GImage>{
                                    height: 16.0,
                                    width: 16.0,
                                    src: dep("crate://self/resources/add_folder.png"),
                                }
                                <GLabel>{
                                    font_size: 8.0,
                                    font_family: (BOLD_FONT2),
                                    text: "Add Folder"
                                }
                            }
                        }
                    }
                }
            }
            url_popup = <GDropDown>{
                mode: Dialog,
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
                            spacing: 8.0,
                            <GLabel>{
                                text: "[ DOWNLOAD URL ]",
                                color: #FF7043,
                                font_family: (BOLD_FONT2),
                                font_size: 12.0,
                            }
                            url_input = <GInput>{
                                theme: Dark,
                                width: Fill,
                                text:"",
                                font_family: (BOLD_FONT2),
                                font_size: 9.0,
                                height: 260.0,
                            }
                            url_copy_btn = <GButton>{
                                slot: {
                                    text: "Copy Download URL"
                                }
                            }
                        }
                    }
                }
            }
            notice_popup = <GDropDown>{
                mode: Dialog,
                height: Fit,
                width: Fit,
                notice_icon = <GIcon>{
                    cursor: Hand,
                    theme: Dark,
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
                                font_size: 9.0,
                            }
                        }
                    }
                }
            }
            notice_dialog = <GDropDown>{
                mode: Dialog,
                height: Fit,
                width: Fit,
                notice_icon = <GIcon>{
                    visible: false,
                    cursor: Hand,
                    theme: Dark,
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
                                font_size: 9.0,
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
                theme: Dark,
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
                        src: dep("crate://self/resources/upload_file.png"),
                    }
                    <GLabel>{
                        font_size: 8.0,
                        font_family: (BOLD_FONT2),
                        text: "Upload"
                    }
                }
            }
            <GButton>{
                theme: Dark,
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
                        src: dep("crate://self/resources/add_file.png"),
                    }
                    <GLabel>{
                        font_size: 8.0,
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
                    font_size: 8.0,
                    font_family: (BOLD_FONT2),
                    text: "MY AWS"
                }
            }
            <GDivider>{
                theme: Dark,
                height: 4.0,
                margin: {
                    top: 2.0,
                    bottom: 2.0,
                }
            }
            path_header = <GBreadCrumb>{
                theme: Dark,
                labels: []
            }
            s3_list = <GVLayout>{
                height: 400.0,
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
    select_target: Option<(usize, String)>,
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

        if let Event::Signal = event {
            let mut state = APP_STATE.lock().unwrap();
            let _ = state.ls();
            state.current.as_ref().map(|res| {
                self.set_dir_file(cx, res);
            });
        }

        self.gview(id!(s3_list)).borrow().map(|list| {
            let mut flag = false;
            for (index, (_, child)) in list.children.iter().enumerate() {
                child
                    .gdrop_down(id!(upload_drop_down))
                    .borrow_mut()
                    .map(|mut dropdown| {
                        if let Some(e) = dropdown.toggled(&actions) {
                            if e.opened {
                                flag = true;
                                // check item is file/dir, is dir do not show share
                                let state = APP_STATE.lock().unwrap();
                                if let Some(list) = state.current.as_ref() {
                                    if list[index].size.is_some() {
                                        self.select_target.replace((
                                            index,
                                            format!(
                                                "s3://{}/{}",
                                                state.s3_path.join("/"),
                                                list[index].name
                                            ),
                                        ));

                                        dropdown.popup(cx, |_cx, popup| {
                                            popup
                                                .container
                                                .gview(id!(share_wrap))
                                                .borrow_mut()
                                                .map(|mut wrap| {
                                                    wrap.visible = true;
                                                });
                                        });
                                    }
                                } else {
                                    dropdown.popup(cx, |_cx, popup| {
                                        popup.container.gview(id!(share_wrap)).borrow_mut().map(
                                            |mut wrap| {
                                                wrap.visible = false;
                                            },
                                        );
                                    });
                                    self.select_target = None;
                                }
                            }
                        }
                    });
                if flag {
                    break;
                }
            }
            if let Some((index, target)) = self.select_target.clone().as_ref() {
                let mut flag = false;
                if list.child_count() < *index || list.child_count() == 0 {
                    return;
                }

                let _ = &list.children[*index]
                    .1
                    .gdrop_down(id!(upload_drop_down))
                    .borrow_mut()
                    .map(|mut dropdown| {
                        // check container share wrap
                        dropdown.popup(cx, |_, popup| {
                            popup
                                .container
                                .gview(id!(share_wrap))
                                .borrow()
                                .map(|share_wrap| {
                                    if share_wrap.finger_down(&actions).is_some() {
                                        flag = true;
                                    }
                                });
                        });
                        if flag {
                            dropdown.close(cx);
                        }
                    });
                if flag {
                    match share(target, 3600.0) {
                        Ok(url) => {
                            self.set_url_note(cx, &url, target);
                        }
                        Err(e) => {
                            self.set_load_note(cx, &e);
                        }
                    }
                }
            }
        });
        self.gdrop_down(id!(url_popup))
            .borrow_mut()
            .map(|mut popup| {
                popup.popup(cx, |_, popup| {
                    popup
                        .container
                        .gbutton(id!(url_copy_btn))
                        .borrow()
                        .map(|btn| {
                            if btn.clicked(&actions).is_some() {
                                let _ = copy_to_clipboard(&self.share_url);
                            }
                        });
                });
            });
    }
}

impl UploadPage {
    pub fn click_upload_file_btn(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .gbutton(id!(upload_file_btn))
            .clicked(actions)
            .is_some()
        {
            let state = APP_STATE.lock().unwrap();
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
                    let to_path = format_s3_path(&state.s3_path);
                    let id = CpId::new(&from_path, &to_path, true);
                    {
                        let mut list = LOAD_LIST.lock().unwrap();
                        list.insert(id.clone(), CpState::default());
                    }
                    self.set_load_note(cx, &format!("{} is uploading", &from_path));

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
                        SignalToUI::set_ui_signal();
                    }
                });
            }
        }
    }

    // init bucket
    pub fn init(&mut self, cx: &mut Cx) {
        let state = APP_STATE.lock().unwrap();

        state.current.as_ref().map(|res| {
            self.set_dir_file(cx, res);
        });
    }
    pub fn update_path_header(&mut self, labels: Vec<String>) {
        self.gbread_crumb(id!(path_header))
            .borrow_mut()
            .map(|mut x| {
                x.labels = labels;
            });
    }
    pub fn handle_path_header(&mut self, cx: &mut Cx, actions: &Actions) {
        self.gbread_crumb(id!(path_header))
            .borrow_mut()
            .map(|mut x| {
                let mut flag = true;
                let mut state = APP_STATE.lock().unwrap();
                if let Some(e) = x.clicked(actions) {
                    if e.index < state.s3_path.len() - 1 {
                        state.s3_path.truncate(e.index + 1);
                    } else {
                        flag = false;
                    }
                } else if x.to_home(actions) {
                    state.s3_path.clear();
                } else {
                    flag = false;
                }
                if flag {
                    x.labels = state.s3_path.clone();
                    // update list
                    let _ = state.ls();
                    state.current.as_ref().map(|res| {
                        self.set_dir_file(cx, res);
                    });
                }
            });
    }
    pub fn update_list(&mut self, cx: &mut Cx, actions: &Actions) -> Option<()> {
        let mut state = APP_STATE.lock().unwrap();
        // let mut target_name: Option<String> = None;
        let mut flag = false;
        self.gview(id!(s3_list)).borrow_mut().map(|list| {
            for (_, (_, child)) in list.children.iter().enumerate() {
                // actions.find
                child.as_gview().gview(id!(item_wrap)).borrow().map(|wrap| {
                    if wrap.finger_up(&actions).is_some() && !wrap.glabel(id!(f_size)).is_visible()
                    {
                        state
                            .s3_path
                            .push(child.as_gview().glabel(id!(f_name)).text());
                        self.update_path_header(state.s3_path.clone());
                        flag = true;
                    }
                });
                if flag {
                    // self.gview(id!(update_loading)).borrow_mut().map(|mut x| {
                    //     x.visible = true;
                    //     x.redraw(cx);
                    // });
                    break;
                }
            }
        });

        if flag {
            let _ = state.ls();
            state.current.as_ref().map(|res| {
                self.set_dir_file(cx, res);
            });

            // self.gview(id!(update_loading)).borrow_mut().map(|mut x| {
            //     x.visible = false;
            //     x.redraw(cx);
            // });
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
                x.popup(cx, |cx, popup| {
                    popup
                        .container
                        .glabel(id!(note_label))
                        .set_text_and_redraw(cx, note);
                });
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
            x.popup(cx, |cx, popup| {
                popup
                    .container
                    .ginput(id!(url_input))
                    .set_text_and_redraw(cx, url);
            });
        });
    }
    pub fn set_note(&mut self, cx: &mut Cx, note: &str) -> () {
        self.gdrop_down(id!(notice_dialog))
            .borrow_mut()
            .map(|mut x| {
                x.open(cx);
                x.popup(cx, |cx, popup| {
                    popup
                        .container
                        .glabel(id!(note_label))
                        .set_text_and_redraw(cx, note);
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
                                x.redraw(cx);
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
