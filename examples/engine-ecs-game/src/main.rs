use std::ops::Deref;

use engine::Spritesheet;
use engine_ecs as engine;
use engine_ecs::wgpu;
use engine_ecs::{components::*, geom::*, Camera, SheetRegion};
use hecs::Entity;
use rand::Rng;
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
struct BottleBundle(Sprite, Transform, Solid, BoxCollision);
////

///Width and height of the window
const W: f32 = 320.0;
const H: f32 = 240.0;
/////

//Testing Variables for Physics
const APPLE_SIZE: Vec2 = Vec2 { x: 16.0, y: 16.0 };
const SPRITE_MAX: usize = 16;
const APPLE_MAX: usize = 128;
const APPLE_INTERVAL: std::ops::Range<u32> = 1..10;
const APPLE_SPEED_RANGE: std::ops::Range<f32> = (-2.0)..(-0.5);
////

//Static UVS
const WALL_UVS: SheetRegion = SheetRegion::new(0, 0, 480, 12, 8, 8);
const SHELF_UVS: SheetRegion = SheetRegion::new(0, 1, 50, 480, 264, 16);
const BAR_UVS: SheetRegion = SheetRegion::new(0, 36, 1, 480, 127, 47);
const BOTTLE_UVS: SheetRegion = SheetRegion::new(0, 1, 1, 480, 3, 11);
//////

///Bundle Vectors
const bottleBundles: Vec<&BottleBundle> = Vec::new();
/// //

struct Game {
    apple_timer: u32,
    score: u32,
    spritesheet: engine::Spritesheet,
    held_bottle: Option<BottleBundle>,
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
        }
    }
    fn update(&mut self, engine: &mut Engine) {
        if engine.frame_number() % 600 == 0 {}

        //This handles user input and moved the guy accordingly
        // let dir = engine.input.key_axis(engine::Key::Left, engine::Key::Right);
        // engine
        //     .world()
        //     .query_one::<&mut Physics>(self.guy)
        //     .unwrap()
        //     .get()
        //     .unwrap()
        //     .vel = Vec2 {
        //     x: dir * 1.0,
        //     y: 0.0,
        // };


        if engine.input.is_mouse_pressed(winit::event::MouseButton::Left) {
            let mouse_position = engine.input.mouse_pos();
    
            // // Check if there is a bottle at the mouse position
            // for (bottle, (_, transform)) in engine.world().query::<(&Bottle, &Transform)>().iter() {
            //     // if transform.contains_point(Vec2::new(mouse_position.x, mouse_position.y)) {
            //     if transform.y == mouse_position.y as f32 {
            //         if let Some(held_bottle) = self.held_bottle {
            //             // Drop the held bottle
            //             self.held_bottle = None;
            //             // Add code to drop the bottle at the mouse position
            //             // ...
                        // let _bottle = engine.spawn(AppleBundle(
                        //     Sprite(self.spritesheet, SheetRegion::new(0, 1, 3, 4, 16, 16)),
                        //     Transform {
                        //         x: mouse_position.x as f32,
                        //         y: mouse_position.y as f32,
                        //         w: APPLE_SIZE.x as u16,
                        //         h: APPLE_SIZE.y as u16,
                        //         rot: 0.0,
                        //     },
                        //     SolidPushable::default(),
                        //     BoxCollision(AABB {
                        //         center: Vec2::ZERO,
                        //         size: APPLE_SIZE,
                        //     }),
                        //     Physics {
                        //         vel: Vec2 {
                        //             x: 0.0,
                        //             y: -0.5,
                        //         },
                        //     },
                        //     Apple(),
                        // ));
            //         } else {
            //             // Pick up the clicked bottle
            //             self.held_bottle = Some(bottle);
            //             // Add code to update the state of the picked up bottle
            //             // ...
            //         }
            //         // Break the loop after handling the first bottle found at the mouse position
            //         break;
            //     }
            // }

            println!("{}:{}", mouse_position.x, mouse_position.y);

            let _bottle = engine.spawn(AppleBundle(
                Sprite(self.spritesheet, SheetRegion::new(0, 1, 3, 4, 16, 16)),
                Transform {
                    x: mouse_position.x as f32/2.5,
                    y: H - mouse_position.y as f32/2.5,
                    w: APPLE_SIZE.x as u16,
                    h: APPLE_SIZE.y as u16,
                    rot: 0.0,
                },
                SolidPushable::default(),
                BoxCollision(AABB {
                    center: Vec2::ZERO,
                    size: APPLE_SIZE,
                }),
                Physics {
                    vel: Vec2 {
                        x: 0.0,
                        y: -2.5,
                    },
                },
                Apple(),
            ));
            
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
    );
    bottleBundles.push(&theBundle);
    engine.spawn(theBundle);
}
