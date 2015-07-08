use core::prelude::*;
static DEBUG: bool = true;

pub type EFI_HANDLE = *const ();
pub struct EFI_GUID(u32, u16, u16, [u8; 8]);

struct EFI_TABLE_HEADER {
    Signature       : u64,
    Revision        : u32,
    HeaderSize      : u32,
    CRC32           : u32,
    Reserved        : u32
}

pub struct EFI_SYSTEM_TABLE {
    Hdr                 : EFI_TABLE_HEADER,
    FirmwareVendor      : *mut u16,
    FirmwareRevision    : u32,
    ConsoleInHandle     : EFI_HANDLE,
    ConIn               : *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle    : EFI_HANDLE,
    ConOut              : *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    ConsoleErrorHandle  : EFI_HANDLE,
    StdErr              : *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    RuntimeServices     : *const EFI_RUNTIME_SERVICES,
    BootServices        : *const EFI_BOOT_SERVICES,
    NumberOfTableEntries: usize,
    ConfigurationTable  : *const EFI_CONFIGURATION_TABLE

}

pub static mut SYSTEM_TABLE : *const EFI_SYSTEM_TABLE = 0 as *const EFI_SYSTEM_TABLE;

// TODO: Populate
struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL;

struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset       : EFI_TEXT_RESET,
    OutputString: EFI_TEXT_STRING
    // TODO: Add the rest of the entries
}

type EFI_TEXT_RESET = *const ();

type EFI_TEXT_STRING = extern "win64" fn (*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
                                          *const u16);

struct EFI_RUNTIME_SERVICES;
struct EFI_BOOT_SERVICES;
struct EFI_CONFIGURATION_TABLE {
    VendorGuid  : EFI_GUID,
    VendorTable : *const ()
}

pub struct SystemTable(*const EFI_SYSTEM_TABLE);

impl SystemTable {
    #[no_stack_check]
    pub fn console(&self) -> Console {
        unsafe {
            let &SystemTable(tbl) = self;
            Console {
                input   : (*tbl).ConIn,
                output  : (*tbl).ConOut
            }
        }
    }
}

pub fn unpack<T>(slice: &[T]) -> (*const T, usize) {
    unsafe {
        transmute(slice)
    }
}

pub trait SimpleTextInput {
}

pub trait SimpleTextOutput {
    unsafe fn write_raw(&self, str: *const u16);

    #[no_stack_check]
    fn write(&self, str: &str) {
        let mut buf = [0u16; 4096];

        let mut i = 0;
        while i < buf.len() && i < str.len() {
            // TODO: Make sure the characters are all ascii
            buf[i] = (&str.as_bytes())[i] as u16;
            i += 1;
        }
        i = buf.len() - 1;
        buf[i] = 0;

        unsafe {
            let (p, _) = unpack(&buf);
            self.write_raw(p);
        }
    }

    #[no_stack_check]
    fn write_many(&self, strs: &[&str]) {
        for str in strs {
            self.write(str);
        }
    }

    fn debug(&self, module: &str, msg: &str) {
        if DEBUG {
            self.write_many(&[
                "DEBUG::", module, "::", msg, "\r\n"
            ]);
        }
    }
}

pub struct Console {
    input   : *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    output  : *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
}

impl SimpleTextOutput for Console {
    #[no_stack_check]
    unsafe fn write_raw(&self, str: *const u16) {
        ((*(*self).output).OutputString)(self.output, str);
    }
}

impl SimpleTextInput for Console {
}

extern "rust-intrinsic" {
    fn transmute<T,U>(val: T) -> U;
}

#[no_mangle]
#[no_stack_check]
pub extern "win64" fn efi_start(_ImageHandle: EFI_HANDLE,
                                sys_table   : *const EFI_SYSTEM_TABLE) -> isize {
    unsafe { SYSTEM_TABLE = sys_table; }
    let console = SystemTable(sys_table).console();
    
    // TODO: Figure out linking for stdlib
    // console.debug("INIT", &format!("efi_start, handle 0x{:X}", _ImageHandle as usize));

    console.debug("INIT", "Starting efi_main");
    ::efi_main(SystemTable(sys_table));
    console.debug("INIT", "efi_main returned, exiting");
    0
}

#[no_mangle]
#[no_stack_check]
pub fn __morestack() {
    // Horrible things will happen if this is ever called...
}

#[no_mangle]
#[no_stack_check]
pub extern fn memset(s: *const u8,
                     c: isize,
                     n: usize) -> *const u8 {
    unsafe {
        let s : &mut [u8] = transmute((s, n));
        let mut i = 0;
        while i < n {
            s[i] = c as u8;
            i += 1;

            asm!("");
        }
    }
    
    s
}
