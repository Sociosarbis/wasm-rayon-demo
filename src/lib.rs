mod ffi;

#[cfg(feature = "parallel")]
use rayon::prelude::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen(module = "/src/random.js" )]
extern "C" {
    #[wasm_bindgen(js_name = jsRandom )]
    fn random() -> f64;
}

#[cfg(feature = "parallel")]
#[wasm_bindgen(js_name = spawnTask)]
pub fn spawn_task() -> Vec<i32> {
    let tasks: Vec<i32> = (1..10).collect();
    let out = tasks.par_iter().map(|&item| {
        println!("{:?}", item);
        let mut out = item;
        for _ in 0..8 {
            if random() > 0.5 {
                out *= item;
            }
        }
        out
    });
    out.collect()
}

#[cfg(not(feature = "parallel"))]
#[wasm_bindgen(js_name = spawnTask)]
pub fn spawn_task() -> Vec<i32> {
    let tasks: Vec<i32> = (1..10).collect();
    let out = tasks.iter().map(|&item| {
        println!("{:?}", item);
        let mut out = item;
        for _ in 0..8 {
            out *= item;
        }
        out
    });
    out.collect()
}
