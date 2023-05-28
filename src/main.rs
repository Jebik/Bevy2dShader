use bevy::{sprite::{Material2d, Material2dPlugin, ColorMaterial, MaterialMesh2dBundle}, render::render_resource::{ShaderRef, AsBindGroup}, prelude::{Commands, ResMut, App, Assets, Mesh, Camera2dBundle, shape, Vec3, Color, default, Transform, Handle, Image, AssetServer, Res}, DefaultPlugins, reflect::TypeUuid};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut materials_s: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });

    
    // cube
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_xyz(250.0, 250.5, 0.0).with_scale(Vec3::splat(128.)),      
        material: materials_s.add(CustomMaterial {
            color: Color::BLUE,
            color_texture: Some(asset_server.load("branding/icon.png")),
        }),
        ..default()
    });

    /*
    
        ),
     */
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
    color_texture: Option<Handle<Image>>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        include_str!("shader.wgsl").into()
    }
}