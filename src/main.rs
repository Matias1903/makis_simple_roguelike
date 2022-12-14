#![allow(dead_code)]
use rltk::{Rltk, GameState, RGB, Point};
use specs::prelude::*;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
pub use visibility_system::VisibilitySystem;
mod monster_ai_system;
pub use monster_ai_system::MonsterAI;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState{ Paused, Running }

pub struct State{
    pub ecs: World,
    pub runstate: RunState 
}

impl State {
    fn run_systems(&mut self){
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State{
    fn tick(&mut self, ctx: &mut Rltk){
        ctx.cls();

        if self.runstate == RunState::Running {        
            self.run_systems();
            self.runstate = RunState::Paused;
        }else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);
        
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for(pos, render) in (&positions, &renderables).join(){
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)}
        }
    }
}


//--------------------------------------Main--------------------------------------
fn main() -> rltk::BError{
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike ECS Game")
        .build()?;
    let mut gs = State{ 
        ecs: World::new(),
        runstate: RunState::Running
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    
    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position{x: player_x, y: player_y})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{visible_tiles : Vec::new(), range : 8, dirty: true})
        .build();

        let mut rng = rltk::RandomNumberGenerator::new();
        for room in map.rooms.iter().skip(1){
            let(x, y) = room.center();

            let glyph: rltk::FontCharType;
            let roll = rng.roll_dice(1, 2);
            match roll {
                1 => {glyph = rltk::to_cp437('g')}
                _ => {glyph = rltk::to_cp437('o')}
            }

            gs.ecs.create_entity()
                .with(Position{x, y})
                .with(Renderable {
                    glyph: glyph,
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK)
                })
                .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
                .with(Monster{})
                .build();
        }
        gs.ecs.insert(map);
        gs.ecs.insert(Point::new(player_x, player_y));

    rltk::main_loop(context, gs)
}