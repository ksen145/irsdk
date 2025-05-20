use std::slice;
use memmap2::Mmap;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct IRSDKStruct {
    shared_mem: Mmap,
    offset: usize,
}

impl IRSDKStruct {
    pub fn new(shared_mem: Mmap, offset: usize) -> Self {
        IRSDKStruct { shared_mem, offset }
    }

    pub fn get_i8(&self, offset: usize) -> i8 {
        self.shared_mem[self.offset + offset] as i8
    }

    pub fn get_bool(&self, offset: usize) -> bool {
        self.shared_mem[self.offset + offset] != 0
    }

    pub fn get_i32(&self, offset: usize) -> i32 {
        let slice = &self.shared_mem[self.offset + offset..self.offset + offset + 4];
        i32::from_le_bytes(slice.try_into().unwrap())
    }

    pub fn get_u32(&self, offset: usize) -> u32 {
        let slice = &self.shared_mem[self.offset + offset..self.offset + offset + 4];
        u32::from_le_bytes(slice.try_into().unwrap())
    }

    pub fn get_f32(&self, offset: usize) -> f32 {
        let slice = &self.shared_mem[self.offset + offset..self.offset + offset + 4];
        f32::from_le_bytes(slice.try_into().unwrap())
    }

    pub fn get_f64(&self, offset: usize) -> f64 {
        let slice = &self.shared_mem[self.offset + offset..self.offset + offset + 8];
        f64::from_le_bytes(slice.try_into().unwrap())
    }

    pub fn get_str(&self, offset: usize, len: usize) -> String {
        let slice = &self.shared_mem[self.offset + offset..self.offset + offset + len];
        String::from_utf8_lossy(&slice[..slice.iter().position(|&x| x == 0).unwrap_or(len)]).to_string()
    }
}

pub struct Header {
    pub version: i32,
    pub status: i32,
    pub tick_rate: i32,
    pub session_info_update: i32,
    pub session_info_len: i32,
    pub session_info_offset: i32,
    pub num_vars: i32,
    pub var_header_offset: i32,
    pub num_buf: i32,
    pub buf_len: i32,
    pub var_buf: Vec<VarBuffer>,
}

impl Header {
    pub fn from_struct(irsdk_struct: &IRSDKStruct) -> Self {
        let version = irsdk_struct.get_i32(0);
        let status = irsdk_struct.get_i32(4);
        let tick_rate = irsdk_struct.get_i32(8);
        let session_info_update = irsdk_struct.get_i32(12);
        let session_info_len = irsdk_struct.get_i32(16);
        let session_info_offset = irsdk_struct.get_i32(20);
        let num_vars = irsdk_struct.get_i32(24);
        let var_header_offset = irsdk_struct.get_i32(28);
        let num_buf = irsdk_struct.get_i32(32);
        let buf_len = irsdk_struct.get_i32(36);

        let mut var_buf = Vec::new();
        for i in 0..num_buf {
            let buf_offset = 48 + (i as usize) * 16;
            let var_buffer = VarBuffer::from_struct(irsdk_struct, buf_offset, buf_len);
            var_buf.push(var_buffer);
        }

        Header {
            version,
            status,
            tick_rate,
            session_info_update,
            session_info_len,
            session_info_offset,
            num_vars,
            var_header_offset,
            num_buf,
            buf_len,
            var_buf,
        }
    }
}

pub struct VarBuffer {
    pub tick_count: i32,
    pub buf_offset: i32,
    pub is_memory_frozen: bool,
    pub frozen_memory: Option<Vec<u8>>,
    pub buf_len: i32,
}

impl VarBuffer {
    pub fn from_struct(irsdk_struct: &IRSDKStruct, offset: usize, buf_len: i32) -> Self {
        let tick_count = irsdk_struct.get_i32(offset);
        let buf_offset = irsdk_struct.get_i32(offset + 4);
        VarBuffer {
            tick_count,
            buf_offset,
            is_memory_frozen: false,
            frozen_memory: None,
            buf_len,
        }
    }

    pub fn freeze(&mut self, shared_mem: &Mmap) {
        if self.buf_offset >= 0 && (self.buf_offset as usize + self.buf_len as usize) <= shared_mem.len() {
            self.frozen_memory = Some(shared_mem[self.buf_offset as usize..self.buf_offset as usize + self.buf_len as usize].to_vec());
            self.is_memory_frozen = true;
        }
    }

    pub fn unfreeze(&mut self) {
        self.frozen_memory = None;
        self.is_memory_frozen = false;
    }

    pub fn get_memory<'a>(&self, shared_mem: &'a Mmap) -> &'a [u8] {
        if self.is_memory_frozen {
            self.frozen_memory.as_ref().unwrap()
        } else {
            &shared_mem[self.buf_offset as usize..self.buf_offset as usize + self.buf_len as usize]
        }
    }

    pub fn get_buf_offset(&self) -> i32 {
        if self.is_memory_frozen {
            0
        } else {
            self.buf_offset
        }
    }
}

pub struct VarHeader {
    pub var_type: i32,
    pub offset: i32,
    pub count: i32,
    pub count_as_time: bool,
    pub name: String,
    pub desc: String,
    pub unit: String,
}

impl VarHeader {
    pub fn from_struct(irsdk_struct: &IRSDKStruct, offset: usize) -> Self {
        VarHeader {
            var_type: irsdk_struct.get_i32(offset),
            offset: irsdk_struct.get_i32(offset + 4),
            count: irsdk_struct.get_i32(offset + 8),
            count_as_time: irsdk_struct.get_bool(offset + 12),
            name: irsdk_struct.get_str(offset + 16, 32),
            desc: irsdk_struct.get_str(offset + 48, 64),
            unit: irsdk_struct.get_str(offset + 112, 32),
        }
    }
}

pub struct DiskSubHeader {
    pub session_start_date: u64,
    pub session_start_time: f64,
    pub session_end_time: f64,
    pub session_lap_count: i32,
    pub session_record_count: i32,
}

impl DiskSubHeader {
    pub fn from_struct(irsdk_struct: &IRSDKStruct, offset: usize) -> Self {
        let slice = &irsdk_struct.shared_mem[offset..offset + 8];
        let session_start_date = u64::from_le_bytes(slice.try_into().unwrap());
        let session_start_time = irsdk_struct.get_f64(offset + 8);
        let session_end_time = irsdk_struct.get_f64(offset + 16);
        let session_lap_count = irsdk_struct.get_i32(offset + 24);
        let session_record_count = irsdk_struct.get_i32(offset + 28);
        DiskSubHeader {
            session_start_date,
            session_start_time,
            session_end_time,
            session_lap_count,
            session_record_count,
        }
    }
}


