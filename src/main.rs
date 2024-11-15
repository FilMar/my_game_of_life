use bevy::{prelude::*, window::PrimaryWindow};
use resources::{Grid, Play, WordTimer};
use std::time::Duration;

mod resources;

const MAX_X: f32 = 2000f32;
const MAX_Y: f32 = 2000f32;
const STEP: usize= 50;
const NEAR: usize = 1;

fn main() {
    App::new()
        .insert_resource(Grid::new(MAX_X, MAX_Y, STEP))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Play(false))
        .insert_resource(WordTimer(Timer::new(
            Duration::from_millis(100),
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, alive_on_click)
        .add_systems(Update, start_on_space)
        .add_systems(Update, evolve_world)
        .run();
}


#[derive(PartialEq)]
enum States {
    Dead,
    Alive,
}

#[derive(Component)]
struct Cell(Entity);

impl States {
    fn evolve(neiboor: u8, is_alive: bool) -> Self {
        match is_alive {
            true => match neiboor {
                2..4 => Self::Alive,
                _ => Self::Dead,
            },
            false => match neiboor {
                3 => Self::Alive,
                _ => Self::Dead,
            },
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn alive_on_click(
    mut commands: Commands,
    grid: Res<Grid>,
    mouse: Res<ButtonInput<MouseButton>>,
    wind: Query<&Window, With<PrimaryWindow>>,
    alives: Query<(&Transform, &Cell)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(pos) = wind.single().cursor_position() {
            let x = wind.single().size().x;
            let y = wind.single().size().y;
            let real_pos = Vec2::new((pos.x - x / 2.0).into(), (-pos.y + y / 2.0).into());
            let real_pos = grid.get_nearest(real_pos.x, real_pos.y);
            match alives.iter().find(|(t, _)| t.translation.xy() == real_pos) {
                Some((_, c)) => despawn_cell(&mut commands, c.0),
                None => spawn_cell(&mut commands, real_pos),
            };
        }
    }
}

fn start_on_space(command: Res<ButtonInput<KeyCode>>, mut play: ResMut<Play>) {
    if command.just_pressed(KeyCode::Space) {
        play.0 = match play.0 {
            true => false,
            false => true,
        };
        println!("premuto play {}", play.0);
    }
}

fn evolve_world(
    mut command: Commands,
    grid: Res<Grid>,
    mut timer: ResMut<WordTimer>,
    time: Res<Time>,
    play: Res<Play>,
    alives: Query<(&Transform, &Cell)>,
) {
    if !play.0 {
        return;
    }
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let mut to_lives = Vec::new();
    let mut to_kills = Vec::new();
    for (t, c) in &alives {
        let index = grid.get_indexs(t.translation.xy());
        println!("current: {:?}", index);
        let neiboor = grid.get_neiboor(index, NEAR);
        let n: Vec<Vec2> = alives
            .iter()
            .filter(|(t, _)| neiboor.contains(&t.translation.xy()))
            .map(|(t, _)| t.translation.xy())
            .collect();
        let current = States::evolve(n.iter().count() as u8, true);
        let dead_neib = neiboor.iter().filter(|p| !n.contains(&p));
        println!("vicini vivi: {:?}", n.iter().map(|p| grid.get_indexs(*p)).collect::<Vec<(usize, usize)>>());
        for pos in dead_neib {
            let index = grid.get_indexs(*pos);
            let neiboor = grid.get_neiboor(index, NEAR);
            println!("morto: {index:?}");
            let n = alives
                .iter()
                .filter(|(t, _)| neiboor.contains(&t.translation.xy()))
                .count();
            if States::evolve(n as u8, false) == States::Alive {
                if !to_lives.contains(pos) {
                    println!("{index:?} ha {n} vicini quindi nasce",);
                    to_lives.push(*pos);
                }
            }
        }
        if current == States::Dead {
            println!("{} vicini quindi muore", n.iter().count());
            to_kills.push(c.0);
        }
    }
    for p in to_lives {
        spawn_cell(&mut command, p);
    }
    for p in to_kills {
        despawn_cell(&mut command, p);
    }
    println!("----------------------------------");
}

// -------------------------------------------------

fn spawn_cell(commands: &mut Commands, pos: Vec2) {
    let mut entity_commands = commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(STEP as f32, STEP as f32)),
            ..Default::default()
        },
        ..Default::default()
    });
    entity_commands.insert(Cell(entity_commands.id()));
}

fn despawn_cell(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn();
}
