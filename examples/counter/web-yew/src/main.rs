
use anyhow::Result;
use bcs::{from_bytes, to_bytes};
use gloo_net::http;
use yew::{html::Scope, prelude::*};

use shared::{
    http::protocol::{HttpHeader, HttpRequest, HttpResponse},
    sse::{SseRequest, SseResponse},
    Effect, Event, Request, ViewModel,
};

#[derive(Default)]
struct RootComponent;

enum CoreMessage {
    Event(Event),
    Response(Vec<u8>, Outcome),
}

pub enum Outcome {
    Http(HttpResponse),
    Sse(SseResponse),
}

impl Component for RootComponent {
    type Message = CoreMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        link.send_message(CoreMessage::Event(Event::StartWatch));

        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        let reqs = match msg {
            CoreMessage::Event(event) => shared::process_event(&to_bytes(&event).unwrap()),
            CoreMessage::Response(uuid, outcome) => shared::handle_response(
                &uuid,
                &match outcome {
                    Outcome::Http(x) => to_bytes(&x).unwrap(),
                    Outcome::Sse(x) => to_bytes(&x).unwrap(),
                },
            ),
        };

        let reqs: Vec<Request<Effect>> = from_bytes(&reqs).unwrap();

        let mut should_render = false;

        for Request { uuid, effect } in reqs {
            match effect {
                Effect::Render(_) => should_render = true,
                Effect::Http(HttpRequest {
                    url,
                    method,
                    headers,
                }) => {
                    wasm_bindgen_futures::spawn_local({
                        let method = match method.as_str() {
                            "GET" => http::Method::GET,
                            "POST" => http::Method::POST,
                            _ => panic!("not yet handling this method"),
                        };
                        let link = link.clone();

                        async move {
                            http(&uuid, &url, method, &headers, &link).await.unwrap();
                        }
                    });
                }
                Effect::ServerSentEvents(SseRequest { url }) => {
                    wasm_bindgen_futures::spawn_local({
                        let link = link.clone();

                        async move {
                            sse(&uuid, &url, &link).await.unwrap();
                        }
                    });
                }
            }
        }

        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let view = shared::view();
        let view: ViewModel = from_bytes(&view).unwrap();

        html! {
            <>
                <section class="section has-text-centered">
                    <p class="title">{"Crux Counter Example"}</p>
                </section>
                <section class="section has-text-centered">
                    <p class="is-size-5">{"Rust Core, Rust Shell (Yew)"}</p>
                </section>
                <section class="container has-text-centered">
                    <p class="is-size-5">{&view.text}</p>
                    <div class="buttons section is-centered">
                        <button class="button is-primary is-warning"
                            onclick={link.callback(|_| CoreMessage::Event(Event::Decrement))}>
                            {"Decrement"}
                        </button>
                        <button class="button is-primary is-danger"
                            onclick={link.callback(|_| CoreMessage::Event(Event::Increment))}>
                            {"Increment"}
                        </button>
                    </div>
                </section>
            </>
        }
    }
}

async fn http(
    uuid: &[u8],
    url: &str,
    method: http::Method,
    headers: &[HttpHeader],
    link: &Scope<RootComponent>,
) -> Result<()> {
    let mut request = http::Request::new(url).method(method);

    for header in headers {
        request = request.header(&header.name, &header.value);
    }

    let response = request.send().await?;
    let body = response.binary().await?;
    link.send_message(CoreMessage::Response(
        uuid.to_vec(),
        Outcome::Http(HttpResponse {
            status: response.status(),
            body,
        }),
    ));

    Ok(())
}

async fn sse(uuid: &[u8], url: &str, link: &Scope<RootComponent>) -> Result<()> {
    use futures_util::StreamExt;
    use js_sys::Uint8Array;
    use wasm_bindgen::{prelude::*, JsCast};
    use wasm_streams::ReadableStream;

    let response = http::Request::new(url).send().await?;

    let raw_body = response.body().unwrap_throw();
    let body = ReadableStream::from_raw(raw_body.dyn_into().unwrap_throw());

    let mut stream = body.into_stream();

    while let Some(Ok(chunk)) = stream.next().await {
        let chunk: Uint8Array = chunk.dyn_into().unwrap();
        link.send_message(CoreMessage::Response(
            uuid.to_vec(),
            Outcome::Sse(SseResponse::Chunk(chunk.to_vec())),
        ));
    }

    Ok(())
}

fn main() {
    yew::Renderer::<RootComponent>::new().render();
}