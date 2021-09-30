mod shared {
    use crux_core::render::Render;
    use crux_macros::Effect;
    use crux_time::{Time, TimeResponse};
    use serde::{Deserialize, Serialize};

    #[derive(Default)]
    pub struct App;

    #[derive(Serialize, Deserialize)]
    pub enum Event {
        TimeGet,
        TimeSet(TimeResponse),
    }

    #[derive(Default, Serialize, Deserialize)]
    pub struct Model {
        pub time: String,
    }

    #