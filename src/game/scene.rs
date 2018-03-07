use game::gameobject::GameObject;
use graphics::texture::FinalTexture;
use graphics::pixel::Pixel;
use std::marker::Sized;
use std::rc::Weak;
use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use game::world::WorldState;
use game::event::Event;

pub enum SceneMsg {
    Continue,
    ExitTo(String),
    Pause
}

pub struct SceneBuilder {
    init: Option<Box<fn(&mut Scene, &WorldState)>>,
    name: String
}

impl SceneBuilder {
    pub fn blank() -> Scene {
        Scene {
            entities: vec![],
            init: Box::new(Scene::default_init),
            name: "Blank".to_string(),
            next_id: 0
        }
    }

    pub fn new(name: String) -> SceneBuilder {
        SceneBuilder {
            init: None,
            name
        }
    }

    pub fn override_init(mut self, init_fn: fn(&mut Scene, &WorldState)) -> SceneBuilder {
        self.init = Some(Box::new(init_fn));
        self
    }

    pub fn build(mut self) -> Scene {
        Scene {
            entities: vec![],
            name: self.name,
            init: match self.init {
                Some(func) => func,
                None => Box::new(Scene::default_init)
            },
            next_id: 0
        }
    }
}

pub struct Scene {
    pub entities: Vec<Box<GameObject>>,
    init: Box<fn(&mut Scene, &WorldState)>,
    name: String,
    next_id: u64
}

impl Scene {
    pub fn tick(&mut self, events: Vec<Event>, world: &WorldState) -> Vec<FinalTexture> {
        for entity in &mut self.entities {
            entity.as_mut().recv("asdf".to_string());
        }
        let collected_textures = {
            let mut texture_collector = vec![];
            for entity in &mut self.entities {
                texture_collector.push(entity.as_mut().render().expect("Whoops"));
            }
            texture_collector
        };
        collected_textures
    }
    fn default_init(scene: &mut Scene, state: &WorldState) {

    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn run_init(&mut self, state: &WorldState) {
        self.init.deref()(self, state);
    }

    pub fn spawn<T : GameObject + 'static>(&mut self, coordinates: (u32,u32)) {
        self.entities.push(Box::new(T::spawn(coordinates, self.next_id)));
        self.next_id += 1;
    }
}