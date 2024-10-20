#![allow(unreachable_patterns)]

use libc::c_char;
use std::ffi::{CStr, CString};
use std::mem::transmute;

use crate::sys;
use sys::keycode::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum Keycode {
    ScancodeMask = SDLK_SCANCODE_MASK as i32,
    Unknown = SDLK_UNKNOWN as i32,
    Return = SDLK_RETURN as i32,
    Escape = SDLK_ESCAPE as i32,
    Backspace = SDLK_BACKSPACE as i32,
    Tab = SDLK_TAB as i32,
    Space = SDLK_SPACE as i32,
    Exclaim = SDLK_EXCLAIM as i32,
    DblApostrophe = SDLK_DBLAPOSTROPHE as i32,
    Hash = SDLK_HASH as i32,
    Dollar = SDLK_DOLLAR as i32,
    Percent = SDLK_PERCENT as i32,
    Ampersand = SDLK_AMPERSAND as i32,
    Apostrophe = SDLK_APOSTROPHE as i32,
    LeftParen = SDLK_LEFTPAREN as i32,
    RightParen = SDLK_RIGHTPAREN as i32,
    Asterisk = SDLK_ASTERISK as i32,
    Plus = SDLK_PLUS as i32,
    Comma = SDLK_COMMA as i32,
    Minus = SDLK_MINUS as i32,
    Period = SDLK_PERIOD as i32,
    Slash = SDLK_SLASH as i32,
    _0 = SDLK_0 as i32,
    _1 = SDLK_1 as i32,
    _2 = SDLK_2 as i32,
    _3 = SDLK_3 as i32,
    _4 = SDLK_4 as i32,
    _5 = SDLK_5 as i32,
    _6 = SDLK_6 as i32,
    _7 = SDLK_7 as i32,
    _8 = SDLK_8 as i32,
    _9 = SDLK_9 as i32,
    Colon = SDLK_COLON as i32,
    Semicolon = SDLK_SEMICOLON as i32,
    Less = SDLK_LESS as i32,
    Equals = SDLK_EQUALS as i32,
    Greater = SDLK_GREATER as i32,
    Question = SDLK_QUESTION as i32,
    At = SDLK_AT as i32,
    LeftBracket = SDLK_LEFTBRACKET as i32,
    Backslash = SDLK_BACKSLASH as i32,
    RightBracket = SDLK_RIGHTBRACKET as i32,
    Caret = SDLK_CARET as i32,
    Underscore = SDLK_UNDERSCORE as i32,
    Grave = SDLK_GRAVE as i32,
    A = SDLK_A as i32,
    B = SDLK_B as i32,
    C = SDLK_C as i32,
    D = SDLK_D as i32,
    E = SDLK_E as i32,
    F = SDLK_F as i32,
    G = SDLK_G as i32,
    H = SDLK_H as i32,
    I = SDLK_I as i32,
    J = SDLK_J as i32,
    K = SDLK_K as i32,
    L = SDLK_L as i32,
    M = SDLK_M as i32,
    N = SDLK_N as i32,
    O = SDLK_O as i32,
    P = SDLK_P as i32,
    Q = SDLK_Q as i32,
    R = SDLK_R as i32,
    S = SDLK_S as i32,
    T = SDLK_T as i32,
    U = SDLK_U as i32,
    V = SDLK_V as i32,
    W = SDLK_W as i32,
    X = SDLK_X as i32,
    Y = SDLK_Y as i32,
    Z = SDLK_Z as i32,
    LeftBrace = SDLK_LEFTBRACE as i32,
    Pipe = SDLK_PIPE as i32,
    RightBrace = SDLK_RIGHTBRACE as i32,
    Tilde = SDLK_TILDE as i32,
    Delete = SDLK_DELETE as i32,
    PlusMinus = SDLK_PLUSMINUS as i32,
    CapsLock = SDLK_CAPSLOCK as i32,
    F1 = SDLK_F1 as i32,
    F2 = SDLK_F2 as i32,
    F3 = SDLK_F3 as i32,
    F4 = SDLK_F4 as i32,
    F5 = SDLK_F5 as i32,
    F6 = SDLK_F6 as i32,
    F7 = SDLK_F7 as i32,
    F8 = SDLK_F8 as i32,
    F9 = SDLK_F9 as i32,
    F10 = SDLK_F10 as i32,
    F11 = SDLK_F11 as i32,
    F12 = SDLK_F12 as i32,
    PrintScreen = SDLK_PRINTSCREEN as i32,
    ScrollLock = SDLK_SCROLLLOCK as i32,
    Pause = SDLK_PAUSE as i32,
    Insert = SDLK_INSERT as i32,
    Home = SDLK_HOME as i32,
    PageUp = SDLK_PAGEUP as i32,
    End = SDLK_END as i32,
    PageDown = SDLK_PAGEDOWN as i32,
    Right = SDLK_RIGHT as i32,
    Left = SDLK_LEFT as i32,
    Down = SDLK_DOWN as i32,
    Up = SDLK_UP as i32,
    NumLockClear = SDLK_NUMLOCKCLEAR as i32,
    KpDivide = SDLK_KP_DIVIDE as i32,
    KpMultiply = SDLK_KP_MULTIPLY as i32,
    KpMinus = SDLK_KP_MINUS as i32,
    KpPlus = SDLK_KP_PLUS as i32,
    KpEnter = SDLK_KP_ENTER as i32,
    Kp1 = SDLK_KP_1 as i32,
    Kp2 = SDLK_KP_2 as i32,
    Kp3 = SDLK_KP_3 as i32,
    Kp4 = SDLK_KP_4 as i32,
    Kp5 = SDLK_KP_5 as i32,
    Kp6 = SDLK_KP_6 as i32,
    Kp7 = SDLK_KP_7 as i32,
    Kp8 = SDLK_KP_8 as i32,
    Kp9 = SDLK_KP_9 as i32,
    Kp0 = SDLK_KP_0 as i32,
    KpPeriod = SDLK_KP_PERIOD as i32,
    Application = SDLK_APPLICATION as i32,
    Power = SDLK_POWER as i32,
    KpEquals = SDLK_KP_EQUALS as i32,
    F13 = SDLK_F13 as i32,
    F14 = SDLK_F14 as i32,
    F15 = SDLK_F15 as i32,
    F16 = SDLK_F16 as i32,
    F17 = SDLK_F17 as i32,
    F18 = SDLK_F18 as i32,
    F19 = SDLK_F19 as i32,
    F20 = SDLK_F20 as i32,
    F21 = SDLK_F21 as i32,
    F22 = SDLK_F22 as i32,
    F23 = SDLK_F23 as i32,
    F24 = SDLK_F24 as i32,
    Execute = SDLK_EXECUTE as i32,
    Help = SDLK_HELP as i32,
    Menu = SDLK_MENU as i32,
    Select = SDLK_SELECT as i32,
    Stop = SDLK_STOP as i32,
    Again = SDLK_AGAIN as i32,
    Undo = SDLK_UNDO as i32,
    Cut = SDLK_CUT as i32,
    Copy = SDLK_COPY as i32,
    Paste = SDLK_PASTE as i32,
    Find = SDLK_FIND as i32,
    Mute = SDLK_MUTE as i32,
    VolumeUp = SDLK_VOLUMEUP as i32,
    VolumeDown = SDLK_VOLUMEDOWN as i32,
    KpComma = SDLK_KP_COMMA as i32,
    KpEqualsAs400 = SDLK_KP_EQUALSAS400 as i32,
    AltErase = SDLK_ALTERASE as i32,
    SysReq = SDLK_SYSREQ as i32,
    Cancel = SDLK_CANCEL as i32,
    Clear = SDLK_CLEAR as i32,
    Prior = SDLK_PRIOR as i32,
    Return2 = SDLK_RETURN2 as i32,
    Separator = SDLK_SEPARATOR as i32,
    Out = SDLK_OUT as i32,
    Oper = SDLK_OPER as i32,
    ClearAgain = SDLK_CLEARAGAIN as i32,
    CrSel = SDLK_CRSEL as i32,
    ExSel = SDLK_EXSEL as i32,
    Kp00 = SDLK_KP_00 as i32,
    Kp000 = SDLK_KP_000 as i32,
    ThousandsSeparator = SDLK_THOUSANDSSEPARATOR as i32,
    DecimalSeparator = SDLK_DECIMALSEPARATOR as i32,
    CurrencyUnit = SDLK_CURRENCYUNIT as i32,
    CurrencySubunit = SDLK_CURRENCYSUBUNIT as i32,
    KpLeftParen = SDLK_KP_LEFTPAREN as i32,
    KpRightParen = SDLK_KP_RIGHTPAREN as i32,
    KpLeftBrace = SDLK_KP_LEFTBRACE as i32,
    KpRightBrace = SDLK_KP_RIGHTBRACE as i32,
    KpTab = SDLK_KP_TAB as i32,
    KpBackspace = SDLK_KP_BACKSPACE as i32,
    KpA = SDLK_KP_A as i32,
    KpB = SDLK_KP_B as i32,
    KpC = SDLK_KP_C as i32,
    KpD = SDLK_KP_D as i32,
    KpE = SDLK_KP_E as i32,
    KpF = SDLK_KP_F as i32,
    KpXor = SDLK_KP_XOR as i32,
    KpPower = SDLK_KP_POWER as i32,
    KpPercent = SDLK_KP_PERCENT as i32,
    KpLess = SDLK_KP_LESS as i32,
    KpGreater = SDLK_KP_GREATER as i32,
    KpAmpersand = SDLK_KP_AMPERSAND as i32,
    KpDblAmpersand = SDLK_KP_DBLAMPERSAND as i32,
    KpVerticalBar = SDLK_KP_VERTICALBAR as i32,
    KpDblVerticalBar = SDLK_KP_DBLVERTICALBAR as i32,
    KpColon = SDLK_KP_COLON as i32,
    KpHash = SDLK_KP_HASH as i32,
    KpSpace = SDLK_KP_SPACE as i32,
    KpAt = SDLK_KP_AT as i32,
    KpExclam = SDLK_KP_EXCLAM as i32,
    KpMemStore = SDLK_KP_MEMSTORE as i32,
    KpMemRecall = SDLK_KP_MEMRECALL as i32,
    KpMemClear = SDLK_KP_MEMCLEAR as i32,
    KpMemAdd = SDLK_KP_MEMADD as i32,
    KpMemSubtract = SDLK_KP_MEMSUBTRACT as i32,
    KpMemMultiply = SDLK_KP_MEMMULTIPLY as i32,
    KpMemDivide = SDLK_KP_MEMDIVIDE as i32,
    KpPlusMinus = SDLK_KP_PLUSMINUS as i32,
    KpClear = SDLK_KP_CLEAR as i32,
    KpClearEntry = SDLK_KP_CLEARENTRY as i32,
    KpBinary = SDLK_KP_BINARY as i32,
    KpOctal = SDLK_KP_OCTAL as i32,
    KpDecimal = SDLK_KP_DECIMAL as i32,
    KpHexadecimal = SDLK_KP_HEXADECIMAL as i32,
    LCtrl = SDLK_LCTRL as i32,
    LShift = SDLK_LSHIFT as i32,
    LAlt = SDLK_LALT as i32,
    LGui = SDLK_LGUI as i32,
    RCtrl = SDLK_RCTRL as i32,
    RShift = SDLK_RSHIFT as i32,
    RAlt = SDLK_RALT as i32,
    RGui = SDLK_RGUI as i32,
    Mode = SDLK_MODE as i32,
    Sleep = SDLK_SLEEP as i32,
    Wake = SDLK_WAKE as i32,
    ChannelIncrement = SDLK_CHANNEL_INCREMENT as i32,
    ChannelDecrement = SDLK_CHANNEL_DECREMENT as i32,
    MediaPlay = SDLK_MEDIA_PLAY as i32,
    MediaPause = SDLK_MEDIA_PAUSE as i32,
    MediaRecord = SDLK_MEDIA_RECORD as i32,
    MediaFastForward = SDLK_MEDIA_FAST_FORWARD as i32,
    MediaRewind = SDLK_MEDIA_REWIND as i32,
    MediaNextTrack = SDLK_MEDIA_NEXT_TRACK as i32,
    MediaPreviousTrack = SDLK_MEDIA_PREVIOUS_TRACK as i32,
    MediaStop = SDLK_MEDIA_STOP as i32,
    MediaEject = SDLK_MEDIA_EJECT as i32,
    MediaPlayPause = SDLK_MEDIA_PLAY_PAUSE as i32,
    MediaSelect = SDLK_MEDIA_SELECT as i32,
    AcNew = SDLK_AC_NEW as i32,
    AcOpen = SDLK_AC_OPEN as i32,
    AcClose = SDLK_AC_CLOSE as i32,
    AcExit = SDLK_AC_EXIT as i32,
    AcSave = SDLK_AC_SAVE as i32,
    AcPrint = SDLK_AC_PRINT as i32,
    AcProperties = SDLK_AC_PROPERTIES as i32,
    AcSearch = SDLK_AC_SEARCH as i32,
    AcHome = SDLK_AC_HOME as i32,
    AcBack = SDLK_AC_BACK as i32,
    AcForward = SDLK_AC_FORWARD as i32,
    AcStop = SDLK_AC_STOP as i32,
    AcRefresh = SDLK_AC_REFRESH as i32,
    AcBookmarks = SDLK_AC_BOOKMARKS as i32,
    SoftLeft = SDLK_SOFTLEFT as i32,
    SoftRight = SDLK_SOFTRIGHT as i32,
    Call = SDLK_CALL as i32,
    EndCall = SDLK_ENDCALL as i32,
}

impl Keycode {
    pub fn from_i32(n: i32) -> Option<Keycode> {
        use self::Keycode::*;
        let n = n as u32;

        Some(match unsafe { transmute(n) } {
            SDLK_SCANCODE_MASK => ScancodeMask,
            SDLK_UNKNOWN => Unknown,
            SDLK_RETURN => Return,
            SDLK_ESCAPE => Escape,
            SDLK_BACKSPACE => Backspace,
            SDLK_TAB => Tab,
            SDLK_SPACE => Space,
            SDLK_EXCLAIM => Exclaim,
            SDLK_DBLAPOSTROPHE => DblApostrophe,
            SDLK_HASH => Hash,
            SDLK_DOLLAR => Dollar,
            SDLK_PERCENT => Percent,
            SDLK_AMPERSAND => Ampersand,
            SDLK_APOSTROPHE => Apostrophe,
            SDLK_LEFTPAREN => LeftParen,
            SDLK_RIGHTPAREN => RightParen,
            SDLK_ASTERISK => Asterisk,
            SDLK_PLUS => Plus,
            SDLK_COMMA => Comma,
            SDLK_MINUS => Minus,
            SDLK_PERIOD => Period,
            SDLK_SLASH => Slash,
            SDLK_0 => _0,
            SDLK_1 => _1,
            SDLK_2 => _2,
            SDLK_3 => _3,
            SDLK_4 => _4,
            SDLK_5 => _5,
            SDLK_6 => _6,
            SDLK_7 => _7,
            SDLK_8 => _8,
            SDLK_9 => _9,
            SDLK_COLON => Colon,
            SDLK_SEMICOLON => Semicolon,
            SDLK_LESS => Less,
            SDLK_EQUALS => Equals,
            SDLK_GREATER => Greater,
            SDLK_QUESTION => Question,
            SDLK_AT => At,
            SDLK_LEFTBRACKET => LeftBracket,
            SDLK_BACKSLASH => Backslash,
            SDLK_RIGHTBRACKET => RightBracket,
            SDLK_CARET => Caret,
            SDLK_UNDERSCORE => Underscore,
            SDLK_GRAVE => Grave,
            SDLK_A => A,
            SDLK_B => B,
            SDLK_C => C,
            SDLK_D => D,
            SDLK_E => E,
            SDLK_F => F,
            SDLK_G => G,
            SDLK_H => H,
            SDLK_I => I,
            SDLK_J => J,
            SDLK_K => K,
            SDLK_L => L,
            SDLK_M => M,
            SDLK_N => N,
            SDLK_O => O,
            SDLK_P => P,
            SDLK_Q => Q,
            SDLK_R => R,
            SDLK_S => S,
            SDLK_T => T,
            SDLK_U => U,
            SDLK_V => V,
            SDLK_W => W,
            SDLK_X => X,
            SDLK_Y => Y,
            SDLK_Z => Z,
            SDLK_LEFTBRACE => LeftBrace,
            SDLK_PIPE => Pipe,
            SDLK_RIGHTBRACE => RightBrace,
            SDLK_TILDE => Tilde,
            SDLK_DELETE => Delete,
            SDLK_PLUSMINUS => PlusMinus,
            SDLK_CAPSLOCK => CapsLock,
            SDLK_F1 => F1,
            SDLK_F2 => F2,
            SDLK_F3 => F3,
            SDLK_F4 => F4,
            SDLK_F5 => F5,
            SDLK_F6 => F6,
            SDLK_F7 => F7,
            SDLK_F8 => F8,
            SDLK_F9 => F9,
            SDLK_F10 => F10,
            SDLK_F11 => F11,
            SDLK_F12 => F12,
            SDLK_PRINTSCREEN => PrintScreen,
            SDLK_SCROLLLOCK => ScrollLock,
            SDLK_PAUSE => Pause,
            SDLK_INSERT => Insert,
            SDLK_HOME => Home,
            SDLK_PAGEUP => PageUp,
            SDLK_END => End,
            SDLK_PAGEDOWN => PageDown,
            SDLK_RIGHT => Right,
            SDLK_LEFT => Left,
            SDLK_DOWN => Down,
            SDLK_UP => Up,
            SDLK_NUMLOCKCLEAR => NumLockClear,
            SDLK_KP_DIVIDE => KpDivide,
            SDLK_KP_MULTIPLY => KpMultiply,
            SDLK_KP_MINUS => KpMinus,
            SDLK_KP_PLUS => KpPlus,
            SDLK_KP_ENTER => KpEnter,
            SDLK_KP_1 => Kp1,
            SDLK_KP_2 => Kp2,
            SDLK_KP_3 => Kp3,
            SDLK_KP_4 => Kp4,
            SDLK_KP_5 => Kp5,
            SDLK_KP_6 => Kp6,
            SDLK_KP_7 => Kp7,
            SDLK_KP_8 => Kp8,
            SDLK_KP_9 => Kp9,
            SDLK_KP_0 => Kp0,
            SDLK_KP_PERIOD => KpPeriod,
            SDLK_APPLICATION => Application,
            SDLK_POWER => Power,
            SDLK_KP_EQUALS => KpEquals,
            SDLK_F13 => F13,
            SDLK_F14 => F14,
            SDLK_F15 => F15,
            SDLK_F16 => F16,
            SDLK_F17 => F17,
            SDLK_F18 => F18,
            SDLK_F19 => F19,
            SDLK_F20 => F20,
            SDLK_F21 => F21,
            SDLK_F22 => F22,
            SDLK_F23 => F23,
            SDLK_F24 => F24,
            SDLK_EXECUTE => Execute,
            SDLK_HELP => Help,
            SDLK_MENU => Menu,
            SDLK_SELECT => Select,
            SDLK_STOP => Stop,
            SDLK_AGAIN => Again,
            SDLK_UNDO => Undo,
            SDLK_CUT => Cut,
            SDLK_COPY => Copy,
            SDLK_PASTE => Paste,
            SDLK_FIND => Find,
            SDLK_MUTE => Mute,
            SDLK_VOLUMEUP => VolumeUp,
            SDLK_VOLUMEDOWN => VolumeDown,
            SDLK_KP_COMMA => KpComma,
            SDLK_KP_EQUALSAS400 => KpEqualsAs400,
            SDLK_ALTERASE => AltErase,
            SDLK_SYSREQ => SysReq,
            SDLK_CANCEL => Cancel,
            SDLK_CLEAR => Clear,
            SDLK_PRIOR => Prior,
            SDLK_RETURN2 => Return2,
            SDLK_SEPARATOR => Separator,
            SDLK_OUT => Out,
            SDLK_OPER => Oper,
            SDLK_CLEARAGAIN => ClearAgain,
            SDLK_CRSEL => CrSel,
            SDLK_EXSEL => ExSel,
            SDLK_KP_00 => Kp00,
            SDLK_KP_000 => Kp000,
            SDLK_THOUSANDSSEPARATOR => ThousandsSeparator,
            SDLK_DECIMALSEPARATOR => DecimalSeparator,
            SDLK_CURRENCYUNIT => CurrencyUnit,
            SDLK_CURRENCYSUBUNIT => CurrencySubunit,
            SDLK_KP_LEFTPAREN => KpLeftParen,
            SDLK_KP_RIGHTPAREN => KpRightParen,
            SDLK_KP_LEFTBRACE => KpLeftBrace,
            SDLK_KP_RIGHTBRACE => KpRightBrace,
            SDLK_KP_TAB => KpTab,
            SDLK_KP_BACKSPACE => KpBackspace,
            SDLK_KP_A => KpA,
            SDLK_KP_B => KpB,
            SDLK_KP_C => KpC,
            SDLK_KP_D => KpD,
            SDLK_KP_E => KpE,
            SDLK_KP_F => KpF,
            SDLK_KP_XOR => KpXor,
            SDLK_KP_POWER => KpPower,
            SDLK_KP_PERCENT => KpPercent,
            SDLK_KP_LESS => KpLess,
            SDLK_KP_GREATER => KpGreater,
            SDLK_KP_AMPERSAND => KpAmpersand,
            SDLK_KP_DBLAMPERSAND => KpDblAmpersand,
            SDLK_KP_VERTICALBAR => KpVerticalBar,
            SDLK_KP_DBLVERTICALBAR => KpDblVerticalBar,
            SDLK_KP_COLON => KpColon,
            SDLK_KP_HASH => KpHash,
            SDLK_KP_SPACE => KpSpace,
            SDLK_KP_AT => KpAt,
            SDLK_KP_EXCLAM => KpExclam,
            SDLK_KP_MEMSTORE => KpMemStore,
            SDLK_KP_MEMRECALL => KpMemRecall,
            SDLK_KP_MEMCLEAR => KpMemClear,
            SDLK_KP_MEMADD => KpMemAdd,
            SDLK_KP_MEMSUBTRACT => KpMemSubtract,
            SDLK_KP_MEMMULTIPLY => KpMemMultiply,
            SDLK_KP_MEMDIVIDE => KpMemDivide,
            SDLK_KP_PLUSMINUS => KpPlusMinus,
            SDLK_KP_CLEAR => KpClear,
            SDLK_KP_CLEARENTRY => KpClearEntry,
            SDLK_KP_BINARY => KpBinary,
            SDLK_KP_OCTAL => KpOctal,
            SDLK_KP_DECIMAL => KpDecimal,
            SDLK_KP_HEXADECIMAL => KpHexadecimal,
            SDLK_LCTRL => LCtrl,
            SDLK_LSHIFT => LShift,
            SDLK_LALT => LAlt,
            SDLK_LGUI => LGui,
            SDLK_RCTRL => RCtrl,
            SDLK_RSHIFT => RShift,
            SDLK_RALT => RAlt,
            SDLK_RGUI => RGui,
            SDLK_MODE => Mode,
            SDLK_SLEEP => Sleep,
            SDLK_WAKE => Wake,
            SDLK_CHANNEL_INCREMENT => ChannelIncrement,
            SDLK_CHANNEL_DECREMENT => ChannelDecrement,
            SDLK_MEDIA_PLAY => MediaPlay,
            SDLK_MEDIA_PAUSE => MediaPause,
            SDLK_MEDIA_RECORD => MediaRecord,
            SDLK_MEDIA_FAST_FORWARD => MediaFastForward,
            SDLK_MEDIA_REWIND => MediaRewind,
            SDLK_MEDIA_NEXT_TRACK => MediaNextTrack,
            SDLK_MEDIA_PREVIOUS_TRACK => MediaPreviousTrack,
            SDLK_MEDIA_STOP => MediaStop,
            SDLK_MEDIA_EJECT => MediaEject,
            SDLK_MEDIA_PLAY_PAUSE => MediaPlayPause,
            SDLK_MEDIA_SELECT => MediaSelect,
            SDLK_AC_NEW => AcNew,
            SDLK_AC_OPEN => AcOpen,
            SDLK_AC_CLOSE => AcClose,
            SDLK_AC_EXIT => AcExit,
            SDLK_AC_SAVE => AcSave,
            SDLK_AC_PRINT => AcPrint,
            SDLK_AC_PROPERTIES => AcProperties,
            SDLK_AC_SEARCH => AcSearch,
            SDLK_AC_HOME => AcHome,
            SDLK_AC_BACK => AcBack,
            SDLK_AC_FORWARD => AcForward,
            SDLK_AC_STOP => AcStop,
            SDLK_AC_REFRESH => AcRefresh,
            SDLK_AC_BOOKMARKS => AcBookmarks,
            SDLK_SOFTLEFT => SoftLeft,
            SDLK_SOFTRIGHT => SoftRight,
            SDLK_CALL => Call,
            SDLK_ENDCALL => EndCall,
            _ => return None,
        })
    }

    pub fn to_ll(self) -> SDL_Keycode {
        unsafe { transmute(self as i32) }
    }
}

use std::fmt;

impl fmt::Display for Keycode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}

impl From<Keycode> for SDL_Keycode {
    fn from(k: Keycode) -> SDL_Keycode {
        k.to_ll()
    }
}

use crate::keyboard::Scancode;

impl Keycode {
    /// Gets the virtual key from a scancode. Returns None if there is no corresponding virtual key.
    #[doc(alias = "SDL_GetKeyFromScancode")]
    pub fn from_scancode(
        scancode: Scancode,
        modstate: SDL_Keymod,
        key_event: bool,
    ) -> Option<Keycode> {
        const UNKNOWN: u32 = sys::keycode::SDLK_UNKNOWN as u32;
        unsafe {
            match sys::keyboard::SDL_GetKeyFromScancode(
                transmute::<u32, sys::scancode::SDL_Scancode>(scancode as u32),
                modstate,
                key_event,
            ) {
                UNKNOWN => None,
                keycode_id => Keycode::from_i32(keycode_id as i32),
            }
        }
    }

    #[doc(alias = "SDL_GetKeyFromName")]
    pub fn from_name(name: &str) -> Option<Keycode> {
        const UNKNOWN: u32 = sys::keycode::SDLK_UNKNOWN as u32;
        unsafe {
            match CString::new(name) {
                Ok(name) => match sys::keyboard::SDL_GetKeyFromName(name.as_ptr() as *const c_char)
                {
                    UNKNOWN => None,
                    keycode_id => Some(Keycode::from_i32(keycode_id as i32).unwrap()),
                },
                // string contains a nul byte - it won't match anything.
                Err(_) => None,
            }
        }
    }

    #[doc(alias = "SDL_GetKeyName")]
    pub fn name(self) -> String {
        // The name string pointer's contents _might_ change, depending on the last call to SDL_GetKeyName.
        // Knowing this, we must always return a new string.
        unsafe {
            let buf = sys::keyboard::SDL_GetKeyName(self.to_ll());
            CStr::from_ptr(buf as *const _).to_str().unwrap().to_owned()
        }
    }
}
