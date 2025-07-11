pub fn convert_sdl_scancode(
    scancode: sdl2::keyboard::Scancode,
) -> Option<bevy_input::keyboard::KeyCode> {
    use bevy_input::keyboard::KeyCode as BevyKeyCode;
    use sdl2::keyboard::Scancode as SdlScancode;

    #[allow(unreachable_patterns)]
    let bevy_key = match scancode {
        // Letter keys
        SdlScancode::A => BevyKeyCode::KeyA,
        SdlScancode::B => BevyKeyCode::KeyB,
        SdlScancode::C => BevyKeyCode::KeyC,
        SdlScancode::D => BevyKeyCode::KeyD,
        SdlScancode::E => BevyKeyCode::KeyE,
        SdlScancode::F => BevyKeyCode::KeyF,
        SdlScancode::G => BevyKeyCode::KeyG,
        SdlScancode::H => BevyKeyCode::KeyH,
        SdlScancode::I => BevyKeyCode::KeyI,
        SdlScancode::J => BevyKeyCode::KeyJ,
        SdlScancode::K => BevyKeyCode::KeyK,
        SdlScancode::L => BevyKeyCode::KeyL,
        SdlScancode::M => BevyKeyCode::KeyM,
        SdlScancode::N => BevyKeyCode::KeyN,
        SdlScancode::O => BevyKeyCode::KeyO,
        SdlScancode::P => BevyKeyCode::KeyP,
        SdlScancode::Q => BevyKeyCode::KeyQ,
        SdlScancode::R => BevyKeyCode::KeyR,
        SdlScancode::S => BevyKeyCode::KeyS,
        SdlScancode::T => BevyKeyCode::KeyT,
        SdlScancode::U => BevyKeyCode::KeyU,
        SdlScancode::V => BevyKeyCode::KeyV,
        SdlScancode::W => BevyKeyCode::KeyW,
        SdlScancode::X => BevyKeyCode::KeyX,
        SdlScancode::Y => BevyKeyCode::KeyY,
        SdlScancode::Z => BevyKeyCode::KeyZ,

        // Number keys
        SdlScancode::Num1 => BevyKeyCode::Digit1,
        SdlScancode::Num2 => BevyKeyCode::Digit2,
        SdlScancode::Num3 => BevyKeyCode::Digit3,
        SdlScancode::Num4 => BevyKeyCode::Digit4,
        SdlScancode::Num5 => BevyKeyCode::Digit5,
        SdlScancode::Num6 => BevyKeyCode::Digit6,
        SdlScancode::Num7 => BevyKeyCode::Digit7,
        SdlScancode::Num8 => BevyKeyCode::Digit8,
        SdlScancode::Num9 => BevyKeyCode::Digit9,
        SdlScancode::Num0 => BevyKeyCode::Digit0,

        // Basic function keys
        SdlScancode::Return => BevyKeyCode::Enter,
        SdlScancode::Escape => BevyKeyCode::Escape,
        SdlScancode::Backspace => BevyKeyCode::Backspace,
        SdlScancode::Tab => BevyKeyCode::Tab,
        SdlScancode::Space => BevyKeyCode::Space,
        SdlScancode::Minus => BevyKeyCode::Minus,
        SdlScancode::Equals => BevyKeyCode::Equal,
        SdlScancode::LeftBracket => BevyKeyCode::BracketLeft,
        SdlScancode::RightBracket => BevyKeyCode::BracketRight,
        SdlScancode::Backslash => BevyKeyCode::Backslash,
        SdlScancode::NonUsBackslash => BevyKeyCode::IntlBackslash,
        SdlScancode::Semicolon => BevyKeyCode::Semicolon,
        SdlScancode::Apostrophe => BevyKeyCode::Quote,
        SdlScancode::Grave => BevyKeyCode::Backquote,
        SdlScancode::Comma => BevyKeyCode::Comma,
        SdlScancode::Period => BevyKeyCode::Period,
        SdlScancode::Slash => BevyKeyCode::Slash,
        SdlScancode::CapsLock => BevyKeyCode::CapsLock,
        SdlScancode::PrintScreen => BevyKeyCode::PrintScreen,
        SdlScancode::ScrollLock => BevyKeyCode::ScrollLock,
        SdlScancode::Pause => BevyKeyCode::Pause,

        // F-keys
        SdlScancode::F1 => BevyKeyCode::F1,
        SdlScancode::F2 => BevyKeyCode::F2,
        SdlScancode::F3 => BevyKeyCode::F3,
        SdlScancode::F4 => BevyKeyCode::F4,
        SdlScancode::F5 => BevyKeyCode::F5,
        SdlScancode::F6 => BevyKeyCode::F6,
        SdlScancode::F7 => BevyKeyCode::F7,
        SdlScancode::F8 => BevyKeyCode::F8,
        SdlScancode::F9 => BevyKeyCode::F9,
        SdlScancode::F10 => BevyKeyCode::F10,
        SdlScancode::F11 => BevyKeyCode::F11,
        SdlScancode::F12 => BevyKeyCode::F12,
        SdlScancode::F13 => BevyKeyCode::F13,
        SdlScancode::F14 => BevyKeyCode::F14,
        SdlScancode::F15 => BevyKeyCode::F15,
        SdlScancode::F16 => BevyKeyCode::F16,
        SdlScancode::F17 => BevyKeyCode::F17,
        SdlScancode::F18 => BevyKeyCode::F18,
        SdlScancode::F19 => BevyKeyCode::F19,
        SdlScancode::F20 => BevyKeyCode::F20,
        SdlScancode::F21 => BevyKeyCode::F21,
        SdlScancode::F22 => BevyKeyCode::F22,
        SdlScancode::F23 => BevyKeyCode::F23,
        SdlScancode::F24 => BevyKeyCode::F24,

        // Navigation keys
        SdlScancode::Insert => BevyKeyCode::Insert,
        SdlScancode::Home => BevyKeyCode::Home,
        SdlScancode::PageUp => BevyKeyCode::PageUp,
        SdlScancode::PageDown => BevyKeyCode::PageDown,
        SdlScancode::Delete => BevyKeyCode::Delete,
        SdlScancode::End => BevyKeyCode::End,

        // Arrow keys
        SdlScancode::Right => BevyKeyCode::ArrowRight,
        SdlScancode::Left => BevyKeyCode::ArrowLeft,
        SdlScancode::Down => BevyKeyCode::ArrowDown,
        SdlScancode::Up => BevyKeyCode::ArrowUp,

        // Function keys
        SdlScancode::Help => BevyKeyCode::Help,
        SdlScancode::Application => BevyKeyCode::ContextMenu,
        SdlScancode::Undo => BevyKeyCode::Undo,
        SdlScancode::Cut => BevyKeyCode::Cut,
        SdlScancode::Copy => BevyKeyCode::Copy,
        SdlScancode::Paste => BevyKeyCode::Paste,
        SdlScancode::Find => BevyKeyCode::Find,

        // Volume keys
        SdlScancode::Mute => BevyKeyCode::AudioVolumeMute,
        SdlScancode::VolumeUp => BevyKeyCode::AudioVolumeUp,
        SdlScancode::VolumeDown => BevyKeyCode::AudioVolumeDown,

        // Modifier keys
        SdlScancode::LCtrl => BevyKeyCode::ControlLeft,
        SdlScancode::LShift => BevyKeyCode::ShiftLeft,
        SdlScancode::LAlt => BevyKeyCode::AltLeft,
        SdlScancode::LGui => BevyKeyCode::SuperLeft,
        SdlScancode::RCtrl => BevyKeyCode::ControlRight,
        SdlScancode::RShift => BevyKeyCode::ShiftRight,
        SdlScancode::RAlt => BevyKeyCode::AltRight,
        SdlScancode::RGui => BevyKeyCode::SuperRight,

        // Media keys
        SdlScancode::AudioNext => BevyKeyCode::MediaTrackNext,
        SdlScancode::AudioPrev => BevyKeyCode::MediaTrackPrevious,
        SdlScancode::AudioStop => BevyKeyCode::MediaStop,
        SdlScancode::AudioPlay => BevyKeyCode::MediaPlayPause,
        SdlScancode::AudioMute => BevyKeyCode::AudioVolumeMute,
        SdlScancode::MediaSelect => BevyKeyCode::MediaPlayPause,

        // Browser keys
        SdlScancode::AcSearch => BevyKeyCode::BrowserSearch,
        SdlScancode::AcHome => BevyKeyCode::BrowserHome,
        SdlScancode::AcBack => BevyKeyCode::BrowserBack,
        SdlScancode::AcForward => BevyKeyCode::BrowserForward,
        SdlScancode::AcStop => BevyKeyCode::BrowserStop,
        SdlScancode::AcRefresh => BevyKeyCode::BrowserRefresh,
        SdlScancode::AcBookmarks => BevyKeyCode::BrowserFavorites,

        // System keys
        SdlScancode::Eject => BevyKeyCode::Eject,
        SdlScancode::Sleep => BevyKeyCode::Sleep,
        SdlScancode::Mail => BevyKeyCode::LaunchMail,

        // Language keys
        SdlScancode::Lang1 => BevyKeyCode::Lang1,
        SdlScancode::Lang2 => BevyKeyCode::Lang2,
        SdlScancode::Lang3 => BevyKeyCode::Lang3,
        SdlScancode::Lang4 => BevyKeyCode::Lang4,
        SdlScancode::Lang5 => BevyKeyCode::Lang5,

        // Numpad keys
        SdlScancode::KpEnter => BevyKeyCode::NumpadEnter,
        SdlScancode::KpBackspace => BevyKeyCode::NumpadBackspace,
        SdlScancode::KpDivide => BevyKeyCode::NumpadDivide,
        SdlScancode::KpMultiply => BevyKeyCode::NumpadMultiply,
        SdlScancode::KpMinus => BevyKeyCode::NumpadSubtract,
        SdlScancode::KpPlus => BevyKeyCode::NumpadAdd,
        SdlScancode::KpDecimal => BevyKeyCode::NumpadDecimal,
        SdlScancode::KpEquals => BevyKeyCode::NumpadEqual,
        SdlScancode::KpEqualsAS400 => BevyKeyCode::NumpadEqual,
        SdlScancode::KpComma => BevyKeyCode::NumpadComma,
        SdlScancode::Kp1 => BevyKeyCode::Numpad1,
        SdlScancode::Kp2 => BevyKeyCode::Numpad2,
        SdlScancode::Kp3 => BevyKeyCode::Numpad3,
        SdlScancode::Kp4 => BevyKeyCode::Numpad4,
        SdlScancode::Kp5 => BevyKeyCode::Numpad5,
        SdlScancode::Kp6 => BevyKeyCode::Numpad6,
        SdlScancode::Kp7 => BevyKeyCode::Numpad7,
        SdlScancode::Kp8 => BevyKeyCode::Numpad8,
        SdlScancode::Kp9 => BevyKeyCode::Numpad9,
        SdlScancode::Kp0 => BevyKeyCode::Numpad0,
        SdlScancode::KpLeftParen => BevyKeyCode::NumpadParenLeft,
        SdlScancode::KpRightParen => BevyKeyCode::NumpadParenRight,
        SdlScancode::KpClear => BevyKeyCode::NumpadClear,
        SdlScancode::KpClearEntry => BevyKeyCode::NumpadClearEntry,
        SdlScancode::KpMemStore => BevyKeyCode::NumpadMemoryStore,
        SdlScancode::KpMemRecall => BevyKeyCode::NumpadMemoryRecall,
        SdlScancode::KpMemClear => BevyKeyCode::NumpadMemoryClear,
        SdlScancode::KpMemAdd => BevyKeyCode::NumpadMemoryAdd,
        SdlScancode::KpMemSubtract => BevyKeyCode::NumpadMemorySubtract,
        SdlScancode::Num => BevyKeyCode::NumLock,

        // Unimplemented scancodes
        SdlScancode::NonUsHash
        | SdlScancode::Kp00
        | SdlScancode::Kp000
        | SdlScancode::KpLeftBrace
        | SdlScancode::KpRightBrace
        | SdlScancode::KpA
        | SdlScancode::KpB
        | SdlScancode::KpC
        | SdlScancode::KpD
        | SdlScancode::KpE
        | SdlScancode::KpF
        | SdlScancode::KpXor
        | SdlScancode::KpPower
        | SdlScancode::KpPercent
        | SdlScancode::KpLess
        | SdlScancode::KpGreater
        | SdlScancode::KpAmpersand
        | SdlScancode::KpDblAmpersand
        | SdlScancode::KpVerticalBar
        | SdlScancode::KpDblVerticalBar
        | SdlScancode::KpColon
        | SdlScancode::KpAt
        | SdlScancode::KpExclam
        | SdlScancode::KpMemMultiply
        | SdlScancode::KpMemDivide
        | SdlScancode::KpPlusMinus
        | SdlScancode::KpBinary
        | SdlScancode::KpOctal
        | SdlScancode::KpHexadecimal
        | SdlScancode::KpTab
        | SdlScancode::KpHash
        | SdlScancode::KpSpace
        | SdlScancode::KpPeriod
        | SdlScancode::AltErase
        | SdlScancode::ThousandsSeparator
        | SdlScancode::DecimalSeparator
        | SdlScancode::CurrencyUnit
        | SdlScancode::CurrencySubUnit
        | SdlScancode::International1
        | SdlScancode::International2
        | SdlScancode::International3
        | SdlScancode::International4
        | SdlScancode::International5
        | SdlScancode::International6
        | SdlScancode::International7
        | SdlScancode::International8
        | SdlScancode::International9
        | SdlScancode::Lang6
        | SdlScancode::Lang7
        | SdlScancode::Lang8
        | SdlScancode::Lang9
        | SdlScancode::SysReq
        | SdlScancode::Cancel
        | SdlScancode::Clear
        | SdlScancode::Prior
        | SdlScancode::Return2
        | SdlScancode::Separator
        | SdlScancode::Out
        | SdlScancode::Oper
        | SdlScancode::ClearAgain
        | SdlScancode::CrSel
        | SdlScancode::ExSel
        | SdlScancode::Execute
        | SdlScancode::Again
        | SdlScancode::Select
        | SdlScancode::BrightnessDown
        | SdlScancode::BrightnessUp
        | SdlScancode::DisplaySwitch
        | SdlScancode::KbdIllumToggle
        | SdlScancode::KbdIllumDown
        | SdlScancode::KbdIllumUp
        | SdlScancode::App1
        | SdlScancode::App2
        | SdlScancode::Calculator
        | SdlScancode::Computer
        | SdlScancode::Stop
        | SdlScancode::Www
        | SdlScancode::Menu
        | SdlScancode::NumLockClear
        | SdlScancode::Power
        | SdlScancode::Mode => return None,
    };
    Some(bevy_key)
}

pub fn convert_sdl_keycode(keycode: sdl2::keyboard::Keycode) -> bevy_input::keyboard::Key {
    use bevy_input::keyboard::Key as BevyKey;
    use sdl2::keyboard::Keycode as SdlKeycode;

    #[allow(unreachable_patterns)]
    match keycode {
        SdlKeycode::BACKSPACE => BevyKey::Backspace,
        SdlKeycode::SPACE => BevyKey::Space,
        SdlKeycode::ESCAPE => BevyKey::Escape,
        SdlKeycode::TAB => BevyKey::Tab,

        SdlKeycode::Kp0
        | SdlKeycode::Kp1
        | SdlKeycode::Kp2
        | SdlKeycode::Kp3
        | SdlKeycode::Kp4
        | SdlKeycode::Kp5
        | SdlKeycode::Kp6
        | SdlKeycode::Kp7
        | SdlKeycode::Kp8
        | SdlKeycode::Kp9
        | SdlKeycode::Num1
        | SdlKeycode::Num2
        | SdlKeycode::Num3
        | SdlKeycode::Num4
        | SdlKeycode::Num5
        | SdlKeycode::Num6
        | SdlKeycode::Num7
        | SdlKeycode::Num8
        | SdlKeycode::Num9
        | SdlKeycode::Num0
        | SdlKeycode::A
        | SdlKeycode::B
        | SdlKeycode::C
        | SdlKeycode::D
        | SdlKeycode::E
        | SdlKeycode::F
        | SdlKeycode::G
        | SdlKeycode::H
        | SdlKeycode::I
        | SdlKeycode::J
        | SdlKeycode::K
        | SdlKeycode::L
        | SdlKeycode::M
        | SdlKeycode::N
        | SdlKeycode::O
        | SdlKeycode::P
        | SdlKeycode::Q
        | SdlKeycode::R
        | SdlKeycode::S
        | SdlKeycode::T
        | SdlKeycode::U
        | SdlKeycode::V
        | SdlKeycode::W
        | SdlKeycode::X
        | SdlKeycode::Y
        | SdlKeycode::Z => BevyKey::Character(keycode.name().into()),
        SdlKeycode::Up => BevyKey::ArrowUp,
        SdlKeycode::Down => BevyKey::ArrowDown,
        SdlKeycode::Left => BevyKey::ArrowLeft,
        SdlKeycode::Right => BevyKey::ArrowRight,

        _ => {
            unimplemented!("{keycode:?}: {}", keycode.name())
        }
    }
}
