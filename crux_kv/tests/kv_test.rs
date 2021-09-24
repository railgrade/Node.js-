mod shared {
    use crux_core::render::Render;
    use crux_kv::{KeyValue, KeyValueOutput};
    use crux_macros::Effect;
    use serde::{Deserialize, Serialize};

    #[derive(Default)]
    pub struct App;

    #[derive(Serialize, Deserialize)]
    pub enum Event {
        Write,
        Read,
        Set(KeyValueOutput),
    }

    #[derive(Default, Serialize, Deserialize)]
    pub struct Model {
        pub value: i32,
        pub successful: bo