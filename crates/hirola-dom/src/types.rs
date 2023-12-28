use hirola_core::effect::EffectAttribute;
use strum::{Display, EnumString};

use crate::effects::attr_on::OnEffect;

#[derive(Debug, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum DomEvent {
    Abort,
    Autocomplete,
    AutocompleteError,
    Blur,
    Cancel,
    CanPlay,
    CanPlayThrough,
    Change,
    Click,
    Close,
    ContextMenu,
    CueChange,
    DblClick,
    Drag,
    DragEnd,
    DragEnter,
    DragExit,
    DragLeave,
    DragOver,
    DragStart,
    Drop,
    DurationChange,
    Emptied,
    Ended,
    Error,
    Focus,
    Input,
    Invalid,
    KeyDown,
    KeyPress,
    KeyUp,
    Load,
    LoadedData,
    LoadedMetadata,
    LoadStart,
    MouseDown,
    MouseEnter,
    MouseLeave,
    MouseMove,
    MouseOut,
    MouseOver,
    MouseUp,
    MouseWheel,
    Pause,
    Play,
    Playing,
    Progress,
    RateChange,
    Reset,
    Resize,
    Scroll,
    Seeked,
    Seeking,
    Select,
    Show,
    Sort,
    Stalled,
    Submit,
    Suspend,
    TimeUpdate,
    Toggle,
    VolumeChange,
    Waiting,
}

impl EffectAttribute for DomEvent {
    type Handler = OnEffect;
    fn read_as_attr(&self) -> String {
        self.to_string().to_lowercase()
    }
}
