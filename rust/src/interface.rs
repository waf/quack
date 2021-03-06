/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::{Arc, Mutex};
use std::ptr::null;

use implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{
    fn from(t: Option<T>) -> COption<T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true,
            }
        } else {
            COption {
                data: T::default(),
                some: false,
            }
        }
    }
}


pub enum QString {}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe { slice::from_raw_parts(str, to_usize(len)) };
    let characters = decode_utf16(utf16.iter().cloned())
        .into_iter()
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}



#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1,
}

#[repr(C)]
pub struct QModelIndex {
    row: c_int,
    internal_id: usize,
}


fn to_usize(n: c_int) -> usize {
    if n < 0 {
        panic!("Cannot cast {} to usize", n);
    }
    n as usize
}


fn to_c_int(n: usize) -> c_int {
    if n > c_int::max_value() as usize {
        panic!("Cannot cast {} to c_int", n);
    }
    n as c_int
}


pub struct ApplicationQObject {}

#[derive(Clone)]
pub struct ApplicationEmitter {
    qobject: Arc<Mutex<*const ApplicationQObject>>,
}

unsafe impl Send for ApplicationEmitter {}

impl ApplicationEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
}

pub trait ApplicationTrait {
    fn new(emit: ApplicationEmitter,
        channels: Channels) -> Self;
    fn emit(&self) -> &ApplicationEmitter;
    fn channels(&self) -> &Channels;
    fn channels_mut(&mut self) -> &mut Channels;
}

#[no_mangle]
pub extern "C" fn application_new(
    application: *mut ApplicationQObject,
    channels: *mut ChannelsQObject,
    channels_new_data_ready: fn(*const ChannelsQObject),
    channels_data_changed: fn(*const ChannelsQObject, usize, usize),
    channels_begin_reset_model: fn(*const ChannelsQObject),
    channels_end_reset_model: fn(*const ChannelsQObject),
    channels_begin_insert_rows: fn(*const ChannelsQObject, usize, usize),
    channels_end_insert_rows: fn(*const ChannelsQObject),
    channels_begin_remove_rows: fn(*const ChannelsQObject, usize, usize),
    channels_end_remove_rows: fn(*const ChannelsQObject),
) -> *mut Application {
    let channels_emit = ChannelsEmitter {
        qobject: Arc::new(Mutex::new(channels)),
        new_data_ready: channels_new_data_ready,
    };
    let model = ChannelsList {
        qobject: channels,
        data_changed: channels_data_changed,
        begin_reset_model: channels_begin_reset_model,
        end_reset_model: channels_end_reset_model,
        begin_insert_rows: channels_begin_insert_rows,
        end_insert_rows: channels_end_insert_rows,
        begin_remove_rows: channels_begin_remove_rows,
        end_remove_rows: channels_end_remove_rows,
    };
    let d_channels = Channels::new(channels_emit, model);
    let application_emit = ApplicationEmitter {
        qobject: Arc::new(Mutex::new(application)),
    };
    let d_application = Application::new(application_emit,
        d_channels);
    Box::into_raw(Box::new(d_application))
}

#[no_mangle]
pub unsafe extern "C" fn application_free(ptr: *mut Application) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn application_channels_get(ptr: *mut Application) -> *mut Channels {
    (&mut *ptr).channels_mut()
}

pub struct ChannelsQObject {}

#[derive(Clone)]
pub struct ChannelsEmitter {
    qobject: Arc<Mutex<*const ChannelsQObject>>,
    new_data_ready: fn(*const ChannelsQObject),
}

unsafe impl Send for ChannelsEmitter {}

impl ChannelsEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn new_data_ready(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

pub struct ChannelsList {
    qobject: *const ChannelsQObject,
    data_changed: fn(*const ChannelsQObject, usize, usize),
    begin_reset_model: fn(*const ChannelsQObject),
    end_reset_model: fn(*const ChannelsQObject),
    begin_insert_rows: fn(*const ChannelsQObject, usize, usize),
    end_insert_rows: fn(*const ChannelsQObject),
    begin_remove_rows: fn(*const ChannelsQObject, usize, usize),
    end_remove_rows: fn(*const ChannelsQObject),
}

impl ChannelsList {
    pub fn data_changed(&self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&self, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, first, last);
    }
    pub fn end_insert_rows(&self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&self, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, first, last);
    }
    pub fn end_remove_rows(&self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait ChannelsTrait {
    fn new(emit: ChannelsEmitter, model: ChannelsList) -> Self;
    fn emit(&self) -> &ChannelsEmitter;
    fn row_count(&self) -> usize;
    fn insert_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn remove_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn can_fetch_more(&self) -> bool {
        false
    }
    fn fetch_more(&mut self) {}
    fn sort(&mut self, u8, SortOrder) {}
    fn name(&self, index: usize) -> &str;
}

#[no_mangle]
pub extern "C" fn channels_new(
    channels: *mut ChannelsQObject,
    channels_new_data_ready: fn(*const ChannelsQObject),
    channels_data_changed: fn(*const ChannelsQObject, usize, usize),
    channels_begin_reset_model: fn(*const ChannelsQObject),
    channels_end_reset_model: fn(*const ChannelsQObject),
    channels_begin_insert_rows: fn(*const ChannelsQObject, usize, usize),
    channels_end_insert_rows: fn(*const ChannelsQObject),
    channels_begin_remove_rows: fn(*const ChannelsQObject, usize, usize),
    channels_end_remove_rows: fn(*const ChannelsQObject),
) -> *mut Channels {
    let channels_emit = ChannelsEmitter {
        qobject: Arc::new(Mutex::new(channels)),
        new_data_ready: channels_new_data_ready,
    };
    let model = ChannelsList {
        qobject: channels,
        data_changed: channels_data_changed,
        begin_reset_model: channels_begin_reset_model,
        end_reset_model: channels_end_reset_model,
        begin_insert_rows: channels_begin_insert_rows,
        end_insert_rows: channels_end_insert_rows,
        begin_remove_rows: channels_begin_remove_rows,
        end_remove_rows: channels_end_remove_rows,
    };
    let d_channels = Channels::new(channels_emit, model);
    Box::into_raw(Box::new(d_channels))
}

#[no_mangle]
pub unsafe extern "C" fn channels_free(ptr: *mut Channels) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn channels_row_count(ptr: *const Channels) -> c_int {
    to_c_int((&*ptr).row_count())
}
#[no_mangle]
pub unsafe extern "C" fn channels_insert_rows(ptr: *mut Channels, row: c_int, count: c_int) -> bool {
    (&mut *ptr).insert_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn channels_remove_rows(ptr: *mut Channels, row: c_int, count: c_int) -> bool {
    (&mut *ptr).remove_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn channels_can_fetch_more(ptr: *const Channels) -> bool {
    (&*ptr).can_fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn channels_fetch_more(ptr: *mut Channels) {
    (&mut *ptr).fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn channels_sort(
    ptr: *mut Channels,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub extern "C" fn channels_data_name(
    ptr: *const Channels, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = unsafe { &*ptr };
    let data = o.name(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}
