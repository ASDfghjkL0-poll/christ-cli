use crate::api::Resolver;
use crate::store::state as session;
use crate::ui::banner::{self, BannerState};
use crate::ui::browser::{self, BrowserState};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::time::{Duration, Instant};

enum AppMode {
    Banner(BannerState),
    Browser(BrowserState),
}

pub struct App {
    mode: AppMode,
    resolver: Resolver,
    should_quit: bool,
    quit_pending: Option<Instant>,
}

impl App {
    pub fn new(show_banner: bool) -> Self {
        let mode = if show_banner {
            AppMode::Banner(BannerState::new())
        } else {
            AppMode::Browser(BrowserState::new())
        };

        Self {
            mode,
            resolver: Resolver::new(),
            should_quit: false,
            quit_pending: None,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        // Load saved session state
        let saved = session::load();
        let has_saved_session = saved.book_index > 0 || saved.chapter > 1;

        // Load the chapter from the saved session (or Genesis 1 for first run)
        let book_name = if has_saved_session {
            crate::data::books::BOOKS
                .get(saved.book_index)
                .map(|b| b.name)
                .unwrap_or("Genesis")
        } else {
            "Genesis"
        };
        let chapter_num = if has_saved_session { saved.chapter.max(1) } else { 1 };

        let initial_chapter = self
            .resolver
            .get_chapter(book_name, chapter_num, "KJV")
            .await
            .ok();

        match &mut self.mode {
            AppMode::Browser(ref mut state) => {
                if has_saved_session {
                    state.restore(&saved);
                }
                state.current_chapter = initial_chapter.clone();
            }
            _ => {}
        }

        let mut pending_initial = Some((initial_chapter, saved));

        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;

            let tick_rate = match &self.mode {
                AppMode::Banner(_) => Duration::from_millis(16),
                AppMode::Browser(_) => Duration::from_millis(50),
            };

            if event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code).await;
                    }
                }
            } else {
                if let AppMode::Banner(ref mut state) = self.mode {
                    state.tick();
                    if state.done {
                        let mut browser = BrowserState::new();
                        if let Some((ch, ref saved)) = pending_initial {
                            let has_saved = saved.book_index > 0 || saved.chapter > 1;
                            if has_saved {
                                browser.restore(saved);
                            }
                            browser.current_chapter = ch;
                        }
                        pending_initial = None;
                        self.mode = AppMode::Browser(browser);
                    }
                }
            }
        }

        // Save session state on quit
        if let AppMode::Browser(ref state) = self.mode {
            session::save(&state.snapshot());
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        if let Some(t) = self.quit_pending {
            if t.elapsed() > Duration::from_secs(2) {
                self.quit_pending = None;
            }
        }

        match &mut self.mode {
            AppMode::Banner(state) => {
                banner::render_banner(frame, area, state);
            }
            AppMode::Browser(state) => {
                browser::render_browser(frame, area, state, self.quit_pending.is_some());
            }
        }
    }

    async fn handle_key(&mut self, key: KeyCode) {
        match &mut self.mode {
            AppMode::Banner(state) => {
                state.done = true;
            }
            AppMode::Browser(state) => {
                if key == KeyCode::Char('q') {
                    if self.quit_pending.is_some() {
                        self.should_quit = true;
                    } else {
                        self.quit_pending = Some(Instant::now());
                    }
                    return;
                }
                self.quit_pending = None;

                match key {
                    KeyCode::Left => {
                        state.prev_panel();
                    }
                    KeyCode::Right => {
                        let should_load = state.next_panel_or_select();
                        if should_load {
                            self.load_chapter().await;
                        }
                    }
                    KeyCode::Up => {
                        state.move_up();
                    }
                    KeyCode::Down => {
                        state.move_down();
                    }
                    KeyCode::Enter => {
                        let should_load = state.select_current();
                        if should_load {
                            self.load_chapter().await;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    async fn load_chapter(&mut self) {
        if let AppMode::Browser(ref mut state) = self.mode {
            state.loading = true;
            let book = state.selected_book_name();
            let chapter = state.selected_chapter;

            match self.resolver.get_chapter(book, chapter, "KJV").await {
                Ok(ch) => {
                    state.current_chapter = Some(ch);
                    state.scripture_scroll = 0;
                }
                Err(e) => {
                    eprintln!("Error loading chapter: {}", e);
                }
            }
            state.loading = false;
        }
    }
}
