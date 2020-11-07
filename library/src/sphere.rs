use std::sync::Mutex;

lazy_static::lazy_static! {
  static ref NEXT_ID: Mutex<u32> = Mutex::new(1);
}

pub struct Sphere {
    pub id: u32,
}

impl Sphere {
    pub fn new() -> Sphere {
        let mut next_id_mtx = NEXT_ID.lock().unwrap();

        let next_id = *next_id_mtx;
        *next_id_mtx += 1;

        Sphere { id: next_id }
    }
}
