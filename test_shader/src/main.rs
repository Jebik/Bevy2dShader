use bevy_sprite::{Material2d, MaterialMesh2dBundle};
use bevy_ecs::prelude::*;
use bevy_reflect::TypeUuid;
use bevy_render::{render_resource::{AsBindGroup, ShaderRef}, texture::Image, color::Color};
use bevy_asset::{Handle, AssetServer, Assets};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}


#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    color: Color,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

fn setup(
    mut commands: Commands, 
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        material: materials.add(CustomMaterial {
            color: Color::RED,
            color_texture: asset_server.load("some_image.png"),
        }),
        ..Default::default()
    });
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        include_str!("shader.wgsl").into()
    }
}