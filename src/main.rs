use piston_window::*;
use std::time::{Duration, Instant};
use std::rc::Rc;
use rand::Rng;
//people who might play: 
//Haelian (yt)


struct Menu {
    pressed_space: bool,
    buttons: ButtonList,
    go: bool,
    quit: bool,
    screen: u8,
    selected_vehicle: u8,
    artifact1: Artifact,
    artifact2: Artifact,
    artifact3: Artifact,
    health_modifier: u32,
    damage_modifier: u32,
    artifacts: Vec<Artifact>,
    health_boost: u32,
    damage_boost: u32,
    button_screens: ButtonListList,
    vehicles: Vec<Player>,
    keep_old: bool,
}

struct Run {
    artifacts: Vec<Artifact>,
    health_modifier: u32,
    damage_modifier: u32,
}

#[derive(Clone)]
struct CustomButton {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    action: Action,
    image: u32,
    text_image: u32,
}

//used for storing things between runs so the clones of the player function correctly in future runs
#[derive(Clone)]
struct TimeLoopStorage {
    max_camera: f64,
    stored_enemies: Vec<Enemy>,
    stored_platforms: Vec<Platform>,
    end_enemy_cool_down: f64,
    end_platform_cool_down: f64,
    playing: bool,
}

#[derive(Clone)]
struct ButtonList {
    buttons: Vec<CustomButton>,
}

#[derive(Clone)]
struct ButtonListList {
    button_lists: Vec<ButtonList>,
}

#[derive(Clone)]
struct Artifact {
    name: String,
    description: String,
    modify_player: Rc<dyn Fn(&mut Player)>,
    image: u32,
}

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

#[derive(Clone)]
struct Mouse {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct Game {
    random_things: RandomThings,
    player: Player,
    clones: PlayerList,
    player_bullets: BulletList,
    enemy_bullets: BulletList,
    enemies: EnemyList,
    in_run: bool,
    tutorial: u8,
    pressed_keys: Keys,
    platforms: PlatformList,
    mouse: Mouse,
    time_loop_storage: TimeLoopStorage,
    time_loop_reading: TimeLoopStorage,
}

#[derive(Clone)]
struct RandomThings {
    platform_cool_down: f64,
    enemy_cool_down: f64,
    camera_distance: f64,
}

#[derive(Clone)]
struct Action {
    action: String,
}

#[derive(Clone)]
struct Player {
    id: String, //what vehicle the player is using
    x: f64, //position of the player
    y: f64, //position of the player
    width: f64, //the size of the player
    height: f64, //the size of the player
    health: f64, //how much health the player has
    speed: f64, //how fast the player can move
    jump: f64, //how high the player can jump
    data_bool: Vec<bool>, //data used by the player's update function
    data_string: Vec<String>, //data used by the player's update function
    data_num: Vec<f64>, //data used by the player's update function
    moves: KeySequence, //sequence of moves the player has made
    apply_inputs: Rc<dyn Fn(&mut Game)>, //move the player based on the inputs
    apply_inputs_as_clone: Rc<dyn Fn(&mut Game, String)>, //move the player based on the inputs as a clone
    reset: Rc<dyn Fn(&mut Player)>, //reset the player to the starting state
    active: bool, //if the player is currently in the game
    image: u32, //the image of the player
    damage: f64, //how much damage the player does
}

#[derive(Clone)]
struct Bullet {
    x: f64, //position of the bullet
    y: f64, //position of the bullet
    width: f64, //the size of the bullet
    height: f64, //the size of the bullet
    speed: f64, //how fast the bullet moves
    direction: f64, //the direction the bullet is moving
    damage: f64, //how much damage the bullet does
    data_bool: Vec<bool>, //data used by the bullet's update function
    data_string: Vec<String>, //data used by the bullet's update function
    data_num: Vec<f64>, //data used by the bullet's update function
    update: Rc<dyn Fn(u32, &mut Game)>, //update the bullet based on the game state
    id: u32, //the id of the bullet
    image: u32, //the image of the bullet
}

#[derive(Clone)]
struct Enemy {
    x: f64, //position of the enemy
    y: f64, //position of the enemy
    width: f64, //the size of the enemy
    height: f64, //the size of the enemy
    health: f64, //how much health the enemy has
    speed: f64, //how fast the enemy moves
    data_bool: Vec<bool>, //data used by the enemy's update function
    data_string: Vec<String>, //data used by the enemy's update function
    data_num: Vec<f64>, //data used by the enemy's update function
    update: Rc<dyn Fn(u32, &mut Game)>, //update the enemy based on the game state
    id: u32, //the id of the enemy
    image: u32, //the image of the enemy
    shoot_cool_down: f64, //how long until the enemy can shoot again
}

#[derive(Clone)]
struct PlayerList {
    players: Vec<Player>,
    add: Rc<dyn Fn(Player, &mut Game)>,
    get: Rc<dyn Fn(String, &mut Game) -> Player>,
    remove: Rc<dyn Fn(String, &mut Game)>,
}

#[derive(Clone)]
struct BulletList {
    bullets: Vec<Bullet>,
    add: Rc<dyn Fn(Bullet, &mut Game)>,
    get: Rc<dyn Fn(u32, &mut Game) -> Bullet>,
    remove: Rc<dyn Fn(u32, &mut Game)>,
}

#[derive(Clone)]
struct EnemyList {
    enemies: Vec<Enemy>,
    add: Rc<dyn Fn(Enemy, &mut Game)>,
    get: Rc<dyn Fn(u32, &mut Game) -> Enemy>,
    remove: Rc<dyn Fn(u32, &mut Game)>,
    get_ids: Rc<dyn Fn(&mut Game) -> Vec<u32>>,
}

#[derive(Clone)]
struct Platform {
    x: f64, //position of the platform
    y: f64, //position of the platform
    width: f64, //the size of the platform
    height: f64, //the size of the platform
    id: u32, //the id of the platform
    image: u32, //the image of the platform
}

#[derive(Clone)]
struct PlatformList {
    platforms: Vec<Platform>,
    add: Rc<dyn Fn(Platform, &mut Game)>,
    get: Rc<dyn Fn(u32, &mut Game) -> Platform>,
    remove: Rc<dyn Fn(u32, &mut Game)>,
}



fn update_clone(agent_id: String, mut state: &mut Game) {
    let get = state.clones.get.clone();
    ((get)(agent_id.clone(), &mut state).apply_inputs_as_clone)(&mut state, agent_id);
}

fn update_player(mut state: &mut Game) {
    add_inputs(get_inputs(&mut state), &mut state.player);
    let apply_inputs = state.player.apply_inputs.clone();
    (apply_inputs)(&mut state);
}

fn update_player_bullet(bullet_id: u32, mut state: &mut Game) {
    let get = state.player_bullets.get.clone();
    ((get)(bullet_id, state).update)(bullet_id, state);
}

fn update_enemy_bullet(bullet_id: u32, mut state: &mut Game) {
    let get = state.enemy_bullets.get.clone();
    ((get)(bullet_id, state).update)(bullet_id, state);
}

fn update_players(mut state: &mut Game) {
    for clone in state.clones.players.clone() {
        update_clone(clone.id, &mut state);
    }

    update_player(&mut state);
}

fn update_bullets(mut state: &mut Game) {
    for bullet in state.enemy_bullets.bullets.clone() {
        update_enemy_bullet(bullet.id, &mut state);
    }

    for bullet in state.player_bullets.bullets.clone() {
        update_player_bullet(bullet.id, &mut state);
    }

    //remove bullets that are off screen
    let mut remove_ids = vec![];
    for bullet in state.enemy_bullets.bullets.iter() {
        if bullet.x + bullet.width/2.0 < 0.0 {
            remove_ids.push(bullet.id);
        } else if bullet.x - bullet.width/2.0 > 1440.0 {
            remove_ids.push(bullet.id);
        }
    }
    for id in remove_ids {
        let remove = state.enemy_bullets.remove.clone();
        (remove)(id, &mut state);
    }
    remove_ids = vec![];
    for bullet in state.player_bullets.bullets.iter() {
        if bullet.x + bullet.width/2.0 < 0.0 {
            remove_ids.push(bullet.id);
        } else if bullet.x - bullet.width/2.0 > 1440.0 {
            remove_ids.push(bullet.id);
        }
    }
    for id in remove_ids {
        let remove = state.player_bullets.remove.clone();
        (remove)(id, &mut state);
    }
}

fn check_hits(mut state: &mut Game) {
    for bullet in state.enemy_bullets.bullets.clone() {
        if bullet.x + 0.1 > state.player.x - state.player.width/2.0 && bullet.x - 0.1 < state.player.x + state.player.width/2.0 && bullet.y + 0.1 > state.player.y - state.player.height/2.0 && bullet.y - 0.1 < state.player.y + state.player.height/2.0 {
            state.player.health -= bullet.damage;
            let remove = state.enemy_bullets.remove.clone();
            (remove)(bullet.id, &mut state);
        }
    }

    for bullet in state.player_bullets.bullets.clone() {
        for mut enemy in state.enemies.enemies.clone() {
            if bullet.x + 0.1 > enemy.x - 0.1 && bullet.x - 0.1 < enemy.x + 0.1 && bullet.y + 0.1 > enemy.y - 0.1 && bullet.y - 0.1 < enemy.y + 0.1 {
                enemy.health -= bullet.damage;
                let remove = state.player_bullets.remove.clone();
                (remove)(bullet.id, &mut state);
            }
        }
    }
}

fn check_deaths(mut state: &mut Game) {
    for clone in state.clones.players.clone() {
        if check_death(&clone) {
            kill(clone.id, &mut state);
        }
    }
    for enemy in state.enemies.enemies.clone() {
        if enemy.health <= 0.0 {
            let remove = state.enemies.remove.clone();
            (remove)(enemy.id, &mut state);
        }
    }
}

fn check_death(agent: &Player) -> bool {
    agent.health <= 0.0
}

fn end_run(mut state: &mut Game) {
    state.in_run = false;

    make_clone(state.player.clone(), &mut state);

    //put things in the time loop storage
    state.time_loop_storage.max_camera = state.time_loop_storage.max_camera.max(state.random_things.camera_distance);
}

fn do_button(button: Action, mut menu: &mut Menu) {
    if button.action == "replay" {
        menu.go = true;
        menu.keep_old = true;
    } else if button.action == "newplay" {
        menu.go = true;
        menu.keep_old = false;
    } else if button.action == "quit" {
        menu.quit = true;
    } else if button.action == "update vehicle" {
        menu.screen = 2;
    } else if button.action == "new vehicle" {
        menu.screen = 3;
    } else if button.action == "back" {
        menu.screen = 0;
    } else if button.action == "artifact1" {
        menu.artifacts.push(menu.artifact1.clone());
        menu.screen = 1;
    } else if button.action == "artifact2" {
        menu.artifacts.push(menu.artifact2.clone());
        menu.screen = 1;
    } else if button.action == "artifact3" {
        menu.artifacts.push(menu.artifact3.clone());
        menu.screen = 1;
    } else if button.action == "health" {
        menu.health_modifier += menu.health_boost;
    } else if button.action == "damage" {
        menu.damage_modifier += menu.damage_boost;
    } else if button.action == "next vehicle" {
        if menu.selected_vehicle == 5 {
            menu.selected_vehicle = 0;
        } else {
            menu.selected_vehicle += 1;
        }
    } else if button.action == "previous vehicle" {
        if menu.selected_vehicle == 0 {
            menu.selected_vehicle = 5;
        } else {
            menu.selected_vehicle -= 1;
        }
    }
}

fn kill(clone_id: String, mut state: &mut Game) {
    //set clone to inactive
    let get = state.clones.get.clone();
    let mut clone = (get)(clone_id.clone(), state);
    clone.active = false;
    let remove = state.clones.remove.clone();
    (remove)(clone_id, state);
    let add = state.clones.add.clone();
    (add)(clone, state);

}

fn check_buttons(menu: &mut Menu, state: &Game) -> Vec<Action> {
    //if the player is pressing space, return the action of the button they have selected
    if menu.pressed_space {
        let mut actions = vec![];
        for button in menu.button_screens.button_lists[menu.screen as usize].buttons.iter() {
            //take 900-y for the mouse y because the y axis is flipped
            if state.mouse.x > button.x - button.width/2.0 && state.mouse.x < button.x + button.width/2.0 && 900.0 - state.mouse.y > button.y - button.height/2.0 && 900.0 - state.mouse.y < button.y + button.height/2.0 {
                actions.push(button.action.clone());
            }
        }
        menu.pressed_space = false;
        return actions;
    }
    vec![]
}

fn add_inputs(inputs: Keys, player: &mut Player) {
    player.moves.sequence.push(inputs);
    player.moves.length += 1;
}

fn get_inputs(state: &Game) -> Keys {
    state.pressed_keys.clone()
}

fn update_camera(mut state: &mut Game) {
    for clone in state.clones.players.iter_mut() {
        clone.x -= 1.0;
    }
    for bullet in state.player_bullets.bullets.iter_mut() {
        bullet.x -= 1.0;
    }
    for bullet in state.enemy_bullets.bullets.iter_mut() {
        bullet.x -= 1.0;
    }
    for enemy in state.enemies.enemies.iter_mut() {
        enemy.x -= 1.0;
    }
    state.player.x -= 1.0;
    state.random_things.camera_distance += 1.0;
}

fn update_enemies(mut state: &mut Game) {
    let get_ids = state.enemies.get_ids.clone();
    for enemy_id in (get_ids)(state) { 
        let get = state.enemies.get.clone();
        ((get)(enemy_id, state).update)(enemy_id, state);
    }
    let mut remove_ids = vec![];
    for enemy in state.enemies.enemies.clone() {
        if enemy.x + enemy.width/2.0 < 0.0 || enemy.health <= 0.0 {
            remove_ids.push(enemy.id);
        }
    }
    for id in remove_ids {
        let remove = state.enemies.remove.clone();
        (remove)(id, state);
    }

    // if past max camera, add new enemies, otherwise play them back from
    // the time loop storage
    let enemy_delay = 4000.0/(state.random_things.camera_distance+1000.0);
    if state.random_things.camera_distance > state.time_loop_reading.max_camera {
        if state.random_things.enemy_cool_down <= 0.0 {
          let mut used_ids = vec![];
            for enemy in state.enemies.enemies.clone() {
            used_ids.push(enemy.id);
            }
            let mut new_id = 0;
            while used_ids.contains(&new_id) {
                new_id += 1;
            }
            let add = state.enemies.add.clone();
            let enemy = Enemy {x: 1440.0 + rand::thread_rng().gen_range(50..200) as f64, y: rand::thread_rng().gen_range(0..900) as f64, width: 150.0, height: 150.0, health: 5.0, speed: 1.0, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                let get = state.enemies.get.clone();
                let mut enemy = (get)(id, state);
                enemy.x -= enemy.speed;
                let remove = state.enemies.remove.clone();
                (remove)(id, state);
                //check for player collisions
                let mut hit_player = false;
                if enemy.x + enemy.width/2.0 > state.player.x - state.player.width/2.0 && enemy.x - enemy.width/2.0 < state.player.x + state.player.width/2.0 && enemy.y + enemy.height/2.0 > state.player.y - state.player.height/2.0 && enemy.y - enemy.height/2.0 < state.player.y + state.player.height/2.0 {
                    if state.player.id != "Smasher".to_string() {
                        state.player.health -= 10.0;
                    } else {
                        state.player.health -= 5.0;
                    }
                    hit_player = true;
                }
                //check for clone collisions
                for clone in state.clones.players.clone() {
                    if enemy.x + enemy.width/2.0 > clone.x - clone.width/2.0 && enemy.x - enemy.width/2.0 < clone.x + clone.width/2.0 && enemy.y + enemy.height/2.0 > clone.y - clone.height/2.0 && enemy.y - enemy.height/2.0 < clone.y + clone.height/2.0 {
                        hit_player = true;
                        let get = state.clones.get.clone();
                        let mut clone = (get)(clone.id.clone(), state);
                        if clone.id != "Smasher".to_string() {
                            clone.health -= 10.0;
                        } else {
                            clone.health -= 5.0;
                        }
                        let remove = state.clones.remove.clone();
                        (remove)(clone.id.clone(), state);
                        let add = state.clones.add.clone();
                        (add)(clone, state);
                    }
                }
                //shooting
                let mut used_ids = vec![];
                for bullet in state.enemy_bullets.bullets.clone() {
                    used_ids.push(bullet.id);
                }
                let mut new_id = 0;
                while used_ids.contains(&new_id) {
                    new_id += 1;
                }
                let add = state.enemy_bullets.add.clone();
                if enemy.shoot_cool_down <= 0.0 {
                    (add)(Bullet {x: enemy.x.clone(), y: enemy.y.clone(), width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: 1.0, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                        let get = state.enemy_bullets.get.clone();
                        let mut bullet = (get)(id, state);
                        bullet.x -= bullet.speed;
                        let remove = state.enemy_bullets.remove.clone();
                        (remove)(id, state);
                        let add = state.enemy_bullets.add.clone();
                        let overlap = state.platforms.platforms.clone().into_iter().filter(|platform| {
                            bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                        }).collect::<Vec<Platform>>();
                        //check for player collisions
                        let mut hit_player = false;
                        if bullet.x + bullet.width/2.0 > state.player.x - state.player.width/2.0 && bullet.x - bullet.width/2.0 < state.player.x + state.player.width/2.0 && bullet.y + bullet.height/2.0 > state.player.y - state.player.height/2.0 && bullet.y - bullet.height/2.0 < state.player.y + state.player.height/2.0 {
                            state.player.health -= bullet.damage;
                            hit_player = true;
                        }
                        //check for clone collisions
                        for clone in state.clones.players.clone() {
                            if bullet.x + bullet.width/2.0 > clone.x - clone.width/2.0 && bullet.x - bullet.width/2.0 < clone.x + clone.width/2.0 && bullet.y + bullet.height/2.0 > clone.y - clone.height/2.0 && bullet.y - bullet.height/2.0 < clone.y + clone.height/2.0 {
                                hit_player = true;
                                let get = state.clones.get.clone();
                                let mut clone = (get)(clone.id.clone(), state);
                                clone.health -= 10.0;
                                let remove = state.clones.remove.clone();
                                (remove)(clone.id.clone(), state);
                                let add = state.clones.add.clone();
                                (add)(clone, state);
                            }
                        }
                        if overlap.len() == 0 && bullet.x + bullet.width/2.0 > 0.0 && !hit_player {
                            (add)(bullet, state);
                        }
                    }), id: new_id, image: 0}, state);
                    enemy.shoot_cool_down = 1.0;
                } else {
                    enemy.shoot_cool_down -= 1.0/100.0;
                }


                if enemy.x + enemy.width/2.0 > 0.0 && enemy.health > 0.0 && !hit_player {
                    let add = state.enemies.add.clone();
                    (add)(enemy.clone(), state);
                }
            }), id: new_id, image: 0, shoot_cool_down: 0.0};
            (add)(enemy.clone(), state);
            state.time_loop_storage.stored_enemies.push(enemy);
            state.random_things.enemy_cool_down = enemy_delay;
        }
    } else {
        if state.random_things.enemy_cool_down <= 0.0 {
            let mut enemy = if state.time_loop_reading.stored_enemies.len() > 0 {
                state.time_loop_reading.stored_enemies.remove(0)
            } else {
                let mut used_ids = vec![];
                for enemy in state.enemies.enemies.clone() {
                    used_ids.push(enemy.id);
                }
                let mut new_id = 0;
                while used_ids.contains(&new_id) {
                    new_id += 1;
                }
                let add = state.enemies.add.clone();
                let enemy = Enemy {x: 1440.0 + rand::thread_rng().gen_range(50..200) as f64, y: rand::thread_rng().gen_range(0..900) as f64, width: 150.0, height: 150.0, health: 5.0, speed: 1.0, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                    let get = state.enemies.get.clone();
                    let mut enemy = (get)(id, state);
                    enemy.x -= enemy.speed;
                    let remove = state.enemies.remove.clone();
                    (remove)(id, state);
                    //check for player collisions
                    let mut hit_player = false;
                    if enemy.x + enemy.width/2.0 > state.player.x - state.player.width/2.0 && enemy.x - enemy.width/2.0 < state.player.x + state.player.width/2.0 && enemy.y + enemy.height/2.0 > state.player.y - state.player.height/2.0 && enemy.y - enemy.height/2.0 < state.player.y + state.player.height/2.0 {
                        if state.player.id != "Smasher".to_string() {
                            state.player.health -= 10.0;
                        } else {
                            state.player.health -= 5.0;
                        }
                        hit_player = true;
                    }
                    //check for clone collisions
                    for clone in state.clones.players.clone() {
                        if enemy.x + enemy.width/2.0 > clone.x - clone.width/2.0 && enemy.x - enemy.width/2.0 < clone.x + clone.width/2.0 && enemy.y + enemy.height/2.0 > clone.y - clone.height/2.0 && enemy.y - enemy.height/2.0 < clone.y + clone.height/2.0 {
                            hit_player = true;
                            let get = state.clones.get.clone();
                            let mut clone = (get)(clone.id.clone(), state);
                            if clone.id != "Smasher".to_string() {
                                clone.health -= 10.0;
                            } else {
                                clone.health -= 5.0;
                            }
                            let remove = state.clones.remove.clone();
                            (remove)(clone.id.clone(), state);
                            let add = state.clones.add.clone();
                            (add)(clone, state);
                        }
                    }
                    //shooting
                    let mut used_ids = vec![];
                    for bullet in state.enemy_bullets.bullets.clone() {
                        used_ids.push(bullet.id);
                    }
                    let mut new_id = 0;
                    while used_ids.contains(&new_id) {
                        new_id += 1;
                    }
                    let add = state.enemy_bullets.add.clone();
                    if enemy.shoot_cool_down <= 0.0 {
                        (add)(Bullet {x: enemy.x.clone(), y: enemy.y.clone(), width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: 1.0, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                            let get = state.enemy_bullets.get.clone();
                            let mut bullet = (get)(id, state);
                            bullet.x -= bullet.speed;
                            let remove = state.enemy_bullets.remove.clone();
                            (remove)(id, state);
                            let add = state.enemy_bullets.add.clone();
                            let overlap = state.platforms.platforms.clone().into_iter().filter(|platform| {
                                bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                            }).collect::<Vec<Platform>>();
                            //check for player collisions
                            let mut hit_player = false;
                            if bullet.x + bullet.width/2.0 > state.player.x - state.player.width/2.0 && bullet.x - bullet.width/2.0 < state.player.x + state.player.width/2.0 && bullet.y + bullet.height/2.0 > state.player.y - state.player.height/2.0 && bullet.y - bullet.height/2.0 < state.player.y + state.player.height/2.0 {
                                state.player.health -= bullet.damage;
                                hit_player = true;
                            }
                            //check for clone collisions
                            for clone in state.clones.players.clone() {
                                if bullet.x + bullet.width/2.0 > clone.x - clone.width/2.0 && bullet.x - bullet.width/2.0 < clone.x + clone.width/2.0 && bullet.y + bullet.height/2.0 > clone.y - clone.height/2.0 && bullet.y - bullet.height/2.0 < clone.y + clone.height/2.0 {
                                    hit_player = true;
                                    let get = state.clones.get.clone();
                                    let mut clone = (get)(clone.id.clone(), state);
                                    clone.health -= 10.0;
                                    let remove = state.clones.remove.clone();
                                    (remove)(clone.id.clone(), state);
                                    let add = state.clones.add.clone();
                                    (add)(clone, state);
                                }
                            }
                            if overlap.len() == 0 && bullet.x + bullet.width/2.0 > 0.0 && !hit_player {
                                (add)(bullet, state);
                            }
                        }), id: new_id, image: 0}, state);
                        enemy.shoot_cool_down = 1.0;
                    } else {
                        enemy.shoot_cool_down -= 1.0/100.0;
                    }
                    if enemy.x + enemy.width/2.0 > 0.0 && enemy.health > 0.0 && !hit_player {
                        let add = state.enemies.add.clone();
                        (add)(enemy.clone(), state);
                    }
                }), id: new_id, image: 0, shoot_cool_down: 0.0};
                state.time_loop_storage.stored_enemies.push(enemy.clone());
                enemy
            };
            let mut used_ids = vec![];
            for enemy in state.enemies.enemies.clone() {
                used_ids.push(enemy.id);
            }
            let mut new_id = 0;
            while used_ids.contains(&new_id) {
                new_id += 1;
            }
            enemy.id = new_id;
            let add = state.enemies.add.clone();
            (add)(enemy.clone(), state);
            state.random_things.enemy_cool_down = enemy_delay;
        }
    }
    state.random_things.enemy_cool_down -= 1.0/100.0;
}

fn make_clone(mut agent: Player, mut state: &mut Game) {
    let reset = agent.reset.clone();
    (reset)(&mut agent);

    agent.moves.step = 0;

    let add = state.clones.add.clone();
    (add)(agent, state);
}

fn update_platforms(mut state: &mut Game) {
    for platform in state.platforms.platforms.iter_mut() {
        platform.x -= 1.0;
    }

    let mut remove_ids = vec![];

    for platform in state.platforms.platforms.clone() {
        if platform.x + platform.width/2.0 < 0.0 {
            remove_ids.push(platform.id);
        }
    }

    for id in remove_ids {
        let remove = state.platforms.remove.clone();
        (remove)(id, &mut state);
    }

    //if past max camera, add new platforms, otherwise play them back from
    //the time loop storage

    if state.random_things.camera_distance > state.time_loop_reading.max_camera {
        //find all used IDs
        let mut used_ids = vec![];
        for platform in state.platforms.platforms.clone() {
            used_ids.push(platform.id);
        }
        //use a new ID
        let mut new_id = 0;
        while used_ids.contains(&new_id) {
            new_id += 1;
        }
        //add a new platform with a random image
        if state.random_things.platform_cool_down <= 0.0 {
            let image = rand::thread_rng().gen_range(0..3);
            let add = state.platforms.add.clone();
            let y = rand::thread_rng().gen_range(0..900) as f64;
            let width = rand::thread_rng().gen_range(200..400) as f64;
            let height = 50.0;
            let x = 1440.0 + width/2.0;
            let platform = Platform {x: x, y: y, width: width, height: height, id: new_id, image: image};
            (add)(platform.clone(), &mut state);
            //add the platform to the time loop storage
            state.time_loop_storage.stored_platforms.push(platform);
            state.random_things.platform_cool_down = 1.5;
        }
    } else {
        if state.random_things.platform_cool_down <= 0.0 {
            //get the first platform from the time loop storage, if it exists
            let mut platform = if state.time_loop_reading.stored_platforms.len() > 0 {
                state.time_loop_reading.stored_platforms.remove(0)
            } else {
                //if there are no platforms in the time loop storage, add a new one
                let image = rand::thread_rng().gen_range(0..3);
                let y = rand::thread_rng().gen_range(0..900) as f64;
                let width = rand::thread_rng().gen_range(200..400) as f64;
                let height = 50.0;
                let x = 1440.0 + width/2.0;
                let platform = Platform {x: x, y: y, width: width, height: height, id: 0, image: image};
                //add the platform to the time loop storage
                state.time_loop_storage.stored_platforms.push(platform.clone());
                platform
            };
            //find all used IDs
            let mut used_ids = vec![];
            for platform in state.platforms.platforms.clone() {
                used_ids.push(platform.id);
            }
            //use a new ID
            let mut new_id = 0;
            while used_ids.contains(&new_id) {
                new_id += 1;
            }
            platform.id = new_id;
            let add = state.platforms.add.clone();
            (add)(platform, &mut state);
            state.random_things.platform_cool_down = 1.5;
        }
    }
    state.random_things.platform_cool_down -= 1.0/100.0;
}

fn update_time_loop(mut state: &mut Game) {
    if state.time_loop_storage.max_camera == state.random_things.camera_distance {
        state.time_loop_reading.playing = true;
    } else {
        state.time_loop_reading.playing = false;
    }
}

fn update_menu(mut menu: &mut Menu, mut state: &mut Game) {
    //if menu.go, begin a run
    if menu.go {
        menu.go = false;
        menu.screen = 0;
        state.in_run = true;
        //set the player to the selected vehicle, if keep_old is false
        //otherwise, keep the player the same, but apply upgrades to all vehicles in the menu
        if !menu.keep_old {
            state.player = menu.vehicles[menu.selected_vehicle as usize].clone();
            //if the player is the swarm vehicle, set id to data_num[3] and increment data_num[3] for the menu car
            if state.player.id == "Swarm".to_string() {
                state.player.id = state.player.data_num[3].to_string();
                menu.vehicles[menu.selected_vehicle as usize].data_num[3] += 1.0;
            }
        } else {
            for vehicle in menu.vehicles.iter_mut() {
                vehicle.health += menu.health_modifier as f64;
                vehicle.damage += menu.damage_modifier as f64;
            }
        }
        let mut reset = state.player.reset.clone();
        (reset)(&mut state.player);
        for clone in state.clones.players.iter_mut() {
            reset = clone.reset.clone();
            (reset)(clone);
            clone.moves.step = 0;
        }
        let mut remove = state.player_bullets.remove.clone();
        for bullet in state.player_bullets.bullets.clone() {
            (remove)(bullet.id, &mut state);
        }
        remove = state.enemy_bullets.remove.clone();
        for bullet in state.enemy_bullets.bullets.clone() {
            (remove)(bullet.id, &mut state);
        }
        remove = state.enemies.remove.clone();
        for enemy in state.enemies.enemies.clone() {
            (remove)(enemy.id, &mut state);
        }
        remove = state.platforms.remove.clone();
        for platform in state.platforms.platforms.clone() {
            (remove)(platform.id, &mut state);
        }
        state.random_things.camera_distance = 0.0;
        state.random_things.enemy_cool_down = 0.0;
        state.random_things.platform_cool_down = 0.0;
        state.time_loop_reading = state.time_loop_storage.clone();
        //remove all clones with the same ID as the player
        let remove = state.clones.remove.clone();
        (remove)(state.player.id.clone(), &mut state)
    }
    //if menu.quit, quit the game
    if menu.quit {
        menu.quit = false;
        state.in_run = false;
    }
}

fn main() {

    

    let mut game = Game {
        random_things: RandomThings {
            platform_cool_down: 0.0,
            enemy_cool_down: 0.0,
            camera_distance: 0.0,
        },
        player: Player {
            damage: 1.0,
            id: "Base".to_string(),
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 60.0,
            health: 70.0,
            speed: 1.0,
            jump: 50.0,
            data_bool: vec![true, false, false],
            data_string: vec![],
            data_num: vec![0.0, 0.0, 0.0],
            moves: KeySequence {sequence: vec![], step: 0, length: 0},
            apply_inputs: Rc::new(|state: &mut Game| {
                if state.player.moves.sequence[state.player.moves.step as usize].a {
                    state.player.data_num[0] -= state.player.speed;
                }
                if state.player.moves.sequence[state.player.moves.step as usize].s {
                    state.player.data_num[1] = state.player.data_num[1].min(0.0);
                }
                if state.player.moves.sequence[state.player.moves.step as usize].d {
                    state.player.data_num[0] += state.player.speed;
                }
                if state.player.moves.sequence[state.player.moves.step as usize].w {
                    if state.player.data_bool[2] {
                        state.player.data_num[1] = state.player.data_num[1].max(state.player.jump);
                    }
                }
                if state.player.moves.sequence[state.player.moves.step as usize].special {
                    if !state.player.data_bool[1] {
                        state.player.data_bool[1] = true;
                        //invert the shooting boolean
                        state.player.data_bool[0] = !state.player.data_bool[0];
                    }
                } else {
                    state.player.data_bool[1] = false;
                }
                //shooting
                if state.player.data_bool[0] && state.player.data_num[2] <= 0.0 {
                    state.player.data_num[2] = 1.0;
                    //find all used IDs
                    let mut used_ids = vec![];
                    for bullet in state.player_bullets.bullets.clone() {
                        used_ids.push(bullet.id);
                    }
                    //use a new ID
                    let mut new_id = 0;
                    while used_ids.contains(&new_id) {
                        new_id += 1;
                    }
                    //add a new bullet
                    let add = state.player_bullets.add.clone();
                    (add)(Bullet {x: state.player.x, y: state.player.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: state.player.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                        let get = state.player_bullets.get.clone();
                        let mut bullet = (get)(id, state);
                        bullet.x += 10.0;
                        let remove = state.player_bullets.remove.clone();
                        (remove)(id, state);
                        let add = state.player_bullets.add.clone();
                        let overlap = state.platforms.platforms.iter().filter(|platform| {
                            bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                        }).collect::<Vec<&Platform>>();
                        //check for enemy collisions
                        let mut hit_enemies = vec![];
                        for mut enemy in state.enemies.enemies.iter_mut() {
                            if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                enemy.health -= bullet.damage;
                                hit_enemies.push(enemy.id);
                            }
                        }
                        if overlap.len() == 0 && hit_enemies.len() == 0 {
                            (add)(bullet, state);
                        }
                    }), id: new_id, image: 0}, state);
                }
                //apply air resistance
                state.player.data_num[0] *= 0.9;
                state.player.data_num[1] *= 0.9;
                //apply gravity
                state.player.data_num[1] -= 1.0;
                //apply bullet cooldown
                if state.player.data_num[2] > 0.0 {
                    state.player.data_num[2] -= 0.05;
                }
                state.player.data_bool[2] = false;
                //check for platform collisions
                for platform in state.platforms.platforms.iter() {
                    if (state.player.x - platform.x).abs() < platform.width/2.0 + state.player.width/2.0 && (state.player.y - platform.y).abs() < platform.height/2.0 + state.player.height/2.0 {
                        //check which side of the platform is closest
                        let left_overlap = (state.player.x + state.player.width/2.0) - (platform.x - platform.width/2.0);
                        let right_overlap = (platform.x + platform.width/2.0) - (state.player.x - state.player.width/2.0);
                        let bottom_overlap = (state.player.y + state.player.height/2.0) - (platform.y - platform.height/2.0);
                        let top_overlap = (platform.y + platform.height/2.0) - (state.player.y - state.player.height/2.0);
                        //find the smallest overlap
                        let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                        //apply the smallest overlap
                        if smallest_overlap == left_overlap {
                            state.player.data_num[0] = state.player.data_num[0].min(0.0);
                            state.player.x = platform.x - platform.width/2.0 - state.player.width/2.0 + 1.0;
                        } else if smallest_overlap == right_overlap {
                            state.player.data_num[0] = state.player.data_num[0].max(0.0);
                            state.player.x = platform.x + platform.width/2.0 + state.player.width/2.0 - 1.0;
                        } else if smallest_overlap == top_overlap {
                            state.player.data_num[1] = state.player.data_num[1].max(0.0);
                            state.player.y = platform.y + platform.height/2.0 + state.player.height/2.0 - 1.0;
                            state.player.data_bool[2] = true;
                        } else if smallest_overlap == bottom_overlap {
                            state.player.data_num[1] = state.player.data_num[1].min(0.0);
                            state.player.y = platform.y - platform.height/2.0 - state.player.height/2.0 + 1.0;
                        }
                    }
                }
                //check for ground collisions
                if state.player.y - state.player.height/2.0 < 0.0 {
                    state.player.data_bool[2] = true;
                    state.player.data_num[1] = state.player.data_num[1].max(0.0);
                    state.player.y = state.player.height/2.0 - 1.0;
                }
                //move the player
                state.player.x += state.player.data_num[0];
                state.player.y += state.player.data_num[1];
                //update the step
                if state.player.moves.step < state.player.moves.length-1 {
                    state.player.moves.step += 1;
                }
            }),
            apply_inputs_as_clone: Rc::new(|state: &mut Game, id: String| {
                let get = state.clones.get.clone();
                let mut clone = (get)(id, state);
                if clone.moves.sequence[clone.moves.step as usize].a {
                    clone.data_num[0] -= clone.speed;
                }
                if clone.moves.sequence[clone.moves.step as usize].s {
                    clone.data_num[1] = clone.data_num[1].min(0.0);
                }
                if clone.moves.sequence[clone.moves.step as usize].d {
                    clone.data_num[0] += clone.speed;
                }
                if clone.moves.sequence[clone.moves.step as usize].w {
                    if clone.data_bool[2] {
                        clone.data_num[1] = clone.data_num[1].max(clone.jump);
                    }
                }
                if clone.moves.sequence[clone.moves.step as usize].special {
                    if !clone.data_bool[1] {
                        clone.data_bool[1] = true;
                        //invert the shooting boolean
                        clone.data_bool[0] = !clone.data_bool[0];
                    }
                } else {
                    clone.data_bool[1] = false;
                }
                //shooting
                if clone.data_bool[0] && clone.data_num[2] <= 0.0 {
                    clone.data_num[2] = 1.0;
                    //find all used IDs
                    let mut used_ids = vec![];
                    for bullet in state.player_bullets.bullets.clone() {
                        used_ids.push(bullet.id);
                    }
                    //use a new ID
                    let mut new_id = 0;
                    while used_ids.contains(&new_id) {
                        new_id += 1;
                    }
                    //add a new bullet
                    let add = state.player_bullets.add.clone();
                    (add)(Bullet {x: clone.x, y: clone.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: clone.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                        let get = state.player_bullets.get.clone();
                        let mut bullet = (get)(id, state);
                        bullet.x += 10.0;
                        let remove = state.player_bullets.remove.clone();
                        (remove)(id, state);
                        let add = state.player_bullets.add.clone();
                        let overlap = state.platforms.platforms.iter().filter(|platform| {
                            bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                        }).collect::<Vec<&Platform>>();
                        //check for enemy collisions
                        let mut hit_enemies = vec![];
                        for mut enemy in state.enemies.enemies.iter_mut() {
                            if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                enemy.health -= bullet.damage;
                                hit_enemies.push(enemy.id);
                            }
                        }
                        if overlap.len() == 0 && hit_enemies.len() == 0 {
                            (add)(bullet, state);
                        }
                    }), id: new_id, image: 0}, state);
                }
                //apply air resistance
                clone.data_num[0] *= 0.9;
                clone.data_num[1] *= 0.9;
                //apply gravity
                clone.data_num[1] -= 1.0;
                //apply bullet cooldown
                if clone.data_num[2] > 0.0 {
                    clone.data_num[2] -= 0.05;
                }
                clone.data_bool[2] = false;
                //check for platform collisions
                for platform in state.platforms.platforms.iter() {
                    if (clone.x - platform.x).abs() < platform.width/2.0 + clone.width/2.0 && (clone.y - platform.y).abs() < platform.height/2.0 + clone.height/2.0 {
                        //check which side of the platform is closest
                        let left_overlap = (clone.x + clone.width/2.0) - (platform.x - platform.width/2.0);
                        let right_overlap = (platform.x + platform.width/2.0) - (clone.x - clone.width/2.0);
                        let bottom_overlap = (clone.y + clone.height/2.0) - (platform.y - platform.height/2.0);
                        let top_overlap = (platform.y + platform.height/2.0) - (clone.y - clone.height/2.0);
                        //find the smallest overlap
                        let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                        //apply the smallest overlap
                        if smallest_overlap == left_overlap {
                            clone.data_num[0] = clone.data_num[0].min(0.0);
                            clone.x = platform.x - platform.width/2.0 - clone.width/2.0 + 1.0;
                        } else if smallest_overlap == right_overlap {
                            clone.data_num[0] = clone.data_num[0].max(0.0);
                            clone.x = platform.x + platform.width/2.0 + clone.width/2.0 - 1.0;
                        } else if smallest_overlap == top_overlap {
                            clone.data_num[1] = clone.data_num[1].max(0.0);
                            clone.y = platform.y + platform.height/2.0 + clone.height/2.0 - 1.0;
                            clone.data_bool[2] = true;
                        } else if smallest_overlap == bottom_overlap {
                            clone.data_num[1] = clone.data_num[1].min(0.0);
                            clone.y = platform.y - platform.height/2.0 - clone.height/2.0 + 1.0;
                        }
                    }
                }
                //check for ground collisions
                if clone.y - clone.height/2.0 < 0.0 {
                    clone.data_bool[2] = true;
                    clone.data_num[1] = clone.data_num[1].max(0.0);
                    clone.y = clone.height/2.0 - 1.0;
                }
                //move the player
                clone.x += clone.data_num[0];
                clone.y += clone.data_num[1];
                //update the step
                if clone.moves.step < clone.moves.length-1 {
                    clone.moves.step += 1;
                }
                let add = state.clones.add.clone();
                (add)(clone, state);
            }),
            reset: Rc::new(|player: &mut Player| {
                player.x = 0.0;
                player.y = 0.0;
                player.health = 100.0;
                player.data_bool = vec![true, false, false];
                player.data_string = vec![];
                player.data_num = vec![0.0, 0.0, 0.0];
            }),
            active: true,
            image: 2,
        },
        /*player: Player {
            damage: 1.0,
            id: "Base".to_string(),
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 60.0,
            health: 100.0,
            speed: 1.0,
            jump: 50.0,
            data_bool: vec![true, false, false],
            data_string: vec![],
            data_num: vec![0.0, 0.0, 0.0],
            moves: KeySequence {sequence: vec![], step: 0, length: 0},
            apply_inputs: Rc::new(|state: &mut Game| {
                if state.player.moves.sequence[state.player.moves.step as usize].a {
                    state.player.data_num[0] -= state.player.speed;
                }
                if state.player.moves.sequence[state.player.moves.step as usize].s {
                    state.player.data_num[1] = state.player.data_num[1].min(0.0);
                }
                if state.player.moves.sequence[state.player.moves.step as usize].d {
                    state.player.data_num[0] += state.player.speed;
                }
                if state.player.moves.sequence[state.player.moves.step as usize].w {
                    if state.player.data_bool[2] {
                        state.player.data_num[1] = state.player.data_num[1].max(state.player.jump);
                    }
                }
                if state.player.moves.sequence[state.player.moves.step as usize].special {
                    if !state.player.data_bool[1] {
                        state.player.data_bool[1] = true;
                        //invert the shooting boolean
                        state.player.data_bool[0] = !state.player.data_bool[0];
                    }
                } else {
                    state.player.data_bool[1] = false;
                }
                //shooting
                if state.player.data_bool[0] && state.player.data_num[2] <= 0.0 {
                    state.player.data_num[2] = 1.0;
                    //find all used IDs
                    let mut used_ids = vec![];
                    for bullet in state.player_bullets.bullets.clone() {
                        used_ids.push(bullet.id);
                    }
                    //use a new ID
                    let mut new_id = 0;
                    while used_ids.contains(&new_id) {
                        new_id += 1;
                    }
                    //add a new bullet
                    let add = state.player_bullets.add.clone();
                    (add)(Bullet {x: state.player.x, y: state.player.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: state.player.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                        let get = state.player_bullets.get.clone();
                        let mut bullet = (get)(id, state);
                        bullet.x += 10.0;
                        let remove = state.player_bullets.remove.clone();
                        (remove)(id, state);
                        let add = state.player_bullets.add.clone();
                        let overlap = state.platforms.platforms.iter().filter(|platform| {
                            bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                        }).collect::<Vec<&Platform>>();
                        //check for enemy collisions
                        let mut hit_enemies = vec![];
                        for mut enemy in state.enemies.enemies.iter_mut() {
                            if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                enemy.health -= bullet.damage;
                                hit_enemies.push(enemy.id);
                            }
                        }
                        if overlap.len() == 0 && hit_enemies.len() == 0 {
                            (add)(bullet, state);
                        }
                    }), id: new_id, image: 0}, state);
                }
                //apply air resistance
                state.player.data_num[0] *= 0.9;
                state.player.data_num[1] *= 0.9;
                //apply gravity
                state.player.data_num[1] -= 1.0;
                //apply bullet cooldown
                if state.player.data_num[2] > 0.0 {
                    state.player.data_num[2] -= 0.05;
                }
                state.player.data_bool[2] = false;
                //check for platform collisions
                for platform in state.platforms.platforms.iter() {
                    if (state.player.x - platform.x).abs() < platform.width/2.0 + state.player.width/2.0 && (state.player.y - platform.y).abs() < platform.height/2.0 + state.player.height/2.0 {
                        //check which side of the platform is closest
                        let left_overlap = (state.player.x + state.player.width/2.0) - (platform.x - platform.width/2.0);
                        let right_overlap = (platform.x + platform.width/2.0) - (state.player.x - state.player.width/2.0);
                        let bottom_overlap = (state.player.y + state.player.height/2.0) - (platform.y - platform.height/2.0);
                        let top_overlap = (platform.y + platform.height/2.0) - (state.player.y - state.player.height/2.0);
                        //find the smallest overlap
                        let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                        //apply the smallest overlap
                        if smallest_overlap == left_overlap {
                            state.player.data_num[0] = state.player.data_num[0].min(0.0);
                            state.player.x = platform.x - platform.width/2.0 - state.player.width/2.0 + 1.0;
                        } else if smallest_overlap == right_overlap {
                            state.player.data_num[0] = state.player.data_num[0].max(0.0);
                            state.player.x = platform.x + platform.width/2.0 + state.player.width/2.0 - 1.0;
                        } else if smallest_overlap == top_overlap {
                            state.player.data_num[1] = state.player.data_num[1].max(0.0);
                            state.player.y = platform.y + platform.height/2.0 + state.player.height/2.0 - 1.0;
                            state.player.data_bool[2] = true;
                        } else if smallest_overlap == bottom_overlap {
                            state.player.data_num[1] = state.player.data_num[1].min(0.0);
                            state.player.y = platform.y - platform.height/2.0 - state.player.height/2.0 + 1.0;
                        }
                    }
                }
                //check for ground collisions
                if state.player.y - state.player.height/2.0 < 0.0 {
                    state.player.data_bool[2] = true;
                    state.player.data_num[1] = state.player.data_num[1].max(0.0);
                    state.player.y = state.player.height/2.0 - 1.0;
                }
                //move the player
                state.player.x += state.player.data_num[0];
                state.player.y += state.player.data_num[1];
                //update the step
                if state.player.moves.step < state.player.moves.length-1 {
                    state.player.moves.step += 1;
                }
            }),
            apply_inputs_as_clone: Rc::new(|state: &mut Game, id: String| {
                let get = state.clones.get.clone();
                let mut clone = (get)(id, state);
                if clone.moves.sequence[clone.moves.step as usize].a {
                    clone.data_num[0] -= clone.speed;
                }
                if clone.moves.sequence[clone.moves.step as usize].s {
                    clone.data_num[1] = clone.data_num[1].min(0.0);
                }
                if clone.moves.sequence[clone.moves.step as usize].d {
                    clone.data_num[0] += clone.speed;
                }
                if clone.moves.sequence[clone.moves.step as usize].w {
                    if clone.data_bool[2] {
                        clone.data_num[1] = clone.data_num[1].max(clone.jump);
                    }
                }
                if clone.moves.sequence[clone.moves.step as usize].special {
                    if !clone.data_bool[1] {
                        clone.data_bool[1] = true;
                        //invert the shooting boolean
                        clone.data_bool[0] = !clone.data_bool[0];
                    }
                } else {
                    clone.data_bool[1] = false;
                }
                //shooting
                if clone.data_bool[0] && clone.data_num[2] <= 0.0 {
                    clone.data_num[2] = 1.0;
                    //find all used IDs
                    let mut used_ids = vec![];
                    for bullet in state.player_bullets.bullets.clone() {
                        used_ids.push(bullet.id);
                    }
                    //use a new ID
                    let mut new_id = 0;
                    while used_ids.contains(&new_id) {
                        new_id += 1;
                    }
                    //add a new bullet
                    let add = state.player_bullets.add.clone();
                    (add)(Bullet {x: clone.x, y: clone.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: clone.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                        let get = state.player_bullets.get.clone();
                        let mut bullet = (get)(id, state);
                        bullet.x += 10.0;
                        let remove = state.player_bullets.remove.clone();
                        (remove)(id, state);
                        let add = state.player_bullets.add.clone();
                        let overlap = state.platforms.platforms.iter().filter(|platform| {
                            bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                        }).collect::<Vec<&Platform>>();
                        //check for enemy collisions
                        let mut hit_enemies = vec![];
                        for mut enemy in state.enemies.enemies.iter_mut() {
                            if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                enemy.health -= bullet.damage;
                                hit_enemies.push(enemy.id);
                            }
                        }
                        if overlap.len() == 0 && hit_enemies.len() == 0 {
                            (add)(bullet, state);
                        }
                    }), id: new_id, image: 0}, state);
                }
                //apply air resistance
                clone.data_num[0] *= 0.9;
                clone.data_num[1] *= 0.9;
                //apply gravity
                clone.data_num[1] -= 1.0;
                //apply bullet cooldown
                if clone.data_num[2] > 0.0 {
                    clone.data_num[2] -= 0.05;
                }
                clone.data_bool[2] = false;
                //check for platform collisions
                for platform in state.platforms.platforms.iter() {
                    if (clone.x - platform.x).abs() < platform.width/2.0 + clone.width/2.0 && (clone.y - platform.y).abs() < platform.height/2.0 + clone.height/2.0 {
                        //check which side of the platform is closest
                        let left_overlap = (clone.x + clone.width/2.0) - (platform.x - platform.width/2.0);
                        let right_overlap = (platform.x + platform.width/2.0) - (clone.x - clone.width/2.0);
                        let bottom_overlap = (clone.y + clone.height/2.0) - (platform.y - platform.height/2.0);
                        let top_overlap = (platform.y + platform.height/2.0) - (clone.y - clone.height/2.0);
                        //find the smallest overlap
                        let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                        //apply the smallest overlap
                        if smallest_overlap == left_overlap {
                            clone.data_num[0] = clone.data_num[0].min(0.0);
                            clone.x = platform.x - platform.width/2.0 - clone.width/2.0 + 1.0;
                        } else if smallest_overlap == right_overlap {
                            clone.data_num[0] = clone.data_num[0].max(0.0);
                            clone.x = platform.x + platform.width/2.0 + clone.width/2.0 - 1.0;
                        } else if smallest_overlap == top_overlap {
                            clone.data_num[1] = clone.data_num[1].max(0.0);
                            clone.y = platform.y + platform.height/2.0 + clone.height/2.0 - 1.0;
                            clone.data_bool[2] = true;
                        } else if smallest_overlap == bottom_overlap {
                            clone.data_num[1] = clone.data_num[1].min(0.0);
                            clone.y = platform.y - platform.height/2.0 - clone.height/2.0 + 1.0;
                        }
                    }
                }
                //check for ground collisions
                if clone.y - clone.height/2.0 < 0.0 {
                    clone.data_bool[2] = true;
                    clone.data_num[1] = clone.data_num[1].max(0.0);
                    clone.y = clone.height/2.0 - 1.0;
                }
                //move the player
                clone.x += clone.data_num[0];
                clone.y += clone.data_num[1];
                //update the step
                if clone.moves.step < clone.moves.length-1 {
                    clone.moves.step += 1;
                }
                let add = state.clones.add.clone();
                (add)(clone, state);
            }),
            reset: Rc::new(|player: &mut Player| {
                player.x = 0.0;
                player.y = 0.0;
                player.health = 100.0;
                player.data_bool = vec![true, false, false];
                player.data_string = vec![];
                player.data_num = vec![0.0, 0.0, 0.0];
            }),
            active: true,
            image: 1,
        },*/
        clones: PlayerList {
            players: vec![],
            add: Rc::new(|agent: Player, mut state: &mut Game| {
                let mut new = true;
                for player in state.clones.players.iter_mut() {
                    if agent.id == player.id {
                        *player = agent.clone();
                        new = false;
                    }
                }
                if new {
                    state.clones.players.push(agent);
                }
            }),
            get: Rc::new(|id: String, state: &mut Game| -> Player {
                for player in state.clones.players.clone() {
                    if player.id == id {
                        return player;
                    }
                }
                state.clones.players[0].clone()
            }),
            remove: Rc::new(|id: String, mut state: &mut Game| {
                let mut i = 0;
                for player in state.clones.players.clone() {
                    if player.id == id {
                        state.clones.players.remove(i);
                    }
                    i += 1;
                }
            }),
        },
        player_bullets: BulletList {
            bullets: vec![],
            add: Rc::new(|agent: Bullet, mut state: &mut Game| {
                let mut new = true;
                for bullet in state.player_bullets.bullets.iter_mut() {
                    if agent.id == bullet.id {
                        *bullet = agent.clone();
                        new = false;
                    }
                }
                if new {
                    state.player_bullets.bullets.push(agent);
                }
            }),
            get: Rc::new(|id: u32, state: &mut Game| -> Bullet {
                for bullet in state.player_bullets.bullets.clone() {
                    if bullet.id == id {
                        return bullet;
                    }
                }
                state.player_bullets.bullets[0].clone()
            }),
            remove: Rc::new(|id: u32, mut state: &mut Game| {
                let mut i = 0;
                for bullet in state.player_bullets.bullets.clone() {
                    if bullet.id == id {
                        state.player_bullets.bullets.remove(i);
                    }
                    i += 1;
                }
            }),
        },
        enemy_bullets: BulletList {
            bullets: vec![],
            add: Rc::new(|agent: Bullet, mut state: &mut Game| {
                let mut new = true;
                for bullet in state.enemy_bullets.bullets.iter_mut() {
                    if agent.id == bullet.id {
                        *bullet = agent.clone();
                        new = false;
                        break
                    }
                }
                if new {
                    state.enemy_bullets.bullets.push(agent);
                }
            }),
            get: Rc::new(|id: u32, state: &mut Game| -> Bullet {
                for bullet in state.enemy_bullets.bullets.clone() {
                    if bullet.id == id {
                        return bullet;
                    }
                }
                state.enemy_bullets.bullets[0].clone()
            }),
            remove: Rc::new(|id: u32, mut state: &mut Game| {
                let mut i = 0;
                for bullet in state.enemy_bullets.bullets.clone() {
                    if bullet.id == id {
                        state.enemy_bullets.bullets.remove(i);
                    }
                    i += 1;
                }
            }),
        },
        enemies: EnemyList {
            enemies: vec![],
            add: Rc::new(|agent: Enemy, mut state: &mut Game| {
                let mut new = true;
                for enemy in state.enemies.enemies.iter_mut() {
                    if agent.id == enemy.id {
                        *enemy = agent.clone();
                        new = false;
                        break
                    }
                }
                if new {
                    state.enemies.enemies.push(agent);
                }
            }),
            get: Rc::new(|id: u32, state: &mut Game| -> Enemy {
                for enemy in state.enemies.enemies.clone() {
                    if enemy.id == id {
                        return enemy;
                    }
                }
                state.enemies.enemies[0].clone()
            }),
            remove: Rc::new(|id: u32, mut state: &mut Game| {
                let mut i = 0;
                for enemy in state.enemies.enemies.clone() {
                    if enemy.id == id {
                        state.enemies.enemies.remove(i);
                    }
                    i += 1;
                }
            }),
            get_ids: Rc::new(|state: &mut Game| -> Vec<u32> {
                let mut ids = vec![];
                for enemy in state.enemies.enemies.iter() {
                    ids.push(enemy.id);
                }
                ids
            }),
        },
        in_run: true,
        tutorial: 0,
        pressed_keys: Keys {
            a: false, s: false, d: false, w:false, special: false, ability: false
        },
        platforms: PlatformList {
            platforms: vec![],
            add: Rc::new(|agent: Platform, mut state: &mut Game| {
                let mut new = true;
                for platform in state.platforms.platforms.iter_mut() {
                    if agent.id == platform.id {
                        *platform = agent.clone();
                        new = false;
                        break
                    }
                }
                if new {
                    state.platforms.platforms.push(agent);
                }
            }),
            get: Rc::new(|id: u32, state: &mut Game| -> Platform {
                for platform in state.platforms.platforms.clone() {
                    if platform.id == id {
                        return platform;
                    }
                }
                state.platforms.platforms[0].clone()
            }),
            remove: Rc::new(|id: u32, mut state: &mut Game| {
                let mut i = 0;
                for platform in state.platforms.platforms.clone() {
                    if platform.id == id {
                        state.platforms.platforms.remove(i);
                    }
                    i += 1;
                }
            }),
        },
        mouse: Mouse {
            x: 0.0,
            y: 0.0,
        },
        time_loop_storage: TimeLoopStorage {
            max_camera: 0.0,
            stored_enemies: vec![],
            stored_platforms: vec![],
            end_enemy_cool_down: 0.0,
            end_platform_cool_down: 0.0,
            playing: false,
        },
        time_loop_reading: TimeLoopStorage {
            max_camera: 0.0,
            stored_enemies: vec![],
            stored_platforms: vec![],
            end_enemy_cool_down: 0.0,
            end_platform_cool_down: 0.0,
            playing: false,
        },
    };

    let mut menu = Menu {
        pressed_space: false,
        go: false,
        quit: false,
        screen: 0,
        selected_vehicle: 0,
        artifact1: Artifact {
            name: "start1".to_string(),
            description: "start1".to_string(),
            image: 0,
            modify_player: Rc::new(|player: &mut Player| {
                player.health += 10.0;
            }),
        },
        artifact2: Artifact {
            name: "start2".to_string(),
            description: "start2".to_string(),
            image: 0,
            modify_player: Rc::new(|player: &mut Player| {
                player.speed += 10.0;
            }),
        },
        artifact3: Artifact {
            name: "start3".to_string(),
            description: "start3".to_string(),
            image: 0,
            modify_player: Rc::new(|player: &mut Player| {
                player.jump += 10.0;
            }),
        },
        buttons: ButtonList {
            buttons: vec![
                CustomButton {x: 720.0, y: 450.0, width: 200.0, height: 50.0, action: Action {action: "play".to_string()}, image: 2, text_image: 100},
                CustomButton {x: 720.0, y: 400.0, width: 200.0, height: 50.0, action: Action {action: "quit".to_string()}, image: 1, text_image: 4},
                CustomButton {x: 720.0, y: 350.0, width: 200.0, height: 50.0, action: Action {action: "update vehicle".to_string()}, image: 13, text_image: 11},
                CustomButton {x: 720.0, y: 300.0, width: 200.0, height: 50.0, action: Action {action: "new vehicle".to_string()}, image: 12, text_image: 6},
                CustomButton {x: 720.0, y: 250.0, width: 200.0, height: 50.0, action: Action {action: "back".to_string()}, image: 0, text_image: 0},
                CustomButton {x: 720.0, y: 200.0, width: 200.0, height: 50.0, action: Action {action: "artifact1".to_string()}, image: 12, text_image: 0},
                CustomButton {x: 720.0, y: 150.0, width: 200.0, height: 50.0, action: Action {action: "artifact2".to_string()}, image: 12, text_image: 0},
                CustomButton {x: 720.0, y: 100.0, width: 200.0, height: 50.0, action: Action {action: "artifact3".to_string()}, image: 12, text_image: 0},
                CustomButton {x: 720.0, y: 50.0, width: 200.0, height: 50.0, action: Action {action: "health".to_string()}, image: 12, text_image: 9},
                CustomButton {x: 720.0, y: 0.0, width: 200.0, height: 50.0, action: Action {action: "damage".to_string()}, image: 12, text_image: 8},
                CustomButton {x: 720.0, y: 0.0, width: 200.0, height: 50.0, action: Action {action: "next vehicle".to_string()}, image: 11, text_image: 5},
                CustomButton {x: 720.0, y: 0.0, width: 200.0, height: 50.0, action: Action {action: "previous vehicle".to_string()}, image: 11, text_image: 5},
            ],
        },
        button_screens: ButtonListList {
            button_lists: vec![
                ButtonList {
                    buttons: vec![
                        CustomButton {x: 220.0, y: 150.0, width: 200.0, height: 200.0, action: Action {action: "artifact1".to_string()}, image: 12, text_image: 100},
                        CustomButton {x: 720.0, y: 150.0, width: 200.0, height: 200.0, action: Action {action: "artifact2".to_string()}, image: 12, text_image: 100},
                        CustomButton {x: 1220.0, y: 150.0, width: 200.0, height: 200.0, action: Action {action: "artifact3".to_string()}, image: 12, text_image: 100},
                    ],
                },
                ButtonList {
                    buttons: vec![
                        CustomButton {x: 520.0, y: 450.0, width: 200.0, height: 200.0, action: Action {action: "update vehicle".to_string()}, image: 13, text_image: 11},
                        CustomButton {x: 920.0, y: 450.0, width: 200.0, height: 200.0, action: Action {action: "new vehicle".to_string()}, image: 12, text_image: 6},
                    ]
                },
                ButtonList {
                    buttons: vec![
                        CustomButton {x: 520.0, y: 450.0, width: 200.0, height: 200.0, action: Action {action: "health".to_string()}, image: 12, text_image: 9},
                        CustomButton {x: 920.0, y: 450.0, width: 200.0, height: 200.0, action: Action {action: "damage".to_string()}, image: 12, text_image: 8},
                        CustomButton {x: 720.0, y: 300.0, width: 200.0, height: 200.0, action: Action {action: "replay".to_string()}, image: 2, text_image: 100},
                    ]
                },
                ButtonList {
                    buttons: vec![
                        CustomButton {x: 1120.0, y: 450.0, width: 200.0, height: 200.0, action: Action {action: "next vehicle".to_string()}, image: 11, text_image: 5},
                        CustomButton {x: 320.0, y: 450.0, width: 200.0, height: 200.0, action: Action {action: "previous vehicle".to_string()}, image: 11, text_image: 5},
                        CustomButton {x: 720.0, y: 250.0, width: 200.0, height: 200.0, action: Action {action: "newplay".to_string()}, image: 2, text_image: 100},
                    ]
                }
            ]
        },
        health_modifier: 0,
        damage_modifier: 0,
        artifacts: vec![],
        health_boost: 0,
        damage_boost: 0,
        vehicles: vec![
            //relatively mobile, low health, low damage, no ability
            Player {
                damage: 1.0,
                id: "Base".to_string(),
                x: 0.0,
                y: 0.0,
                width: 50.0,
                height: 60.0,
                health: 70.0,
                speed: 1.0,
                jump: 50.0,
                data_bool: vec![true, false, false],
                data_string: vec![],
                data_num: vec![0.0, 0.0, 0.0],
                moves: KeySequence {sequence: vec![], step: 0, length: 0},
                apply_inputs: Rc::new(|state: &mut Game| {
                    if state.player.moves.sequence[state.player.moves.step as usize].a {
                        state.player.data_num[0] -= state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].s {
                        state.player.data_num[1] = state.player.data_num[1].min(0.0);
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].d {
                        state.player.data_num[0] += state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].w {
                        if state.player.data_bool[2] {
                            state.player.data_num[1] = state.player.data_num[1].max(state.player.jump);
                        }
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].special {
                        if !state.player.data_bool[1] {
                            state.player.data_bool[1] = true;
                            //invert the shooting boolean
                            state.player.data_bool[0] = !state.player.data_bool[0];
                        }
                    } else {
                        state.player.data_bool[1] = false;
                    }
                    //shooting
                    if state.player.data_bool[0] && state.player.data_num[2] <= 0.0 {
                        state.player.data_num[2] = 1.0;
                        //find all used IDs
                        let mut used_ids = vec![];
                        for bullet in state.player_bullets.bullets.clone() {
                            used_ids.push(bullet.id);
                        }
                        //use a new ID
                        let mut new_id = 0;
                        while used_ids.contains(&new_id) {
                            new_id += 1;
                        }
                        //add a new bullet
                        let add = state.player_bullets.add.clone();
                        (add)(Bullet {x: state.player.x, y: state.player.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: state.player.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                            let get = state.player_bullets.get.clone();
                            let mut bullet = (get)(id, state);
                            bullet.x += 10.0;
                            let remove = state.player_bullets.remove.clone();
                            (remove)(id, state);
                            let add = state.player_bullets.add.clone();
                            let overlap = state.platforms.platforms.iter().filter(|platform| {
                                bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                            }).collect::<Vec<&Platform>>();
                            //check for enemy collisions
                            let mut hit_enemies = vec![];
                            for mut enemy in state.enemies.enemies.iter_mut() {
                                if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                    enemy.health -= bullet.damage;
                                    hit_enemies.push(enemy.id);
                                }
                            }
                            if overlap.len() == 0 && hit_enemies.len() == 0 {
                                (add)(bullet, state);
                            }
                        }), id: new_id, image: 0}, state);
                    }
                    //apply air resistance
                    state.player.data_num[0] *= 0.9;
                    state.player.data_num[1] *= 0.9;
                    //apply gravity
                    state.player.data_num[1] -= 1.0;
                    //apply bullet cooldown
                    if state.player.data_num[2] > 0.0 {
                        state.player.data_num[2] -= 0.05;
                    }
                    state.player.data_bool[2] = false;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (state.player.x - platform.x).abs() < platform.width/2.0 + state.player.width/2.0 && (state.player.y - platform.y).abs() < platform.height/2.0 + state.player.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (state.player.x + state.player.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (state.player.x - state.player.width/2.0);
                            let bottom_overlap = (state.player.y + state.player.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (state.player.y - state.player.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                state.player.data_num[0] = state.player.data_num[0].min(0.0);
                                state.player.x = platform.x - platform.width/2.0 - state.player.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                state.player.data_num[0] = state.player.data_num[0].max(0.0);
                                state.player.x = platform.x + platform.width/2.0 + state.player.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                state.player.data_num[1] = state.player.data_num[1].max(0.0);
                                state.player.y = platform.y + platform.height/2.0 + state.player.height/2.0 - 1.0;
                                state.player.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                state.player.data_num[1] = state.player.data_num[1].min(0.0);
                                state.player.y = platform.y - platform.height/2.0 - state.player.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if state.player.y - state.player.height/2.0 < 0.0 {
                        state.player.data_bool[2] = true;
                        state.player.data_num[1] = state.player.data_num[1].max(0.0);
                        state.player.y = state.player.height/2.0 - 1.0;
                    }
                    //move the player
                    state.player.x += state.player.data_num[0];
                    state.player.y += state.player.data_num[1];
                    //update the step
                    if state.player.moves.step < state.player.moves.length-1 {
                        state.player.moves.step += 1;
                    }
                }),
                apply_inputs_as_clone: Rc::new(|state: &mut Game, id: String| {
                    let get = state.clones.get.clone();
                    let mut clone = (get)(id, state);
                    if clone.moves.sequence[clone.moves.step as usize].a {
                        clone.data_num[0] -= clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].s {
                        clone.data_num[1] = clone.data_num[1].min(0.0);
                    }
                    if clone.moves.sequence[clone.moves.step as usize].d {
                        clone.data_num[0] += clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].w {
                        if clone.data_bool[2] {
                            clone.data_num[1] = clone.data_num[1].max(clone.jump);
                        }
                    }
                    if clone.moves.sequence[clone.moves.step as usize].special {
                        if !clone.data_bool[1] {
                            clone.data_bool[1] = true;
                            //invert the shooting boolean
                            clone.data_bool[0] = !clone.data_bool[0];
                        }
                    } else {
                        clone.data_bool[1] = false;
                    }
                    //shooting
                    if clone.data_bool[0] && clone.data_num[2] <= 0.0 {
                        clone.data_num[2] = 1.0;
                        //find all used IDs
                        let mut used_ids = vec![];
                        for bullet in state.player_bullets.bullets.clone() {
                            used_ids.push(bullet.id);
                        }
                        //use a new ID
                        let mut new_id = 0;
                        while used_ids.contains(&new_id) {
                            new_id += 1;
                        }
                        //add a new bullet
                        let add = state.player_bullets.add.clone();
                        (add)(Bullet {x: clone.x, y: clone.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: clone.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                            let get = state.player_bullets.get.clone();
                            let mut bullet = (get)(id, state);
                            bullet.x += 10.0;
                            let remove = state.player_bullets.remove.clone();
                            (remove)(id, state);
                            let add = state.player_bullets.add.clone();
                            let overlap = state.platforms.platforms.iter().filter(|platform| {
                                bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                            }).collect::<Vec<&Platform>>();
                            //check for enemy collisions
                            let mut hit_enemies = vec![];
                            for mut enemy in state.enemies.enemies.iter_mut() {
                                if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                    enemy.health -= bullet.damage;
                                    hit_enemies.push(enemy.id);
                                }
                            }
                            if overlap.len() == 0 && hit_enemies.len() == 0 {
                                (add)(bullet, state);
                            }
                        }), id: new_id, image: 0}, state);
                    }
                    //apply air resistance
                    clone.data_num[0] *= 0.9;
                    clone.data_num[1] *= 0.9;
                    //apply gravity
                    clone.data_num[1] -= 1.0;
                    //apply bullet cooldown
                    if clone.data_num[2] > 0.0 {
                        clone.data_num[2] -= 0.05;
                    }
                    clone.data_bool[2] = false;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (clone.x - platform.x).abs() < platform.width/2.0 + clone.width/2.0 && (clone.y - platform.y).abs() < platform.height/2.0 + clone.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (clone.x + clone.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (clone.x - clone.width/2.0);
                            let bottom_overlap = (clone.y + clone.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (clone.y - clone.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                clone.data_num[0] = clone.data_num[0].min(0.0);
                                clone.x = platform.x - platform.width/2.0 - clone.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                clone.data_num[0] = clone.data_num[0].max(0.0);
                                clone.x = platform.x + platform.width/2.0 + clone.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                clone.data_num[1] = clone.data_num[1].max(0.0);
                                clone.y = platform.y + platform.height/2.0 + clone.height/2.0 - 1.0;
                                clone.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                clone.data_num[1] = clone.data_num[1].min(0.0);
                                clone.y = platform.y - platform.height/2.0 - clone.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if clone.y - clone.height/2.0 < 0.0 {
                        clone.data_bool[2] = true;
                        clone.data_num[1] = clone.data_num[1].max(0.0);
                        clone.y = clone.height/2.0 - 1.0;
                    }
                    //move the player
                    clone.x += clone.data_num[0];
                    clone.y += clone.data_num[1];
                    //update the step
                    if clone.moves.step < clone.moves.length-1 {
                        clone.moves.step += 1;
                    }
                    let add = state.clones.add.clone();
                    (add)(clone, state);
                }),
                reset: Rc::new(|player: &mut Player| {
                    player.x = 0.0;
                    player.y = 0.0;
                    player.health = 100.0;
                    player.data_bool = vec![true, false, false];
                    player.data_string = vec![];
                    player.data_num = vec![0.0, 0.0, 0.0];
                }),
                active: true,
                image: 2,
            },
            //high health, collision damage, slow, heals
            Player {
                damage: 0.0,
                id: "Smasher".to_string(),
                x: 0.0,
                y: 0.0,
                width: 50.0,
                height: 60.0,
                health: 500.0,
                speed: 0.5,
                jump: 40.0,
                data_bool: vec![true, false, false],
                data_string: vec![],
                data_num: vec![0.0, 0.0, 0.0],
                moves: KeySequence {sequence: vec![], step: 0, length: 0},
                apply_inputs: Rc::new(|state: &mut Game| {
                    if state.player.moves.sequence[state.player.moves.step as usize].a {
                        state.player.data_num[0] -= state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].s {
                        state.player.data_num[1] = state.player.data_num[1].min(0.0);
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].d {
                        state.player.data_num[0] += state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].w {
                        if state.player.data_bool[2] {
                            state.player.data_num[1] = state.player.data_num[1].max(state.player.jump);
                        }
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].special {
                        //stop moving, but slowly regenerate health
                        state.player.speed = 0.0;
                        if state.player.health < 500.0 {
                            state.player.health += 0.01;
                        }
                    } else {
                        state.player.speed = 0.5;
                    }
                    //no shooting for the smasher, they are melee
                    //apply air resistance
                    state.player.data_num[0] *= 0.9;
                    state.player.data_num[1] *= 0.9;
                    //apply gravity
                    state.player.data_num[1] -= 1.0;
                    //apply bullet cooldown
                    if state.player.data_num[2] > 0.0 {
                        state.player.data_num[2] -= 0.05;
                    }
                    state.player.data_bool[2] = false;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (state.player.x - platform.x).abs() < platform.width/2.0 + state.player.width/2.0 && (state.player.y - platform.y).abs() < platform.height/2.0 + state.player.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (state.player.x + state.player.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (state.player.x - state.player.width/2.0);
                            let bottom_overlap = (state.player.y + state.player.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (state.player.y - state.player.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                state.player.data_num[0] = state.player.data_num[0].min(0.0);
                                state.player.x = platform.x - platform.width/2.0 - state.player.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                state.player.data_num[0] = state.player.data_num[0].max(0.0);
                                state.player.x = platform.x + platform.width/2.0 + state.player.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                state.player.data_num[1] = state.player.data_num[1].max(0.0);
                                state.player.y = platform.y + platform.height/2.0 + state.player.height/2.0 - 1.0;
                                state.player.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                state.player.data_num[1] = state.player.data_num[1].min(0.0);
                                state.player.y = platform.y - platform.height/2.0 - state.player.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if state.player.y - state.player.height/2.0 < 0.0 {
                        state.player.data_bool[2] = true;
                        state.player.data_num[1] = state.player.data_num[1].max(0.0);
                        state.player.y = state.player.height/2.0 - 1.0;
                    }
                    //move the player
                    state.player.x += state.player.data_num[0];
                    state.player.y += state.player.data_num[1];
                    //update the step
                    if state.player.moves.step < state.player.moves.length-1 {
                        state.player.moves.step += 1;
                    }
                }),
                apply_inputs_as_clone: Rc::new(|state: &mut Game, id: String| {
                    let get = state.clones.get.clone();
                    let mut clone = (get)(id, state);
                    if clone.moves.sequence[clone.moves.step as usize].a {
                        clone.data_num[0] -= clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].s {
                        clone.data_num[1] = clone.data_num[1].min(0.0);
                    }
                    if clone.moves.sequence[clone.moves.step as usize].d {
                        clone.data_num[0] += clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].w {
                        if clone.data_bool[2] {
                            clone.data_num[1] = clone.data_num[1].max(clone.jump);
                        }
                    }
                    if clone.moves.sequence[clone.moves.step as usize].special {
                        //stop moving, but slowly regenerate health
                        clone.speed = 0.0;
                        if clone.health < 500.0 {
                            clone.health += 0.01;
                        }
                    } else {
                        clone.speed = 0.5;
                    }
                    //no shooting for the smasher, they are melee
                    //apply air resistance
                    clone.data_num[0] *= 0.9;
                    clone.data_num[1] *= 0.9;
                    //apply gravity
                    clone.data_num[1] -= 1.0;
                    //apply bullet cooldown
                    if clone.data_num[2] > 0.0 {
                        clone.data_num[2] -= 0.05;
                    }
                    clone.data_bool[2] = false;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (clone.x - platform.x).abs() < platform.width/2.0 + clone.width/2.0 && (clone.y - platform.y).abs() < platform.height/2.0 + clone.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (clone.x + clone.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (clone.x - clone.width/2.0);
                            let bottom_overlap = (clone.y + clone.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (clone.y - clone.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                clone.data_num[0] = clone.data_num[0].min(0.0);
                                clone.x = platform.x - platform.width/2.0 - clone.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                clone.data_num[0] = clone.data_num[0].max(0.0);
                                clone.x = platform.x + platform.width/2.0 + clone.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                clone.data_num[1] = clone.data_num[1].max(0.0);
                                clone.y = platform.y + platform.height/2.0 + clone.height/2.0 - 1.0;
                                clone.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                clone.data_num[1] = clone.data_num[1].min(0.0);
                                clone.y = platform.y - platform.height/2.0 - clone.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if clone.y - clone.height/2.0 < 0.0 {
                        clone.data_bool[2] = true;
                        clone.data_num[1] = clone.data_num[1].max(0.0);
                        clone.y = clone.height/2.0 - 1.0;
                    }
                    //move the player
                    clone.x += clone.data_num[0];
                    clone.y += clone.data_num[1];
                    //update the step
                    if clone.moves.step < clone.moves.length-1 {
                        clone.moves.step += 1;
                    }
                    let add = state.clones.add.clone();
                    (add)(clone, state);
                }),
                reset: Rc::new(|player: &mut Player| {
                    player.x = 0.0;
                    player.y = 0.0;
                    player.health = 100.0;
                    player.data_bool = vec![true, false, false];
                    player.data_string = vec![];
                    player.data_num = vec![0.0, 0.0, 0.0];
                }),
                active: true,
                image: 0,
            },
            //high damage arcing projectiles, low health, slow, shoots explosive projectile on ability
            Player {
                damage: 100.0,
                id: "Cannoneer".to_string(),
                x: 0.0,
                y: 0.0,
                width: 50.0,
                height: 60.0,
                health: 70.0,
                speed: 0.5,
                jump: 50.0,
                data_bool: vec![true, false, false],
                data_string: vec![],
                data_num: vec![0.0, 0.0, 0.0, 0.0],
                moves: KeySequence {sequence: vec![], step: 0, length: 0},
                apply_inputs: Rc::new(|state: &mut Game| {
                    if state.player.moves.sequence[state.player.moves.step as usize].a {
                        state.player.data_num[0] -= state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].s {
                        state.player.data_num[1] = state.player.data_num[1].min(0.0);
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].d {
                        state.player.data_num[0] += state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].w {
                        if state.player.data_bool[2] {
                            state.player.data_num[1] = state.player.data_num[1].max(state.player.jump);
                        }
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].special {
                        if !state.player.data_bool[1] {
                            if state.player.data_num[2] <= 0.0 {
                                state.player.data_bool[1] = true;
                                //shoot an explosive arcing projectile
                                //find all used IDs
                                let mut used_ids = vec![];
                                for bullet in state.player_bullets.bullets.clone() {
                                    used_ids.push(bullet.id);
                                }
                                //use a new ID
                                let mut new_id = 0;
                                while used_ids.contains(&new_id) {
                                    new_id += 1;
                                }
                                //add a new bullet
                                let add = state.player_bullets.add.clone();
                                (add)(Bullet {x: state.player.x, y: state.player.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: state.player.damage, data_bool: vec![], data_string: vec![], data_num: vec![3.0, 6.0], update: Rc::new(|id: u32, mut state: &mut Game| {
                                    let get = state.player_bullets.get.clone();
                                    let mut bullet = (get)(id, state);
                                    bullet.x += bullet.data_num[0];
                                    bullet.y += bullet.data_num[1];
                                    bullet.data_num[1] -= 1.0/15.0;
                                    let remove = state.player_bullets.remove.clone();
                                    (remove)(id, state);
                                    let add = state.player_bullets.add.clone();
                                    let overlap = state.platforms.platforms.iter().filter(|platform| {
                                        bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                                    }).collect::<Vec<&Platform>>();
                                    //check for enemy collisions
                                    let mut hit_enemies = vec![];
                                    for mut enemy in state.enemies.enemies.iter_mut() {
                                        if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                            enemy.health -= bullet.damage;
                                            hit_enemies.push(enemy.id);
                                        }
                                    }
                                    if overlap.len() == 0 && hit_enemies.len() == 0 {
                                        (add)(bullet, state);
                                    } else {
                                        //HUGE explosion, damaging all enemies near the bullet
                                        for mut enemy in state.enemies.enemies.iter_mut() {
                                            if (bullet.x - enemy.x).abs() < 300.0 && (bullet.y - enemy.y).abs() < 300.0 {
                                                enemy.health -= bullet.damage;
                                            }
                                        }
                                    }
                                }), id: new_id, image: 0}, state);
                                state.player.data_num[2] = 5.0;
                            }
                        } else {
                            state.player.data_bool[1] = false;
                        }
                    }
                    state.player.data_bool[2] = false;
                    if state.player.data_num[2] > 0.0 {
                        state.player.data_num[2] -= 0.05;
                    }
                    //no basic shooting for the cannoneer
                    //apply air resistance
                    state.player.data_num[0] *= 0.9;
                    state.player.data_num[1] *= 0.9;
                    //apply gravity
                    state.player.data_num[1] -= 1.0;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (state.player.x - platform.x).abs() < platform.width/2.0 + state.player.width/2.0 && (state.player.y - platform.y).abs() < platform.height/2.0 + state.player.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (state.player.x + state.player.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (state.player.x - state.player.width/2.0);
                            let bottom_overlap = (state.player.y + state.player.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (state.player.y - state.player.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                state.player.data_num[0] = state.player.data_num[0].min(0.0);
                                state.player.x = platform.x - platform.width/2.0 - state.player.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                state.player.data_num[0] = state.player.data_num[0].max(0.0);
                                state.player.x = platform.x + platform.width/2.0 + state.player.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                state.player.data_num[1] = state.player.data_num[1].max(0.0);
                                state.player.y = platform.y + platform.height/2.0 + state.player.height/2.0 - 1.0;
                                state.player.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                state.player.data_num[1] = state.player.data_num[1].min(0.0);
                                state.player.y = platform.y - platform.height/2.0 - state.player.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if state.player.y - state.player.height/2.0 < 0.0 {
                        state.player.data_bool[2] = true;
                        state.player.data_num[1] = state.player.data_num[1].max(0.0);
                        state.player.y = state.player.height/2.0 - 1.0;
                    }
                    //move the player
                    state.player.x += state.player.data_num[0];
                    state.player.y += state.player.data_num[1];
                    //update the step
                    if state.player.moves.step < state.player.moves.length-1 {
                        state.player.moves.step += 1;
                    }
                }),
                apply_inputs_as_clone: Rc::new(|state: &mut Game, id: String| {
                    let get = state.clones.get.clone();
                    let mut clone = (get)(id, state);
                    if clone.moves.sequence[clone.moves.step as usize].a {
                        clone.data_num[0] -= clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].s {
                        clone.data_num[1] = clone.data_num[1].min(0.0);
                    }
                    if clone.moves.sequence[clone.moves.step as usize].d {
                        clone.data_num[0] += clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].w {
                        if clone.data_bool[2] {
                            clone.data_num[1] = clone.data_num[1].max(clone.jump);
                        }
                    }
                    if clone.moves.sequence[clone.moves.step as usize].special {
                        if !clone.data_bool[1] {
                            if clone.data_num[2] <= 0.0 {
                                clone.data_bool[1] = true;
                                //shoot an explosive projectile
                                //find all used IDs
                                let mut used_ids = vec![];
                                for bullet in state.player_bullets.bullets.clone() {
                                    used_ids.push(bullet.id);
                                }
                                //use a new ID
                                let mut new_id = 0;
                                while used_ids.contains(&new_id) {
                                    new_id += 1;
                                }
                                //add a new bullet
                                let add = state.player_bullets.add.clone();
                                (add)(Bullet {x: clone.x, y: clone.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: clone.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                                    let get = state.player_bullets.get.clone();
                                    let mut bullet = (get)(id, state);
                                    bullet.x += 10.0;
                                    let remove = state.player_bullets.remove.clone();
                                    (remove)(id, state);
                                    let add = state.player_bullets.add.clone();
                                    let overlap = state.platforms.platforms.iter().filter(|platform| {
                                        bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                                    }).collect::<Vec<&Platform>>();
                                    //check for enemy collisions
                                    let mut hit_enemies = vec![];
                                    for mut enemy in state.enemies.enemies.iter_mut() {
                                        if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                            enemy.health -= bullet.damage;
                                            hit_enemies.push(enemy.id);
                                        }
                                    }
                                    if overlap.len() == 0 && hit_enemies.len() == 0 {
                                        (add)(bullet, state);
                                    } else {
                                        //large explosion, damaging all enemies near the bullet
                                        for mut enemy in state.enemies.enemies.iter_mut() {
                                            if (bullet.x - enemy.x).abs() < 50.0 && (bullet.y - enemy.y).abs() < 50.0 {
                                                enemy.health -= bullet.damage;
                                            }
                                        }
                                    }
                                }), id: new_id, image: 0}, state);
                                clone.data_num[2] = 20.0;
                            }
                        } else {
                            clone.data_bool[1] = false;
                        }
                    }
                    clone.data_bool[2] = false;
                    if clone.data_num[2] > 0.0 {
                        clone.data_num[2] -= 0.05;
                    }
                    //shooting
                    if clone.data_bool[0] && clone.data_num[3] <= 0.0 {
                        //find all used IDs
                        let mut used_ids = vec![];
                        for bullet in state.player_bullets.bullets.clone() {
                            used_ids.push(bullet.id);
                        }
                        //use a new ID
                        let mut new_id = 0;
                        while used_ids.contains(&new_id) {
                            new_id += 1;
                        }
                        //add a new bullet
                        let add = state.player_bullets.add.clone();
                        (add)(Bullet {x: clone.x, y: clone.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: clone.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                            let get = state.player_bullets.get.clone();
                            let mut bullet = (get)(id, state);
                            bullet.x += 10.0;
                            let remove = state.player_bullets.remove.clone();
                            (remove)(id, state);
                            let add = state.player_bullets.add.clone();
                            let overlap = state.platforms.platforms.iter().filter(|platform| {
                                bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                            }).collect::<Vec<&Platform>>();
                            //check for enemy collisions
                            let mut hit_enemies = vec![];
                            for mut enemy in state.enemies.enemies.iter_mut() {
                                if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                    enemy.health -= bullet.damage;
                                    hit_enemies.push(enemy.id);
                                }
                            }
                            if overlap.len() == 0 && hit_enemies.len() == 0 {
                                (add)(bullet, state);
                            }
                        }), id: new_id, image: 0}, state);
                        //apply bullet cooldown
                        clone.data_num[3] = 1.0;
                    }
                    //apply air resistance
                    clone.data_num[0] *= 0.9;
                    clone.data_num[1] *= 0.9;
                    //apply gravity
                    clone.data_num[1] -= 1.0;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (clone.x - platform.x).abs() < platform.width/2.0 + clone.width/2.0 && (clone.y - platform.y).abs() < platform.height/2.0 + clone.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (clone.x + clone.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (clone.x - clone.width/2.0);
                            let bottom_overlap = (clone.y + clone.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (clone.y - clone.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                clone.data_num[0] = clone.data_num[0].min(0.0);
                                clone.x = platform.x - platform.width/2.0 - clone.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                clone.data_num[0] = clone.data_num[0].max(0.0);
                                clone.x = platform.x + platform.width/2.0 + clone.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                clone.data_num[1] = clone.data_num[1].max(0.0);
                                clone.y = platform.y + platform.height/2.0 + clone.height/2.0 - 1.0;
                                clone.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                clone.data_num[1] = clone.data_num[1].min(0.0);
                                clone.y = platform.y - platform.height/2.0 - clone.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if clone.y - clone.height/2.0 < 0.0 {
                        clone.data_bool[2] = true;
                        clone.data_num[1] = clone.data_num[1].max(0.0);
                        clone.y = clone.height/2.0 - 1.0;
                    }
                    //move the player
                    clone.x += clone.data_num[0];
                    clone.y += clone.data_num[1];
                    //update the step
                    if clone.moves.step < clone.moves.length-1 {
                        clone.moves.step += 1;
                    }
                    let add = state.clones.add.clone();
                    (add)(clone, state);
                }),
                reset: Rc::new(|player: &mut Player| {
                    player.x = 0.0;
                    player.y = 0.0;
                    player.health = 70.0;
                    player.data_bool = vec![true, false, false];
                    player.data_string = vec![];
                    player.data_num = vec![0.0, 0.0, 0.0, 0.0];
                }),
                active: true,
                image: 3,
            },
            //overall quite weak, but there can be multiple clones of it, so it just looks weak here
            Player {
                damage: 0.7,
                id: "Swarm".to_string(),
                x: 0.0,
                y: 0.0,
                width: 50.0,
                height: 60.0,
                health: 50.0,
                speed: 0.8,
                jump: 35.0,
                data_bool: vec![true, false, false],
                data_string: vec![],
                data_num: vec![0.0, 0.0, 0.0, 1.0],
                moves: KeySequence {sequence: vec![], step: 0, length: 0},
                apply_inputs: Rc::new(|state: &mut Game| {
                    if state.player.moves.sequence[state.player.moves.step as usize].a {
                        state.player.data_num[0] -= state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].s {
                        state.player.data_num[1] = state.player.data_num[1].min(0.0);
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].d {
                        state.player.data_num[0] += state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].w {
                        if state.player.data_bool[2] {
                            state.player.data_num[1] = state.player.data_num[1].max(state.player.jump);
                        }
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].special {
                        if !state.player.data_bool[1] {
                            state.player.data_bool[1] = true;
                            //invert the shooting boolean
                            state.player.data_bool[0] = !state.player.data_bool[0];
                        }
                    } else {
                        state.player.data_bool[1] = false;
                    }
                    //shooting
                    if state.player.data_bool[0] && state.player.data_num[2] <= 0.0 {
                        state.player.data_num[2] = 1.0;
                        //find all used IDs
                        let mut used_ids = vec![];
                        for bullet in state.player_bullets.bullets.clone() {
                            used_ids.push(bullet.id);
                        }
                        //use a new ID
                        let mut new_id = 0;
                        while used_ids.contains(&new_id) {
                            new_id += 1;
                        }
                        //add a new bullet
                        let add = state.player_bullets.add.clone();
                        (add)(Bullet {x: state.player.x, y: state.player.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: state.player.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                            let get = state.player_bullets.get.clone();
                            let mut bullet = (get)(id, state);
                            bullet.x += 10.0;
                            let remove = state.player_bullets.remove.clone();
                            (remove)(id, state);
                            let add = state.player_bullets.add.clone();
                            let overlap = state.platforms.platforms.iter().filter(|platform| {
                                bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                            }).collect::<Vec<&Platform>>();
                            //check for enemy collisions
                            let mut hit_enemies = vec![];
                            for mut enemy in state.enemies.enemies.iter_mut() {
                                if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                    enemy.health -= bullet.damage;
                                    hit_enemies.push(enemy.id);
                                }
                            }
                            if overlap.len() == 0 && hit_enemies.len() == 0 {
                                (add)(bullet, state);
                            }
                        }), id: new_id, image: 0}, state);
                    }
                    //apply air resistance
                    state.player.data_num[0] *= 0.9;
                    state.player.data_num[1] *= 0.9;
                    //apply gravity
                    state.player.data_num[1] -= 1.0;
                    //apply bullet cooldown
                    if state.player.data_num[2] > 0.0 {
                        state.player.data_num[2] -= 0.05;
                    }
                    state.player.data_bool[2] = false;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (state.player.x - platform.x).abs() < platform.width/2.0 + state.player.width/2.0 && (state.player.y - platform.y).abs() < platform.height/2.0 + state.player.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (state.player.x + state.player.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (state.player.x - state.player.width/2.0);
                            let bottom_overlap = (state.player.y + state.player.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (state.player.y - state.player.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                state.player.data_num[0] = state.player.data_num[0].min(0.0);
                                state.player.x = platform.x - platform.width/2.0 - state.player.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                state.player.data_num[0] = state.player.data_num[0].max(0.0);
                                state.player.x = platform.x + platform.width/2.0 + state.player.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                state.player.data_num[1] = state.player.data_num[1].max(0.0);
                                state.player.y = platform.y + platform.height/2.0 + state.player.height/2.0 - 1.0;
                                state.player.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                state.player.data_num[1] = state.player.data_num[1].min(0.0);
                                state.player.y = platform.y - platform.height/2.0 - state.player.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if state.player.y - state.player.height/2.0 < 0.0 {
                        state.player.data_bool[2] = true;
                        state.player.data_num[1] = state.player.data_num[1].max(0.0);
                        state.player.y = state.player.height/2.0 - 1.0;
                    }
                    //move the player
                    state.player.x += state.player.data_num[0];
                    state.player.y += state.player.data_num[1];
                    //update the step
                    if state.player.moves.step < state.player.moves.length-1 {
                        state.player.moves.step += 1;
                    }
                }),
                apply_inputs_as_clone: Rc::new(|state: &mut Game, id: String| {
                    let get = state.clones.get.clone();
                    let mut clone = (get)(id, state);
                    if clone.moves.sequence[clone.moves.step as usize].a {
                        clone.data_num[0] -= clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].s {
                        clone.data_num[1] = clone.data_num[1].min(0.0);
                    }
                    if clone.moves.sequence[clone.moves.step as usize].d {
                        clone.data_num[0] += clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].w {
                        if clone.data_bool[2] {
                            clone.data_num[1] = clone.data_num[1].max(clone.jump);
                        }
                    }
                    if clone.moves.sequence[clone.moves.step as usize].special {
                        if !clone.data_bool[1] {
                            clone.data_bool[1] = true;
                            //invert the shooting boolean
                            clone.data_bool[0] = !clone.data_bool[0];
                        }
                    } else {
                        clone.data_bool[1] = false;
                    }
                    //shooting
                    if clone.data_bool[0] && clone.data_num[2] <= 0.0 {
                        clone.data_num[2] = 1.8;
                        //find all used IDs
                        let mut used_ids = vec![];
                        for bullet in state.player_bullets.bullets.clone() {
                            used_ids.push(bullet.id);
                        }
                        //use a new ID
                        let mut new_id = 0;
                        while used_ids.contains(&new_id) {
                            new_id += 1;
                        }
                        //add a new bullet
                        let add = state.player_bullets.add.clone();
                        (add)(Bullet {x: clone.x, y: clone.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: clone.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                            let get = state.player_bullets.get.clone();
                            let mut bullet = (get)(id, state);
                            bullet.x += 10.0;
                            let remove = state.player_bullets.remove.clone();
                            (remove)(id, state);
                            let add = state.player_bullets.add.clone();
                            let overlap = state.platforms.platforms.iter().filter(|platform| {
                                bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                            }).collect::<Vec<&Platform>>();
                            //check for enemy collisions
                            let mut hit_enemies = vec![];
                            for mut enemy in state.enemies.enemies.iter_mut() {
                                if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                    enemy.health -= bullet.damage;
                                    hit_enemies.push(enemy.id);
                                }
                            }
                            if overlap.len() == 0 && hit_enemies.len() == 0 {
                                (add)(bullet, state);
                            }
                        }), id: new_id, image: 0}, state);
                    }
                    //apply air resistance
                    clone.data_num[0] *= 0.9;
                    clone.data_num[1] *= 0.9;
                    //apply gravity
                    clone.data_num[1] -= 1.0;
                    //apply bullet cooldown
                    if clone.data_num[2] > 0.0 {
                        clone.data_num[2] -= 0.05;
                    }
                    clone.data_bool[2] = false;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (clone.x - platform.x).abs() < platform.width/2.0 + clone.width/2.0 && (clone.y - platform.y).abs() < platform.height/2.0 + clone.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (clone.x + clone.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (clone.x - clone.width/2.0);
                            let bottom_overlap = (clone.y + clone.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (clone.y - clone.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                clone.data_num[0] = clone.data_num[0].min(0.0);
                                clone.x = platform.x - platform.width/2.0 - clone.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                clone.data_num[0] = clone.data_num[0].max(0.0);
                                clone.x = platform.x + platform.width/2.0 + clone.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                clone.data_num[1] = clone.data_num[1].max(0.0);
                                clone.y = platform.y + platform.height/2.0 + clone.height/2.0 - 1.0;
                                clone.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                clone.data_num[1] = clone.data_num[1].min(0.0);
                                clone.y = platform.y - platform.height/2.0 - clone.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if clone.y - clone.height/2.0 < 0.0 {
                        clone.data_bool[2] = true;
                        clone.data_num[1] = clone.data_num[1].max(0.0);
                        clone.y = clone.height/2.0 - 1.0;
                    }
                    //move the player
                    clone.x += clone.data_num[0];
                    clone.y += clone.data_num[1];
                    //update the step
                    if clone.moves.step < clone.moves.length-1 {
                        clone.moves.step += 1;
                    }
                    let add = state.clones.add.clone();
                    (add)(clone, state);
                }),
                reset: Rc::new(|player: &mut Player| {
                    player.x = 0.0;
                    player.y = 0.0;
                    player.health = 50.0;
                    player.data_bool = vec![true, false, false];
                    player.data_string = vec![];
                    player.data_num = vec![0.0, 0.0, 0.0];
                }),
                active: true,
                image: 4,
            },
            //high agility
            Player {
                damage: 0.8,
                id: "mcQueen".to_string(),
                x: 0.0,
                y: 0.0,
                width: 50.0,
                height: 60.0,
                health: 65.0,
                speed: 2.0,
                jump: 70.0,
                data_bool: vec![true, false, false],
                data_string: vec![],
                data_num: vec![0.0, 0.0, 0.0],
                moves: KeySequence {sequence: vec![], step: 0, length: 0},
                apply_inputs: Rc::new(|state: &mut Game| {
                    if state.player.moves.sequence[state.player.moves.step as usize].a {
                        state.player.data_num[0] -= state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].s {
                        state.player.data_num[1] = state.player.data_num[1].min(0.0);
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].d {
                        state.player.data_num[0] += state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].w {
                        if state.player.data_bool[2] {
                            state.player.data_num[1] = state.player.data_num[1].max(state.player.jump);
                        }
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].special {
                        if !state.player.data_bool[1] {
                            state.player.data_bool[1] = true;
                            //invert the shooting boolean
                            state.player.data_bool[0] = !state.player.data_bool[0];
                        }
                    } else {
                        state.player.data_bool[1] = false;
                    }
                    //shooting
                    if state.player.data_bool[0] && state.player.data_num[2] <= 0.0 {
                        state.player.data_num[2] = 1.0;
                        //find all used IDs
                        let mut used_ids = vec![];
                        for bullet in state.player_bullets.bullets.clone() {
                            used_ids.push(bullet.id);
                        }
                        //use a new ID
                        let mut new_id = 0;
                        while used_ids.contains(&new_id) {
                            new_id += 1;
                        }
                        //add a new bullet
                        let add = state.player_bullets.add.clone();
                        (add)(Bullet {x: state.player.x, y: state.player.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: state.player.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                            let get = state.player_bullets.get.clone();
                            let mut bullet = (get)(id, state);
                            bullet.x += 10.0;
                            let remove = state.player_bullets.remove.clone();
                            (remove)(id, state);
                            let add = state.player_bullets.add.clone();
                            let overlap = state.platforms.platforms.iter().filter(|platform| {
                                bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                            }).collect::<Vec<&Platform>>();
                            //check for enemy collisions
                            let mut hit_enemies = vec![];
                            for mut enemy in state.enemies.enemies.iter_mut() {
                                if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                    enemy.health -= bullet.damage;
                                    hit_enemies.push(enemy.id);
                                }
                            }
                            if overlap.len() == 0 && hit_enemies.len() == 0 {
                                (add)(bullet, state);
                            }
                        }), id: new_id, image: 0}, state);
                    }
                    //apply air resistance
                    state.player.data_num[0] *= 0.9;
                    state.player.data_num[1] *= 0.9;
                    //apply gravity
                    state.player.data_num[1] -= 1.0;
                    //apply bullet cooldown
                    if state.player.data_num[2] > 0.0 {
                        state.player.data_num[2] -= 0.05;
                    }
                    state.player.data_bool[2] = false;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (state.player.x - platform.x).abs() < platform.width/2.0 + state.player.width/2.0 && (state.player.y - platform.y).abs() < platform.height/2.0 + state.player.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (state.player.x + state.player.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (state.player.x - state.player.width/2.0);
                            let bottom_overlap = (state.player.y + state.player.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (state.player.y - state.player.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                state.player.data_num[0] = state.player.data_num[0].min(0.0);
                                state.player.x = platform.x - platform.width/2.0 - state.player.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                state.player.data_num[0] = state.player.data_num[0].max(0.0);
                                state.player.x = platform.x + platform.width/2.0 + state.player.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                state.player.data_num[1] = state.player.data_num[1].max(0.0);
                                state.player.y = platform.y + platform.height/2.0 + state.player.height/2.0 - 1.0;
                                state.player.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                state.player.data_num[1] = state.player.data_num[1].min(0.0);
                                state.player.y = platform.y - platform.height/2.0 - state.player.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if state.player.y - state.player.height/2.0 < 0.0 {
                        state.player.data_bool[2] = true;
                        state.player.data_num[1] = state.player.data_num[1].max(0.0);
                        state.player.y = state.player.height/2.0 - 1.0;
                    }
                    //move the player
                    state.player.x += state.player.data_num[0];
                    state.player.y += state.player.data_num[1];
                    //update the step
                    if state.player.moves.step < state.player.moves.length-1 {
                        state.player.moves.step += 1;
                    }
                }),
                apply_inputs_as_clone: Rc::new(|state: &mut Game, id: String| {
                    let get = state.clones.get.clone();
                    let mut clone = (get)(id, state);
                    if clone.moves.sequence[clone.moves.step as usize].a {
                        clone.data_num[0] -= clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].s {
                        clone.data_num[1] = clone.data_num[1].min(0.0);
                    }
                    if clone.moves.sequence[clone.moves.step as usize].d {
                        clone.data_num[0] += clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].w {
                        if clone.data_bool[2] {
                            clone.data_num[1] = clone.data_num[1].max(clone.jump);
                        }
                    }
                    if clone.moves.sequence[clone.moves.step as usize].special {
                        if !clone.data_bool[1] {
                            clone.data_bool[1] = true;
                            //invert the shooting boolean
                            clone.data_bool[0] = !clone.data_bool[0];
                        }
                    } else {
                        clone.data_bool[1] = false;
                    }
                    //shooting
                    if clone.data_bool[0] && clone.data_num[2] <= 0.0 {
                        clone.data_num[2] = 1.0;
                        //find all used IDs
                        let mut used_ids = vec![];
                        for bullet in state.player_bullets.bullets.clone() {
                            used_ids.push(bullet.id);
                        }
                        //use a new ID
                        let mut new_id = 0;
                        while used_ids.contains(&new_id) {
                            new_id += 1;
                        }
                        //add a new bullet
                        let add = state.player_bullets.add.clone();
                        (add)(Bullet {x: clone.x, y: clone.y, width: 10.0, height: 10.0, speed: 10.0, direction: 0.0, damage: clone.damage, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {
                            let get = state.player_bullets.get.clone();
                            let mut bullet = (get)(id, state);
                            bullet.x += 10.0;
                            let remove = state.player_bullets.remove.clone();
                            (remove)(id, state);
                            let add = state.player_bullets.add.clone();
                            let overlap = state.platforms.platforms.iter().filter(|platform| {
                                bullet.x + bullet.width/2.0 > platform.x - platform.width/2.0 && bullet.x - bullet.width/2.0 < platform.x + platform.width/2.0 && bullet.y + bullet.height/2.0 > platform.y - platform.height/2.0 && bullet.y - bullet.height/2.0 < platform.y + platform.height/2.0
                            }).collect::<Vec<&Platform>>();
                            //check for enemy collisions
                            let mut hit_enemies = vec![];
                            for mut enemy in state.enemies.enemies.iter_mut() {
                                if bullet.x + bullet.width/2.0 > enemy.x - enemy.width/2.0 && bullet.x - bullet.width/2.0 < enemy.x + enemy.width/2.0 && bullet.y + bullet.height/2.0 > enemy.y - enemy.height/2.0 && bullet.y - bullet.height/2.0 < enemy.y + enemy.height/2.0 {
                                    enemy.health -= bullet.damage;
                                    hit_enemies.push(enemy.id);
                                }
                            }
                            if overlap.len() == 0 && hit_enemies.len() == 0 {
                                (add)(bullet, state);
                            }
                        }), id: new_id, image: 0}, state);
                    }
                    //apply air resistance
                    clone.data_num[0] *= 0.9;
                    clone.data_num[1] *= 0.9;
                    //apply gravity
                    clone.data_num[1] -= 1.0;
                    //apply bullet cooldown
                    if clone.data_num[2] > 0.0 {
                        clone.data_num[2] -= 0.05;
                    }
                    clone.data_bool[2] = false;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (clone.x - platform.x).abs() < platform.width/2.0 + clone.width/2.0 && (clone.y - platform.y).abs() < platform.height/2.0 + clone.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (clone.x + clone.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (clone.x - clone.width/2.0);
                            let bottom_overlap = (clone.y + clone.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (clone.y - clone.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                clone.data_num[0] = clone.data_num[0].min(0.0);
                                clone.x = platform.x - platform.width/2.0 - clone.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                clone.data_num[0] = clone.data_num[0].max(0.0);
                                clone.x = platform.x + platform.width/2.0 + clone.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                clone.data_num[1] = clone.data_num[1].max(0.0);
                                clone.y = platform.y + platform.height/2.0 + clone.height/2.0 - 1.0;
                                clone.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                clone.data_num[1] = clone.data_num[1].min(0.0);
                                clone.y = platform.y - platform.height/2.0 - clone.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if clone.y - clone.height/2.0 < 0.0 {
                        clone.data_bool[2] = true;
                        clone.data_num[1] = clone.data_num[1].max(0.0);
                        clone.y = clone.height/2.0 - 1.0;
                    }
                    //move the player
                    clone.x += clone.data_num[0];
                    clone.y += clone.data_num[1];
                    //update the step
                    if clone.moves.step < clone.moves.length-1 {
                        clone.moves.step += 1;
                    }
                    let add = state.clones.add.clone();
                    (add)(clone, state);
                }),
                reset: Rc::new(|player: &mut Player| {
                    player.x = 0.0;
                    player.y = 0.0;
                    player.health = 65.0;
                    player.data_bool = vec![true, false, false];
                    player.data_string = vec![];
                    player.data_num = vec![0.0, 0.0, 0.0];
                }),
                active: true,
                image: 5,
            },
            //mid health, doesn't shoot, generates kills over time
            Player {
                damage: 1.0,
                id: "Generator".to_string(),
                x: 0.0,
                y: 0.0,
                width: 50.0,
                height: 60.0,
                health: 70.0,
                speed: 1.0,
                jump: 50.0,
                data_bool: vec![true, false, false],
                data_string: vec![],
                data_num: vec![0.0, 0.0, 0.0],
                moves: KeySequence {sequence: vec![], step: 0, length: 0},
                apply_inputs: Rc::new(|state: &mut Game| {
                    if state.player.moves.sequence[state.player.moves.step as usize].a {
                        state.player.data_num[0] -= state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].s {
                        state.player.data_num[1] = state.player.data_num[1].min(0.0);
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].d {
                        state.player.data_num[0] += state.player.speed;
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].w {
                        if state.player.data_bool[2] {
                            state.player.data_num[1] = state.player.data_num[1].max(state.player.jump);
                        }
                    }
                    if state.player.moves.sequence[state.player.moves.step as usize].special {
                        if !state.player.data_bool[1] {
                            state.player.data_bool[1] = true;
                            //invert the speed generation boolean
                            state.player.data_bool[0] = !state.player.data_bool[0];
                            if state.player.data_bool[0] {
                                state.player.speed = 0.0;
                            } else {
                                state.player.speed = 1.0;
                            }
                        }
                    } else {
                        state.player.data_bool[1] = false;
                    }
                    //apply air resistance
                    state.player.data_num[0] *= 0.9;
                    state.player.data_num[1] *= 0.9;
                    //apply gravity
                    state.player.data_num[1] -= 1.0;
                    //apply bullet cooldown
                    if state.player.data_num[2] > 0.0 {
                        state.player.data_num[2] -= 0.05;
                    }
                    state.player.data_bool[2] = false;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (state.player.x - platform.x).abs() < platform.width/2.0 + state.player.width/2.0 && (state.player.y - platform.y).abs() < platform.height/2.0 + state.player.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (state.player.x + state.player.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (state.player.x - state.player.width/2.0);
                            let bottom_overlap = (state.player.y + state.player.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (state.player.y - state.player.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                state.player.data_num[0] = state.player.data_num[0].min(0.0);
                                state.player.x = platform.x - platform.width/2.0 - state.player.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                state.player.data_num[0] = state.player.data_num[0].max(0.0);
                                state.player.x = platform.x + platform.width/2.0 + state.player.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                state.player.data_num[1] = state.player.data_num[1].max(0.0);
                                state.player.y = platform.y + platform.height/2.0 + state.player.height/2.0 - 1.0;
                                state.player.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                state.player.data_num[1] = state.player.data_num[1].min(0.0);
                                state.player.y = platform.y - platform.height/2.0 - state.player.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if state.player.y - state.player.height/2.0 < 0.0 {
                        state.player.data_bool[2] = true;
                        state.player.data_num[1] = state.player.data_num[1].max(0.0);
                        state.player.y = state.player.height/2.0 - 1.0;
                    }
                    //move the player
                    state.player.x += state.player.data_num[0];
                    state.player.y += state.player.data_num[1];
                    //update the step
                    if state.player.moves.step < state.player.moves.length-1 {
                        state.player.moves.step += 1;
                    }
                }),
                apply_inputs_as_clone: Rc::new(|state: &mut Game, id: String| {
                    let get = state.clones.get.clone();
                    let mut clone = (get)(id, state);
                    if clone.moves.sequence[clone.moves.step as usize].a {
                        clone.data_num[0] -= clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].s {
                        clone.data_num[1] = clone.data_num[1].min(0.0);
                    }
                    if clone.moves.sequence[clone.moves.step as usize].d {
                        clone.data_num[0] += clone.speed;
                    }
                    if clone.moves.sequence[clone.moves.step as usize].w {
                        if clone.data_bool[2] {
                            clone.data_num[1] = clone.data_num[1].max(clone.jump);
                        }
                    }
                    if clone.moves.sequence[clone.moves.step as usize].special {
                        if !clone.data_bool[1] {
                            clone.data_bool[1] = true;
                            //invert the speed generation boolean
                            clone.data_bool[0] = !clone.data_bool[0];
                            if clone.data_bool[0] {
                                clone.speed = 0.0;
                            } else {
                                clone.speed = 1.0;
                            }
                        }
                    } else {
                        clone.data_bool[1] = false;
                    }
                    //apply air resistance
                    clone.data_num[0] *= 0.9;
                    clone.data_num[1] *= 0.9;
                    //apply gravity
                    clone.data_num[1] -= 1.0;
                    //apply bullet cooldown
                    if clone.data_num[2] > 0.0 {
                        clone.data_num[2] -= 0.05;
                    }
                    clone.data_bool[2] = false;
                    //check for platform collisions
                    for platform in state.platforms.platforms.iter() {
                        if (clone.x - platform.x).abs() < platform.width/2.0 + clone.width/2.0 && (clone.y - platform.y).abs() < platform.height/2.0 + clone.height/2.0 {
                            //check which side of the platform is closest
                            let left_overlap = (clone.x + clone.width/2.0) - (platform.x - platform.width/2.0);
                            let right_overlap = (platform.x + platform.width/2.0) - (clone.x - clone.width/2.0);
                            let bottom_overlap = (clone.y + clone.height/2.0) - (platform.y - platform.height/2.0);
                            let top_overlap = (platform.y + platform.height/2.0) - (clone.y - clone.height/2.0);
                            //find the smallest overlap
                            let smallest_overlap = left_overlap.min(right_overlap).min(top_overlap).min(bottom_overlap);
                            //apply the smallest overlap
                            if smallest_overlap == left_overlap {
                                clone.data_num[0] = clone.data_num[0].min(0.0);
                                clone.x = platform.x - platform.width/2.0 - clone.width/2.0 + 1.0;
                            } else if smallest_overlap == right_overlap {
                                clone.data_num[0] = clone.data_num[0].max(0.0);
                                clone.x = platform.x + platform.width/2.0 + clone.width/2.0 - 1.0;
                            } else if smallest_overlap == top_overlap {
                                clone.data_num[1] = clone.data_num[1].max(0.0);
                                clone.y = platform.y + platform.height/2.0 + clone.height/2.0 - 1.0;
                                clone.data_bool[2] = true;
                            } else if smallest_overlap == bottom_overlap {
                                clone.data_num[1] = clone.data_num[1].min(0.0);
                                clone.y = platform.y - platform.height/2.0 - clone.height/2.0 + 1.0;
                            }
                        }
                    }
                    //check for ground collisions
                    if clone.y - clone.height/2.0 < 0.0 {
                        clone.data_bool[2] = true;
                        clone.data_num[1] = clone.data_num[1].max(0.0);
                        clone.y = clone.height/2.0 - 1.0;
                    }
                    //move the player
                    clone.x += clone.data_num[0];
                    clone.y += clone.data_num[1];
                    //update the step
                    if clone.moves.step < clone.moves.length-1 {
                        clone.moves.step += 1;
                    }
                    let add = state.clones.add.clone();
                    (add)(clone, state);
                }),
                reset: Rc::new(|player: &mut Player| {
                    player.x = 0.0;
                    player.y = 0.0;
                    player.health = 70.0;
                    player.data_bool = vec![true, false, false];
                    player.data_string = vec![];
                    player.data_num = vec![0.0, 0.0, 0.0];
                }),
                active: true,
                image: 1,
            }
        ],
        keep_old: false,
    };
    //don't add blank things for indexing
        //add a blank move to the player
        //game.player.moves.sequence.push(Keys {a: false, s: false, d: false, w:false, special: false, ability: false});

        //add a blank player to the list
        //let add = game.clones.add.clone();
        //(add)(Player {height: 0.0, width: 0.0, jump: 0.0, x: 0.0, y: 0.0, health: 0.0, speed: 0.0, data_bool: vec![], data_string: vec![], data_num: vec![], moves: KeySequence {sequence: vec![Keys {a: false, s: false, d: false, w:false, special: false, ability: false}], step: 0, length: 1}, apply_inputs: Rc::new(|state: &mut Game| {}), apply_inputs_as_clone: Rc::new(|state: &mut Game, id: String| {}), reset: Rc::new(|player: &mut Player| {}), active: true, image: 0, id: "0".to_string()}, &mut game);

        //add a blank platform to the list
        //let add = game.platforms.add.clone();
        //(add)(Platform {x: 0.0, y: 0.0, width: 0.0, height: 0.0, id: 0, image: 0}, &mut game);

        //add a blank bullet to both lists
        //let add = game.player_bullets.add.clone();
        //(add)(Bullet {x: 0.0, y: 0.0, width: 0.0, height: 0.0, speed: 0.0, direction: 0.0, damage: 0.0, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {}), id: 0, image: 0}, &mut game);
        //let add = game.enemy_bullets.add.clone();
        //(add)(Bullet {x: 0.0, y: 0.0, width: 0.0, height: 0.0, speed: 0.0, direction: 0.0, damage: 0.0, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {}), id: 0, image: 0}, &mut game);

        //add a blank enemy to the list
        //let add = game.enemies.add.clone();
        //(add)(Enemy {x: 0.0, y: 0.0, health: 0.0, speed: 0.0, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: &mut Game| {}), id: 0, image: 0, width: 0.0, height: 0.0, shoot_cool_down: 0.0}, &mut game);

        //update the player's image
        //game.player.image = 1;

    let mut in_run = true;
    let mut window: PistonWindow = WindowSettings::new(
            "Chronodrive: Cycle of Steel", 
            [1440, 900]
        )
        .exit_on_esc(true)
        .build()
        .expect("window failed to build");

    let mut last_update = Instant::now();
    let update_interval = Duration::from_secs_f64(0.01); //update interval in seconds

    let mut this_vehicle = 1;

    let mut key_sequence = KeySequence {sequence: vec![], step: 0, length: 0};

    
    //load images
        //get the background image
            let background = Texture::from_path(
            &mut window.create_texture_context(),
            "assets/images/background2.jpeg",
            Flip::None,
            &TextureSettings::new(),
            ).expect("background image failed to load");



        //make a list of the platform images
        let platform1 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/platform1.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("platform1 image failed to load");

        let platform2 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/platform2.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("platform2 image failed to load");

        let platform3 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/platform3.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("platform3 image failed to load");

        let platform_images = vec![platform1, platform2, platform3];



        //make a list of the player images
        let player1 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player1.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("player1 image failed to load");

        let player2 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player2.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("player2 image failed to load");

        let player3 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player3.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("player3 image failed to load");

        let player4 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player4.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("player4 image failed to load");

        let player5 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player5.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("player5 image failed to load");

        let player6 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player6.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("player6 image failed to load");

        let player_images = vec![player1, player2, player3, player4, player5, player6];



        //make a list of the enemy images
        let enemy = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/enemy.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("enemy image failed to load");

        let enemy_images = vec![enemy];



        //make a list of the bullet images
        let bullet = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/bullet.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("bullet image failed to load");

        let bullet_images = vec![bullet];



        //get the menu image
        let menu_image = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/menu.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("menu image failed to load");



        //get the mouse image
        let mouse = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/cursor.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("mouse image failed to load");
        //get the button images
        let button_back = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/back.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_loop = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/loop.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_play_1 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/play_1.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_play_2 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/play_2.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_play_3 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/play_3.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_play_4 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/play_4.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_play_5 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/play_5.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_play_6 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/play_6.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_play_7 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/play_7.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_slack = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/slack.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_slider = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/slider.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_switch_1 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/switch_1.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_switch_2 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/switch_2.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let button_upgrade = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/buttons/upgrade.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button image failed to load");

        let buttons = vec![button_back, button_loop, button_play_1, button_play_2, button_play_3, button_play_4, button_play_5, button_play_6, button_play_7, button_slack, button_slider, button_switch_1, button_switch_2, button_upgrade];

        //get the button text images
        let button_text_batte = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/batte.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_bealth = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/bealth.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_damage = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/damage.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_end_run_1 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/end_run_1.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_end_run_2 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/end_run_2.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_next = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/next.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_switch = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/switch.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_upgrade_daalge = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/upgrade_daalge.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_upgrade_damage = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/upgrade_damage.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_upgrade_health_1 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/upgrade_health_1.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_upgrade_health_2 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/upgrade_health_2.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_text_upgrade = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/buttons/text/upgrade.png",
        Flip::None,
        &TextureSettings::new(),
        ).expect("button text image failed to load");

        let button_texts = vec![button_text_batte, button_text_bealth, button_text_damage, button_text_end_run_1, button_text_end_run_2, button_text_next, button_text_switch, button_text_upgrade_daalge, button_text_upgrade_damage, button_text_upgrade_health_1, button_text_upgrade_health_2, button_text_upgrade];

    while let Some(event) = window.next() {
        if let Some(_) = event.update_args() {
            let now = Instant::now();
            if now.duration_since(last_update) >= update_interval {
                last_update = now;
                if game.in_run {
                    update_players(&mut game);
                    update_bullets(&mut game);
                    update_enemies(&mut game);
                    check_hits(&mut game);
                    check_deaths(&mut game);
                    update_platforms(&mut game);
                    update_camera(&mut game);
                    update_time_loop(&mut game);
                    if check_death(&game.player) {
                        end_run(&mut game);
                        menu.screen = 0;
                    }
                } else {
                    if in_run {
                        let mut buttons = check_buttons(&mut menu, &game);
                        for button in buttons {
                            do_button(button, &mut menu);
                        }
                        update_menu(&mut menu, &mut game);
                    } else {

                    }
                }
                if in_run {
                    key_sequence.sequence.push(game.pressed_keys.clone());
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
                    game.pressed_keys.a = true;
                }
                Key::S => {
                    game.pressed_keys.s = true
                }
                Key::D => {
                    game.pressed_keys.d = true;
                }
                Key::W => {
                    game.pressed_keys.w = true;
                }
                Key::Q => {
                    game.pressed_keys.special = true;
                }
                Key::E => {
                    game.pressed_keys.ability = true;
                }
                Key::Z => {
                    in_run = false;
                }
                Key::Space => {
                    menu.pressed_space = true;
                }
                //end run button for testing
                Key::X => {
                    game.player.health = 0.0;
                }
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::A => {
                    game.pressed_keys.a = false
                }
                Key::S => {
                    game.pressed_keys.s = false
                }
                Key::D => {
                    game.pressed_keys.d = false
                }
                Key::W => {
                    game.pressed_keys.w = false
                }
                Key::Q => {
                    game.pressed_keys.special = false
                }
                Key::E => {
                    game.pressed_keys.ability = false
                }
                Key::Space => {
                    menu.pressed_space = false;
                }
                _ => {}
            }
        }

        //track the mouse
        if let Some(pos) = event.mouse_cursor_args() {
            game.mouse.x = pos[0];
            game.mouse.y = pos[1];
        }
        

        // Draw the window's contents
        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 1.0, 1.0], g); // Clear the screen with white color

            if game.in_run {

                //draw the background at full size
                let image_size = background.get_size();
                image(&background, c.transform.scale(1440.0/(image_size.0 as f64), 900.0/(image_size.1 as f64)), g);
                
                //draw the platforms
                for platform in game.platforms.platforms.iter() {
                    let image_size = platform_images[platform.image as usize].get_size();
                    image(&platform_images[platform.image as usize], c.transform.scale(platform.width/(image_size.0 as f64), platform.height/(image_size.1 as f64)).trans((platform.x - platform.width/2.0)/platform.width*(image_size.0 as f64), (900.0 - (platform.y + platform.height/2.0))/platform.height*(image_size.1 as f64)), g);
                }

                //draw the player
                let image_size = player_images[game.player.image as usize].get_size();
                image(&player_images[game.player.image as usize], c.transform.scale(game.player.width/(image_size.0 as f64), game.player.height/(image_size.1 as f64)).trans((game.player.x - game.player.width/2.0)/game.player.width*(image_size.0 as f64), (900.0 - (game.player.y + game.player.height/2.0))/game.player.height*(image_size.1 as f64)), g);

                //draw the clones
                for clone in game.clones.players.iter() {
                    let image_size = player_images[clone.image as usize].get_size();
                    image(&player_images[clone.image as usize], c.transform.scale(game.player.width/(image_size.0 as f64), game.player.height/(image_size.1 as f64)).trans((clone.x - clone.width/2.0)/clone.width*(image_size.0 as f64), (900.0 - (clone.y + clone.height/2.0))/clone.height*(image_size.1 as f64)), g);
                }

                //draw the player bullets
                for bullet in game.player_bullets.bullets.iter() {
                    let image_size = bullet_images[bullet.image as usize].get_size();
                    image(&bullet_images[bullet.image as usize], c.transform.scale(bullet.width/(image_size.0 as f64), bullet.height/(image_size.1 as f64)).trans((bullet.x - bullet.width/2.0)/bullet.width*(image_size.0 as f64), (900.0 - (bullet.y + bullet.height/2.0))/bullet.height*(image_size.1 as f64)), g);
                }

                //draw the enemy bullets
                for bullet in game.enemy_bullets.bullets.iter() {
                    let image_size = bullet_images[bullet.image as usize].get_size();
                    image(&bullet_images[bullet.image as usize], c.transform.scale(bullet.width/(image_size.0 as f64), bullet.height/(image_size.1 as f64)).trans((bullet.x - bullet.width/2.0)/bullet.width*(image_size.0 as f64), (900.0 - (bullet.y + bullet.height/2.0))/bullet.height*(image_size.1 as f64)), g);
                }

                //draw the enemies
                for enemy in game.enemies.enemies.iter() {
                    let image_size = enemy_images[enemy.image as usize].get_size();
                    image(&enemy_images[enemy.image as usize], c.transform.scale(enemy.width/(image_size.0 as f64), enemy.height/(image_size.1 as f64)).trans((enemy.x - enemy.width/2.0)/enemy.width*(image_size.0 as f64), (900.0 - (enemy.y + enemy.height/2.0))/enemy.height*(image_size.1 as f64)), g);
                }
            } else {
                //draw the menu background, filling the screen
                let image_size = menu_image.get_size();
                image(&menu_image, c.transform.scale(1440.0/(image_size.0 as f64), 900.0/(image_size.1 as f64)), g);
                //draw the buttons
                for button in menu.button_screens.button_lists[menu.screen as usize].buttons.iter() {
                    let image_size = buttons[button.image as usize].get_size();
                    image(&buttons[button.image as usize], c.transform.scale(button.width/(image_size.0 as f64), button.height/(image_size.1 as f64)).trans((button.x - button.width/2.0)/button.width*(image_size.0 as f64), (900.0 - (button.y + button.height/2.0))/button.height*(image_size.1 as f64)), g);
                }
                //draw the button text 0.8* button height below the buttons, at half the height but the correct aspect ratio
                for button in menu.button_screens.button_lists[menu.screen as usize].buttons.iter() {
                    if button.text_image != 100 {
                        let button_size = button.height;
                        let image_size = button_texts[button.text_image as usize].get_size();
                        let aspect_ratio = image_size.0 as f64 / image_size.1 as f64;
                        let height = button_size / 3.0;
                        let width = height * aspect_ratio;
                        let y_offset = button_size;
                        image(&button_texts[button.text_image as usize], c.transform.scale(width/(image_size.0 as f64), height/(image_size.1 as f64)).trans((button.x - width/2.0)/width*(image_size.0 as f64), (900.0 - (button.y + button.height/2.0) + y_offset)/height*(image_size.1 as f64)), g);
                    }
                }

                //if on the vehicle select screen, draw the selected vehicle
                if menu.screen == 3 {
                    let player_image = &player_images[menu.vehicles[menu.selected_vehicle as usize].image as usize];
                    let player_width = menu.vehicles[menu.selected_vehicle as usize].width;
                    let player_height = menu.vehicles[menu.selected_vehicle as usize].height;
                    let image_size = player_image.get_size();
                    image(player_image, c.transform.scale(player_width/(image_size.0 as f64), player_height/(image_size.1 as f64)).trans((720.0 - player_width/2.0)/player_width*(image_size.0 as f64), (450.0 - player_height/2.0)/player_height*(image_size.1 as f64)), g);
                }
            }
            //draw the mouse
            let x_scale = 0.1;
            let y_scale = 0.1;
            image(&mouse, c.transform.scale(x_scale, y_scale).trans(game.mouse.x/x_scale, game.mouse.y/y_scale), g);
        });
    }
}
