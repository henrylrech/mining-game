use std::{thread, time::Duration};

use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, List, ListDirection, Padding, Paragraph, Tabs, Widget},
    DefaultTerminal,
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::{ore::{starting_ores, Ore, OreType}, shop::{starting_upgrades, Upgrade, UpgradeType}};

pub struct App {
    money: u32,
    ores: Vec<Ore>,
    upgrades: Vec<Upgrade>,
    state: AppState,
    selected_tab: SelectedTab
}

impl Default for App {
    fn default() -> Self {
        Self {
            ores: starting_ores(),
            upgrades: starting_upgrades(),
            money: 0,
            state: AppState::Running,
            selected_tab: SelectedTab::Cave
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Cave")]
    Cave,
    #[strum(to_string = "Shop")]
    Shop,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.kind {
                KeyEventKind::Press => {

                    match self.selected_tab {
                        SelectedTab::Cave => {
                            for ore in self.ores.iter_mut() {
                                if key.code == ore.char {
                                    ore.mine(&mut self.money);
                                    return Ok(());
                                }
                            }
                        },
                        SelectedTab::Shop => {
                            match key.code {
                                KeyCode::Char('0') => self.buy_upgrade(0),
                                KeyCode::Char('1') => self.buy_upgrade(1),
                                KeyCode::Char('2') => self.buy_upgrade(2),
                                KeyCode::Char('3') => self.buy_upgrade(3),
                                KeyCode::Char('4') => self.buy_upgrade(4),
                                KeyCode::Char('5') => self.buy_upgrade(5),
                                KeyCode::Char('6') => self.buy_upgrade(6),
                                KeyCode::Char('7') => self.buy_upgrade(7),
                                KeyCode::Char('8') => self.buy_upgrade(8),
                                KeyCode::Char('9') => self.buy_upgrade(9),
                                _ => {}
                            }
                        },
                    }

                    match key.code {
                        KeyCode::Right => self.next_tab(),
                        KeyCode::Left => self.previous_tab(),
                        KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    fn start_auto_miners(&mut self) {
        for ore in self.ores.iter_mut() {
            thread::spawn(move || {
                loop {
                    ore.mine(&mut self.money);
                    thread::sleep(Duration::from_secs(5));
                }
            });
        }
    }

    fn buy_upgrade(&mut self, index: u8) {
        let mut cont: u8 = 0;
        let mut upgrtype_to_buy: Option<UpgradeType> = None;
        let mut ore_to_upgrade: Option<OreType> = None;
        for upgr in self.upgrades.iter_mut() {
            if upgr.visible(self.money) {
                if cont == index {
                    let res = upgr.buy(&mut self.money);
                    if res.is_ok() {
                        upgrtype_to_buy = Some(upgr.upgrade_type.clone());
                        ore_to_upgrade = Some(upgr.ore_type.clone());
                        break;
                    }
                } else {
                    cont += 1;
                }
            }
        }

        if let (Some(upgrtype), Some(ore)) = (upgrtype_to_buy, ore_to_upgrade) {
            self.do_upgrade(&upgrtype, &ore);
        }
    }

    fn do_upgrade(&mut self, upgrade_type: &UpgradeType, ore_type: &OreType) -> Option<()> {
        for ore in self.ores.iter_mut() {
            if ore.ore_type == *ore_type {
                ore.upgrade(&upgrade_type);
                return Some(());
            }
        }

        None
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf, self.money);
        self.render_tabs(tabs_area, buf);

        match self.selected_tab {
            SelectedTab::Cave => self.selected_tab.render_cave(inner_area, buf, &self.ores),
            SelectedTab::Shop => self.selected_tab.render_shop(inner_area, buf, &self.upgrades, &self.money),
        }
        render_footer(footer_area, buf, self.selected_tab);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer, money: u32) {
    format!("Money: {}", money).as_str().bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer, selected_tab: SelectedTab) {
    let footer = match selected_tab {
        SelectedTab::Cave => "◄ ► to change tab | Press q to quit",
        SelectedTab::Shop => "◄ ► to change tab | Press q to quit",
    };

    Line::raw(footer)
        .centered()
        .render(area, buf)
}

fn ore_line(ore: &Ore) -> Line<'_>{

    let line: Line<'_> = Line::from(vec![
        Span::styled(format!("{} | ", ore.char), Style::default().fg(Color::Blue)),
        Span::styled(ore.name.as_str(), Style::default().fg(Color::White)),
        Span::styled(format!(" | Amount: {}", ore.count), Style::default()),
        Span::styled(format!(" | Value: {} ", ore.value), Style::default().fg(Color::Yellow)),
        Span::styled("⛏️".repeat(ore.auto_miners.try_into().unwrap()), Style::default()),
    ]);

    line
}

fn upgr_line(upgrade: &Upgrade, count: u8) -> Line<'_>{

    let line: Line<'_> = Line::from(vec![
        Span::styled(format!("{} - ", count.to_string()), Style::default().fg(Color::Red)),
        Span::styled(upgrade.desc.as_str(), Style::default().fg(Color::White)),
        Span::styled(format!(" | Cost: {}", upgrade.cost), Style::default().fg(Color::Blue)),
    ]);

    line
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render_cave(self, area: Rect, buf: &mut Buffer, ores: &Vec<Ore>) {
        let mut lines: Vec<Line<'_>> = vec![];

            for ore in ores {
                if !ore.locked {
                    lines.push(ore_line(ore))
                }
            }

            let text = Text::from(lines);
        Paragraph::new(text)
            .block(self.block())
            .render(area, buf);
    }

    fn render_shop(self, area: Rect, buf: &mut Buffer, upgrades: &Vec<Upgrade>, money: &u32) {
        let mut lines: Vec<Line<'_>> = vec![];

        let mut zero_available = true;
        let mut cont: u8 = 0;
        for upgr in upgrades {
            if upgr.visible(*money) {
                zero_available = false;
                lines.push(upgr_line(upgr, cont));
                cont += 1;
            }
        }

        if zero_available {
            lines.push(Line::from(vec![
                Span::styled("No upgrades available... Try mining a bit more!", Style::default().fg(Color::Red)),
            ]));
        }

        List::new(lines)
            .block(self.block())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom)
            .render(area, buf);
    }

    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Cave => tailwind::GRAY,
            Self::Shop => tailwind::YELLOW,
        }
    }
}