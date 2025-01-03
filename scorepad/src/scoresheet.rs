use crate::components::{IntegerInput, PlayerNameInput};
use leptos::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlayerScores {
    id: Uuid,
    name: RwSignal<String>,
    scores: Vec<Score>,
}

impl PlayerScores {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: RwSignal::new(name.to_string()),
            scores: vec![],
        }
    }

    pub fn add(&mut self, score: Score) {
        self.scores.push(score);
    }

    pub fn remove(&mut self, id: Uuid) {
        self.retain(|score| score.id != id);
    }

    pub fn sum(&self) -> i64 {
        self.scores.iter().map(|score| score.get()).sum()
    }

    fn retain(&mut self, mut f: impl FnMut(&Score) -> bool) {
        self.scores.retain(|score| {
            let retain = f(score);
            if !retain {
                score.value.dispose();
            }
            retain
        })
    }

    pub fn set_name(&mut self, name: String) {
        self.name.set(name);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Score {
    id: Uuid,
    value: RwSignal<i64>,
}

impl Default for Score {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Score {
    fn new(value: i64) -> Self {
        let value = RwSignal::new(value);
        Self {
            id: Uuid::new_v4(),
            value,
        }
    }

    fn get(&self) -> i64 {
        self.value.get()
    }
}

#[component]
pub fn Scoresheet() -> impl IntoView {
    // Replace single player with vector of players
    let (players, set_players) = signal::<Vec<PlayerScores>>(vec![PlayerScores::new("Player 1")]);

    //Initialize first round for all players
    set_players.update(|players| {
        for player in players {
            player.add(Score::default());
        }
    });

    // Create a derived signal for the sums
    let player_sums = move || {
        players
            .get()
            .iter()
            .map(|player| (player.id, player.sum()))
            .collect::<Vec<_>>()
    };

    let player_sum = move |id: Uuid| {
        player_sums()
            .iter()
            .find(|(pid, _)| *pid == id)
            .map(|(_, sum)| *sum)
            .unwrap_or(0)
    };

    view! {
        <div class="min-h-screen flex flex-col items-center justify-center p-4">
            <h1 class="text-4xl font-bold text-white mb-8">"Scoresheet"</h1>

            <table class="w-full max-w-lg bg-gray-700 text-white border border-gray-600 rounded-lg">
                <thead>
                    <tr>
                        <For
                            each=move || players.get()
                            key=|player| player.id
                            children=move |player| {
                                view! {
                                    <th class="px-4 py-2">
                                        <PlayerNameInput
                                            setter=player.name.write_only()
                                            class="bg-transparent text-white text-center w-full focus:outline-none focus:border-b focus:border-blue-500"
                                        />
                                    </th>
                                }
                            }
                        />
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || (0..players.get()[0].scores.len())
                        key=|index| *index
                        children=move |row_index| {
                            view! {
                                <tr>
                                    <For
                                        each=move || players.get()
                                        key=|player| player.id
                                        children=move |player| {
                                            let score = &player.scores[row_index];
                                            view! {
                                                <td>
                                                    <IntegerInput
                                                        setter=score.value.write_only()
                                                        class="w-full px-4 py-2 bg-gray-800 text-white border border-gray-600 text-center focus:outline-none focus:border-blue-500 transition-colors"
                                                    />
                                                </td>
                                            }
                                        }
                                    />
                                </tr>
                            }
                        }
                    />
                </tbody>
                <tfoot>
                    <tr>
                        <For
                            each=move || players.get()
                            key=|player| player.id
                            children=move |player| {
                                let id = player.id;
                                view! {
                                    <td class="px-4 py-2 text-center">{move || player_sum(id)}</td>
                                }
                            }
                        />
                    </tr>
                </tfoot>
            </table>

            // TODO: Rounds should be automatically added when a new score is added to the end.
            <button
                on:click=move |_| {
                    set_players
                        .update(|players| {
                            for player in players {
                                player.add(Score::default());
                            }
                        })
                }
                class="bg-blue-500 text-white px-4 py-2 rounded-lg mt-4"
            >
                "Add Round"
            </button>

            <button
                on:click=move |_| {
                    set_players
                        .update(|players| {
                            let mut new_player = PlayerScores::new("Player 3");
                            for _ in 0..players[0].scores.len() {
                                new_player.add(Score::default());
                            }
                            players.push(new_player);
                        })
                }
                class="bg-blue-500 text-white px-4 py-2 rounded-lg mt-4"
            >
                "Add Player"
            </button>
        </div>
    }
}
