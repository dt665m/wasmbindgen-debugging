use wasm_bindgen::{prelude::*, JsValue};

fn main() {
    let test = get_test().unwrap().unwrap();
    let payload = Req::new("hello world".to_owned());
    test.request(payload).unwrap();
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Req {
    method: String,
}

#[wasm_bindgen]
impl Req {
    pub fn new(method: String) -> Req {
        Req { method }
    }

    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.clone()
    }
}

#[wasm_bindgen(inline_js = "export function get_test() { return window.testing }")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn get_test() -> Result<Option<Testing>, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Testing;

    #[wasm_bindgen(catch, method)]
    fn request(_: &Testing, args: Req) -> Result<JsValue, JsValue>;
}
