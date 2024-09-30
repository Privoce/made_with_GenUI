use gen_components::{
    components::{
        view::{GView, GViewWidgetExt, GViewWidgetRefExt},
        drop_down::GDropDownWidgetExt,
        image::GImageWidgetExt,
        label::{GLabelWidgetExt, GLabelWidgetRefExt},
    },
    utils::HeapLiveIdPathExp,
};
use makepad_widgets::*;

use crate::utils::{
    format_size,
    lifetime::{Executor, Lifetime},
    LsResult, APP_STATE,
};

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
        background_color: #161616,
        upload_item: <GHLayout>{
            event_order: Down,
            height: 36.0,
            width: Fill,
            align: {
                y: 0.5
            },
            padding:{right: 12.0},
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
                    spacing: 8.0,
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
                proportion: 0.25,
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

            notice_popup = <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: BottomRight,
                notice_icon = <GIcon>{
                    cursor: Hand,
                    theme: Dark,
                    icon_type: Notice,
                    height: 16.0,
                    width: 16.0
                },
                popup :<GToolTip> {
                    height: 120.0,
                    width: 160.0,
                    background_color: #1E1E1E,
                    container: {
                        height: Fill,
                        width: Fill,
                        flow: Down,
                        align: {
                            x: 0.5,
                            y: 0.5
                        },
                        note_label = <GLabel>{
                            margin: 6.0,
                            width: Fill,
                            text:"",
                            font_family: (BOLD_FONT2),
                            font_size: 8.0,
                        }
                    }
                }
            }
        }
        <GHLayout>{
            height: Fit,
            padding: 8.0,
            spacing: 12.0,
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
            self.lifetime
                .init()
                .execute(|| self.init(cx))
                .map(|lifetime| {
                    self.lifetime = lifetime;
                });
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));

        self.update_list(cx, &actions).map(|_| {
            return;
        });

        self.gview(id!(s3_list)).borrow().map(|list| {
            let path = list.scope_path.clone();
            for (index, (_, _child)) in list.children.iter().enumerate() {
                let mut flag = false;
                self.gdrop_down(id!(upload_drop_down))
                    .borrow_mut()
                    .map(|mut drop_down| {
                        drop_down.popup(cx, |_cx, popup| {
                            popup
                                .container
                                .gview(id!(share_wrap))
                                .borrow()
                                .map(|share_wrap| {
                                    if let Some(e) = share_wrap.finger_down(&actions) {
                                        let mut c_p = path.clone();
                                        c_p.push(LiveId(index as u64));

                                        if e.path.contains(&c_p).unwrap() {
                                            flag = true;
                                        }
                                    }
                                });
                        });
                    });
                if flag {
                    dbg!(index);
                    break;
                }
            }
        });
    }
}

impl UploadPage {
    // init bucket
    pub fn init(&mut self, cx: &mut Cx) {
        let state = APP_STATE.lock().unwrap();

        state.current.as_ref().map(|res| {
            self.set_dir_file(cx, res);
        });
    }
    pub fn update_list(&mut self, cx: &mut Cx, actions: &Actions) -> Option<()> {
        let mut state = APP_STATE.lock().unwrap();
        // let mut target_name: Option<String> = None;
        let mut flag = false;
        self.gview(id!(s3_list)).borrow().map(|list| {
            for (_, (_, child)) in list.children.iter().enumerate() {
                // actions.find
                child.as_gview().gview(id!(item_wrap)).borrow().map(|wrap| {
                    if wrap.finger_up(&actions).is_some() && !wrap.glabel(id!(f_size)).is_visible()
                    {
                        state
                            .s3_path
                            .push(child.as_gview().glabel(id!(f_name)).text());
                        flag = true;
                    }
                });
                if flag {
                    self.gview(id!(update_loading)).borrow_mut().map(|mut x| {
                        x.visible = true;
                        x.redraw(cx);
                    });
                    break;
                }
            }
        });

        if flag {
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

    pub fn set_note(&mut self, cx: &mut Cx, note: &str) -> () {
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
