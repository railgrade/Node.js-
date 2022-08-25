pub mod platform;

use serde::{Deserialize, Serialize};

pub use crux_core::App;
use crux_core::{render::Render, Capability};
use crux_http::Http;
use crux_kv::{KeyValue, KeyValueOutput};
use crux_macros::Effect;
use crux_platform::Platform;
use crux_time::{Time, TimeResponse};

use platform::{PlatformCapabilities, PlatformEvent};

const CAT_LOADING_URL: &str = "https://c.tenor.com/qACzaJ1EBVYAAAAd/tenor.gif";
const FACT_API_URL: &str = "https://catfact.ninja/fact";
const IMAGE_API_URL: &str = "https://aws.random.cat/meow";

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct CatFact {
    fact: String,
    length: i32,
}

impl CatFact {
    fn format(&self) -> String {
        format!("{} ({} bytes)", self.fact, self.length)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    cat_fact: Option<CatFact>,
    cat_image: Option<CatImage>,
    platform: platform::Model,
    time: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CatImage {
    pub file: String,
}

impl Default for CatImage {
    fn default() -> Self {
        Self {
            file: CAT_LOADING_URL.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ViewModel {
    pub fact: String,
    pub image: Option<CatImage>,
    pub platform: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
    // events from the shell
    None,
    Clear,
    Get,
    Fetch,
    GetPlatform,
    Restore, // restore state

    // events local to the core
    Platform(PlatformEvent),
    SetState(KeyValueOutput), // receive the data to restore state with
    CurrentTime(TimeResponse),
    #[serde(skip)]
    SetFact(crux_http::Result<crux_http::Response<CatFact>>),
    #[serde(skip)]
    SetImage(crux_http::Result<crux_http::Response<CatImage>>),
}

#[derive(Default)]
pub struct CatFacts {
    platform: platform::Platform,
}

#[derive(Effect)]
#[effect(app = "CatFacts")]
pub struct CatFactCapabilities {
    pub http: Http<Event>,
    pub key_value: KeyValue<Event>,
    pub platform: Platform<Event>,
    pub render: Render<Event>,
    pub time: Time<Event>,
}

// Allow easily using Platform as a submodule
impl From<&CatFactCapabilities> for PlatformCapabilities {
    fn from(incoming: &CatFactCapabilities) -> Self {
        PlatformCapabilities {
            platform: incoming.platform.map_event(super::Event::Platform),
            render: incoming.render.map_event(super::Event::Platform),
        }
    }
}

impl App for CatFacts {
    type Model = Model;
    type Event = Event;
    type ViewModel = ViewModel;
    type Capabilities = CatFactCapabilities;

    fn update(&self, msg: Event, model: &mut Model, caps: &CatFactCapabilities) {
        match msg {
            Event::GetPlatform => {
                self.platform
                    .update(PlatformEvent::Get, &mut model.platform, &caps.into())
            }
            Event::Platform(msg) => self.platform.update(msg, &mut model.platform, &caps.into()),
            Event::Clear => {
                model.cat_fact = None;
                model.cat_image = None;
                let bytes = serde_json::to_vec(&model).unwrap();

                caps.key_value.write("state", bytes, |_| Event::None);
                caps.render.render();
            }
            Event::Get => {
                if let Some(_fact) = &model.cat_fact {
                    caps.render.render()
                } else {
                    self.update(Event::Fetch, model, caps)
                }
            }
            Event::Fetch => {
                model.cat_image = Some(CatImage::default());

                caps.http
                    .get(FACT_API_URL)
                    .expect_json()
                    .send(Event::SetFact);

                caps.http
                    .get(IMAGE_API_URL)
                    .expect_json()
                    .send(Event::SetImage);

                caps.render.render();
            }
            Event::SetFact(Ok(mut response)) => {
                // TODO check status
                model.cat_fact = Some(response.take_body().unwrap());

                let bytes = serde_json::to_vec(&model).unwrap();
                caps.key_value.write("state", bytes, |_| Event::None);

                caps.time.get(Event::CurrentTime);
            }
            Event::SetImage(Ok(mut response)) => {
                // TODO check status
       