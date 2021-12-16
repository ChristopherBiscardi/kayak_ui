use bevy::{
    math::Vec2,
    prelude::{App as BevyApp, AssetServer, Commands, Res, ResMut, World},
    window::{WindowDescriptor, Windows},
    DefaultPlugins,
};
use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
use kayak_ui::core::{bind, render, rsx, widget, Binding, Bound, Index, MutableBound};
use kayak_widgets::{App, Text, Window};

#[derive(Clone, PartialEq)]
struct GlobalCount(pub u32);

#[widget]
fn Counter(context: &mut KayakContext) {
    let global_count = {
        if let Ok(world) = context.get_global_state::<World>() {
            if let Some(global_count) = world.get_resource::<Binding<GlobalCount>>() {
                global_count.clone()
            } else {
                return;
            }
        } else {
            return;
        }
    };

    context.bind(&global_count);

    let global_count = global_count.get().0;

    rsx! {
        <>
            <Window position={(50.0, 50.0)} size={(300.0, 300.0)} title={"Counter Example".to_string()}>
                <Text size={32.0} content={format!("Current Count: {}", global_count).to_string()}>{}</Text>
            </Window>
        </>
    }
}

fn startup(
    mut commands: Commands,
    windows: Res<Windows>,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());

    font_mapping.add(asset_server.load("roboto.kayak_font"));

    let window_size = if let Some(window) = windows.get_primary() {
        Vec2::new(window.width(), window.height())
    } else {
        panic!("Couldn't find primary window!");
    };

    commands.insert_resource(bind(GlobalCount(0)));

    let context = BevyContext::new(window_size.x, window_size.y, |styles, context| {
        render! {
            <App styles={Some(styles.clone())}>
                <Counter />
            </App>
        }
    });
    commands.insert_resource(context);
}

fn count_up(global_count: Res<Binding<GlobalCount>>) {
    global_count.set(GlobalCount(global_count.get().0 + 1));
}

fn main() {
    BevyApp::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("UI Example"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyKayakUIPlugin)
        .add_startup_system(startup)
        .add_system(count_up)
        .run();
}