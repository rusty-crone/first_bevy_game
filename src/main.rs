use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("Hello World");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Todd Crone".to_string())));
    commands.spawn((Person, Name("Sam Smith".to_string())));
    commands.spawn((Person, Name("Tyler Griffin".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Todd Crone" {
            name.0 = "Bad Ass".to_string();
            break;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}


