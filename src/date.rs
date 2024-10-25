use std::time::SystemTime;

pub struct FullDateAndTime {
    pub seconds: u8, // 0-59
    pub minutes: u8, // 0-59
    pub hours: u8,   // 0-23
    pub day: u8,     // 1-31
    pub month: u8,   // 1-12
    pub year: i32,   // 0-2099
                     // weekday: &'static str,   // Sunday-Monday
                     // monthname: &'static str, // January-December
                     // day_of_year: u32,        // 0-366
                     // is_leap_year: bool,      // leap year ?
                     //    is_dst: bool,       // is daylight saving time FIXME how to get this?
                     //    utc_offset: u32,    // offset from utc time in seconds FIXME how to get this?
}

#[derive(Debug)]
struct Date {
    day: u8,   // 1-31
    month: u8, // 1-12
    year: i32, // 0-2099
}

#[derive(Debug)]
struct Time {
    seconds: u8, // 0-59
    minutes: u8, // 0-59
    hours: u8,   // 0-23
}

// const WEEKDAY: &'static [&'static str; 7] = &[
//     "Sunday",
//     "Monday",
//     "Tuesday",
//     "Wednesday",
//     "Thrusday",
//     "Frieday",
//     "Saturday",
// ];
//
// const MONTH_NAME: &'static [&'static str; 12] = &[
//     "January",
//     "February",
//     "March",
//     "April",
//     "May",
//     "June",
//     "Juli",
//     "August",
//     "September",
//     "October",
//     "November",
//     "December",
// ];
//
// const LAST_DAY_OF_MONTH_LEAP: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
// const LAST_DAY_OF_MONTH_COMMMON: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
//
// fn is_leap_year(year: i32) -> bool {
//     if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
//         return true;
//     }
//     false
// }

fn get_utc_offset() -> i32 {
    120
}

fn get_days_from_seconds(seconds: u64) -> u64 {
    let days = seconds / 86_400;
    days
}

fn get_time_from_seconds(seconds: u64) -> Time {
    let mut s = seconds;
    let h = s / 3_600;
    s -= h * 3_600;
    let m = s / 60;
    s -= m * 60;
    let t = Time {
        hours: h as u8,
        minutes: m as u8,
        seconds: s as u8,
    };
    t
}

// weekdays: 0 = Monday, 1 = Tuesday ... 6 = Sunday
// fn get_weekday_from_days(days: u64) -> u8 {
//     let d: i64 = days as i64;
//     let weekday = if d >= -4 {
//         (d + 4) % 7
//     } else {
//         (d + 5) % 7 + 6
//     };
//     weekday as u8
// }
//
// fn get_day_of_year(year: i32, month: u8, day: u8) -> u32 {
//     let mut d: u32 = day as u32;
//     if is_leap_year(year) {
//         for n in 0..month - 1 {
//             d += LAST_DAY_OF_MONTH_LEAP[n as usize] as u32;
//         }
//     } else {
//         for n in 0..month - 1 {
//             d += LAST_DAY_OF_MONTH_COMMMON[n as usize] as u32;
//         }
//     }
//     d
// }

fn get_date_from_days(days: u64) -> Date {
    let z: i64 = days as i64 + 719_468;
    let era = if z >= 0 {
        z / 146_097
    } else {
        (z - 146_096) / 146_097
    };
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let mut result = Date {
        day: d as u8,
        month: m as u8,
        year: y as i32,
    };
    if m <= 2 {
        result.year = (y + m) as i32;
    }
    result
}

// fn get_days_from_date(year: u32, month: u8, day: u8) -> u64 {
//     let mut y: i64 = year as i64;
//     let m = month as i64;
//     let d = day as i64;
//     if m <= 2 {
//         y -= m;
//     }
//     let era: i64 = if y >= 0 { y / 400 } else { (y - 399) / 400 };
//     let yoe: i64 = y - era / 400;
//     let doy: i64 = if m > 2 {
//         (153 * (m - 3) + 2) / 5 + d - 1
//     } else {
//         (153 * (m + 9) + 2) / 5 + d - 1
//     };
//     let doe: i64 = yoe * 365 + yoe / 4 - yoe / 100 + doy;
//     let result: u64 = (era * 146_097 + doe - 719_468) as u64;
//     result
// }

fn get_date(seconds: u64) -> FullDateAndTime {
    let dys: u64 = get_days_from_seconds(seconds);
    let d: Date = get_date_from_days(dys);
    let t: Time = get_time_from_seconds(seconds - (dys * 86_400));
    let fdt = FullDateAndTime {
        seconds: t.seconds,
        minutes: t.minutes,
        hours: t.hours,
        day: d.day,
        month: d.month,
        year: d.year,
        // monthname: MONTH_NAME[(d.month - 1) as usize],
        // weekday: WEEKDAY[get_weekday_from_days(dys) as usize],
        // day_of_year: get_day_of_year(d.year, d.month, d.day),
        // is_leap_year: is_leap_year(d.year),
    };
    // dbg!(fdt.day_of_year);
    // dbg!(fdt.monthname);
    // dbg!(fdt.weekday);
    fdt
}

pub fn get_current_date() -> FullDateAndTime {
    let mut now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    now += get_utc_offset() as u64;
    let current_date: FullDateAndTime = get_date(now);
    current_date
}

pub fn get_current_date_as_string() -> String {
    let current_date: FullDateAndTime = get_current_date();
    let current_date_string: String = format!(
        "{:4}-{:02}-{:02} {:02}:{:02}:{:02}",
        current_date.year,
        current_date.month,
        current_date.day,
        current_date.hours,
        current_date.minutes,
        current_date.seconds
    );
    current_date_string
}

// pub fn get_current_date_as_string() -> String {
//     let current_date: FullDateAndTime = get_current_date();
//     let current_date_string: String = format!(
//         "{:02}.{:02}.{:4} {:02}:{:02}:{:02}",
//         current_date.day,
//         current_date.month,
//         current_date.year,
//         current_date.hours,
//         current_date.minutes,
//         current_date.seconds
//     );
//     current_date_string
// }
