pub mod game;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, EventTarget};

use crate::game::{Mode, Puzzle, puzzle};

const MODES: [Mode; 4] = [Mode::Easy, Mode::Medium, Mode::Hard, Mode::Extreme];
const MODE_LABELS: [&str; 4] = [
    "Easy (3 filled)",
    "Medium (2 filled)",
    "Hard (1 filled)",
    "Extreme (0 filled)",
];

const HTML: &str = r#"
<div class="app">
    <h1>1-2-3-4</h1>
    <p class="subtitle">Fill the grid using 1, 2, 3, 4 exactly once. Odd squares must touch other odd squares. Select a mode to get started.</p>
    <div class="mode-selector" id="mode-selector"></div>
    <div class="grid-container">
        <table class="grid">
            <tr><td id="cell-00" class="cell empty"></td><td id="cell-01" class="cell empty"></td></tr>
            <tr><td id="cell-10" class="cell empty"></td><td id="cell-11" class="cell empty"></td></tr>
        </table>
    </div>
    <div class="actions">
        <button id="check-btn" class="check-btn">Check Solution</button>
        <button id="new-btn" class="new-btn">New Puzzle</button>
    </div>
    <p id="message" class="message"></p>
</div>
"#;

struct Dom {
    cells: [Element; 4],
    mode_buttons: [Element; 4],
    message: Element,
}

struct State {
    mode: Mode,
    clicks: u64,
    puzzle: Option<Puzzle>,
    message: String,
    dom: Dom,
}

fn next_value(current: Option<u8>) -> Option<u8> {
    match current {
        None => Some(1),
        Some(1) => Some(2),
        Some(2) => Some(3),
        Some(3) => Some(4),
        Some(4) => None,
        _ => None,
    }
}

impl State {
    fn render(&self) {
        for (i, m) in MODES.iter().enumerate() {
            self.dom.mode_buttons[i].set_class_name(if *m == self.mode {
                "mode-btn active"
            } else {
                "mode-btn"
            });
        }

        if let Some(ref puzzle) = self.puzzle {
            for (idx, cell) in self.dom.cells.iter().enumerate() {
                let row = idx / 2;
                let col = idx % 2;
                let val = puzzle.grid.get(row, col);
                let prefilled = puzzle.prefilled[row][col];
                cell.set_class_name(match (prefilled, val) {
                    (true, _) => "cell prefilled",
                    (_, Some(_)) => "cell filled",
                    _ => "cell empty",
                });
                cell.set_text_content(val.map(|n| n.to_string()).as_deref());
            }
        } else {
            for cell in &self.dom.cells {
                cell.set_class_name("cell empty");
                cell.set_text_content(None);
            }
        }

        self.dom.message.set_class_name("message");
        self.dom.message.set_text_content(None);
        if !self.message.is_empty() {
            self.dom.message.set_text_content(Some(&self.message));
            self.dom
                .message
                .set_class_name(if self.message == "Correct!" {
                    "message success"
                } else {
                    "message error"
                });
        }
    }
}

fn add_click(el: &Element, f: impl 'static + FnMut()) {
    let closure = Closure::new(f);
    el.unchecked_ref::<EventTarget>()
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let root = document.get_element_by_id("app").unwrap();
    root.set_inner_html(HTML);

    let cells = [
        root.query_selector("#cell-00").unwrap().unwrap(),
        root.query_selector("#cell-01").unwrap().unwrap(),
        root.query_selector("#cell-10").unwrap().unwrap(),
        root.query_selector("#cell-11").unwrap().unwrap(),
    ];

    let mode_selector = root.query_selector("#mode-selector").unwrap().unwrap();
    let mode_buttons: [Element; 4] = std::array::from_fn(|i| {
        let btn = document.create_element("button").unwrap();
        btn.set_class_name("mode-btn");
        btn.set_text_content(Some(MODE_LABELS[i]));
        mode_selector.append_child(&btn).unwrap();
        btn
    });

    let message_el = root.query_selector("#message").unwrap().unwrap();
    let check_btn = root.query_selector("#check-btn").unwrap().unwrap();
    let new_btn = root.query_selector("#new-btn").unwrap().unwrap();

    let state = Rc::new(RefCell::new(State {
        mode: Mode::Easy,
        clicks: 0,
        puzzle: None,
        message: String::new(),
        dom: Dom {
            cells,
            mode_buttons,
            message: message_el,
        },
    }));

    for (i, m) in MODES.iter().enumerate() {
        let mode = *m;
        let s = state.clone();
        let el = state.borrow().dom.mode_buttons[i].clone();
        add_click(&el, move || {
            let mut s = s.borrow_mut();
            s.clicks = s.clicks.wrapping_add(1);
            s.mode = mode;
            s.puzzle = Some(puzzle::generate(mode, s.clicks));
            s.message.clear();
            s.render();
        });
    }

    for idx in 0..4 {
        let row = idx / 2;
        let col = idx % 2;
        let s = state.clone();
        let el = state.borrow().dom.cells[idx].clone();
        add_click(&el, move || {
            let mut s = s.borrow_mut();
            s.clicks = s.clicks.wrapping_add(1);
            if let Some(ref mut pz) = s.puzzle
                && !pz.prefilled[row][col]
            {
                let current = pz.grid.get(row, col);
                pz.grid.set(row, col, next_value(current));
                s.message.clear();
            }
            s.render();
        });
    }

    {
        let s = state.clone();
        let el = check_btn.clone();
        add_click(&el, move || {
            let mut s = s.borrow_mut();
            s.clicks = s.clicks.wrapping_add(1);
            if let Some(ref pz) = s.puzzle {
                if puzzle::is_valid_solution(&pz.grid) {
                    s.message = "Correct!".into();
                } else {
                    s.message = "Incorrect".into();
                }
            }
            s.render();
        });
    }

    {
        let s = state.clone();
        let el = new_btn.clone();
        add_click(&el, move || {
            let mut s = s.borrow_mut();
            s.clicks = s.clicks.wrapping_add(1);
            s.puzzle = Some(puzzle::generate(s.mode, s.clicks));
            s.message.clear();
            s.render();
        });
    }

    state.borrow().render();
}
