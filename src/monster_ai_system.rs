use specs::prelude::*;
use super::{Viewshed, Monster};
use rltk::{Point, console};

pub struct MonsterAI{}

impl <'a> System<'a> for MonsterAI{
    type SystemData = (ReadStorage<'a, Viewshed>,
                       ReadExpect<'a, Point>,
                       ReadStorage<'a, Monster>);

    fn run(&mut self, data: Self::SystemData){
        let(viewshed, player_pos, monster) = data;
        let mut number = 0;

        for(viewshed, _monster)in (&viewshed, &monster).join(){
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("Monster shouts insults {}", number));
                number = number + 1;
            }
        }
    }
}