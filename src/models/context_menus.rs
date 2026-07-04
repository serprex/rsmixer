use crate::{
    actor_system::Ctx,
    entry::{Entry, EntryIdentifier, EntryKind, EntryType},
    models::{PulseAudioAction, UserAction},
    scrollable,
    ui::{widgets::ToolWindowWidget, Rect, Scrollable},
};

#[derive(PartialEq, Clone)]
pub enum ContextMenuOption {
    ChangeCardProfile(String, String),
    Kill,
    Move,
    Suspend,
    Resume,
    SetAsDefault,
    InputExactVolume,
}

impl ContextMenuOption {
    pub fn as_str(&self) -> &str {
        match self {
            ContextMenuOption::ChangeCardProfile(_, s) => s,
            ContextMenuOption::Kill => "Kill",
            ContextMenuOption::Move => "Move",
            ContextMenuOption::Suspend => "Suspend",
            ContextMenuOption::Resume => "Resume",
            ContextMenuOption::SetAsDefault => "Set as default",
            ContextMenuOption::InputExactVolume => "Input exact volume value",
        }
    }
}

pub enum ContextMenuEffect {
    None,
    MoveEntry,
}

scrollable!(
    ContextMenu,
    fn selected(&self) -> usize {
        self.selected
    },
    fn len(&self) -> usize {
        self.options.len()
    },
    fn set_selected(&mut self, selected: usize) -> bool {
        if selected < self.options.len() {
            self.selected = selected;
            true
        } else {
            false
        }
    },
    fn element_height(&self, _index: usize) -> u16 {
        1
    }
);

pub struct ContextMenu {
    pub options: Vec<ContextMenuOption>,
    selected: usize,
    pub horizontal_scroll: usize,
    pub area: Rect,
    pub tool_window: ToolWindowWidget,
}

impl ContextMenu {
    pub fn new(entry: &Entry) -> Self {
        let play = match &entry.entry_kind {
            EntryKind::PlayEntry(play) => Some(play),
            EntryKind::CardEntry(_) => None,
        };
        let card = match &entry.entry_kind {
            EntryKind::PlayEntry(_) => None,
            EntryKind::CardEntry(card) => Some(card),
        };
        let options: Vec<ContextMenuOption> = match entry.entry_type {
            EntryType::Source | EntryType::Sink => vec![
                if play.unwrap().suspended {
                    ContextMenuOption::Resume
                } else {
                    ContextMenuOption::Suspend
                },
                ContextMenuOption::SetAsDefault,
                ContextMenuOption::InputExactVolume,
            ],
            EntryType::SinkInput => vec![
                ContextMenuOption::Move,
                ContextMenuOption::Kill,
                ContextMenuOption::InputExactVolume,
            ],
            EntryType::SourceOutput => vec![ContextMenuOption::InputExactVolume],
            EntryType::Card => card
                .unwrap()
                .profiles
                .iter()
                .map(|p| {
                    ContextMenuOption::ChangeCardProfile(p.name.clone(), p.description.clone())
                })
                .collect(),
        };

        Self {
            options,
            selected: 0,
            horizontal_scroll: 0,
            area: Rect::default(),
            tool_window: ToolWindowWidget::default(),
        }
    }

    pub fn resolve(&self, ident: EntryIdentifier, ctx: &Ctx) -> ContextMenuEffect {
        match &self.options[self.selected] {
            ContextMenuOption::Move => {
                return ContextMenuEffect::MoveEntry;
            }
            ContextMenuOption::InputExactVolume => {
                ctx.send_to("event_loop", UserAction::InputVolumeValue);
            }
            ContextMenuOption::ChangeCardProfile(name, _) => {
                ctx.send_to(
                    "pulseaudio",
                    PulseAudioAction::ChangeCardProfile(ident, name.clone()),
                );
            }
            ContextMenuOption::Suspend => {
                ctx.send_to("pulseaudio", PulseAudioAction::SetSuspend(ident, true));
            }
            ContextMenuOption::Resume => {
                ctx.send_to("pulseaudio", PulseAudioAction::SetSuspend(ident, false));
            }
            ContextMenuOption::Kill => {
                ctx.send_to("pulseaudio", PulseAudioAction::KillEntry(ident));
            }
            _ => {}
        };

        ContextMenuEffect::None
    }

    pub fn max_horizontal_scroll(&self) -> usize {
        let (start, end) = self.visible_start_end(self.area.height);
        let longest = self
            .options
            .iter()
            .skip(start)
            .take(end - start)
            .map(|o| o.as_str().len())
            .max();

        match longest {
            None => 0,
            Some(l) => l / self.area.width as usize,
        }
    }
}
impl Default for ContextMenu {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            selected: 0,
            horizontal_scroll: 0,
            area: Rect::default(),
            tool_window: ToolWindowWidget::default(),
        }
    }
}
