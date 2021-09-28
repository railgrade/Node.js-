mod shared {
    use crux_core::render::Render;
    use crux_macros::Effect;
    use crux_platform::{Platform, PlatformResponse};
    use serde::{Deserialize, Serialize};

    #[derive(Default)]
    pub struct App;

    #[derive(Serialize, Deserialize)]
    pub enum Event {
        PlatformGet,
        PlatformSet(PlatformResponse),
    }

    #[derive(Default, Serialize, Deserialize)]
    pub struct Model {
        pub platform: String,
    }

    #[derive(Serialize, Deserialize, Default)]
    pub struct ViewModel {
        pub platform: String,
    }

    impl crux_core::App for App {
        type Event = Event;
        type Model = Model;
        type ViewModel = ViewModel;
        type Capabilities = Capabilities;

        fn update(&self, event: Event, model: &mut Model, caps: &Capabilities) {
            match event {
                Event::PlatformGet => caps.platform.get(Event::PlatformSet),
                Event::PlatformSet(platform) => {
                    model.platform = platform.0;
                    caps.render.render()
                }
            }
        }

        fn view(&self, model: &Self::Model) -> Self::ViewModel {
            ViewModel {
                platform: model.platform.clone(),
   