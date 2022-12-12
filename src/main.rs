#![allow(dead_code)]
use rltk::{Rltk, GameState, RGB};
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

pub struct State{
    ecs: World
}

impl State {
    fn run_systems(&mut self){
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State{
    fn tick(&mut self, ctx: &mut Rltk){
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

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
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    
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

        for room in map.rooms.iter().skip(1){
            let(x, y) = room.center();
            gs.ecs.create_entity()
                .with(Position{x, y})
                .with(Renderable {
                    glyph: rltk::to_cp437('g'),
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK)
                })
                .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
                .build();
        }
        gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}