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
        pub successful: bool,
    }

    #[derive(Serialize, Deserialize, Default)]
    pub struct ViewModel {
        pub result: String,
    }

    impl crux_core::App for App {
        type Event = Event;
        type Model = Model;
        type ViewModel = ViewModel;

        type Capabilities = Capabilities;

        fn update(&self, event: Event, model: &mut Model, caps: &Capabilities) {
            match event {
                Event::Write => {
                    caps.key_value
                        .write("test", 42i32.to_ne_bytes().to_vec(), Event::Set);
                }
                Event::Set(KeyValueOutput::Write(success)) => {
                    model.successful = success;
                    caps.render.render()
                }
                Event::Read => caps.key_value.read("test", Event::Set),
                Event::Set(KeyValueOutput::Read(value)) => {
                    if let Some(value) = value {
                        // TODO: should KeyValueOutput::Read be generic over the value type?
                        let (int_bytes, _rest) = value.split_at(std::mem::size_of::<i32>());
                        model.value = i32::from_ne_bytes(int_bytes.try_into().unwrap());
                    }
                    caps.render.render()
                }
            }
        }

        fn view(&self, model: &Self::Model) -> Self::ViewModel {
            ViewModel {
                result: format!("Success: {}, Value: {}", model.successful, model.value),
            }
        }
    }

    #[derive(Effect)]
    pub struct Capabilities {
        pub key_value: KeyValue<Event>