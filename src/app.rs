use yew::prelude::*;

use crate::game::{self, Mode, Puzzle, puzzle};

pub enum Msg {
    SelectMode(Mode),
    SetCell(usize, usize, Option<u8>),
    CheckSolution,
    NewPuzzle,
}

pub struct App {
    puzzle: Option<Puzzle>,
    mode: Mode,
    clicks: u64,
    message: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            puzzle: None,
            mode: Mode::Easy,
            clicks: 0,
            message: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.clicks = self.clicks.wrapping_add(1);
        match msg {
            Msg::SelectMode(mode) => {
                self.mode = mode;
                self.puzzle = Some(game::puzzle::generate(mode, self.clicks));
                self.message.clear();
                true
            }
            Msg::SetCell(row, col, value) => {
                if let Some(ref mut puzzle) = self.puzzle {
                    puzzle.grid.set(row, col, value);
                }
                self.message.clear();
                true
            }
            Msg::CheckSolution => {
                let puzzle = match &self.puzzle {
                    Some(p) => p,
                    None => return false,
                };
                if puzzle::is_valid_solution(&puzzle.grid) {
                    self.message = "Correct!".into();
                } else {
                    self.message = "Incorrect".into();
                }
                true
            }
            Msg::NewPuzzle => {
                self.puzzle = Some(game::puzzle::generate(self.mode, self.clicks));
                self.message.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let modes = [
            (Mode::Easy, "Easy (3 filled)"),
            (Mode::Medium, "Medium (2 filled)"),
            (Mode::Hard, "Hard (1 filled)"),
            (Mode::Extreme, "Extreme (0 filled)"),
        ];

        html! {
            <div class="app">
                <h1>{"1-2-3-4"}</h1>
                <p class="subtitle">{"Fill the grid using 1, 2, 3, 4 exactly once. Odd squares must touch other odd squares."}</p>

                <div class="mode-selector">
                    {for modes.iter().map(|(mode, label)| {
                        let mode = *mode;
                        let cb = ctx.link().callback(move |_| Msg::SelectMode(mode));
                        html! {
                            <button class={if self.mode == mode { "mode-btn active" } else { "mode-btn" }} onclick={cb}>
                                {label}
                            </button>
                        }
                    })}
                </div>

                <div class="grid-container">
                    {self.render_grid(ctx)}
                </div>

                <div class="actions">
                    <button class="check-btn" onclick={ctx.link().callback(|_| Msg::CheckSolution)}>
                        {"Check Solution"}
                    </button>
                    <button class="new-btn" onclick={ctx.link().callback(|_| Msg::NewPuzzle)}>
                        {"New Puzzle"}
                    </button>
                </div>

                if !self.message.is_empty() {
                    <p class={if self.message == "Correct!" { "message success" } else { "message error" }}>
                        {&self.message}
                    </p>
                }
            </div>
        }
    }
}

impl App {
    fn render_grid(&self, ctx: &Context<Self>) -> Html {
        let puzzle = match &self.puzzle {
            Some(p) => p,
            None => return html! { <p>{"Select a mode to begin."}</p> },
        };

        html! {
            <table class="grid">
                {for (0..2).map(|row| {
                    html! {
                        <tr>
                            {for (0..2).map(|col| {
                                self.render_cell(ctx, row, col, puzzle)
                            })}
                        </tr>
                    }
                })}
            </table>
        }
    }

    fn render_cell(&self, ctx: &Context<Self>, row: usize, col: usize, puzzle: &Puzzle) -> Html {
        let cell = puzzle.grid.get(row, col);
        let prefilled = puzzle.prefilled[row][col];

        let class = if prefilled {
            "cell prefilled"
        } else if cell.is_some() {
            "cell filled"
        } else {
            "cell empty"
        };

        let value = match cell {
            Some(n) => n.to_string(),
            None => String::new(),
        };

        let onclick = if prefilled {
            None
        } else {
            let cb = ctx
                .link()
                .callback(move |_| Msg::SetCell(row, col, next_value(cell)));
            Some(cb)
        };

        html! {
            <td class={class} onclick={onclick}>
                {value}
            </td>
        }
    }
}

/// Cycle through 1..=4, then None when the cell is clicked.
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
