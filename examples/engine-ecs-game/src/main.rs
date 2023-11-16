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
        //     x: dir * GUY_SPEED,
        //     y: 0.0,
        // };

        // This part of the code handeld spawning random apples
        // let mut rng = rand::thread_rng();
        // let mut apple_count = 0;
        // let mut to_remove = vec![];
        // for (apple, (_, trf)) in engine.world().query::<(&Apple, &Transform)>().iter() {
        //     if trf.y < -8.0 {
        //         to_remove.push(apple);
        //     } else {
        //         apple_count += 1;
        //     }
        // }
        // for apple in to_remove {
        //     engine.despawn(apple).unwrap();
        // }
        // if self.apple_timer > 0 {
        //     self.apple_timer -= 1;
        // } else if apple_count < APPLE_MAX {
        //     let _apple = engine.spawn(AppleBundle(
        //         Sprite(self.spritesheet, SheetRegion::new(0, 1, 3, 4, 16, 16)),
        //         Transform {
        //             x: rng.gen_range(8.0..(W - 8.0)),
        //             y: H + 8.0,
        //             w: APPLE_SIZE.x as u16,
        //             h: APPLE_SIZE.y as u16,
        //             rot: 0.0,
        //         },
        //         SolidPushable::default(),
        //         BoxCollision(AABB {
        //             center: Vec2::ZERO,
        //             size: APPLE_SIZE,
        //         }),
        //         Physics {
        //             vel: Vec2 {
        //                 x: 0.0,
        //                 y: rng.gen_range(APPLE_SPEED_RANGE),
        //             },
        //         },
        //         Apple(),
        //     ));
        //     self.apple_timer = rng.gen_range(APPLE_INTERVAL);
        // }
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
