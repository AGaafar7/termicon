use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct FileMeta {
    pub permissions: String,
    pub user: String,
    pub group: String,
    pub size: String,
    pub modified: String,
}

pub fn format_metadata(meta: &Metadata) -> FileMeta {
    let mode = meta.mode();
    let permissions = format_mode(mode);
    let (user, group) = resolve_user_group(meta.uid(), meta.gid());
    let size = format_size(meta.len(), meta.is_dir());
    let modified = format_time(meta.mtime());

    FileMeta {
        permissions,
        user,
        group,
        size,
        modified,
    }
}

/// Formats POSIX file mode bits (e.g., drwxr-xr-x)
fn format_mode(mode: u32) -> String {
    let file_type = match mode & 0o170000 {
        0o040000 => 'd', // directory
        0o120000 => 'l', // symlink
        0o020000 => 'c', // char device
        0o060000 => 'b', // block device
        0o010000 => 'p', // FIFO/pipe
        0o140000 => 's', // socket
        _ => '-',        // regular file
    };

    let user_r = if mode & 0o400 != 0 { 'r' } else { '-' };
    let user_w = if mode & 0o200 != 0 { 'w' } else { '-' };
    let user_x = if mode & 0o100 != 0 { 'x' } else { '-' };

    let group_r = if mode & 0o040 != 0 { 'r' } else { '-' };
    let group_w = if mode & 0o020 != 0 { 'w' } else { '-' };
    let group_x = if mode & 0o010 != 0 { 'x' } else { '-' };

    let other_r = if mode & 0o004 != 0 { 'r' } else { '-' };
    let other_w = if mode & 0o002 != 0 { 'w' } else { '-' };
    let other_x = if mode & 0o001 != 0 { 'x' } else { '-' };

    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        file_type, user_r, user_w, user_x, group_r, group_w, group_x, other_r, other_w, other_x
    )
}

/// Converts raw bytes to human-readable format (B, KB, MB, GB)
fn format_size(bytes: u64, is_dir: bool) -> String {
    if is_dir {
        return "-".to_string();
    }
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.1} GB", b / GB)
    } else if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.1} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

/// Resolves POSIX UID and GID to names via macOS getpwuid_r / getgrgid_r
fn resolve_user_group(uid: u32, gid: u32) -> (String, String) {
    let user = unsafe {
        let passwd = libc::getpwuid(uid);
        if !passwd.is_null() && !(*passwd).pw_name.is_null() {
            std::ffi::CStr::from_ptr((*passwd).pw_name)
                .to_string_lossy()
                .into_owned()
        } else {
            uid.to_string()
        }
    };

    let group = unsafe {
        let group_ptr = libc::getgrgid(gid);
        if !group_ptr.is_null() && !(*group_ptr).gr_name.is_null() {
            std::ffi::CStr::from_ptr((*group_ptr).gr_name)
                .to_string_lossy()
                .into_owned()
        } else {
            gid.to_string()
        }
    };

    (user, group)
}

/// Formats POSIX mtime into standard terminal timestamp (e.g. "Nov 02 18:20")
fn format_time(mtime: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let six_months = 180 * 86400;

    let mut tm = std::mem::MaybeUninit::<libc::tm>::uninit();
    unsafe {
        libc::localtime_r(&mtime, tm.as_mut_ptr());
        let tm = tm.assume_init();

        let months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun",
            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let mon = months.get(tm.tm_mon as usize).unwrap_or(&"???");
        let day = tm.tm_mday;

        // If file is older than 6 months or in the future, display the year; otherwise display time
        if (now - mtime).abs() > six_months {
            format!("{} {:2}  {:4}", mon, day, tm.tm_year + 1900)
        } else {
            format!("{} {:2} {:02}:{:02}", mon, day, tm.tm_hour, tm.tm_min)
        }
    }
}