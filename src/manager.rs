#[repr(transparent)]
pub struct SendFlags(_);
pub struct Unlocked;



pub struct Manager<State = Unlocked> {
    paths: Vec<(String, String)>,
    state: std::marker::PhantomData<State>
}

impl Manager {
    pub fn new() -> Self {
        let home_dir = std::env::var("HOME");
        if home_dir.is_err() {
            panic!("Home directory not found!")
        }
        Manager {
            paths: Vec::new(),
            state: Default::default(),
        }
    }
}