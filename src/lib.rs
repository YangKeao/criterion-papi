extern crate libpapi_sys;

use criterion::measurement::{Measurement, ValueFormatter};
use libpapi_sys::*;

mod formatter;

use formatter::InsFormatter;

pub struct TotIns {
    event_set: std::os::raw::c_int,
}

impl TotIns {
    pub fn new() -> TotIns {
        let event_set = PAPI_NULL;
        let mut papi_tot_ins: std::os::raw::c_int = 0;

        unsafe {
            let retval = PAPI_library_init(PAPI_VER_CURRENT);
            if retval != PAPI_VER_CURRENT {
                panic!("PAPI_library_init failed {}", retval)
            }

            let retval = PAPI_event_name_to_code(std::ffi::CStr::from_bytes_with_nul_unchecked(b"PAPI_TOT_INS\0").as_ptr(), &mut papi_tot_ins);
            if retval != PAPI_OK as i32 {
                panic!("PAPI_event_name_to_code failed {}", retval)
            }

            let retval = PAPI_add_event(event_set.clone(), papi_tot_ins as i32);
            if retval != PAPI_OK as i32 {
                panic!("PAPI_add_event failed {}", retval)
            }
        }

        return TotIns {
            event_set,
        }
    }
}

impl Measurement for TotIns {
    type Intermediate = i64;
    type Value = i64;

    fn start(&self) -> Self::Intermediate {
        let mut values = [0i64];

        unsafe {
            let retval = PAPI_start(self.event_set.clone());
            if retval != PAPI_OK as i32 {
                panic!("PAPI_start failed {}", retval)
            }

            let retval = PAPI_read(self.event_set.clone(), values.as_mut_ptr());
            if retval != PAPI_OK as i32 {
                panic!("PAPI_read failed")
            }
        }

        values[0]
    }

    fn end(&self, i: Self::Intermediate) -> Self::Value {
        let mut values = [0i64];

        unsafe {
            let retval = PAPI_stop(self.event_set.clone(), values.as_mut_ptr());
            if retval != PAPI_OK as i32 {
                panic!("PAPI_read failed")
            }
        }

        values[0] - i
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        v1 + v2
    }

    fn zero(&self) -> Self::Value {
        0
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        *value as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &InsFormatter
    }
}
