use std::collections::HashMap;
use std::fs::File;
use memmap2::MmapOptions;
use crate::constants::*;
use crate::structs::*;

#[derive(Debug)]
pub enum IBTError {
    NotInitialized,
    FileAccessError(String),
    MemoryAccessError,
}

pub struct IBT {
    ibt_file: Option<File>,
    shared_mem: Option<Mmap>,
    header: Option<Header>,
    disk_header: Option<DiskSubHeader>,
    var_headers: Option<Vec<VarHeader>>,
    var_headers_dict: Option<HashMap<String, VarHeader>>,
    var_headers_names: Option<Vec<String>>,
    session_info_dict: Option<HashMap<String, serde_yaml::Value>>,
}

impl IBT {
    pub fn new() -> Self {
        IBT {
            ibt_file: None,
            shared_mem: None,
            header: None,
            disk_header: None,
            var_headers: None,
            var_headers_dict: None,
            var_headers_names: None,
            session_info_dict: None,
        }
    }

    pub fn open(&mut self, ibt_file: &str) -> Result<(), IBTError> {
        self.ibt_file = Some(File::open(ibt_file).map_err(|e| IBTError::FileAccessError(e.to_string()))?);
        self.shared_mem = Some(unsafe {
            MmapOptions::new()
                .map(&self.ibt_file.as_ref().unwrap())
                .map_err(|e| IBTError::FileAccessError(e.to_string()))?
        });
        if let Some(shared_mem) = &self.shared_mem {
            let irsdk_struct = IRSDKStruct::new(shared_mem.clone(), 0);
            self.header = Some(Header::from_struct(&irsdk_struct));
            self.disk_header = Some(DiskSubHeader::from_struct(&irsdk_struct, 112));
        }
        Ok(())
    }

    pub fn close(&mut self) {
        self.shared_mem = None;
        self.ibt_file = None;
        self.header = None;
        self.disk_header = None;
        self.var_headers = None;
        self.var_headers_dict = None;
        self.var_headers_names = None;
        self.session_info_dict = None;
    }

    pub fn get(&self, index: i32, key: &str) -> Option<serde_yaml::Value> {
        if self.header.is_none() || self.disk_header.is_none() {
            return None;
        }
        let disk_header = self.disk_header.as_ref().unwrap();
        if index < 0 || index >= disk_header.session_record_count {
            return None;
        }
        if let Some(var_headers_dict) = &self.var_headers_dict {
            if let Some(var_header) = var_headers_dict.get(key) {
                if let Some(shared_mem) = &self.shared_mem {
                    if let Some(header) = &self.header {
                        let var_offset = var_header.offset + header.var_buf[0].buf_offset + index * header.buf_len;
                        // Чтение данных в зависимости от типа
                        // ...
                    }
                }
            }
        }
        None
    }
}
