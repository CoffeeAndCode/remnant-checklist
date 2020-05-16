mod data;
mod storage;

use serde_derive::{Deserialize, Serialize};
use storage::StorageService;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    filter: Filter,
    search: String,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    completed: bool,
    data_type: data::DataType,
    icon: char,
    id: u32,
    name: String,
    url: String,
}

impl Entry {
    fn id(&self) -> String {
        format!("{}-{}", self.data_type, self.id)
    }
}

pub enum Goal {
    MarkItemAsComplete,
    MarkItemAsIncomplete,
    VisitGunfireGames,
    VisitPersonalSite,
}

pub enum Msg {
    SetFilter(Filter),
    ShareApp(String),
    Toggle(String),
    TrackGoal(Goal),
    UpdateSearch(String),
}

#[wasm_bindgen(module = "/src/js/share.js")]
extern "C" {
    #[wasm_bindgen(js_name = canShare)]
    fn can_share() -> bool;
    fn share(title: Option<String>, text: Option<String>, url: String) -> bool;
}

#[wasm_bindgen(module = "/src/js/stats.js")]
extern "C" {
    #[wasm_bindgen(js_name = markItemAsComplete)]
    fn track_mark_item_as_complete();
    #[wasm_bindgen(js_name = markItemAsIncomplete)]
    fn track_mark_item_as_incomplete();
    #[wasm_bindgen(js_name = visitPersonalSite)]
    fn track_visit_personal_site();
    #[wasm_bindgen(js_name = visitGunfireGamesSite)]
    fn track_visit_gunfire_games_site();
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new().unwrap();
        let entries = storage.restore_or_default();

        let state = State {
            entries,
            filter: Filter::Active,
            search: "".into(),
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
                true
            }
            Msg::ShareApp(url) => {
                share(Some("Remnant Checklist".into()), None, url);
                false
            }
            Msg::Toggle(id) => {
                let goal = if self.state.toggle(id) {
                    Goal::MarkItemAsComplete
                } else {
                    Goal::MarkItemAsIncomplete
                };
                self.link.send_message(Msg::TrackGoal(goal));
                self.storage.store(&self.state.entries);
                true
            }
            Msg::TrackGoal(goal) => {
                match goal {
                    Goal::MarkItemAsComplete => track_mark_item_as_complete(),
                    Goal::MarkItemAsIncomplete => track_mark_item_as_incomplete(),
                    Goal::VisitGunfireGames => track_visit_gunfire_games_site(),
                    Goal::VisitPersonalSite => track_visit_personal_site(),
                }
                false
            }
            Msg::UpdateSearch(value) => {
                self.state.search = value;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="app-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1 class="logo-container"><img alt="Remnant logo" class="img-fluid logo" src="/images/remnant-logo.png" /></h1>
                        <input
                            class="input-search"
                            placeholder="Search..."
                            oninput=self.link.callback(|e: InputData| Msg::UpdateSearch(e.value))
                            type="text"
                            value=""
                        />
                    </header>
                    <section class="main">
                        <ul class="todo-list">
                            { for self.state.entries.iter().filter(|e| self.state.filter.fit(e) && e.name.to_lowercase().contains(&self.state.search.to_lowercase()))
                                .map(|val| self.view_entry(val)) }
                        </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            <strong>{ self.state.total_incomplete() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </ul>
                    </footer>
                </section>
                <footer class="info">
                    <ul class="list-unstyled m0">
                        <li>{ "Created by " }<a href="https://coffee.dev" onclick=self.link.callback(|_| Msg::TrackGoal(Goal::VisitPersonalSite)) rel="noopener noreferrer" target="_blank">{ "Jonathan Knapp" }</a></li>
                        <li>
                            { "Game and artwork © " }
                            <a href="https://www.remnantgame.com" onclick=self.link.callback(|_| Msg::TrackGoal(Goal::VisitGunfireGames)) rel="noopener noreferrer" target="_blank">{ "Gunfire Games, LLC" }</a>
                        </li>
                    </ul>
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
                <button class="btn-share" onclick=self.link.callback(|_| Msg::ShareApp(String::from("https://remnant.coffee.dev")))>{ "Share" }<span class="d-sm-none">{ " This App" }</span></button>
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

    fn view_entry(&self, entry: &Entry) -> Html {
        let mut class = "todo".to_string();
        if entry.completed {
            class.push_str(" completed");
        }
        let id = entry.id();

        html! {
            <li class=class key=entry.id()>
                <div class="row view">
                    <div class="row-label">
                        <input class="toggle" id=entry.id() type="checkbox" checked={entry.completed} onclick=self.link.callback(move |_| Msg::Toggle(id.clone())) />
                        <label for=entry.id()>{ format!("{} {}", entry.name, entry.icon) }</label>
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
    // World(data::World),
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
            // Filter::World(world) => format!("#/world/{}", world).into(),
        }
    }
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
            // Filter::World(_) => true,
        }
    }
}

impl State {
    fn total_incomplete(&self) -> usize {
        self.entries
            .iter()
            .map(|x| if x.completed { 0 } else { 1 })
            .sum()
    }

    fn toggle(&mut self, id: String) -> bool {
        let mut entry = self.entries.iter_mut().find(|x| x.id() == id).unwrap();
        entry.completed = !entry.completed;
        entry.completed
    }
}
