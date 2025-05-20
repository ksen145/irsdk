pub const VERSION: &str = "1.3.5";
pub const SIM_STATUS_URL: &str = "http://127.0.0.1:32034/get_sim_status?object=simStatus";
pub const DATA_VALID_EVENT_NAME: &str = "Local\\IRSDKDataValidEvent";
pub const MEM_MAP_FILE: &str = "Local\\IRSDKMemMapFileName";
pub const MEM_MAP_FILE_SIZE: usize = 1164 * 1024;
pub const BROADCAST_MSG_NAME: &str = "IRSDK_BROADCASTMSG";

pub const VAR_TYPE_MAP: [&str; 6] = ["i8", "bool", "i32", "u32", "f32", "f64"];
pub const YAML_CODE_PAGE: &str = "windows-1252";

pub const STATUS_CONNECTED: i32 = 1;

pub mod engine_warnings {
    pub const WATER_TEMP_WARNING: u32 = 0x01;
    pub const FUEL_PRESSURE_WARNING: u32 = 0x02;
    pub const OIL_PRESSURE_WARNING: u32 = 0x04;
    pub const ENGINE_STALLED: u32 = 0x08;
    pub const PIT_SPEED_LIMITER: u32 = 0x10;
    pub const REV_LIMITER_ACTIVE: u32 = 0x20;
    pub const OIL_TEMP_WARNING: u32 = 0x40;
}

pub mod flags {
    pub const CHECKERED: u32 = 0x0001;
    pub const WHITE: u32 = 0x0002;
    pub const GREEN: u32 = 0x0004;
    pub const YELLOW: u32 = 0x0008;
    pub const RED: u32 = 0x0010;
    pub const BLUE: u32 = 0x0020;
    pub const DEBRIS: u32 = 0x0040;
    pub const CROSSED: u32 = 0x0080;
    pub const YELLOW_WAVING: u32 = 0x0100;
    pub const ONE_LAP_TO_GREEN: u32 = 0x0200;
    pub const GREEN_HELD: u32 = 0x0400;
    pub const TEN_TO_GO: u32 = 0x0800;
    pub const FIVE_TO_GO: u32 = 0x1000;
    pub const RANDOM_WAVING: u32 = 0x2000;
    pub const CAUTION: u32 = 0x4000;
    pub const CAUTION_WAVING: u32 = 0x8000;

    pub const BLACK: u32 = 0x010000;
    pub const DISQUALIFY: u32 = 0x020000;
    pub const SERVICIBLE: u32 = 0x040000;
    pub const FURLED: u32 = 0x080000;
    pub const REPAIR: u32 = 0x100000;

    pub const START_HIDDEN: u32 = 0x10000000;
    pub const START_READY: u32 = 0x20000000;
    pub const START_SET: u32 = 0x40000000;
    pub const START_GO: u32 = 0x80000000;
}

pub mod trk_loc {
    pub const NOT_IN_WORLD: i32 = -1;
    pub const OFF_TRACK: i32 = 0;
    pub const IN_PIT_STALL: i32 = 1;
    pub const APPROACHING_PITS: i32 = 2;
    pub const ON_TRACK: i32 = 3;
}

pub mod trk_surf {
    pub const NOT_IN_WORLD: i32 = -1;
    pub const UNDEFINED: i32 = 0;
    pub const ASPHALT_1: i32 = 1;
    pub const ASPHALT_2: i32 = 2;
    pub const ASPHALT_3: i32 = 3;
    pub const ASPHALT_4: i32 = 4;
    pub const CONCRETE_1: i32 = 5;
    pub const CONCRETE_2: i32 = 6;
    pub const RACING_DIRT_1: i32 = 7;
    pub const RACING_DIRT_2: i32 = 8;
    pub const PAINT_1: i32 = 9;
    pub const PAINT_2: i32 = 10;
    pub const RUMBLE_1: i32 = 11;
    pub const RUMBLE_2: i32 = 12;
    pub const RUMBLE_3: i32 = 13;
    pub const RUMBLE_4: i32 = 14;
    pub const GRASS_1: i32 = 15;
    pub const GRASS_2: i32 = 16;
    pub const GRASS_3: i32 = 17;
    pub const GRASS_4: i32 = 18;
    pub const DIRT_1: i32 = 19;
    pub const DIRT_2: i32 = 20;
    pub const DIRT_3: i32 = 21;
    pub const DIRT_4: i32 = 22;
    pub const SAND: i32 = 23;
    pub const GRAVEL_1: i32 = 24;
    pub const GRAVEL_2: i32 = 25;
    pub const GRASSCRETE: i32 = 26;
    pub const ASTROTURF: i32 = 27;
}

pub mod session_state {
    pub const INVALID: i32 = 0;
    pub const GET_IN_CAR: i32 = 1;
    pub const WARMUP: i32 = 2;
    pub const PARADE_LAPS: i32 = 3;
    pub const RACING: i32 = 4;
    pub const CHECKERED: i32 = 5;
    pub const COOL_DOWN: i32 = 6;
}

pub mod camera_state {
    pub const IS_SESSION_SCREEN: u32 = 0x0001;
    pub const IS_SCENIC_ACTIVE: u32 = 0x0002;
    pub const CAM_TOOL_ACTIVE: u32 = 0x0004;
    pub const UI_HIDDEN: u32 = 0x0008;
    pub const USE_AUTO_SHOT_SELECTION: u32 = 0x0010;
    pub const USE_TEMPORARY_EDITS: u32 = 0x0020;
    pub const USE_KEY_ACCELERATION: u32 = 0x0040;
    pub const USE_KEY10X_ACCELERATION: u32 = 0x0080;
    pub const USE_MOUSE_AIM_MODE: u32 = 0x0100;
}

pub mod broadcast_msg {
    pub const CAM_SWITCH_POS: u32 = 0;
    pub const CAM_SWITCH_NUM: u32 = 1;
    pub const CAM_SET_STATE: u32 = 2;
    pub const REPLAY_SET_PLAY_SPEED: u32 = 3;
    pub const REPLAY_SET_PLAY_POSITION: u32 = 4;
    pub const REPLAY_SEARCH: u32 = 5;
    pub const REPLAY_SET_STATE: u32 = 6;
    pub const RELOAD_TEXTURES: u32 = 7;
    pub const CHAT_COMMAND: u32 = 8;
    pub const PIT_COMMAND: u32 = 9;
    pub const TELEM_COMMAND: u32 = 10;
    pub const FFB_COMMAND: u32 = 11;
    pub const REPLAY_SEARCH_SESSION_TIME: u32 = 12;
    pub const VIDEO_CAPTURE: u32 = 13;
}

pub mod chat_command_mode {
    pub const MACRO: u32 = 0;
    pub const BEGIN_CHAT: u32 = 1;
    pub const REPLY: u32 = 2;
    pub const CANCEL: u32 = 3;
}

pub mod pit_command_mode {
    pub const CLEAR: u32 = 0;
    pub const WS: u32 = 1;
    pub const FUEL: u32 = 2;
    pub const LF: u32 = 3;
    pub const RF: u32 = 4;
    pub const LR: u32 = 5;
    pub const RR: u32 = 6;
    pub const CLEAR_TIRES: u32 = 7;
    pub const FR: u32 = 8;
    pub const CLEAR_WS: u32 = 9;
    pub const CLEAR_FR: u32 = 10;
    pub const CLEAR_FUEL: u32 = 11;
}

pub mod telem_command_mode {
    pub const STOP: u32 = 0;
    pub const START: u32 = 1;
    pub const RESTART: u32 = 2;
}

pub mod rpy_state_mode {
    pub const ERASE_TAPE: u32 = 0;
}

pub mod reload_textures_mode {
    pub const ALL: u32 = 0;
    pub const CAR_IDX: u32 = 1;
}

pub mod rpy_srch_mode {
    pub const TO_START: u32 = 0;
    pub const TO_END: u32 = 1;
    pub const PREV_SESSION: u32 = 2;
    pub const NEXT_SESSION: u32 = 3;
    pub const PREV_LAP: u32 = 4;
    pub const NEXT_LAP: u32 = 5;
    pub const PREV_FRAME: u32 = 6;
    pub const NEXT_FRAME: u32 = 7;
    pub const PREV_INCIDENT: u32 = 8;
    pub const NEXT_INCIDENT: u32 = 9;
}

pub mod rpy_pos_mode {
    pub const BEGIN: u32 = 0;
    pub const CURRENT: u32 = 1;
    pub const END: u32 = 2;
}

pub mod cs_mode {
    pub const AT_INCIDENT: i32 = -3;
    pub const AT_LEADER: i32 = -2;
    pub const AT_EXCITING: i32 = -1;
}

pub mod pit_sv_flags {
    pub const LF_TIRE_CHANGE: u32 = 0x01;
    pub const RF_TIRE_CHANGE: u32 = 0x02;
    pub const LR_TIRE_CHANGE: u32 = 0x04;
    pub const RR_TIRE_CHANGE: u32 = 0x08;
    pub const FUEL_FILL: u32 = 0x10;
    pub const WINDSHIELD_TEAROFF: u32 = 0x20;
    pub const FAST_REPAIR: u32 = 0x40;
}

pub mod pit_sv_status {
    pub const NONE: u32 = 0;
    pub const IN_PROGRESS: u32 = 1;
    pub const COMPLETE: u32 = 2;
    pub const TOO_FAR_LEFT: u32 = 100;
    pub const TOO_FAR_RIGHT: u32 = 101;
    pub const TOO_FAR_FORWARD: u32 = 102;
    pub const TOO_FAR_BACK: u32 = 103;
    pub const BAD_ANGLE: u32 = 104;
    pub const CANT_FIX_THAT: u32 = 105;
}

pub mod pace_mode {
    pub const SINGLE_FILE_START: u32 = 0;
    pub const DOUBLE_FILE_START: u32 = 1;
    pub const SINGLE_FILE_RESTART: u32 = 2;
    pub const DOUBLE_FILE_RESTART: u32 = 3;
    pub const NOT_PACING: u32 = 4;
}

pub mod pace_flags {
    pub const END_OF_LINE: u32 = 0x0001;
    pub const FREE_PASS: u32 = 0x0002;
    pub const WAVED_AROUND: u32 = 0x0004;
}

pub mod car_left_right {
    pub const OFF: u32 = 0;
    pub const CLEAR: u32 = 1;
    pub const CAR_LEFT: u32 = 2;
    pub const CAR_RIGHT: u32 = 3;
    pub const CAR_LEFT_RIGHT: u32 = 4;
    pub const TWO_CARS_LEFT: u32 = 5;
    pub const TWO_CARS_RIGHT: u32 = 6;
}

pub mod ffb_command_mode {
    pub const FFB_COMMAND_MAX_FORCE: u32 = 0;
}

pub mod video_capture_mode {
    pub const TRIGGER_SCREEN_SHOT: u32 = 0;
    pub const START_VIDEO_CAPTURE: u32 = 1;
    pub const END_VIDEO_CAPTURE: u32 = 2;
    pub const TOGGLE_VIDEO_CAPTURE: u32 = 3;
    pub const SHOW_VIDEO_TIMER: u32 = 4;
    pub const HIDE_VIDEO_TIMER: u32 = 5;
}

pub mod track_wetness {
    pub const UNKNOWN: u32 = 0;
    pub const DRY: u32 = 1;
    pub const MOSTLY_DRY: u32 = 2;
    pub const VERY_LIGHTLY_WET: u32 = 3;
    pub const LIGHTLY_WET: u32 = 4;
    pub const MODERATELY_WET: u32 = 5;
    pub const VERY_WET: u32 = 6;
    pub const EXTREMELY_WET: u32 = 7;
}
