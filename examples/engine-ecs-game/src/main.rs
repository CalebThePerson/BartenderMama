use std::ops::Deref;

use engine::Spritesheet;
use engine_ecs as engine;
use engine_ecs::wgpu;
use engine_ecs::{components::*, geom::*, Camera, SheetRegion};
use hecs::Entity;
use rand::Rng;
use winit::dpi::LogicalPosition;
use winit::window::{self, Window};
type Engine = engine::Engine<Game>;
use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
};


//Bundles
#[derive(hecs::Bundle)]
struct EnvBundle(Sprite, Transform, Solid, BoxCollision);

#[derive(hecs::Bundle)]
struct DecoBundle(Sprite, Transform);

//Bottle Bundle, feel free to add on as you see fit
#[derive(hecs::Bundle)]
struct BottleBundle(Sprite, Transform, Solid, BoxCollision, bool, String);

#[derive(hecs::Bundle)]
struct GlassBundle(Sprite, Transform, bool); //Getting rid of Solid, BoxCollision because we need to be able to queruy for it plus it shouldnt need those two things
                                             ////

///Width and height of the window
const W: f32 = 320.0;
const H: f32 = 240.0;
/////


//Static UVS
const SHELF_UVS: SheetRegion = SheetRegion::new(0, 1, 50, 480, 264, 16);
const BAR_UVS: SheetRegion = SheetRegion::new(0, 45, 1, 480, 127, 47);
const BOTTLE_UVS: SheetRegion = SheetRegion::new(0, 1, 1, 480, 3, 11);
const GLASS_UVS: SheetRegion = SheetRegion::new(0, 36, 1, 480, 7, 7);
//////

struct Game {
    held_bottle: Option<Entity>,
    reset: bool,
    pour: bool,
    glass_state: u32,
    glass: Entity,
    audio: AudioManager
}
#[derive(Clone)]
struct Bottle {
    name: String,
    compatible: Vec<Bottle>
}
impl Bottle {
    // fn name(&self) -> String {
    //     self.name.clone()
    // }

    // fn tequila() -> Self {
    //     let compatible_drinks = vec![Bottle::lychee(), Bottle::limejuice()];

    //     Bottle { name: "tequila".into(), compatible: (compatible_drinks) }
    // }

    // fn lychee() -> Self {
    //     let compatible_drinks = vec![Bottle::tequila(), Bottle::limejuice(), ];

    //     Bottle { name: "lychee".into(), compatible: (compatible_drinks) }
    // }

    // fn limejuice() -> Self {
    //     let compatible_drinks = vec![Bottle::vodka(), Bottle::tequila(), Bottle::jagermeister()];
    //     Bottle { name: "limejuice".into(), compatible: (compatible_drinks) }
    // }

    // fn redbull() -> Self {
    //     let compatible_drinks = vec![Bottle::jagermeister(), Bottle::vodka()];

    //     Bottle { name: "redbull".into(), compatible: (compatible_drinks) }
    // }

    // fn jagermeister() -> Self {
    //     let compatible_drinks = vec![Bottle::redbull()];

    //     Bottle { name: "jagermeister".into(), compatible: (compatible_drinks) }
    // }

    // fn vodka() -> Self {
    //     let compatible_drinks = vec![Bottle::lychee(), Bottle::redbull()];
    //     Bottle { name: "vodka".into(), compatible: (compatible_drinks) }
    // }
}

impl engine::Game for Game {
    const DT: f32 = 1.0 / 120.0;
    fn new(engine: &mut Engine) -> Self {
        engine.set_camera(Camera {
            screen_pos: [0.0, 0.0],
            screen_size: [W, H],
        });
        #[cfg(target_arch = "wasm32")]
        let sprite_img = {
            let img_bytes = include_bytes!("content/demo.png");
            image::load_from_memory_with_format(&img_bytes, image::ImageFormat::Png)
                .map_err(|e| e.to_string())
                .unwrap()
                .into_rgba8()
        };
        #[cfg(not(target_arch = "wasm32"))]
        let bar_img = image::open("content/bar_sheet.png").unwrap().into_rgba8();
        let spritesheet = engine.add_spritesheet(&[&bar_img], Some("The Bar"));

        //Spawning the bar in
        make_bar(spritesheet, engine, W / 2.0, 15.0, W, 47.0);
        make_shelf(spritesheet, engine, W / 2.0, 60.0 + 20.0, 160.0, 16.0);
        make_shelf(spritesheet, engine, W / 2.0, 100.0 + 20.0, 160.0, 16.0);
        let glass = make_glass(spritesheet, engine, W / 2.0, 45.0, 17.0, 19.0);

        let bottles = vec![1, 6, 11, 16, 21, 31];

        for (index, &item) in bottles.iter().enumerate() {
            //Making bottles on bottom shelf
            make_bottle(
                spritesheet,
                engine,
                (W / 3.0) + (index * 20) as f32,
                95.0,
                3.0 * 2.9,
                11.0 * 2.6,
                SheetRegion::new(0, item, 1, 480, 3, 11),
                "tequila".into()
            );
        }

        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        Game {
            held_bottle: None,
            reset: false,
            glass_state: 0,
            glass: glass,
            pour: false,
            audio: manager
        }
    }

    fn update(&mut self, engine: &mut Engine) {
        if engine.frame_number() % 600 == 0 {}

        if self.pour {
            engine.updateGlass(self.glass_state, self.glass);
            self.pour = false;
        }

        if self.reset {
            let sound = StaticSoundData::from_file(
                "content/sound/pick.mp3",
                StaticSoundSettings::default().volume(2.5),
            )
            .unwrap();
            self.audio.play(sound);
            
            engine.resetBottles(); // Resets all the bottles positions and activiates on the second mouse click
            self.reset = false;
        }

        if self.held_bottle.is_some() {
            println!("moving");
            for (bottle, (sprite, trans, solid, collision, isBottle)) in engine
                .world()
                .query::<(&Sprite, &mut Transform, &Solid, &BoxCollision, &bool)>()
                .iter()
            {
                //
                if !self.held_bottle.is_none() {
                    if bottle == self.held_bottle.unwrap() {
                        println!("same bottle");
                        if engine.input.is_key_pressed(engine::Key::Space) {
                            if self.glass_state < 3 {
                                self.glass_state += 1;
                            }
                            self.pour = true;

                            let sound = StaticSoundData::from_file(
                                "content/sound/pour.mp3",
                                StaticSoundSettings::default().volume(2.5),
                            )
                            .unwrap();
        
                            self.audio.play(sound);
                        }
                        if engine.input.is_key_down(engine::Key::Space) {
                            println!("space");
                            trans.rotc_Sprite();
                        } else {
                            trans.rotcounter_Sprite();
                        }
                        let (mouseX, mouseY) = engine.mouse_localized(H);
                        println!("{}, {}", mouseX, mouseY);
                        trans.moveSprite(mouseX, mouseY);
                        if engine
                            .input
                            .is_mouse_pressed(winit::event::MouseButton::Left)
                        {
                            self.held_bottle = None;
                            self.reset = true;
                        }
                    }
                }
            }
        } else if engine
            .input
            .is_mouse_pressed(winit::event::MouseButton::Left)
        {
            // let mouse_position = engine.input.mouse_pos();

            let (mut mouseX, mut mouseY) = engine.mouse_localized(H);

            println!("{}, {}", mouseX, mouseY);

            for (glass, (sprite, trans, isBottle)) in engine
            .world()
            .query::<(&Sprite, &mut Transform, &bool)>()
            .iter()
            {
                if trans.detectMouseCollision(mouseX as f64, mouseY as f64) {
                    if glass == self.glass {
                        self.glass_state = 0;
                        self.pour = true;
                        println!("empty glass: {}", self.glass_state);

                        let sound = StaticSoundData::from_file(
                            "content/sound/empty.mp3",
                            StaticSoundSettings::default().volume(2.5),
                        )
                        .unwrap();
    
                        self.audio.play(sound);
                    }
                }
            }

            for (bottle, (sprite, trans, solid, collision, isBottle)) in engine
                .world()
                .query::<(&Sprite, &mut Transform, &Solid, &BoxCollision, &bool)>()
                .iter()
            {
                if trans.detectMouseCollision(mouseX as f64, mouseY as f64) {
                    self.held_bottle = Some(bottle);
                    let sound = StaticSoundData::from_file(
                        "content/sound/pick.mp3",
                        StaticSoundSettings::default().volume(2.5),
                    )
                    .unwrap();

                    self.audio.play(sound);
                    
                }
            }
        }
    }
    fn handle_collisions(
        &mut self,
        engine: &mut Engine,
        _contacts: impl Iterator<Item = engine::Contact>,
        triggers: impl Iterator<Item = engine::Contact>,
    ) {}
    fn render(&mut self, engine: &mut Engine) {}
}
fn main() {
    Engine::new(winit::window::WindowBuilder::new()).run();
}

fn make_shelf(
    spritesheet: engine_ecs::Spritesheet,
    engine: &mut Engine,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    let theBundle = EnvBundle(
        Sprite(spritesheet, SHELF_UVS),
        Transform {
            x,
            y,
            w: w as u16,
            h: h as u16,
            rot: 0.0,
        },
        Solid::default(),
        BoxCollision(AABB {
            center: Vec2::ZERO,
            size: Vec2 { x: 160.0, y: h },
        }),
    );

    engine.spawn(theBundle);
}
fn make_bar(
    spritesheet: engine_ecs::Spritesheet,
    engine: &mut Engine,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Entity {
    engine.spawn(EnvBundle(
        Sprite(spritesheet, BAR_UVS),
        Transform {
            x,
            y,
            w: w as u16,
            h: h as u16,
            rot: 0.0,
        },
        Solid::default(),
        BoxCollision(AABB {
            center: Vec2::ZERO,
            size: Vec2 { x: W, y: 47.0 },
        }),
    ))
}

fn make_bottle(
    spritesheet: engine_ecs::Spritesheet,
    engine: &mut Engine,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    UVS: SheetRegion,
    bottle: String
) {
    let theBundle = BottleBundle(
        Sprite(spritesheet, UVS),
        Transform {
            x,
            y,
            w: w as u16,
            h: h as u16,
            rot: 0.0,
        },
        Solid::default(),
        BoxCollision(AABB {
            center: Vec2::ZERO,
            size: Vec2 { x: w, y: h },
        }),
        true,
        bottle
    );

    engine.spawn(theBundle);
}

fn make_glass(
    spritesheet: engine_ecs::Spritesheet,
    engine: &mut Engine,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Entity {
    engine.spawn(GlassBundle(
        Sprite(spritesheet, GLASS_UVS),
        Transform {
            x,
            y,
            w: w as u16,
            h: h as u16,
            rot: 0.0,
        },
        true,
    ))
}

fn find_index_by_coordinates(
    vector: &Vec<&BottleBundle>,
    target_x: f32,
    target_y: f32,
) -> Option<usize> {
    vector
        .iter()
        .position(|bundle| bundle.1.x == target_x && bundle.1.y == target_y)
}
