use gen_components::components::{
    card::GCard,
    file_upload::{event::GFileUploadEvent, GUploadWidgetExt},
    label::GLabelWidgetExt,
    table::{
        row::{GTableRowRef, GTableRowWidgetRefExt},
        GTableWidgetExt,
    },
};
use makepad_widgets::*;

use crate::aws_structs::LsResult;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    MainView = {{MainView}}{
        flow: Down,
        ls_item: <GTRow>{
            height: 32.0,
            width: Fill,
            <GTCell>{
                height: Fill,
                width: 300.0,
                align: {x: 0.1, y: 0.5},
                name_str = <GLabel>{
                    color: #E36640,
                    text: "-",
                }
            }
            <GTCell>{
                height: Fill,
                width: Fill,
                align: {x: 0.1, y: 0.5},
                date_str = <GLabel>{
                    color: #1C2128,
                    text: "-",
                }
            }
        }
        split_view = <GSplitter>{
            axis: Vertical,
            align: FromB(200),
            b: <GVLayout>{
                align: {x: 0.5, y: 0.5},
                <GCard>{
                    theme: Dark,
                    height: 100.0,
                    width: 300.0,
                    flow: Down,
                    align: {x: 0.5, y: 0.5},
                    upload = <GUpload>{
                        icon: {
                            theme: Warning
                        }
                        height: 70.0,
                    }
                    <GLabel>{
                        color: #FF7043,
                        height: Fit,
                        text: "Choose Dir as Upload Bucket",
                    }
                }
            },
            a: <GVLayout>{
                height: Fit,
                width: Fill,
                padding: 6.0,
                scroll_bars: <GScrollBars> {}
                <GHLayout>{
                    height: 36.0,
                    width: Fill,
                    spacing: 6.0,
                    align: {x: 0.0, y: 0.5},
                    <GLabel>{
                        font_size: 12.0,
                        text: "Bucket: S3 "
                    }
                    <GLabel>{
                        font_size: 12.0,
                        text: "List",
                    }
                    <GIcon>{
                        theme: Dark,
                        icon_type: Fresh,
                        height: 16.0,
                        width: 16.0,
                        cursor: Hand
                    }
                }
                <GHLayout>{
                    height: Fit,
                    width: Fill,
                    ls_loading = <GLoading>{
                       visible: false,
                    }
                    ls_table = <GTable>{
                        mode: Virtual,
                        height: Fit,
                        width: Fill,
                        header: {
                            height: Fit,
                            width: Fill,
                            <GTRow>{
                                height: 32.0,
                                width: Fill,
                                <GTCell>{
                                    height: Fill,
                                    width: 300.0,
                                    align: {x: 0.1, y: 0.5},
                                    <GLabel>{
                                        color: #E36640,
                                        text: "Name",
                                    }
                                }
                                <GTCell>{
                                    height: Fill,
                                    width: Fill,
                                    align: {x: 0.1, y: 0.5},
                                    <GLabel>{
                                        color: #1C2128,
                                        text: "Date",
                                    }
                                }
                            }
                        }

                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct MainView {
    #[deref]
    pub super_widget: GCard,
    #[rust]
    pub bucket: String,
    #[rust]
    pub ls_tables: Vec<LsResult>,
    #[live]
    pub ls_item: Option<LivePtr>,
    #[rust]
    pub ls_children: ComponentMap<LiveId, GTableRowRef>,
}

impl LiveHook for MainView {}

impl MainView {
    pub fn set_bucket(&mut self, cx: &mut Cx, bucket: &str, ls_tables: Option<Vec<LsResult>>) {
        self.bucket = bucket.to_string();
        ls_tables.map(|ls_tables| {
            // self.splitter(id!(split_view));
            self.gtable(id!(ls_table)).borrow_mut().map(|mut table| {
                for (index, ls_table) in ls_tables.iter().enumerate() {
                    let row = self
                        .ls_children
                        .get_or_insert(cx, LiveId(index as u64), |cx| {
                            WidgetRef::new_from_ptr(cx, self.ls_item).as_gtable_row()
                        });
                    row.borrow_mut().map(|row| {
                        for (_, cell) in row.children.iter() {
                            cell.borrow_mut().map(|cell| {
                                cell.glabel(id!(name_str))
                                    .set_text_and_redraw(cx, &ls_table.dir);
                                cell.glabel(id!(date_str))
                                    .set_text_and_redraw(cx, &ls_table.date);
                            });
                        }
                    });
                }
                table.body_virtual.children = self.ls_children.clone();
                table.body_virtual.walk.height = Size::Fit;
            });
            self.ls_tables = ls_tables;
        });
    }
}

impl Widget for MainView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));

        self.gupload(id!(upload)).borrow_mut().map(|x| {
            // x.handle_after_select(&actions).map(|x|{
            //     dbg!("selected", x);
            // });
            if let Some(selected) = x.after_select(&actions){
                dbg!("selected", selected);
            }
        });
    }
}
