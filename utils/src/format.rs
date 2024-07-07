use separator::{separated_float, separated_int, separated_uint_with_output, Separatable};

/// Display KB or KiB if `short` is false, otherwise if `short` is true
/// and the value is greater than 1MB or 1MiB, display units using [`as_data_size()`].
pub fn as_kb(bytes: f64, si: bool, short: bool) -> String {
    let unit = if si { 1000_f64 } else { 1024_f64 };
    if short && bytes > unit.powi(2) {
        as_data_size(bytes, si)
    } else {
        let suffix = if si { " KB" } else { " KiB" };
        let kb = bytes / unit; //(( * 100.) as u64) as f64 / 100.;
        format_with_precision(kb) + suffix
    }
}

/// Display MB or MiB if `short` is false, otherwise if `short` is true
/// and the value is greater than 1GB or 1GiB, display units using [`as_data_size()`].
pub fn as_mb(bytes: f64, si: bool, short: bool) -> String {
    let unit = if si { 1000_f64 } else { 1024_f64 };
    if short && bytes > unit.powi(3) {
        as_data_size(bytes, si)
    } else {
        let suffix = if si { " MB" } else { " MiB" };
        let mb = bytes / unit.powi(2); //(( * 100.) as u64) as f64 / 100.;
        format_with_precision(mb) + suffix
    }
}

/// Display GB or GiB if `short` is false, otherwise if `short` is true
/// and the value is greater than 1TB or 1TiB, display units using [`as_data_size()`].
pub fn as_gb(bytes: f64, si: bool, short: bool) -> String {
    let unit = if si { 1000_f64 } else { 1024_f64 };
    if short && bytes > unit.powi(4) {
        as_data_size(bytes, si)
    } else {
        let suffix = if si { " GB" } else { " GiB" };
        let gb = bytes / unit.powi(3); //(( * 100.) as u64) as f64 / 100.;
        format_with_precision(gb) + suffix
    }
}

/// Display units dynamically formatted based on the size of the value.
pub fn as_data_size(bytes: f64, si: bool) -> String {
    let unit = if si { 1000_f64 } else { 1024_f64 };
    let mut size = bytes;
    let mut unit_str = " B";

    if size >= unit.powi(4) {
        size /= unit.powi(4);
        unit_str = " TB";
    } else if size >= unit.powi(3) {
        size /= unit.powi(3);
        unit_str = " GB";
    } else if size >= unit.powi(2) {
        size /= unit.powi(2);
        unit_str = " MB";
    } else if size >= unit {
        size /= unit;
        unit_str = " KB";
    }

    format_with_precision(size) + unit_str
}

/// Format supplied value as a float with 2 decimal places.
pub fn format_as_float(f: f64, short: bool) -> String {
    if short {
        if f < 1000.0 {
            format_with_precision(f)
        } else if f < 1000000.0 {
            format_with_precision(f / 1000.0) + " K"
        } else if f < 1000000000.0 {
            format_with_precision(f / 1000000.0) + " M"
        } else if f < 1000000000000.0 {
            format_with_precision(f / 1000000000.0) + " G"
        } else if f < 1000000000000000.0 {
            format_with_precision(f / 1000000000000.0) + " T"
        } else if f < 1000000000000000000.0 {
            format_with_precision(f / 1000000000000000.0) + " P"
        } else {
            format_with_precision(f / 1000000000000000000.0) + " E"
        }
    } else {
        f.separated_string()
    }
}

/// Format supplied value as a float with 2 decimal places.
fn format_with_precision(f: f64) -> String {
    if f.fract() < 0.01 {
        separated_float!(format!("{}", f.trunc()))
    } else {
        separated_float!(format!("{:.2}", f))
    }
}
