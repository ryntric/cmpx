use crate::ui::root::RootView;
use gpui::{
    App, AppContext, Bounds, SharedString, Size, TitlebarOptions, WindowBackgroundAppearance,
    WindowBounds, WindowDecorations, WindowOptions, px,
};

const TITLE_BAR: &str = "cmpx";

pub(crate) fn launch(ctx: &mut App) {
    let display_id = ctx.primary_display().map(|display| display.id());
    let size = Size::new(px(800f32), px(600f32));
    let bounds = Bounds::centered(display_id, size, ctx);
    let title_bar_options = TitlebarOptions {
        title: Some(SharedString::new_static(TITLE_BAR)),
        ..Default::default()
    };

    let window_options = WindowOptions {
        display_id,
        window_bounds: Some(WindowBounds::Maximized(bounds)),
        titlebar: Some(title_bar_options),
        window_min_size: Some(size),
        window_background: WindowBackgroundAppearance::Opaque,
        window_decorations: Some(WindowDecorations::Client),
        ..Default::default()
    };

    ctx.open_window(window_options, |_window, ctx| {
        ctx.new(|_| RootView::default())
    })
    .unwrap();
    ctx.activate(true);
}
