use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use super::components::{ButtonRect, DummyLabel};

#[derive(Bundle)]
pub struct Container {
    pub ui: SpatialBundle,
}

impl Container {
    pub fn new(pos: Vec3) -> Self {
        Self {
            ui: SpatialBundle {
                transform: Transform::from_translation(pos),
                visibility: Visibility::Inherited,
                ..Default::default()
            },
        }
    }

    pub fn hidden(self) -> Self {
        Self {
            ui: SpatialBundle {
                visibility: Visibility::Hidden,
                ..self.ui
            },
        }
    }
}

#[derive(Bundle)]
pub struct MaterialMesh<T>
where
    T: Material2d,
{
    pub ui: MaterialMesh2dBundle<T>,
}

impl<T> MaterialMesh<T>
where
    T: Material2d,
{
    pub fn new(pos: Vec3, mesh_handle: Handle<Mesh>, material_handle: Handle<T>) -> Self {
        Self {
            ui: MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
        }
    }
}

/// TODO: find this in libraries
pub fn spawn_rounded_rectangle<T: Bundle>(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    pos: Vec3,
    size: Vec2,
    background_color: &str,
    border_radius: f32,
    label: T,
) -> Entity {
    let (width, height) = (size.x, size.y);
    let root = commands.spawn((label, Container::new(pos))).id();

    let color = ColorMaterial::from(Color::hex(background_color).unwrap());
    let material_handle = materials.add(color);
    let circle = Mesh::from(Circle::new(border_radius));
    let center_rect = Mesh::from(Rectangle::new(
        width - border_radius * 2.0,
        height - border_radius * 2.0,
    ));
    let vertical_rect = Mesh::from(Rectangle::new(border_radius, height - border_radius * 2.0));
    let horizontal_rect = Mesh::from(Rectangle::new(width - border_radius * 2.0, border_radius));

    let top_left = commands
        .spawn(MaterialMesh::new(
            Vec3::new(
                -width / 2.0 + border_radius,
                height / 2.0 - border_radius,
                0.0,
            ),
            meshes.add(circle.clone()),
            material_handle.clone(),
        ))
        .id();
    let top_right = commands
        .spawn(MaterialMesh::new(
            Vec3::new(
                width / 2.0 - border_radius,
                height / 2.0 - border_radius,
                0.0,
            ),
            meshes.add(circle.clone()),
            material_handle.clone(),
        ))
        .id();
    let bottom_left = commands
        .spawn(MaterialMesh::new(
            Vec3::new(
                -width / 2.0 + border_radius,
                -height / 2.0 + border_radius,
                0.0,
            ),
            meshes.add(circle.clone()),
            material_handle.clone(),
        ))
        .id();
    let bottom_right = commands
        .spawn(MaterialMesh::new(
            Vec3::new(
                width / 2.0 - border_radius,
                -height / 2.0 + border_radius,
                0.0,
            ),
            meshes.add(circle.clone()),
            material_handle.clone(),
        ))
        .id();
    let center = commands
        .spawn(MaterialMesh::new(
            Vec3::new(0.0, 0.0, 0.0),
            meshes.add(center_rect),
            material_handle.clone(),
        ))
        .id();
    let left = commands
        .spawn(MaterialMesh::new(
            Vec3::new(-width / 2.0 + border_radius / 2.0, 0.0, 0.0),
            meshes.add(vertical_rect.clone()),
            material_handle.clone(),
        ))
        .id();
    let right = commands
        .spawn(MaterialMesh::new(
            Vec3::new(width / 2.0 - border_radius / 2.0, 0.0, 0.0),
            meshes.add(vertical_rect),
            material_handle.clone(),
        ))
        .id();
    let top = commands
        .spawn(MaterialMesh::new(
            Vec3::new(0.0, height / 2.0 - border_radius / 2.0, 0.0),
            meshes.add(horizontal_rect.clone()),
            material_handle.clone(),
        ))
        .id();
    let bottom = commands
        .spawn(MaterialMesh::new(
            Vec3::new(0.0, -height / 2.0 + border_radius / 2.0, 0.0),
            meshes.add(horizontal_rect),
            material_handle.clone(),
        ))
        .id();
    commands.entity(root).push_children(&[
        top_left,
        top_right,
        bottom_left,
        bottom_right,
        center,
        left,
        right,
        top,
        bottom,
    ]);
    root
}

pub fn spawn_rounded_rectangle_with_border<T: Bundle>(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    pos: Vec3,
    size: Vec2,
    background_color: &str,
    border_radius: f32,
    border_color: &str,
    border_width: f32,
    label: T,
) -> Entity {
    let (width, height) = (size.x, size.y);
    let outer_rect = spawn_rounded_rectangle(
        commands,
        materials,
        meshes,
        pos,
        size,
        border_color,
        border_radius,
        DummyLabel,
    );
    let inner_rect = spawn_rounded_rectangle(
        commands,
        materials,
        meshes,
        Vec3::new(pos.x, pos.y, pos.z + 1.0),
        Vec2::new(width - border_width * 2.0, height - border_width * 2.0),
        background_color,
        border_radius / 2.0,
        DummyLabel,
    );
    let root = commands.spawn((label, Container::new(pos))).id();
    commands
        .entity(root)
        .push_children(&[outer_rect, inner_rect]);
    root
}

pub fn spawn_text<T: Bundle>(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
    text: &str,
    font_size: f32,
    color: &str,
    bold: bool,
    label: T,
) -> Entity {
    let font = if bold {
        asset_server.load("Verdana_Bold.ttf")
    } else {
        asset_server.load("Verdana.ttf")
    };
    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        color: Color::hex(color).unwrap(),
    };
    let text_justification = JustifyText::Center;
    let text_bundle = Text2dBundle {
        text: Text::from_section(text, text_style.clone()).with_justify(text_justification),
        transform: Transform::from_translation(pos),
        ..default()
    };
    commands.spawn((label, text_bundle)).id()
}

pub fn spawn_button<A: Bundle, B: Bundle>(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    asset_server: &Res<AssetServer>,
    abs_parent_pos: Vec3,
    pos: Vec3,
    size: Vec2,
    text: &str,
    background_color: &str,
    box_label: A,
    text_label: B,
) -> Entity {
    let rect = Rect::new(
        pos.x + abs_parent_pos.x - size.x / 2.0,
        pos.y + abs_parent_pos.y - size.y / 2.0,
        pos.x + abs_parent_pos.x + size.x / 2.0,
        pos.y + abs_parent_pos.y + size.y / 2.0,
    );
    let root = commands
        .spawn((box_label, ButtonRect(rect), Container::new(pos)))
        .id();
    let button_box = spawn_rounded_rectangle_with_border(
        commands,
        materials,
        meshes,
        Vec3::new(0.0, 0.0, 0.0),
        size,
        background_color,
        5.0,
        "#eeeeee",
        2.0,
        DummyLabel,
    );
    let button_text = spawn_text(
        commands,
        asset_server,
        Vec3::new(0.0, 0.0, 2.0),
        text,
        16.0,
        "#ffffff",
        true,
        text_label,
    );
    commands
        .entity(root)
        .push_children(&[button_box, button_text]);
    root
}
