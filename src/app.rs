mod data;

use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::prelude::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "yew.todomvc.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    filter: Filter,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    completed: bool,
    data_type: data::DataType,
    icon: char,
    name: String,
    url: String,
}

pub enum Msg {
    SetFilter(Filter),
    ShareApp(String),
    ToggleAll,
    Toggle(usize),
}

#[wasm_bindgen(module = "/src/js/share.js")]
extern "C" {
    #[wasm_bindgen(js_name = canShare)]
    fn can_share() -> bool;
    fn share(title: String, text: String, url: String) -> bool;
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let entries = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                let mut entries = data::remnant_trait_entries();
                entries.append(&mut data::amulet_entries());
                entries.append(&mut data::armor_set_entries());
                entries.append(&mut data::head_armor_entries());
                entries.append(&mut data::body_armor_entries());
                entries.append(&mut data::leg_armor_entries());
                entries.append(&mut data::emote_entries());
                entries.append(&mut data::ring_entries());
                entries.append(&mut data::hand_gun_entries());
                entries.append(&mut data::long_gun_entries());
                entries.append(&mut data::melee_weapon_entries());
                entries
            }
        };

        let state = State {
            entries,
            filter: Filter::All,
        };
        App {
            link,
            storage,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ShareApp(url) => {
                share(
                    "Remnant Checklist".into(),
                    "Track your unlocked items with an offline enabled webapp!".into(),
                    url,
                );
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
            }
        }
        self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "remnant" }</h1>
                    </header>
                    <section class="main">
                        <input class="toggle-all" type="checkbox" checked=self.state.is_all_completed() onclick=self.link.callback(|_| Msg::ToggleAll) />
                        <ul class="todo-list">
                            { for self.state.entries.iter().filter(|e| self.state.filter.fit(e))
                                .enumerate()
                                .map(|val| self.view_entry(val)) }
                        </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            <strong>{ self.state.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </ul>
                    </footer>
                </section>
                <footer class="info">
                    <p>{ "Written by " }<a href="https://coffee.dev" target="_blank">{ "Jonathan Knapp" }</a></p>
                    { self.view_share() }
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_share(&self) -> Html {
        if can_share() {
            html! {
                <p>
                    <button onclick=self.link.callback(|_| Msg::ShareApp(String::from("https://remnant.coffee.dev")))>{ "Share This App" }</button>
                </p>
            }
        } else {
            html! {}
        }
    }

    fn view_filter(&self, filter: Filter) -> Html {
        let flt = filter.clone();

        html! {
            <li>
                <a class=if self.state.filter == flt { "selected" } else { "not-selected" }
                   href=&flt
                   onclick=self.link.callback(move |_| Msg::SetFilter(flt.clone()))>
                    { filter }
                </a>
            </li>
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
        let mut class = "todo".to_string();
        if entry.completed {
            class.push_str(" completed");
        }

        html! {
            <li class=class>
                <div class="row view">
                    <div>
                        <input class="toggle" type="checkbox" checked={entry.completed} onclick=self.link.callback(move |_| Msg::Toggle(idx)) />
                        <label>{ format!("{} {}", entry.name, entry.icon) }</label>
                    </div>
                    <a class="wiki-link" href={ entry.url.clone() } rel="noopener noreferrer" target="_blank" title={format!("View {} on fextralife wiki", &entry.name)}>{ "wiki ↱" }</a>
                </div>
            </li>
        }
    }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
        }
    }
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }
}

impl State {
    fn total(&self) -> usize {
        self.entries.len()
    }

    fn is_all_completed(&self) -> bool {
        let mut filtered_iter = self
            .entries
            .iter()
            .filter(|e| self.filter.fit(e))
            .peekable();

        if filtered_iter.peek().is_none() {
            return false;
        }

        filtered_iter.all(|e| e.completed)
    }

    fn toggle_all(&mut self, value: bool) {
        for entry in self.entries.iter_mut() {
            if self.filter.fit(entry) {
                entry.completed = value;
            }
        }
    }

    fn toggle(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.completed = !entry.completed;
    }
}
