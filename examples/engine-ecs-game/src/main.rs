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

// Stuff for testing not really important
struct Apple();
struct Guy();
struct Bottle();

// Bundles

struct GuyBundle(Sprite, Transform, Pushable, BoxCollision, Physics, Guy);
#[derive(hecs::Bundle)]

struct AppleBundle(
    Sprite, //
    Transform,
    SolidPushable,
    BoxCollision,
    Physics,
    Apple,
);
/////

//Bundles
#[derive(hecs::Bundle)]
struct EnvBundle(Sprite, Transform, Solid, BoxCollision);

#[derive(hecs::Bundle)]
struct DecoBundle(Sprite, Transform);

//Bottle Bundle, feel free to add on as you see fit
#[derive(hecs::Bundle)]
struct BottleBundle(Sprite, Transform, Solid, BoxCollision, bool);

#[derive(hecs::Bundle)]
struct GlassBundle(Sprite, Transform, bool); //Getting rid of Solid, BoxCollision because we need to be able to queruy for it plus it shouldnt need those two things
                                             ////

///Width and height of the window
const W: f32 = 320.0;
const H: f32 = 240.0;
/////

//Testing Variables for Physics
const APPLE_SIZE: Vec2 = Vec2 { x: 10.0, y: 20.0 };
const SPRITE_MAX: usize = 16;
const APPLE_MAX: usize = 128;
const APPLE_INTERVAL: std::ops::Range<u32> = 1..10;
const APPLE_SPEED_RANGE: std::ops::Range<f32> = (-2.0)..(-0.5);
////

//Static UVS
const SHELF_UVS: SheetRegion = SheetRegion::new(0, 1, 50, 480, 264, 16);
const BAR_UVS: SheetRegion = SheetRegion::new(0, 45, 1, 480, 127, 47);
const BOTTLE_UVS: SheetRegion = SheetRegion::new(0, 1, 1, 480, 3, 11);
const GLASS_UVS: SheetRegion = SheetRegion::new(0, 36, 1, 480, 7, 7);

//////

///Bundle Vectors
const bottleBundles: Vec<&BottleBundle> = Vec::new(); //Delete later dont really need
/// //

struct Game {
    apple_timer: u32,
    score: u32,
    spritesheet: engine::Spritesheet,
    held_bottle: Option<Entity>,
    reset: bool,
    glassState: u32,
    glass: Entity,
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
        let glass = make_glass(spritesheet, engine, W / 2.0, 45.0, 14.0, 14.0);

        for i in 0..5 {
            //Making bottles on bottom shelf
            make_bottle(
                spritesheet,
                engine,
                (W / 3.0) + (i * 20) as f32,
                90.0,
                3.0 * 2.6,
                11.0 * 2.6,
            );
        }

        Game {
            apple_timer: 0,
            score: 0,
            spritesheet,
            held_bottle: None,
            reset: false,
            glassState: 0,
            glass: glass,
        }
    }

    fn update(&mut self, engine: &mut Engine) {
        if engine.frame_number() % 600 == 0 {}

        if self.reset {
            engine.resetBottles(); // Resets all the bottles positions and activiates on the second mouse click
            engine.updateGlass(self.glassState, self.glass);
            self.glassState += 1;
            if self.glassState > 3 {
                self.glassState = 0;
            }
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
                        if engine.input.is_key_down(engine::Key::Space) {
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

            for (bottle, (sprite, trans, solid, collision, isBottle)) in engine
                .world()
                .query::<(&Sprite, &mut Transform, &Solid, &BoxCollision, &bool)>()
                .iter()
            {
                if trans.detectMouseCollision(mouseX, mouseY) {
                    println!("Detected");

                    self.held_bottle = Some(bottle);
                }
            }
        }
    }
    fn handle_collisions(
        &mut self,
        engine: &mut Engine,
        _contacts: impl Iterator<Item = engine::Contact>,
        triggers: impl Iterator<Item = engine::Contact>,
    ) {
        for engine::Contact(thing_a, thing_b, _amt) in triggers {
            let ent_a = engine.world().entity(thing_a).unwrap();
            let ent_b = engine.world().entity(thing_b).unwrap();
            if ent_a.has::<Apple>() && ent_b.has::<Guy>() {
                engine.despawn(thing_a).unwrap();
                self.score += 1;
            }
        }
    }
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
) {
    let theBundle = BottleBundle(
        Sprite(spritesheet, BOTTLE_UVS),
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
