
mod note;

use std::ops::Range;

use automerge::Change;
use crux_core::{render::Render, App};
use crux_kv::{KeyValue, KeyValueOutput};
use crux_macros::Effect;
use serde::{Deserialize, Serialize};

use crate::capabilities::{
    pub_sub::PubSub,
    timer::{Timer, TimerOutput},
};

pub use note::Note;

use self::note::EditObserver;

#[derive(Default)]
pub struct NoteEditor;

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    Open,
    Insert(String),
    Replace(usize, usize, String),
    MoveCursor(usize),
    Select(usize, usize),
    Backspace,
    Delete,
    ReceiveChanges(Vec<u8>),
    EditTimer(TimerOutput),
    Written(KeyValueOutput),
    Load(KeyValueOutput),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum TextCursor {
    Position(usize),
    Selection(Range<usize>),
}

impl Default for TextCursor {
    fn default() -> Self {
        TextCursor::Position(0)
    }
}

#[derive(Default)]
struct EditTimer {
    current_id: Option<u64>,
    next_id: u64,
}

impl EditTimer {
    fn start(&mut self, timer: &Timer<Event>) {
        if let Some(id) = self.current_id {
            println!("Cancelling timer {id}");
            timer.cancel(id);
        }
        self.current_id = None;

        println!("Starting timer {}", self.next_id);
        timer.start(self.next_id, EDIT_TIMER, Event::EditTimer);
    }

    fn was_created(&mut self, id: u64) {
        println!("Timer {id} created, setting next_id to {}", id + 1);
        self.next_id = id + 1;
        self.current_id = Some(id);
    }

    fn finished(&mut self, id: u64) {
        println!("Timer {id} finished");
        self.current_id = None;
    }
}

#[derive(Default)]
pub struct Model {
    note: Note,
    cursor: TextCursor,
    edit_timer: EditTimer,
}

// Same as Model for now, but may change
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ViewModel {
    text: String,
    cursor: TextCursor,
}

impl From<&Model> for ViewModel {
    fn from(model: &Model) -> Self {
        ViewModel {
            text: model.note.text(),
            cursor: model.cursor.clone(),
        }
    }
}

#[derive(Effect)]
#[effect(app = "NoteEditor")]
pub struct Capabilities {
    timer: Timer<Event>,
    render: Render<Event>,
    pub_sub: PubSub<Event>,
    key_value: KeyValue<Event>,
}

const EDIT_TIMER: usize = 1000;

impl App for NoteEditor {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;

    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match event {
            Event::Insert(text) => {
                let mut change = match &model.cursor {
                    TextCursor::Position(idx) => model.note.splice_text(*idx, 0, &text),
                    TextCursor::Selection(range) => {
                        model
                            .note
                            .splice_text(range.start, range.end - range.start, &text)
                    }
                };

                caps.pub_sub.publish(change.bytes().to_vec());
                model.edit_timer.start(&caps.timer);

                let len = text.chars().count();
                let idx = match &model.cursor {
                    TextCursor::Position(idx) => *idx,
                    TextCursor::Selection(range) => range.start,
                };
                model.cursor = TextCursor::Position(idx + len);

                caps.render.render();
            }
            Event::Replace(from, to, text) => {
                let idx = from + text.chars().count();
                model.cursor = TextCursor::Position(idx);

                let mut change = model.note.splice_text(from, to - from, &text);

                caps.pub_sub.publish(change.bytes().to_vec());
                model.edit_timer.start(&caps.timer);

                caps.render.render();
            }
            Event::MoveCursor(idx) => {
                model.cursor = TextCursor::Position(idx);

                caps.render.render();
            }
            Event::Select(from, to) => {
                model.cursor = TextCursor::Selection(from..to);

                caps.render.render();
            }
            Event::Backspace | Event::Delete => {
                let (new_index, mut change) = match &model.cursor {
                    TextCursor::Position(idx) => {
                        let idx = *idx;
                        let (remove, new_idx) = match event {
                            Event::Backspace => ((idx - 1)..idx, idx - 1),
                            Event::Delete => (idx..(idx + 1), idx),
                            _ => unreachable!(),
                        };

                        let change =
                            model
                                .note
                                .splice_text(remove.start, remove.end - remove.start, "");

                        (new_idx, change)
                    }
                    TextCursor::Selection(range) => {
                        let change =
                            model
                                .note
                                .splice_text(range.start, range.end - range.start, "");

                        (range.start, change)
                    }
                };

                model.cursor = TextCursor::Position(new_index);

                caps.pub_sub.publish(change.bytes().to_vec());
                model.edit_timer.start(&caps.timer);

                caps.render.render();
            }
            Event::ReceiveChanges(bytes) => {
                let change = Change::from_bytes(bytes).expect("a valid change");
                let mut observer = CursorObserver {
                    cursor: model.cursor.clone(),
                };

                model.note.apply_changes_with([change], &mut observer);
                model.cursor = observer.cursor;

                caps.render.render();
            }
            Event::EditTimer(TimerOutput::Created { id }) => {
                model.edit_timer.was_created(id);
            }
            Event::EditTimer(TimerOutput::Finished { id }) => {
                model.edit_timer.finished(id);

                caps.key_value
                    .write("note", model.note.save(), Event::Written);
            }
            Event::Written(_) => {
                // FIXME assuming successful write
            }
            Event::Open => caps.key_value.read("note", Event::Load),
            Event::Load(KeyValueOutput::Read(Some(data))) => {
                model.note = Note::load(&data);

                caps.pub_sub.subscribe(Event::ReceiveChanges);
                caps.render.render();
            }
            Event::Load(KeyValueOutput::Read(None)) => {
                model.note = Note::new();

                caps.key_value
                    .write("note", model.note.save(), Event::Written);
                caps.pub_sub.subscribe(Event::ReceiveChanges);

                caps.render.render();
            }
            Event::Load(KeyValueOutput::Write(_)) => unreachable!(),
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        model.into()
    }
}

struct CursorObserver {
    cursor: TextCursor,
}

impl EditObserver for CursorObserver {
    fn body_insert(&mut self, loc: usize, len: usize, _text: &str) {
        self.update_cursor(loc, len as isize);
    }

    fn body_remove(&mut self, loc: usize, len: usize) {
        self.update_cursor(loc, -(len as isize));
    }
}

impl CursorObserver {
    fn update_cursor(&mut self, loc: usize, delta: isize) {
        self.cursor = match &self.cursor {
            TextCursor::Position(position) => {
                let pos = *position as isize;

                if loc < *position {
                    TextCursor::Position((pos + delta) as usize)
                } else {
                    self.cursor.clone()
                }
            }
            TextCursor::Selection(range) => {
                let (start, end) = (range.start as isize, range.end as isize);

                match range {
                    _ if loc < range.start => {
                        let new_range = ((start + delta) as usize)..((end + delta) as usize);

                        TextCursor::Selection(new_range)
                    }
                    _ if loc >= range.start && loc < range.end => {
                        let new_range = range.start..((end + delta) as usize);

                        TextCursor::Selection(new_range)
                    }
                    _ => self.cursor.clone(),
                }
            }
        };
    }
}

#[cfg(test)]
mod editing_tests {
    use crux_core::{render::RenderOperation, testing::AppTester};

    use super::*;

    #[test]
    fn renders_text_and_cursor() {
        let app = AppTester::<NoteEditor, _>::default();

        let model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Position(2),
            ..Default::default()
        };
        let actual = app.view(&model);

        let expected = ViewModel {
            text: "hello".to_string(),
            cursor: TextCursor::Position(2),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn moves_cursor() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Position(3),
            ..Default::default()
        };

        let update = app.update(Event::MoveCursor(5), &mut model);
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "hello".to_string());
        assert_eq!(view.cursor, TextCursor::Position(5));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }

    #[test]
    fn changes_selection() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Position(3),
            ..Default::default()
        };

        let update = app.update(Event::Select(2, 5), &mut model);
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "hello".to_string());
        assert_eq!(view.cursor, TextCursor::Selection(2..5));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }

    #[test]
    fn inserts_text_at_cursor_and_renders() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Position(3),
            ..Default::default()
        };

        let update = app.update(Event::Insert("l to the ".to_string()), &mut model);
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "hell to the lo".to_string());
        assert_eq!(view.cursor, TextCursor::Position(12));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }

    #[test]
    fn replaces_selection_and_renders() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Selection(3..5),
            ..Default::default()
        };

        let update = app.update(Event::Insert("ter skelter".to_string()), &mut model);
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "helter skelter".to_string());
        assert_eq!(view.cursor, TextCursor::Position(14));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }

    #[test]
    fn replaces_range_and_renders() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Position(3),
            ..Default::default()
        };

        let update = app.update(Event::Replace(1, 4, "i, y".to_string()), &mut model);
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "hi, yo".to_string());
        assert_eq!(view.cursor, TextCursor::Position(5));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }

    #[test]
    fn replaces_empty_range_and_renders() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Position(3),
            ..Default::default()
        };

        let update = app.update(
            Event::Replace(1, 1, "ey, just saying h".to_string()),
            &mut model,
        );
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "hey, just saying hello".to_string());
        assert_eq!(view.cursor, TextCursor::Position(18));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }

    #[test]
    fn removes_character_before_cursor() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Position(2),
            ..Default::default()
        };

        let update = app.update(Event::Backspace, &mut model);
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "hllo".to_string());
        assert_eq!(view.cursor, TextCursor::Position(1));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }

    #[test]
    fn removes_character_after_cursor() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Position(2),
            ..Default::default()
        };

        let update = app.update(Event::Delete, &mut model);
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "helo".to_string());
        assert_eq!(view.cursor, TextCursor::Position(2));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }

    #[test]
    fn removes_selection_on_delete() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Selection(2..4),
            ..Default::default()
        };

        let update = app.update(Event::Delete, &mut model);
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "heo".to_string());
        assert_eq!(view.cursor, TextCursor::Position(2));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }

    #[test]
    fn removes_selection_on_backspace() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Selection(2..4),
            ..Default::default()
        };

        let update = app.update(Event::Backspace, &mut model);
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "heo".to_string());
        assert_eq!(view.cursor, TextCursor::Position(2));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }

    #[test]
    fn handles_emoji() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            // the emoji has a skintone modifier, which is a separate unicode character
            note: Note::with_text("Hello 🙌🏻 world."),
            cursor: TextCursor::Selection(3..12),
            ..Default::default()
        };

        // Replace the ' w' after the emoji
        let update = app.update(Event::Replace(8, 10, "🥳🙌🏻 w".to_string()), &mut model);
        let expected_effect = Effect::Render(RenderOperation);

        let view = app.view(&model);

        assert_eq!(view.text, "Hello 🙌🏻🥳🙌🏻 world.".to_string());
        assert_eq!(view.cursor, TextCursor::Position(13));

        assert!(
            update.effects.iter().any(|e| e == &expected_effect),
            "didn't render"
        );
    }
}

#[cfg(test)]
mod save_load_tests {
    use crux_core::testing::AppTester;
    use crux_kv::KeyValueOperation;

    use crate::capabilities::timer::{TimerOperation, TimerOutput};

    use super::*;

    #[test]
    fn opens_a_document() {
        let app = AppTester::<NoteEditor, _>::default();
        let mut note = Note::with_text("LOADED");

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Selection(2..4),
            ..Default::default()
        };

        // this will eventually take a document ID
        let update = app.update(Event::Open, &mut model);
        let key_value_effs = update
            .effects
            .iter()
            .filter(|e| matches!(e.as_ref(), Effect::KeyValue(_op)))
            .collect::<Vec<_>>();

        assert_eq!(key_value_effs.len(), 1);
        assert!(
            matches!(key_value_effs[0].as_ref(), Effect::KeyValue(KeyValueOperation::Read(key)) if key == &"note".to_string()),
            "Expected a read with key 'note', got {:?}",
            key_value_effs[0].as_ref()
        );

        // Read was successful
        let update = key_value_effs[0].resolve(&KeyValueOutput::Read(Some(note.save())));
        assert_eq!(update.events.len(), 1);

        for e in update.events {
            let update = app.update(e, &mut model);
            let renders = update
                .effects
                .iter()
                .any(|e| matches!(e.as_ref(), Effect::Render(_)));

            assert!(renders)
        }

        assert_eq!(app.view(&model).text, "LOADED");
    }

    #[test]
    fn creates_a_document_if_it_cant_open_one() {
        let app = AppTester::<NoteEditor, _>::default();

        let mut model = Model {
            note: Note::with_text("hello"),
            cursor: TextCursor::Selection(2..4),
            ..Default::default()
        };

        // this will eventually take a document ID
        let update = app.update(Event::Open, &mut model);
        let key_value_effs = update
            .effects
            .iter()
            .filter(|e| matches!(e.as_ref(), Effect::KeyValue(_op)))
            .collect::<Vec<_>>();

        assert_eq!(key_value_effs.len(), 1);
        assert!(
            matches!(key_value_effs[0].as_ref(), Effect::KeyValue(KeyValueOperation::Read(key)) if key == &"note".to_string()),
            "Expected a read with key 'note', got {:?}",
            key_value_effs[0].as_ref()
        );

        // Read was unsuccsessful
        let update = key_value_effs[0].resolve(&KeyValueOutput::Read(None));
        assert_eq!(update.events.len(), 1);

        for e in update.events {