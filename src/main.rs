use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people).add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    } 
}


#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
