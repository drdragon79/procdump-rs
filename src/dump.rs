use windows::Win32::{
    System::{
        ProcessStatus::*,
        Threading::*,
        Diagnostics::Debug::*,
    },
    Storage::FileSystem::*
};
use windows::Win32::Foundation::{
    HANDLE,
    HMODULE,
    GENERIC_WRITE,
    GENERIC_READ,
};
use windows::core::*;
use chrono::prelude::*;
use crate::arguments::Arguments;
use std::process;

const M: &str = "[-] ";
const P: &str = "[+] ";

pub fn dump(args: Arguments) {
    let pid = args.pid;

    // Get handle to process with `pid`
    let hprocess = match openprocess(pid) {
        Ok(handle) => {
            println!("{P}Got handle to the process!");
            handle
        }
        Err(error) => {
            let invalidpid = 2147942487u32 as i32;
            match error.code().0 {
                x if x == invalidpid => println!("{M}No process with PID: {pid}"),
                _ => {
                    println!("{M}Error: {error}");
                    println!("{M}Make sure you have administrator and SeDebugPrivigilege!");
                }
            }
            process::exit(0)
        }
    };

    // Get processname!
    match getprocessname(&hprocess) {
        Ok(name) => println!("{P}Process name: {name}"),
        Err(error) => println!("{M}Unable to get process name: {error}")
    }

    // set filename
    let filename = args.output
        .unwrap_or(format!("{}_{}.dmp", pid, Utc::now().timestamp()));
    minidump(pid, &hprocess, &filename)
        .unwrap_or_else(|e| {
            println!("{M}Error while dumping memory: {e}");
            process::exit(0);
        });
    println!("{P}Dump written to: {filename}");
}


fn getprocessname(hprocess: &HANDLE) -> Result<String> {
    let hmodule = HMODULE::default();
    let mut basename = [0u16; 1024];
    let lpbasename = &mut basename;

    unsafe {
        let bytes = GetModuleBaseNameW(
            *hprocess,
            hmodule,
            lpbasename
        );
        if bytes == 0 {
            Err(Error::from_win32())
        } else {
            Ok(bytes)
        }
    }?;
    Ok(
        String::from_utf16(lpbasename)
        .unwrap()
    )
}

fn minidump(processid: u32, hprocess: &HANDLE, outfile: &str) -> Result<()> {
    println!("{P}Dumping memory of process ID: {processid}");
    let mut filename = outfile
        .encode_utf16()
        .collect::<Vec<u16>>();
    filename.push(0);
    let lpfilename = PCWSTR::from_raw(filename.as_ptr());
    let dwdesiredaccess = GENERIC_WRITE.0 | GENERIC_READ.0;
    let dwsharemode = FILE_SHARE_NONE;
    let dwcreationdisposition = CREATE_ALWAYS;
    let dwflagsandattributes = FILE_ATTRIBUTE_NORMAL;
    let htemplatefile = HANDLE::default();

    let hfile = unsafe {
        CreateFileW(
            lpfilename,
            dwdesiredaccess,
            dwsharemode,
            None,
            dwcreationdisposition,
            dwflagsandattributes,
            htemplatefile
        )
    }?;

    println!("{P}File {outfile} created!");

    // Execute MiniDumpWriteDump
    let dumptype = MiniDumpWithFullMemory;
    unsafe {
        MiniDumpWriteDump(
            *hprocess,
            processid,
            hfile,
            dumptype,
            None,
            None,
            None,
        )
    }
}

fn openprocess(dwprocessid: u32) -> Result<HANDLE> {
    let dwdesiredaccess = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ;
    let binherithandle = false;

    unsafe {
        OpenProcess(
            dwdesiredaccess,
            binherithandle,
            dwprocessid
        )
    }
}