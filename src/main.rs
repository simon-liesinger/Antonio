use piston_window::*;
use std::time::{Duration, Instant};
use std::rc::Rc;

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
struct Game {
    player: Player,
    clones: PlayerList,
    player_bullets: BulletList,
    enemy_bullets: BulletList,
    enemies: EnemyList,
    in_run: bool,
    tutorial: u8,
    pressed_keys: Keys,
    platforms: PlatformList,
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
    health: u32, //how much health the player has
    speed: f64, //how fast the player can move
    jump: f64, //how high the player can jump
    data_bool: Vec<bool>, //data used by the player's update function
    data_string: Vec<String>, //data used by the player's update function
    data_num: Vec<f64>, //data used by the player's update function
    moves: KeySequence, //sequence of moves the player has made
    apply_inputs: Rc<dyn Fn(Game) -> Game>, //move the player based on the inputs
    reset: Rc<dyn Fn(Player) -> Player>, //reset the player to the starting state
    active: bool, //if the player is currently in the game
    image: u32, //the image of the player
}

#[derive(Clone)]
struct Bullet {
    x: f64, //position of the bullet
    y: f64, //position of the bullet
    speed: f64, //how fast the bullet moves
    direction: f64, //the direction the bullet is moving
    damage: u32, //how much damage the bullet does
    data_bool: Vec<bool>, //data used by the bullet's update function
    data_string: Vec<String>, //data used by the bullet's update function
    data_num: Vec<f64>, //data used by the bullet's update function
    update: Rc<dyn Fn(u32, Game) -> Game>, //update the bullet based on the game state
    id: u32, //the id of the bullet
    image: u32, //the image of the bullet
}

#[derive(Clone)]
struct Enemy {
    x: f64, //position of the enemy
    y: f64, //position of the enemy
    health: u32, //how much health the enemy has
    speed: f64, //how fast the enemy moves
    data_bool: Vec<bool>, //data used by the enemy's update function
    data_string: Vec<String>, //data used by the enemy's update function
    data_num: Vec<f64>, //data used by the enemy's update function
    update: Rc<dyn Fn(u32, Game) -> Game>, //update the enemy based on the game state
    id: u32, //the id of the enemy
    image: u32, //the image of the enemy
}

#[derive(Clone)]
struct PlayerList {
    players: Vec<Player>,
    add: Rc<dyn Fn(Player, Game) -> Game>,
    get: Rc<dyn Fn(String, Game) -> Player>,
    remove: Rc<dyn Fn(String, Game) -> Game>,
}

#[derive(Clone)]
struct BulletList {
    bullets: Vec<Bullet>,
    add: Rc<dyn Fn(Bullet, Game) -> Game>,
    get: Rc<dyn Fn(u32, Game) -> Bullet>,
    remove: Rc<dyn Fn(u32, Game) -> Game>,
}

#[derive(Clone)]
struct EnemyList {
    enemies: Vec<Enemy>,
    add: Rc<dyn Fn(Enemy, Game) -> Game>,
    get: Rc<dyn Fn(u32, Game) -> Enemy>,
    remove: Rc<dyn Fn(u32, Game) -> Game>,
    get_ids: Rc<dyn Fn(Game) -> Vec<u32>>,
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
    add: Rc<dyn Fn(Platform, Game) -> Game>,
    get: Rc<dyn Fn(u32, Game) -> Platform>,
    remove: Rc<dyn Fn(u32, Game) -> Game>,
}



fn update_clone(agent_id: String, mut state: Game) -> Game {
    state = ((state.clones.get)(agent_id, state.clone()).apply_inputs)(state.clone());

    state
}

fn update_player(mut state: Game) -> Game {
    state.player = add_inputs(get_inputs(state.clone()), state.player.clone());
    state = (state.player.apply_inputs)(state.clone());

    state
}

fn update_player_bullet(bullet_id: u32, mut state: Game) -> Game {
    state = ((state.player_bullets.get)(bullet_id, state.clone()).update)(bullet_id, state.clone());

    state
}

fn update_enemy_bullet(bullet_id: u32, mut state: Game) -> Game {
    state = ((state.enemy_bullets.get)(bullet_id, state.clone()).update)(bullet_id, state.clone());

    state
}

fn update_players(mut state: Game) -> Game {
    for clone in state.clones.players.clone() {
        state = update_clone(clone.id.clone(), state.clone());
    }

    state = update_player(state.clone());

    state
}

fn update_bullets(mut state: Game) -> Game {
    for bullet in state.enemy_bullets.bullets.clone() {
        state = update_enemy_bullet(bullet.id.clone(), state.clone());
    }

    for bullet in state.player_bullets.bullets.clone() {
        state = update_player_bullet(bullet.id.clone(), state.clone());
    }

    state
}

fn check_hits(mut state: Game) -> Game {
    state
}

fn check_deaths(mut state: Game) -> Game {
    for clone in state.clones.players.clone() {
        if check_death(clone.clone()) {
            state = kill(clone.id.clone(), state.clone());
        }
    }

    state
}

fn check_death(agent: Player) -> bool {
    agent.health == 0
}

fn end_run(mut state: Game) -> Game {
    state.in_run = false;

    state = make_clone(state.player.clone(), state.clone());

    for clone in state.clones.players.clone() {
        state = kill(clone.id, state.clone());
    }

    state
}

fn do_button(button: Action, state: Game) -> Game {
    state
}

fn kill(clone_id: String, mut state: Game) -> Game {
    state = (state.clones.remove)(clone_id, state.clone());

    state
}

fn check_buttons() -> Vec<Action> {
    vec![]
}

fn add_inputs(inputs: Keys, player: Player) -> Player {
    player
}

fn get_inputs(state: Game) -> Keys {
    state.pressed_keys
}

fn update_camera(mut state: Game) -> Game {
    for clone in state.clones.players.iter_mut() {
        clone.x -= 1.0
    }
    for bullet in state.player_bullets.bullets.iter_mut() {
        bullet.x -= 1.0
    }
    for bullet in state.enemy_bullets.bullets.iter_mut() {
        bullet.x -= 1.0
    }
    for enemy in state.enemies.enemies.iter_mut() {
        enemy.x -= 1.0
    }

    state
}

fn update_enemies(mut state: Game) -> Game {
    for enemy_id in (state.enemies.get_ids)(state.clone()) {
        state = ((state.enemies.get)(enemy_id, state.clone()).update)(enemy_id, state)
    }

    state
}

fn make_clone(agent: Player, mut state: Game) -> Game {
    let mut new_clone = agent.clone();
    new_clone = (new_clone.reset)(new_clone.clone());

    state = (state.clones.add)(new_clone, state.clone());

    state
}

fn update_platforms(mut state: Game) -> Game {
    for platform in state.platforms.platforms.clone() {
        if platform.x + platform.width/2.0 < 0.0 {
            state = (state.platforms.remove)(platform.id, state.clone());
        }
    }
    let mut usedIDs = vec![];
    for platform in state.platforms.platforms.clone() {
        usedIDs.push(platform.id);
    }
    let mut newID = 0;
    while usedIDs.contains(&newID) {
        newID += 1;
    }
    //add a new platform with a random image
    state = (state.platforms.add)(Platform {x: 0.0, y: 0.0, width: 100.0, height: 10.0, id: newID, image: 1}, state.clone());

    state
}

fn main() {



    let mut game = Game {
        player: Player {
            id: "Base".to_string(),
            x: 0.0,
            y: 0.0,
            health: 100,
            speed: 1.0,
            jump: 1.0,
            data_bool: vec![],
            data_string: vec![],
            data_num: vec![],
            moves: KeySequence {sequence: vec![], step: 0, length: 0},
            apply_inputs: Rc::new(|mut state: Game| -> Game {
                if state.player.moves.sequence[state.player.moves.step as usize].w {
                    state.player.y += state.player.speed;
                }
                if state.player.moves.sequence[state.player.moves.step as usize].a {
                    state.player.x -= state.player.speed;
                }
                if state.player.moves.sequence[state.player.moves.step as usize].s {
                    state.player.y -= state.player.speed;
                }
                if state.player.moves.sequence[state.player.moves.step as usize].d {
                    state.player.x += state.player.speed;
                }
                state
            }),
            reset: Rc::new(|mut player: Player| -> Player {
                player.x = 0.0;
                player.y = 0.0;
                player.health = 100;
                player.data_bool = vec![];
                player.data_string = vec![];
                player.data_num = vec![];
                player
            }),
            active: true,
            image: 0,
        },
        clones: PlayerList {
            players: vec![],
            add: Rc::new(|agent: Player, mut state: Game| -> Game {
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
                state
            }),
            get: Rc::new(|id: String, mut state: Game| -> Player {
                for player in state.clones.players.clone() {
                    if player.id == id {
                        return player;
                    }
                }
                state.clones.players[0].clone()
            }),
            remove: Rc::new(|id: String, mut state: Game| -> Game {
                for i in 0..state.clones.players.len() {
                    if state.clones.players[i].id == id {
                        state.clones.players.remove(i);
                    }
                }
                state
            }),
        },
        player_bullets: BulletList {
            bullets: vec![],
            add: Rc::new(|agent: Bullet, mut state: Game| -> Game {
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
                state
            }),
            get: Rc::new(|id: u32, state: Game| -> Bullet {
                for bullet in state.player_bullets.bullets.clone() {
                    if bullet.id == id {
                        return bullet;
                    }
                }
                state.player_bullets.bullets[0].clone()
            }),
            remove: Rc::new(|id: u32, mut state: Game| -> Game {
                for i in 0..state.player_bullets.bullets.len() {
                    if state.player_bullets.bullets[i].id == id {
                        state.player_bullets.bullets.remove(i);
                    }
                }
                state
            }),
        },
        enemy_bullets: BulletList {
            bullets: vec![],
            add: Rc::new(|agent: Bullet, mut state: Game| -> Game {
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
                state
            }),
            get: Rc::new(|id: u32, state: Game| -> Bullet {
                for bullet in state.enemy_bullets.bullets.clone() {
                    if bullet.id == id {
                        return bullet;
                    }
                }
                state.enemy_bullets.bullets[0].clone()
            }),
            remove: Rc::new(|id: u32, mut state: Game| -> Game {
                for i in 0..state.enemy_bullets.bullets.len() {
                    if state.enemy_bullets.bullets[i].id == id {
                        state.enemy_bullets.bullets.remove(i);
                    }
                }
                state
            }),
        },
        enemies: EnemyList {
            enemies: vec![],
            add: Rc::new(|agent: Enemy, mut state: Game| -> Game {
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
                state
            }),
            get: Rc::new(|id: u32, state: Game| -> Enemy {
                for enemy in state.enemies.enemies.clone() {
                    if enemy.id == id {
                        return enemy;
                    }
                }
                state.enemies.enemies[0].clone()
            }),
            remove: Rc::new(|id: u32, mut state: Game| -> Game {
                for i in 0..state.enemies.enemies.len() {
                    if state.enemies.enemies[i].id == id {
                        state.enemies.enemies.remove(i);
                    }
                }
                state
            }),
            get_ids: Rc::new(|state: Game| -> Vec<u32> {
                let mut ids = vec![];
                for enemy in state.enemies.enemies {
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
            add: Rc::new(|agent: Platform, mut state: Game| -> Game {
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
                state
            }),
            get: Rc::new(|id: u32, state: Game| -> Platform {
                for platform in state.platforms.platforms.clone() {
                    if platform.id == id {
                        return platform;
                    }
                }
                state.platforms.platforms[0].clone()
            }),
            remove: Rc::new(|id: u32, mut state: Game| -> Game {
                for i in 0..state.platforms.platforms.len() {
                    if state.platforms.platforms[i].id == id {
                        state.platforms.platforms.remove(i);
                    }
                }
                state
            }),
        },
    };

    //add a blank move to the player
    game.player.moves.sequence.push(Keys {a: false, s: false, d: false, w:false, special: false, ability: false});

    //add a blank player to the list
    game = (game.clones.add)(game.player.clone(), game.clone());

    //add a blank platform to the list
    game = (game.platforms.add)(Platform {x: 0.0, y: 0.0, width: 100.0, height: 10.0, id: 0, image: 0}, game.clone());

    //add a blank bullet to both lists
    game = (game.player_bullets.add)(Bullet {x: 0.0, y: 0.0, speed: 0.0, direction: 0.0, damage: 0, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: Game| -> Game {state}), id: 0, image: 0}, game.clone());
    game = (game.enemy_bullets.add)(Bullet {x: 0.0, y: 0.0, speed: 0.0, direction: 0.0, damage: 0, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: Game| -> Game {state}), id: 0, image: 0}, game.clone());

    //add a blank enemy to the list
    game = (game.enemies.add)(Enemy {x: 0.0, y: 0.0, health: 0, speed: 0.0, data_bool: vec![], data_string: vec![], data_num: vec![], update: Rc::new(|id: u32, mut state: Game| -> Game {state}), id: 0, image: 0}, game.clone());

    //update the player's image
    game.player.image = 1;

    let mut in_run = true;
    let mut window: PistonWindow = WindowSettings::new("Chronodrive: Cycle of Steel", [1440, 900])
        .exit_on_esc(true)
        .build()
        .expect("window failed to build");

    let mut last_update = Instant::now();
    let update_interval = Duration::from_secs_f64(0.01); // Update every second

    let mut pressed_keys = Keys {a: false, s: false, d: false, w:false, special: false, ability: false};

    let mut this_vehicle = 1;

    let mut key_sequence = KeySequence {sequence: vec![], step: 0, length: 0};


    //get the background image
    let background = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/background.jpeg",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();



    //make a list of the platform images
    let platform1 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/platform1.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let platform2 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/platform2.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let platform3 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/platform3.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let platform4 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/platform4.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let platform_images = vec![platform1, platform2, platform3, platform4];



    //make a list of the player images
    let player1 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player1.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let player2 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player2.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let player3 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player3.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let player4 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player4.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let player5 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/player5.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let player_images = vec![player1, player2, player3, player4, player5];



    //make a list of the enemy images
    let enemy1 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/enemy1.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let enemy2 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/enemy2.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let enemy3 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/enemy3.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let enemy4 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/enemy4.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let enemy_images = vec![enemy1, enemy2, enemy3, enemy4];



    //make a list of the bullet images
    let bullet1 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/bullet1.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let bullet2 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/bullet2.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let bullet3 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/bullet3.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let bullet4 = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/bullet4.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let bullet_images = vec![bullet1, bullet2, bullet3, bullet4];



    //get the menu image
    let menu = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/menu.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();



    //get the mouse image
    let mouse = Texture::from_path(
        &mut window.create_texture_context(),
        "assets/images/mouse.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();



    while let Some(event) = window.next() {
        if let Some(_) = event.update_args() {
            let now = Instant::now();
            if now.duration_since(last_update) >= update_interval {
                last_update = now;
                if game.in_run {
                    //game = build_platforms(update_camera(check_deaths(check_hits(update_enemies(update_bullets(update_players(game.clone())))))));
                    game = update_players(game.clone());
                    game = update_bullets(game.clone());
                    game = update_enemies(game.clone());
                    game = check_hits(game.clone());
                    game = check_deaths(game.clone());
                    game = update_camera(game.clone());
                    game = update_platforms(game.clone());
                    if check_death(game.player.clone()) {
                        game = end_run(game.clone());
                    }
                } else {
                    let mut buttons = check_buttons();
                    for button in buttons {
                        game = do_button(button, game.clone());
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
            
            if game.in_run {
                //draw the player's health
                rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 100.0, 10.0], c.transform.scale(1.0, 1.0), g);
                rectangle([0.0, 1.0, 0.0, 1.0], [0.0, 0.0, game.player.health as f64, 10.0], c.transform.scale(1.0, 1.0), g);

                //draw the background at full size
                image(&background, c.transform.scale(1440.0/1280.0, 900.0/1280.0), g);
            
                //draw the platforms
                for platform in game.platforms.platforms.clone() {
                    image(&platform_images[platform.image as usize], c.transform.scale(platform.width/1280.0, platform.height/1280.0), g);
                }

                //draw the player
                image(&player_images[game.player.image as usize], c.transform.scale(0.1, 0.1).trans(game.player.x, game.player.y), g);

                //draw the clones
                for clone in game.clones.players.clone() {
                    image(&player_images[clone.image as usize], c.transform.scale(0.1, 0.1).trans(clone.x, clone.y), g);
                }

                //draw the player bullets
                for bullet in game.player_bullets.bullets.clone() {
                    image(&bullet_images[bullet.image as usize], c.transform.scale(0.1, 0.1).trans(bullet.x, bullet.y), g);
                }

                //draw the enemy bullets
                for bullet in game.enemy_bullets.bullets.clone() {
                    image(&bullet_images[bullet.image as usize], c.transform.scale(0.1, 0.1).trans(bullet.x, bullet.y), g);
                }

                //draw the enemies
                for enemy in game.enemies.enemies.clone() {
                    image(&enemy_images[enemy.image as usize], c.transform.scale(0.1, 0.1).trans(enemy.x, enemy.y), g);
                }
            } else {
                //draw the menu background
                image(&menu, c.transform.scale(1440.0/1280.0, 900.0/1280.0), g);
                
                //draw the buttons
                //for button in buttons {
                //    rectangle([0.0, 0.0, 0.0, 1.0], [button.x, button.y, button.width, button.height], c.transform.scale(1.0, 1.0), g);
                //}
            }
            //draw the mouse
            image(&mouse, c.transform.scale(0.1, 0.1).trans(0.0, 0.0), g);
        });
    }
}
