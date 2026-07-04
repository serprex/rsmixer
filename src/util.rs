use pulse::volume;

pub fn volume_to_percent(volume: volume::ChannelVolumes) -> u16 {
    let avg = volume.avg().0;

    let base_delta = (volume::Volume::NORMAL.0 as f32 - volume::Volume::MUTED.0 as f32) / 100.0;

    ((avg - volume::Volume::MUTED.0) as f32 / base_delta).round() as u16
}

pub fn percent_to_volume(target_percent: i16) -> u32 {
    let base_delta = (volume::Volume::NORMAL.0 as f32 - volume::Volume::MUTED.0 as f32) / 100.0;

    if target_percent < 0 {
        volume::Volume::MUTED.0
    } else if target_percent == 100 {
        volume::Volume::NORMAL.0
    } else if target_percent >= 150 {
        (volume::Volume::NORMAL.0 as f32 * 1.5) as u32
    } else if target_percent < 100 {
        volume::Volume::MUTED.0 + target_percent as u32 * base_delta as u32
    } else {
        volume::Volume::NORMAL.0 + (target_percent - 100) as u32 * base_delta as u32
    }
}

#[macro_export]
macro_rules! unwrap_or_return {
    ($x:expr_2021, $y:expr_2021) => {
        match $x {
            Some(x) => x,
            None => {
                return $y;
            }
        }
    };
    ($x:expr_2021) => {
        unwrap_or_return!($x, ())
    };
}

#[macro_export]
macro_rules! error {
    ($($x:expr_2021),*) => {
        log::error!("[{}] {}", LOGGING_MODULE, format!($($x),*));
    }
}

#[macro_export]
macro_rules! debug {
    ($($x:expr_2021),*) => {
        log::debug!("[{}] {}", LOGGING_MODULE, format!($($x),*));
    }
}

#[macro_export]
macro_rules! info {
    ($($x:expr_2021),*) => {
        log::info!("[{}] {}", LOGGING_MODULE, format!($($x),*));
    }
}

#[macro_export]
macro_rules! warn {
    ($($x:expr_2021),*) => {
        log::warn!("[{}] {}", LOGGING_MODULE, format!($($x),*));
    }
}
