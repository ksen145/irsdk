use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::thread;
use std::str;
use memmap2::MmapOptions;
use regex::Regex;
use reqwest::blocking::Client;
use serde_yaml;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::*;
use crate::constants::*;
use crate::structs::*;
use windows::Win32::System::Threading::{OpenEventW, WaitForSingleObject, SYNCHRONIZE};
use windows::Win32::UI::WindowsAndMessaging::{RegisterWindowMessageW, SendNotifyMessageW, HWND_BROADCAST};
use windows::Win32::System::Memory::{CreateFileMappingW, MapViewOfFile, FILE_MAP_READ, INVALID_HANDLE_VALUE};
use windows::core::PCWSTR;
use std::ptr;
use std::slice;


#[derive(Debug)]
pub enum IRSDKError {
    NotInitialized,
    ConnectionFailed(String),
    MemoryAccessError,
    YamlParseError(String),
    WindowsAPIError(String),
}

pub struct SessionData {
    data: Option<serde_yaml::Value>,
    data_last: Option<serde_yaml::Value>,
    data_binary: Option<Vec<u8>>,
    update: Option<i32>,
    async_session_info_update: Option<i32>,
}

impl SessionData {
    fn new() -> Self {
        SessionData {
            data: None,
            data_last: None,
            data_binary: None,
            update: None,
            async_session_info_update: None,
        }
    }
}

pub struct IRSDK {
    parse_yaml_async: bool,
    is_initialized: bool,
    last_session_info_update: i32,
    shared_mem: Option<Mmap>,
    header: Option<Header>,
    data_valid_event: Option<HANDLE>,
    var_headers: Option<Vec<VarHeader>>,
    var_headers_dict: Option<HashMap<String, VarHeader>>,
    var_headers_names: Option<Vec<String>>,
    var_buffer_latest: Option<VarBuffer>,
    session_info_dict: HashMap<String, SessionData>,
    broadcast_msg_id: Option<u32>,
    test_file: Option<File>,
    workaround_connected_state: i32,
}

impl IRSDK {
    pub fn new(parse_yaml_async: bool) -> Self {
        IRSDK {
            parse_yaml_async,
            is_initialized: false,
            last_session_info_update: 0,
            shared_mem: None,
            header: None,
            data_valid_event: None,
            var_headers: None,
            var_headers_dict: None,
            var_headers_names: None,
            var_buffer_latest: None,
            session_info_dict: HashMap::new(),
            broadcast_msg_id: None,
            test_file: None,
            workaround_connected_state: 0,
        }
    }

    pub fn is_connected(&mut self) -> bool {
        if let Some(header) = &self.header {
            if header.status == STATUS_CONNECTED {
                self.workaround_connected_state = 0;
            }
            if self.workaround_connected_state == 0 && header.status != STATUS_CONNECTED {
                self.workaround_connected_state = 1;
            }
            if self.workaround_connected_state == 1 && (self.get("SessionNum").is_none() || self.test_file.is_some()) {
                self.workaround_connected_state = 2;
            }
            if self.workaround_connected_state == 2 && self.get("SessionNum").is_some() {
                self.workaround_connected_state = 3;
            }
        }
        self.header.is_some() && 
        (self.test_file.is_some() || self.data_valid_event.is_some()) && 
        (self.header.as_ref().map_or(false, |h| h.status == STATUS_CONNECTED) || self.workaround_connected_state == 3)
    }

    pub fn session_info_update(&self) -> i32 {
        self.header.as_ref().map_or(0, |h| h.session_info_update)
    }

    pub fn var_headers_names(&mut self) -> Vec<String> {
        if self.var_headers_names.is_none() {
            self.var_headers_names = Some(
                self.var_headers()
                    .iter()
                    .map(|vh| vh.name.clone())
                    .collect()
            );
        }
        self.var_headers_names.clone().unwrap_or_default()
    }

    pub fn startup(&mut self, test_file: Option<&str>, dump_to: Option<&str>) -> Result<bool, IRSDKError> {
        if test_file.is_none() {
            if !self.check_sim_status() {
                return Err(IRSDKError::ConnectionFailed("IRacing is not running".to_string()));
            }
            let event_name = format!("{}\0", DATA_VALID_EVENT_NAME);
            let event_name_w: Vec<u16> = event_name.encode_utf16().chain(std::iter::once(0)).collect();
            self.data_valid_event = unsafe {
                let handle = OpenEventW(SYNCHRONIZE.0, FALSE, PCWSTR(event_name_w.as_ptr()));
                if handle.0 == 0 {
                    return Err(IRSDKError::WindowsAPIError("Failed to open event".to_string()));
                }
                Some(handle)
            };
        }

        if !self.wait_valid_data_event() {
            self.data_valid_event = None;
            return Err(IRSDKError::ConnectionFailed("Failed to wait for valid data event".to_string()));
        }

        if self.shared_mem.is_none() {
            match test_file {
                Some(path) => {
                    let file = File::open(path).map_err(|e| IRSDKError::ConnectionFailed(e.to_string()))?;
                    self.test_file = Some(file);
                    self.shared_mem = Some(unsafe {
                        MmapOptions::new()
                            .map(&self.test_file.as_ref().unwrap())
                            .map_err(|_| IRSDKError::MemoryAccessError)?
                    });
                }
                None => {
                    let map_name = format!("{}\0", MEM_MAP_FILE);
                    let map_name_w: Vec<u16> = map_name.encode_utf16().chain(std::iter::once(0)).collect();
                    unsafe {
                        let handle = CreateFileMappingW(
                            INVALID_HANDLE_VALUE,
                            ptr::null_mut(),
                            FILE_MAP_READ.0,
                            0,
                            MEM_MAP_FILE_SIZE as u32,
                            PCWSTR(map_name_w.as_ptr()),
                        );
                        if handle.0 == 0 {
                            return Err(IRSDKError::WindowsAPIError("Failed to create file mapping".to_string()));
                        }

                        let view = MapViewOfFile(
                            handle,
                            FILE_MAP_READ.0,
                            0,
                            0,
                            MEM_MAP_FILE_SIZE,
                        );
                        if view.is_null() {
                            return Err(IRSDKError::WindowsAPIError("Failed to map view of file".to_string()));
                        }

                        let slice = slice::from_raw_parts(view as *const u8, MEM_MAP_FILE_SIZE);
                        self.shared_mem = Some(Mmap::from(slice.to_vec())); // Unsafe conversion to Mmap, needs to be redone
                    }
                }
            }
        }

        if let Some(mem) = &self.shared_mem {
            if let Some(dump_path) = dump_to {
                let mut f = File::create(dump_path).map_err(|e| IRSDKError::ConnectionFailed(e.to_string()))?;
                f.write_all(&mem[..]).map_err(|e| IRSDKError::ConnectionFailed(e.to_string()))?;
            }
            let irsdk_struct = IRSDKStruct::new(mem.clone(), 0);
            self.header = Some(Header::from_struct(&irsdk_struct));
            if let Some(header) = &self.header {
                self.is_initialized = header.version >= 1 && !header.var_buf.is_empty();
            }
        }

        Ok(self.is_initialized)
    }



    pub fn shutdown(&mut self) {
        self.is_initialized = false;
        self.last_session_info_update = 0;
        self.shared_mem = None;
        self.header = None;
        self.data_valid_event = None;
        self.var_headers = None;
        self.var_headers_dict = None;
        self.var_headers_names = None;
        self.var_buffer_latest = None;
        self.session_info_dict.clear();
        self.broadcast_msg_id = None;
        if self.test_file.is_some() {
            self.test_file = None;
        }
    }


    pub fn parse_to(&self, to_file: &str) -> Result<(), IRSDKError> {
        if !self.is_initialized {
            return Err(IRSDKError::NotInitialized);
        }
        let mut f = File::create(to_file).map_err(|e| IRSDKError::ConnectionFailed(e.to_string()))?;
        if let Some(header) = &self.header {
            if let Some(shared_mem) = &self.shared_mem {
                let session_data = &shared_mem[header.session_info_offset as usize..(header.session_info_offset + header.session_info_len) as usize];
                f.write_all(session_data).map_err(|e| IRSDKError::ConnectionFailed(e.to_string()))?;
                let var_headers_dict = self.var_headers_dict.as_ref().unwrap_or(&HashMap::new());
                let mut lines = Vec::new();
                for key in var_headers_dict.keys() {
                    if let Some(value) = self.get(key) {
                        lines.push(format!("{:32}{}", key, value));
                    }
                }
                lines.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                f.write_all(lines.join("\n").as_bytes()).map_err(|e| IRSDKError::ConnectionFailed(e.to_string()))?;
            }
        }
        Ok(())
    }

    pub fn cam_switch_pos(&mut self, position: i32, group: i32, camera: i32) -> bool {
        self.broadcast_msg(broadcast_msg::CAM_SWITCH_POS, position, group, camera)
    }

    pub fn cam_switch_num(&mut self, car_number: &str, group: i32, camera: i32) -> bool {
        let car_num = self.pad_car_num(car_number);
        self.broadcast_msg(broadcast_msg::CAM_SWITCH_NUM, car_num, group, camera)
    }

    pub fn cam_set_state(&mut self, camera_state: u32) -> bool {
        self.broadcast_msg(broadcast_msg::CAM_SET_STATE, camera_state as i32, 0, 0)
    }

    pub fn replay_set_play_speed(&mut self, speed: i32, slow_motion: bool) -> bool {
        self.broadcast_msg(broadcast_msg::REPLAY_SET_PLAY_SPEED, speed, if slow_motion { 1 } else { 0 }, 0)
    }

    pub fn replay_set_play_position(&mut self, pos_mode: u32, frame_num: i32) -> bool {
        self.broadcast_msg(broadcast_msg::REPLAY_SET_PLAY_POSITION, pos_mode as i32, frame_num, 0)
    }

    pub fn replay_search(&mut self, search_mode: u32) -> bool {
        self.broadcast_msg(broadcast_msg::REPLAY_SEARCH, search_mode as i32, 0, 0)
    }

    pub fn replay_set_state(&mut self, state_mode: u32) -> bool {
        self.broadcast_msg(broadcast_msg::REPLAY_SET_STATE, state_mode as i32, 0, 0)
    }

    pub fn reload_all_textures(&mut self) -> bool {
        self.broadcast_msg(broadcast_msg::RELOAD_TEXTURES, reload_textures_mode::ALL as i32, 0, 0)
    }

    pub fn reload_texture(&mut self, car_idx: i32) -> bool {
        self.broadcast_msg(broadcast_msg::RELOAD_TEXTURES, reload_textures_mode::CAR_IDX as i32, car_idx, 0)
    }

    pub fn chat_command(&mut self, chat_command_mode: u32) -> bool {
        self.broadcast_msg(broadcast_msg::CHAT_COMMAND, chat_command_mode as i32, 0, 0)
    }

    pub fn chat_command_macro(&mut self, macro_num: i32) -> bool {
        self.broadcast_msg(broadcast_msg::CHAT_COMMAND, chat_command_mode::MACRO as i32, macro_num, 0)
    }

    pub fn pit_command(&mut self, pit_command_mode: u32, var: i32) -> bool {
        self.broadcast_msg(broadcast_msg::PIT_COMMAND, pit_command_mode as i32, var, 0)
    }

    pub fn telem_command(&mut self, telem_command_mode: u32) -> bool {
        self.broadcast_msg(broadcast_msg::TELEM_COMMAND, telem_command_mode as i32, 0, 0)
    }

    pub fn ffb_command(&mut self, ffb_command_mode: u32, value: f32) -> bool {
        let int_val = (value * 65536.0) as i32;
        self.broadcast_msg(broadcast_msg::FFB_COMMAND, ffb_command_mode as i32, int_val, 0)
    }

    pub fn replay_search_session_time(&mut self, session_num: i32, session_time_ms: i32) -> bool {
        self.broadcast_msg(broadcast_msg::REPLAY_SEARCH_SESSION_TIME, session_num, session_time_ms, 0)
    }

    pub fn video_capture(&mut self, video_capture_mode: u32) -> bool {
        self.broadcast_msg(broadcast_msg::VIDEO_CAPTURE, video_capture_mode as i32, 0, 0)
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        if let Some(var_headers_dict) = &self.var_headers_dict {
            if let Some(var_header) = var_headers_dict.get(key) {
                if let Some(var_buf_latest) = &self.var_buffer_latest {
                    if let Some(shared_mem) = &self.shared_mem {
                        let mem = var_buf_latest.get_memory(shared_mem);
                        let offset = var_buf_latest.get_buf_offset() as usize + var_header.offset as usize;
                        let type_size = match var_header.var_type {
                            0 => 1,
                            1 => 1,
                            2 => 4,
                            3 => 4,
                            4 => 4,
                            5 => 8,
                            _ => return None,
                        };
                        let total_size = type_size * var_header.count as usize;
                        if offset + total_size > mem.len() {
                            return None;
                        }
                        let mut values = Vec::new();
                        for i in 0..var_header.count as usize {
                            let start = offset + i * type_size;
                            let value = match var_header.var_type {
                                0 => mem[start].to_string(),
                                1 => (mem[start] != 0).to_string(),
                                2 => i32::from_le_bytes(mem[start..start + 4].try_into().unwrap()).to_string(),
                                3 => u32::from_le_bytes(mem[start..start + 4].try_into().unwrap()).to_string(),
                                4 => f32::from_le_bytes(mem[start..start + 4].try_into().unwrap()).to_string(),
                                5 => f64::from_le_bytes(mem[start..start + 8].try_into().unwrap()).to_string(),
                                _ => return None,
                            };
                            values.push(value);
                        }
                        return if var_header.count == 1 {
                            Some(values[0].clone())
                        } else {
                            Some(format!("[{}]", values.join(", ")))
                        };
                    }
                }
            }
        }
        self.get_session_info(key).map(|v| v.to_string())
    }

    pub fn freeze_var_buffer_latest(&mut self) {
        self.unfreeze_var_buffer_latest();
        self.wait_valid_data_event();
        if let Some(header) = &self.header {
            if let Some(shared_mem) = &self.shared_mem {
                let mut latest = header.var_buf.clone();
                latest.sort_by_key(|v| v.tick_count);
                latest.reverse();
                if let Some(mut var_buf) = latest.get(0).cloned() {
                    var_buf.freeze(shared_mem);
                    self.var_buffer_latest = Some(var_buf);
                }
            }
        }
    }

    pub fn unfreeze_var_buffer_latest(&mut self) {
        if let Some(mut var_buf) = self.var_buffer_latest.take() {
            var_buf.unfreeze();
            self.var_buffer_latest = None;
        }
    }

    pub fn get_session_info_update_by_key(&self, key: &str) -> Option<i32> {
        self.session_info_dict.get(key).and_then(|data| data.update)
    }

    fn check_sim_status(&self) -> bool {
        let client = Client::new();
        match client.get(SIM_STATUS_URL).send() {
            Ok(response) => {
                if let Ok(text) = response.text() {
                    return text.contains("running:1");
                }
                false
            }
            Err(e) => {
                println!("Failed connect to IRacing: {}", e);
                false
            }
        }
    }

    fn wait_valid_data_event(&self) -> bool {
        if let Some(event) = self.data_valid_event {
            unsafe {
                WaitForSingleObject(event, 32) == 0
            }
        } else {
            true
        }
    }


    fn var_headers(&mut self) -> Vec<VarHeader> {
        if self.var_headers.is_none() {
            if let Some(header) = &self.header {
                if let Some(shared_mem) = &self.shared_mem {
                    let mut headers = Vec::new();
                    for i in 0..header.num_vars {
                        let offset = header.var_header_offset as usize + (i as usize * 144);
                        let irsdk_struct = IRSDKStruct::new(shared_mem.clone(), offset);
                        headers.push(VarHeader::from_struct(&irsdk_struct, 0));
                    }
                    self.var_headers = Some(headers);
                }
            }
        }
        self.var_headers.clone().unwrap_or_default()
    }

    fn var_headers_dict(&mut self) -> HashMap<String, VarHeader> {
        if self.var_headers_dict.is_none() {
            let mut dict = HashMap::new();
            for var_header in self.var_headers() {
                dict.insert(var_header.name.clone(), var_header);
            }
            self.var_headers_dict = Some(dict);
        }
        self.var_headers_dict.clone().unwrap_or_default()
    }

    fn var_buffer_latest(&mut self) -> Option<VarBuffer> {
        if let Some(header) = &self.header {
            let mut var_bufs = header.var_buf.clone();
            var_bufs.sort_by_key(|v| v.tick_count);
            var_bufs.reverse();
            var_bufs.get(1).cloned()
        } else {
            None
        }
    }

    fn get_session_info(&mut self, key: &str) -> Option<serde_yaml::Value> {
        if let Some(header) = &self.header {
            if self.last_session_info_update < header.session_info_update {
                self.last_session_info_update = header.session_info_update;
                for session_data in self.session_info_dict.values_mut() {
                    if session_data.data.is_some() {
                        session_data.data_last = session_data.data.clone();
                    }
                    session_data.data = None;
                }
            }
        }

        let entry = self.session_info_dict.entry(key.to_string()).or_insert_with(SessionData::new);
        if entry.data.is_some() {
            return entry.data.clone();
        }

        if self.parse_yaml_async {
            if entry.async_session_info_update.unwrap_or(0) < self.last_session_info_update {
                entry.async_session_info_update = Some(self.last_session_info_update);
                let key_clone = key.to_string();
                let mut entry_clone = entry.clone();
                thread::spawn(move || {
                    Self::parse_yaml(&key_clone, &mut entry_clone);
                });
            }
        } else {
            Self::parse_yaml(key, entry);
        }
        entry.data.clone()
    }

    fn get_session_info_binary(&self, key: &str) -> Option<Vec<u8>> {
        if let Some(header) = &self.header {
            if let Some(shared_mem) = &self.shared_mem {
                let start = header.session_info_offset as usize;
                let end = start + header.session_info_len as usize;
                let search_str = format!("\n{}:\n", key);
                let search_bytes = search_str.as_bytes();
                if let Some(pos) = shared_mem[start..end].windows(search_bytes.len()).position(|window| window == search_bytes) {
                    let match_start = start + pos
                    let search_end = b"\n\n";
                    if let Some(end_pos) = shared_mem[match_start + 1..end].windows(search_end.len()).position(|window| window == search_end) {
                        let match_end = match_start + 1 + end_pos;
                        return Some(shared_mem[match_start + 1..match_end].to_vec());
                    }
                }
            }
        }
        None
    }

    fn parse_yaml(key: &str, session_data: &mut SessionData) {
        let binary_data = self.get_session_info_binary(key);

        if binary_data.is_none() {
            if session_data.data_last.is_some() {
                session_data.data = session_data.data_last.clone();
            }
            return;
        }

        let binary_data = binary_data.unwrap();

        if session_data.data_binary.as_ref() == Some(&binary_data) && session_data.data_last.is_some() {
            session_data.data = session_data.data_last.clone();
            return;
        }
        session_data.data_binary = Some(binary_data.clone());

        let yaml_src = str::from_utf8(&binary_data)
            .unwrap_or("")
            .replace(|c: char| !c.is_ascii_printable(), "")
            .trim_end_matches('\0');

        let yaml_src = if key == "DriverInfo" {
            let re = Regex::new(r"(DriverSetupName|UserName|TeamName|AbbrevName|Initials): (.*)").unwrap();
            re.replace_all(&yaml_src, |caps: &regex::Captures| {
                format!("{} \"{}\"", &caps[1], &caps[2].replace('"', "\\\"").replace('\\', "\\\\"))
            }).to_string()
        } else {
            yaml_src.to_string()
        };

        let yaml_src = Regex::new(r"(\w+: )(,.*)").unwrap()
            .replace_all(&yaml_src, |caps: &regex::Captures| {
                format!("{} \"{}\"", &caps[1], &caps[2])
            }).to_string();

        match serde_yaml::from_str(&yaml_src) {
            Ok(result) => {
                if let Some(map) = result.as_mapping() {
                    if let Some(value) = map.get(key) {
                        session_data.data = Some(value.clone());
                        if session_data.data.is_some() {
                            session_data.update = Some(self.last_session_info_update);
                        } else if session_data.data_last.is_some() {
                            session_data.data = session_data.data_last.clone();
                        }
                    }
                }
            }
            Err(e) => {
                println!("YAML parse error: {}", e);
                if session_data.data_last.is_some() {
                    session_data.data = session_data.data_last.clone();
                }
            }
        }
    }

    fn broadcast_msg_id(&mut self) -> u32 {
        if self.broadcast_msg_id.is_none() {
            let msg_name = format!("{}\0", BROADCAST_MSG_NAME);
            let msg_name_w: Vec<u16> = msg_name.encode_utf16().chain(std::iter::once(0)).collect();
            self.broadcast_msg_id = unsafe {
                Some(RegisterWindowMessageW(PCWSTR(msg_name_w.as_ptr())))
            };
        }
        self.broadcast_msg_id.unwrap()
    }

    fn broadcast_msg(&mut self, broadcast_type: u32, var1: i32, var2: i32, var3: i32) -> bool {
        let msg_id = self.broadcast_msg_id();
        unsafe {
            SendNotifyMessageW(
                HWND_BROADCAST,
                msg_id,
                (broadcast_type as u32 | (var1 as u32) << 16) as u32,
                (var2 as u32 | (var3 as u32) << 16) as u32
            ).as_bool()
        }
    }

    fn pad_car_num(&self, num: &str) -> i32 {
        let num: i32 = num.parse().unwrap_or(0);
        let num_str = num.to_string();
        let num_len = num_str.len();
        let zero_count = num_len - num_str.trim_start_matches('0').len();
        let adjusted_zero = if zero_count > 0 && num_len == zero_count {
            zero_count - 1
        } else {
            zero_count
        };

        if adjusted_zero > 0 {
            let num_place = if num > 99 { 3 } else if num > 9 { 2 } else { 1 };
            num + 1000 * (num_place + adjusted_zero) as i32
        } else {
            num
        }
    }
}


