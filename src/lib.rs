use std::marker::PhantomData;

use bevy::prelude::*;

pub trait HealthTrait {
    fn current(&self) -> u32;
    fn max(&self) -> u32;
}

#[derive(Resource, Clone)]
pub struct HealthBarRes {
    font: Handle<Font>,
}

#[derive(Component)]
struct HealthBarAttach {
    attached_to: Entity,
}

// just to keep track so we dont spawn the same thing twice
#[derive(Component)]
pub struct HealthBar {
    pub offset: Vec2,
    pub size: f32,
    pub color: Color,
}
impl Default for HealthBar {
    fn default() -> Self {
        Self {
            offset: Vec2::new(0.0, 0.0),
            size: 10.,
            color: Color::RED,
        }
    }
}

#[derive(Bundle)]
struct HealthBarBundle {
    healthbar: HealthBarAttach,
    // offset: Transform,
    #[bundle]
    text: TextBundle,
}

pub struct HealthBarPlugin<HEALTH, MAINCAM> {
    pub fontpath: &'static str,
    pub auto_create_healthbars: bool,
    _h: PhantomData<(HEALTH, MAINCAM)>,
}
impl<HEALTH, MAINCAM> HealthBarPlugin<HEALTH, MAINCAM> {
    pub fn new(fontpath: &'static str) -> Self {
        Self {
            fontpath,
            auto_create_healthbars: true,
            _h: default(),
        }
    }
    pub fn automatic_bar_creation(mut self, enabled: bool) -> Self {
        self.auto_create_healthbars = enabled;
        self
    }
}

impl<HEALTH, MAINCAM> Plugin for HealthBarPlugin<HEALTH, MAINCAM>
where
    HEALTH: 'static + HealthTrait + Send + Sync + Component,
    MAINCAM: 'static + Send + Sync + Component,
{
    fn build(&self, app: &mut App) {
        let ass = app.world.resource::<AssetServer>();
        let font = ass.load(self.fontpath);
        app.insert_resource(HealthBarRes { font });
        // build the plugin
        if self.auto_create_healthbars {
            app.add_system(add_healthbars_to_entites_with_health::<HEALTH>);
        }
        app.add_system(spawn_health_bar_children::<HEALTH, MAINCAM>);
        app.add_system(update_healthbars::<HEALTH, MAINCAM>);
        app.add_system(despawn_unattached_healthbars::<HEALTH>);
    }
}

fn add_healthbars_to_entites_with_health<HEALTH>(
    mut commands: Commands,
    entities: Query<Entity, (With<HEALTH>, Without<HealthBar>)>,
) where
    HEALTH: Component,
{
    for entity in entities.iter() {
        if let Some(mut ec) = commands.get_entity(entity) {
            ec.insert(HealthBar::default());
        }
    }
}

fn despawn_unattached_healthbars<HEALTH: Component>(
    mut commands: Commands,
    healthbars: Query<(Entity, &HealthBarAttach), Without<HealthBar>>,
    entites: Query<(&HEALTH, &Transform), With<HealthBar>>,
) {
    for (hb_entity, attach) in healthbars.iter() {
        // despawn the healthbar
        if let Err(_) = entites.get(attach.attached_to) {
            if let Some(ec) = commands.get_entity(hb_entity) {
                ec.despawn_recursive()
            }
        }
    }
}

fn update_healthbars<HEALTH, MAINCAM>(
    hres: Res<HealthBarRes>,
    mut healthbars: Query<
        (
            Entity,
            &mut Text,
            &mut Style,
            &mut Transform,
            &HealthBarAttach,
            // &mut Visibility,
        ),
        Without<HealthBar>,
    >,
    entites: Query<(&HEALTH, &Transform, &HealthBar)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MAINCAM>>,
) where
    HEALTH: HealthTrait + Component,
    MAINCAM: Component,
{
    for (_hb_entity, mut hb_text, mut hb_style, mut hb_transform, hb_attach) in
        healthbars.iter_mut()
    {
        if let Ok((e_health, e_transform, e_bar)) = entites.get(hb_attach.attached_to) {
            let bartrans = get_sceen_transform_and_visibility(&camera_q, e_transform);
            *hb_transform = bartrans;
            // *hb_visibility = visibility;

            hb_style.position.left = Val::Px(bartrans.translation.x + e_bar.offset.x);
            hb_style.position.bottom = Val::Px(bartrans.translation.y + e_bar.offset.y);

            // println!("updating healthbar");
            let current = e_health.current();
            let max = e_health.max();
            let style = TextStyle {
                font: hres.font.clone(),
                font_size: e_bar.size,
                color: e_bar.color,
            };
            *hb_text = Text::from_section(format!("{current}/{max}"), style);
        }
    }
}

fn spawn_health_bar_children<HEALTH, MAINCAM>(
    mut commands: Commands,
    hres: Res<HealthBarRes>,
    entities: Query<(Entity, &HEALTH, &Transform, &HealthBar), Added<HealthBar>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MAINCAM>>,
) where
    HEALTH: HealthTrait + Component,
    MAINCAM: Component,
{
    for (entity, health, transform, hbar) in entities.iter() {
        // spawn the healthbar as a child
        let style = TextStyle {
            font: hres.font.clone(),
            font_size: hbar.size,
            color: hbar.color,
        };
        let current = health.current();
        let max = health.max();
        let text = Text::from_section(format!("{current}/{max}"), style);
        let bartrans = get_sceen_transform_and_visibility(&camera_q, transform);

        commands.spawn(
            // Healthbarbundle
            HealthBarBundle {
                healthbar: HealthBarAttach {
                    attached_to: entity,
                },
                text: TextBundle {
                    transform: bartrans,
                    text,
                    // visibility,
                    ..default()
                }
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    ..default()
                }),
            },
        );
    }
}

fn get_sceen_transform_and_visibility<MAINCAM: Component>(
    camera_q: &Query<(&Camera, &GlobalTransform), With<MAINCAM>>,
    transform: &Transform,
) -> Transform {
    let (camera, cam_gt) = camera_q.single();
    let pos = camera.world_to_viewport(cam_gt, transform.translation);
    let bartrans = if let Some(pos) = pos {
        Transform::from_xyz(pos.x, pos.y, 1.)
    } else {
        Transform::default()
    };
    bartrans
}
