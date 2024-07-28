use piston_window::*;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Keys {
    a: bool,
    s: bool,
    d: bool,
    w: bool,
    special: bool,
    ability: bool,
}

#[derive(Clone)]
struct KeySequence {
    sequence: Vec<Keys>,
    step: u16,
    length: u16,
}

fn update_clone(agent_ID: u32, state: Game) -> Game {
    state = state.clones.get(agent_ID).move(agent_ID, state.clone())

    state
}

fn update_player(state: Game) -> Game {
    state.player = add_inputs(get_inputs, state.player.clone())
    state = state.player.move(state.clone())

    state
}

fn update_player_bullet(bullet_ID: u32, state: Game) -> Game {
    state = state.player_bullets.get(bullet_ID).update(bullet_ID, state.clone())

    state
}

fn update_enemy_bullet(bullet_ID: u32, state: Game) -> Game {
    state = state.enemy_bullets.get(bullet_ID).update(bullet_ID, state.clone())

    state
}

fn update_players(state: Game) -> Game {
    for clone in state.clones.get_all() {
        state = update_clone(clone.clone(), state.clone())
    }
    state.player = update_player(state.clone())

    state
}

fn update_bullets(state: Game) -> Game {
    for bullet in state.enemy_bullets.get_all() {
        state = update_enemy_bullet(bullet.ID.clone(), state.clone())
    }

    for bullet in state.player_bullets.get_all() {
        state = update_player_bullet(bullet.ID.clone(), state.clone())
    }

    state
}

fn check_hits(state: Game) -> Game {
    state
}

fn check_deaths(state: Game) -> Game {
    for clone in state.clones.get_all() {
        if check_death(clone.clone()) {
            state = kill(clone.ID, state.clone())
        }
    }

    state
}

fn check_death(agent: Player) -> bool {
    agent.health == 0
}

fn end_run(state: Game) -> Game {
    state.in_run = false

    state = make_clone(state.player.clone())

    state
}

fn do_button(button: Button, state: Game) -> Game {

}

fn main() {
    let mut in_run = true;
    let mut window: PistonWindow = WindowSettings::new("Piston Window Example", [640, 480]).exit_on_esc(true).build().expect("YOU BAD AT CODE");

    let mut last_update = Instant::now();
    let update_interval = Duration::from_secs_f64(0.01); // Update every second

    let mut pressed_keys = Keys {a: false, s: false, d: false, w:false, special: false, ability: false};

    let mut this_vehicle = 1;

    let mut key_sequence = KeySequence {sequence: vec![], step: 0, length: 0};

    let grass = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/tiles/diving-2328703_1920 2.jpg",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    while let Some(event) = window.next() {
        if let Some(_) = event.update_args() {
            let now = Instant::now();
            if now.duration_since(last_update) >= update_interval {
                last_update = now;
                if game.in_run {
                    //game = update_camera(check_deaths(check_hits(update_enemies(update_bullets(update_players(game.clone()))))))
                    game = update_players(game.clone())
                    game = update_bullets(game.clone())
                    game = update_enemies(game.clone())
                    game = check_hits(game.clone())
                    game = check_deaths(game.clone())
                    game = update_camera(game.clone())
                    if check_death(game.player.clone()) {
                        game = end_run(game.clone())
                    }
                } else {
                    buttons = check_buttons()
                    for button in buttons {
                        game = do_button(button, game.clone())
                    }
                }
                if in_run {
                    key_sequence.sequence.push(pressed_keys.clone());
                    key_sequence.length += 1
                } else {
                    if key_sequence.step < key_sequence.length - 1 {
                        key_sequence.step += 1;
                    }
                }
            }
        }

        
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::A => {
                    pressed_keys.a = true
                }
                Key::S => {
                    pressed_keys.s = true
                }
                Key::D => {
                    pressed_keys.d = true;
                }
                Key::W => {
                    pressed_keys.w = true;
                }
                Key::Q => {
                    pressed_keys.special = true;
                }
                Key::E => {
                    pressed_keys.ability = true;
                }
                Key::Z => {
                    println!("playing back");
                    in_run = false;
                }
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::A => {
                    pressed_keys.a = false
                }
                Key::S => {
                    pressed_keys.s = false
                }
                Key::D => {
                    pressed_keys.d = false
                }
                Key::W => {
                    pressed_keys.w = false
                }
                Key::Q => {
                    pressed_keys.special = false
                }
                Key::E => {
                    pressed_keys.ability = false
                }
                _ => {}
            }
        }
        

        // Draw the window's contents
        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 1.0, 1.0], g); // Clear the screen with white color
            
            let mut keys = pressed_keys.clone();
            if ! in_run {
                keys = key_sequence.sequence[key_sequence.step as usize].clone();
            }
            if keys.w {
                rectangle([1.0, 0.0, 0.0, 1.0], [100.0, 90.0, 10.0, 10.0], c.transform, g);
            }
            if keys.a {
                rectangle([1.0, 0.0, 0.0, 1.0], [90.0, 100.0, 10.0, 10.0], c.transform, g);
            }
            if keys.s {
                rectangle([1.0, 0.0, 0.0, 1.0], [100.0, 100.0, 10.0, 10.0], c.transform, g);
            }
            if keys.d {
                rectangle([1.0, 0.0, 0.0, 1.0], [110.0, 100.0, 10.0, 10.0], c.transform, g);
            }
            if keys.special {
                rectangle([1.0, 0.0, 0.0, 1.0], [90.0, 90.0, 10.0, 10.0], c.transform, g);
            }
            if keys.ability {
                rectangle([1.0, 0.0, 0.0, 1.0], [110.0, 90.0, 10.0, 10.0], c.transform, g);
            }
        });
    }
}
