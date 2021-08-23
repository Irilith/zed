use crate::Settings;
use gpui::{
    action,
    elements::{
        Align, ConstrainedBox, Container, Flex, MouseEventHandler, ParentElement as _, Svg,
    },
    AnyViewHandle, AppContext, Element as _, ElementBox,
};

pub struct Sidebar {
    side: Side,
    items: Vec<Item>,
    active_item_ix: Option<usize>,
}

#[derive(Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

struct Item {
    icon_path: &'static str,
    view: AnyViewHandle,
}

action!(ToggleSidebarItem, ToggleArg);

#[derive(Clone)]
pub struct ToggleArg {
    side: Side,
    item_index: usize,
}

impl Sidebar {
    pub fn new(side: Side) -> Self {
        Self {
            side,
            items: Default::default(),
            active_item_ix: None,
        }
    }

    pub fn add_item(&mut self, icon_path: &'static str, view: AnyViewHandle) {
        self.items.push(Item { icon_path, view });
    }

    pub fn toggle_item(&mut self, item_ix: usize) {
        if self.active_item_ix == Some(item_ix) {
            self.active_item_ix = None;
        } else {
            self.active_item_ix = Some(item_ix)
        }
    }

    pub fn active_item(&self) -> Option<&AnyViewHandle> {
        self.active_item_ix
            .and_then(|ix| self.items.get(ix))
            .map(|item| &item.view)
    }

    pub fn render(&self, settings: &Settings, cx: &AppContext) -> ElementBox {
        let side = self.side;
        let line_height = cx.font_cache().line_height(
            cx.font_cache().default_font(settings.ui_font_family),
            settings.ui_font_size,
        );

        Container::new(
            Flex::column()
                .with_children(self.items.iter().enumerate().map(|(item_index, item)| {
                    let theme = if Some(item_index) == self.active_item_ix {
                        &settings.theme.active_sidebar_icon
                    } else {
                        &settings.theme.sidebar_icon
                    };
                    enum SidebarButton {}
                    MouseEventHandler::new::<SidebarButton, _>(item.view.id(), cx, |_| {
                        ConstrainedBox::new(
                            Align::new(
                                ConstrainedBox::new(
                                    Svg::new(item.icon_path).with_color(theme.color).boxed(),
                                )
                                .with_height(line_height)
                                .boxed(),
                            )
                            .boxed(),
                        )
                        .with_height(line_height + 16.0)
                        .boxed()
                    })
                    .on_click(move |cx| {
                        cx.dispatch_action(ToggleSidebarItem(ToggleArg { side, item_index }))
                    })
                    .boxed()
                }))
                .boxed(),
        )
        .with_style(&settings.theme.sidebar)
        .boxed()
    }
}
