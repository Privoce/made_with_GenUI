use gen_components::components::{
    button::GButtonWidgetExt,
    card::{GCard, GCardWidgetExt, GCardWidgetRefExt},
    drop_down::GDropDownWidgetExt,
    icon::GIconWidgetExt,
    image::GImageWidgetExt,
    label::GLabelWidgetExt,
};
use makepad_widgets::*;

use crate::utils::{ls, APP_STATE};

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
            height: 32.0,
            width: Fill,
            align: {
                y: 0.5
            },
            spacing: 6.0,
            file_icon = <GImage>{
                visible: false,
                src: dep("crate://self/resources/file.png"),
            }
            folder_icon = <GImage>{
                src: dep("crate://self/resources/folder.png"),
            }
            <GVLayout>{
                height: Fill,
                width: Fill,
                align: {
                    y: 0.5
                },
                f_name = <GLabel>{
                    font_size: 8.0,
                    font_family: (BOLD_FONT2),
                    text: "Personal"
                }
                f_size = <GLabel>{
                    color: #656F7C,
                    font_size: 7.0,
                    font_family: (BOLD_FONT2),
                    text: ""
                }
            }
            <GDropDown>{
                mode: Drawer,
                height: Fit,
                width: Fit,
                position: Bottom,
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
                                    src: dep("crate://self/resources/share.png"),
                                }
                                <GLabel>{
                                    font_size: 8.0,
                                    font_family: (BOLD_FONT2),
                                    text: "Share"
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
                                    src: dep("crate://self/resources/delete.png"),
                                }
                                <GLabel>{
                                    font_size: 8.0,
                                    font_family: (BOLD_FONT2),
                                    text: "Delete"
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
                                    src: dep("crate://self/resources/download.png"),
                                }
                                <GLabel>{
                                    font_size: 8.0,
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
            // <GDropDown>{
            //     mode: Drawer,
            //     height: Fit,
            //     width: Fit,
            //     position: Bottom,
            //     trigger = <GIcon>{
            //         cursor: Hand,
            //         theme: Dark,
            //         icon_type: Add,
            //         height: 16.0,
            //         width: 16.0
            //     }
            //     popup :<GDrawer> {
            //         container: {
            //             flow: Down,
            //             padding: 8.0,
            //             spacing: 4.0,
            //             background_visible: true,
            //             <GLabel>{
            //                 margin: {
            //                     top: 4.0,
            //                     bottom: 4.0,
            //                 },
            //                 font_size: 9.0,
            //                 font_family: (BOLD_FONT2),
            //                 text: "Add To AWS S3"
            //             }
            //             <GVLayout>{
            //                 height: Fit,
            //                 <GHLayout>{
            //                     height: Fit,
            //                     align: {
            //                         x: 0.0,
            //                         y: 0.5,
            //                     },
            //                     padding: 8.0,
            //                     spacing: 12.0,
            //                     <GImage>{
            //                         height: 16.0,
            //                         width: 16.0,
            //                         src: dep("crate://self/resources/upload_file.png"),
            //                     }
            //                     <GLabel>{
            //                         font_size: 8.0,
            //                         font_family: (BOLD_FONT2),
            //                         text: "Upload File"
            //                     }
            //                 }
            //                 <GHLayout>{
            //                     height: Fit,
            //                     align: {
            //                         x: 0.0,
            //                         y: 0.5,
            //                     },
            //                     padding: 8.0,
            //                     spacing: 12.0,
            //                     <GImage>{
            //                         height: 16.0,
            //                         width: 16.0,
            //                         src: dep("crate://self/resources/upload_folder.png"),
            //                     }
            //                     <GLabel>{
            //                         font_size: 8.0,
            //                         font_family: (BOLD_FONT2),
            //                         text: "Upload Folder"
            //                     }
            //                 }
            //             }
            //             <GDivider>{
            //                 height: 4.0,
            //                 theme: Dark
            //             }
            //             <GVLayout>{
            //                 height: Fit,
            //                 <GHLayout>{
            //                     height: Fit,
            //                     align: {
            //                         x: 0.0,
            //                         y: 0.5,
            //                     },
            //                     padding: 8.0,
            //                     spacing: 12.0,
            //                     <GImage>{
            //                         height: 16.0,
            //                         width: 16.0,
            //                         src: dep("crate://self/resources/add_file.png"),
            //                     }
            //                     <GLabel>{
            //                         font_size: 8.0,
            //                         font_family: (BOLD_FONT2),
            //                         text: "Add File"
            //                     }
            //                 }
            //                 <GHLayout>{
            //                     height: Fit,
            //                     align: {
            //                         x: 0.0,
            //                         y: 0.5,
            //                     },
            //                     padding: 8.0,
            //                     spacing: 12.0,
            //                     <GImage>{
            //                         height: 16.0,
            //                         width: 16.0,
            //                         src: dep("crate://self/resources/add_folder.png"),
            //                     }
            //                     <GLabel>{
            //                         font_size: 8.0,
            //                         font_family: (BOLD_FONT2),
            //                         text: "Add Folder"
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // }

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
                spacing: 6.0,
                scroll_bars: <GScrollBars>{}
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct UploadPage {
    #[deref]
    pub super_widget: GCard,
    #[live]
    pub upload_item: Option<LivePtr>,
}

impl LiveHook for UploadPage {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // get app state and check login is true?
        let mut state = APP_STATE.lock().unwrap();
        if !state.check {
            let _ = state.get_confih_credentials();
        }

        if state.login {
            // use ls to check s3 bucket
            match ls() {
                Ok(res) => {
                    // get upload item as ptr and create list in s3_list
                    self.gcard(id!(s3_list)).borrow_mut().map(|mut s3_list| {
                        let mut list: Vec<(LiveId, WidgetRef)> = Vec::new();
                        for (index, f) in res.iter().enumerate() {
                            list.push((
                                LiveId(index as u64),
                                WidgetRef::new_from_ptr(cx, self.upload_item),
                            ));

                            if let Some((_, target)) = list.last_mut() {
                                target.as_gcard().borrow_mut().map(|mut t_card| {
                                    t_card.glabel(id!(f_name)).set_text_and_redraw(cx, &f.dir);

                                    if f.size.is_some() {
                                        // means file
                                        t_card
                                            .glabel(id!(f_size))
                                            .set_text_and_redraw(cx, &f.size.unwrap().to_string());
                                        t_card.gimage(id!(file_icon)).borrow_mut().map(|mut x| {
                                            x.visible = true;
                                        });
                                        t_card.gimage(id!(folder_icon)).borrow_mut().map(
                                            |mut x| {
                                                x.visible = false;
                                            },
                                        );
                                    } else {
                                        // fold
                                    }
                                });
                            }
                        }

                        s3_list.children.extend(list.into_iter());
                    });
                }
                Err(e) => {
                    self.set_note(cx, &e);
                }
            }
        } else {
            self.set_note(cx, "Can not Auto Config, You may need to get S3 Access Key with Secret and Download AWS Cli tool.");
        }
    }
}

impl Widget for UploadPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));
        self.to_main_page(cx, scope, &actions).map(|_| {
            return;
        });
    }
}

impl UploadPage {
    pub fn to_main_page(
        &mut self,
        cx: &mut Cx,
        scope: &mut Scope,
        actions: &Actions,
    ) -> Option<()> {
        self.gbutton(id!(to_main)).borrow().map(|x| {
            if x.clicked(actions).is_some() {
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    StackNavigationAction::NavigateTo(live_id!(application_pages.main_frame)),
                );
            }
        })
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
}
