use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use bevy_dither_post_process::components::DitherPostProcessSettings;
use bevy_headless_render::components::HeadlessRenderSource;
use ratatui::style::Style;

// TODO: MULTIPLE WINDOWS (probably behind feature flag)
// INFO: need abstraction for launching terminal emulators
//
// /// Structure to refer to a terminal window entity
// #[derive(Clone, Debug)]
// pub enum TerminalWindowRef {
//     /// Refers to the primary window created by default in the terminal the command is run in
//     Primary,
//     /// Direct reference to an terminal window entity
//     Entity(Entity),
// }
//
// #[derive(Component, Debug)]
// pub struct TerminalWindow;


/// Marker component for terminal display
#[derive(Component, Debug)]
#[component(on_add = on_add_terminal_display)]
pub struct TerminalDisplay {
    /// Level of dithering performed on image
    pub dither_level: u32,
    /// Style applied to rendered text
    pub style: Style,
}

fn on_add_terminal_display(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let dither_level = world
        .entity(entity)
        .get::<TerminalDisplay>()
        .unwrap()
        .dither_level;

    let terminal_size = crossterm::terminal::size().unwrap();
    let size = Extent3d {
        width: (terminal_size.0 as u32) * 2,
        height: (terminal_size.1 as u32) * 4,
        depth_or_array_layers: 1,
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::R8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);
    let image_handle = asset_server.add(image);

    let headless_render_source = HeadlessRenderSource::new(&asset_server, image_handle.clone());
    let post_process_settings = DitherPostProcessSettings::new(dither_level, &asset_server);
    world
        .commands()
        .entity(entity)
        .insert((headless_render_source, post_process_settings));
    if let Some(mut camera) = world.entity_mut(entity).get_mut::<Camera>() {
        camera.target = image_handle.into();
    } else {
        world.commands().entity(entity).insert(Camera {
            target: image_handle.into(),
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::LinearRgba(LinearRgba::BLACK)),
            ..Default::default()
        });
    }
}
