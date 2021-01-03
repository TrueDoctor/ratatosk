use super::components::*;

use crate::boxes::RBox;
use crate::collide::Collidable;
use crate::events::{Event, Key, Keyboard};
use crate::io;
use crate::math::{Mat3, Vec2};
use crate::resources::{self, registry, GetStore};
use crate::EngineError;
use specs::join::JoinIter;
use specs::prelude::*;
use specs_hierarchy::*;

lazy_static::lazy_static! {
    pub static ref KEYBOARD: Keyboard = Keyboard::new();
}

pub struct EventSystem;
pub struct SimpleVelocitySystem;
pub struct VelocitySystem;
pub struct GravitationSystem;
pub struct RenderSystem;
pub struct MovementSystem;
pub struct CheckPresentSystem;
pub struct UpdateAnimationSystem;

impl<'a> System<'a> for SimpleVelocitySystem {
    type SystemData = (
        WriteStorage<'a, Pos>,
        ReadStorage<'a, Vel>,
        ReadStorage<'a, Mass>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut pos, vel, mass, dt): Self::SystemData) {
        for (vel, pos, _) in (&vel, &mut pos, !&mass).join() {
            pos.0 += vel.0 * dt.0.as_secs_f32();
        }
    }
}

impl<'a> System<'a> for VelocitySystem {
    type SystemData = (
        WriteStorage<'a, Pos>,
        ReadStorage<'a, Vel>,
        ReadStorage<'a, Mass>,
        ReadStorage<'a, Collider>,
        ReadStorage<'a, Terrain>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut pos, vel, mass, collider, terrain, dt): Self::SystemData) {
        for (vel, pos, _) in (&vel, &mut pos, !&mass).join() {
            pos.0 += vel.0 * dt.0.as_secs_f32();
        }
        /*(&collider, &vel, !&terrain, &mut pos, &mass)
        .par_join()
        .for_each(|(col1, vel, _, pos1, mass)| {
            for (col2, _, pos2) in (&collider, &terrain, &pos).join() {
                // TODO: collision code
            }
        })*/
    }
}

impl<'a> System<'a> for GravitationSystem {
    type SystemData = (
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Mass>,
        Read<'a, Gravitation>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut vel, mass, g, dt): Self::SystemData) {
        for (vel, _) in (&mut vel, &mass).join() {
            vel.0 += g.0 * dt.0.as_secs_f32();
        }
    }
}

impl<'a> System<'a> for UpdateAnimationSystem {
    type SystemData = (
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Collider>,
        WriteStorage<'a, SubCollider>,
        WriteStorage<'a, Vulnerable>,
        WriteStorage<'a, Damaging>,
        ReadStorage<'a, Present>,
        Entities<'a>,
        ReadExpect<'a, Hierarchy<Animation>>,
        Read<'a, ElapsedTime>,
    );

    fn run(
        &mut self,
        (
            mut animations,
            mut sprite,
            mut mat3,
            collider,
            mut sub,
            mut vul,
            mut dmg,
            present,
            entities,
            anim_h,
            elapsed,
        ): Self::SystemData,
    ) {
        let res = &mut *resources::RESOURCE_TABLE.write();
        for (mut animation, collider, e, _) in
            (&mut animations, &collider, &entities, &present).join()
        {
            let cha: Result<&mut Box<resources::Character>, EngineError> =
                res.get_mut(animation.id as usize);
            if let Ok(cha) = cha {
                if cha.animation_name() != animation.animation {
                    cha.set_animation(
                        animation.animation.as_str(),
                        elapsed.0.as_secs_f32() - animation.start,
                        0.0,
                        0.2, // fade time TODO make adjustable
                    )
                    .unwrap();
                    animation.start = elapsed.0.as_secs_f32();
                }

                let sprites = cha
                    .interpolate(elapsed.0.as_secs_f32() - animation.start)
                    .unwrap();
                let ci = anim_h.children(e);
                let mut ci: Vec<_> = ci.to_vec();
                ci.sort_unstable_by_key(|x| x.id());
                let mut ci = ci.iter();
                for (i, s) in sprites.enumerate() {
                    let s = s.unwrap();
                    let c = ci.next().cloned().unwrap_or_else(|| {
                        let e = entities.create();
                        match collider.mapping.get(&(i as u32)) {
                            Some(HitboxType::Damaging) => {
                                dmg.insert(e, Damaging { damage: 0.0 }).unwrap();
                            }
                            Some(HitboxType::Vulnerable) => {
                                vul.insert(e, Vulnerable { armor: 0.0 }).unwrap();
                            }
                            _ => (),
                        }
                        e
                    });
                    if let Some((sprite, mat3, sub)) =
                        JoinIter::get(&mut (&mut sprite, &mut mat3, &mut sub).join(), c, &entities)
                    {
                        *mat3 = Transform {
                            mat3: s.transform,
                            parent: e,
                        };
                        *sprite = Sprite {
                            id: animation.id,
                            sub_id: s.att_id,
                        };
                        *sub = SubCollider {
                            collider: Collidable::RBox(RBox::from(&s.transform)),
                            parent: e,
                        }
                    }
                }

                //for sprite in (sprites.join()).filter(|s| s.)
            }
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Animation>,
        ReadStorage<'a, Scale>,
        ReadStorage<'a, Present>,
        ReadStorage<'a, Transform>,
        ReadExpect<'a, Hierarchy<Animation>>,
        Entities<'a>,
        Read<'a, ElapsedTime>,
        Write<'a, SystemApi>,
        Write<'a, TextureIds>,
    );

    fn run(
        &mut self,
        (
            pos,
            sprite,
            anim,
            scale,
            present,
            transform,
            hierarchy,
            entities,
            elapsed,
            mut sys,
            mut tex_ids,
        ): Self::SystemData,
    ) {
        let mut sprites = Vec::new();
        for (pos, sprite, scale, _) in (&pos, &sprite, &scale, &present).join() {
            sprites.push(resources::Sprite::new(
                Mat3::translation(pos.0.x(), pos.0.y()) * Mat3::scaling(scale.0.x(), scale.0.y()),
                sprite.id,
                sprite.sub_id,
            ))
        }
        for (pos, _, scale, entity, _) in (&pos, &anim, &scale, &entities, &present).join() {
            let trans = Mat3::translation(pos.0.x(), pos.0.y());
            let scale = Mat3::scaling(scale.0.x(), scale.0.y());
            for &entity in hierarchy.children(entity) {
                if let Some((transform, sprite)) =
                    (&transform, &sprite).join().get(entity, &entities)
                {
                    sprites.push(resources::Sprite::new(
                        trans * scale * transform.mat3,
                        sprite.id,
                        sprite.sub_id,
                    ))
                }
            }
        }
        let mut dirty = false;
        for sp in &sprites {
            if !tex_ids.0.contains(&sp.tex_id) {
                tex_ids.0.push(sp.tex_id);
                dirty = true;
            }
        }
        if dirty {
            sys.0.push_textures(tex_ids.0.clone());
        }
        sys.0.push_sprites(sprites);
    }
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Scale>,
        ReadStorage<'a, Speed>,
    );

    fn run(&mut self, (mut anim, mut vel, mut scale, speed): Self::SystemData) {
        for (anim, vel, scale, speed) in (&mut anim, &mut vel, &mut scale, &speed).join() {
            anim.animation = if KEYBOARD.get(Key::ARROW_RIGHT) {
                scale.0 = Vec2::new(1.0, scale.0.y());
                vel.0 = Vec2::new(speed.0, 0.0);
                "walking".to_owned()
            } else if KEYBOARD.get(Key::ARROW_LEFT) {
                scale.0 = Vec2::new(-1.0, scale.0.y());
                vel.0 = Vec2::new(-speed.0, 0.0);
                "walking".to_owned()
            } else {
                vel.0 = Vec2::new(0.0, 0.0);
                "standing".to_owned()
            };
        }
    }
}

impl<'a> System<'a> for CheckPresentSystem {
    type SystemData = (
        ReadStorage<'a, Animation>,
        ReadStorage<'a, Sprite>,
        Entities<'a>,
        WriteStorage<'a, Present>,
    );

    fn run(&mut self, (anim, sprite, entities, mut present): Self::SystemData) {
        let res = &*resources::RESOURCE_TABLE.read();

        let mut modified = Vec::new();
        for (sprite, entity, _) in (&sprite, &entities, !&present).join() {
            if res.resource_present(sprite.id as usize) {
                modified.push(entity);
            }
        }
        for (anim, entity, _) in (&anim, &entities, !&present).join() {
            if res.resource_present(anim.id as usize) {
                modified.push(entity);
            }
        }
        for item in modified {
            let _ = present
                .insert(item, Present)
                .map_err(|e| log::debug!("{}", e));
        }
    }
}

impl<'a> System<'a> for EventSystem {
    type SystemData = (Write<'a, SystemApi>,);

    fn run(&mut self, mut sys: Self::SystemData) {
        let sys = &mut *sys.0;
        loop {
            let message = sys.0.poll_message().unwrap();
            match message {
                io::Message::None => break,
                io::Message::SystemInternal => continue,
                io::Message::Event(event) => {
                    log::trace!("event: {:?}", event);
                    match event {
                        Event::KeyDown(_, Key::KEY_P) => sys.0.play_sound(registry::SOUND.id),
                        Event::KeyDown(_, Key::KEY_S) => sys.0.stop_sound(registry::SOUND.id),
                        Event::KeyDown(_, Key::DIGIT1) => {
                            log::set_max_level(log::LevelFilter::Info)
                        }
                        Event::KeyDown(_, Key::DIGIT2) => {
                            log::set_max_level(log::LevelFilter::Debug)
                        }
                        Event::KeyDown(_, Key::DIGIT3) => {
                            log::set_max_level(log::LevelFilter::Trace)
                        }
                        Event::KeyDown(_, key) => KEYBOARD.set(key, true),
                        Event::KeyUp(_, key) => KEYBOARD.set(key, false),
                        _ => (),
                    }
                }
            }
        }
    }
}
