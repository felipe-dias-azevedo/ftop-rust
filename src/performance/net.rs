use psutil::network::NetIoCountersCollector;
use crate::console::byteutils::{ByteValue, get_bytevalue_from};

pub struct NetUsageRaw {
    pub download: u64,
    pub upload: u64
}

pub struct NetUsage {
    pub download: ByteValue,
    pub upload: ByteValue
}

pub fn get_net_usage(netio: &mut NetIoCountersCollector) -> Option<NetUsageRaw> {

    let net_usage = netio.net_io_counters().ok();

    if net_usage.is_none() {
        return None;
    }

    let net_usage = net_usage.unwrap();

    let net_usage = NetUsageRaw {
        download: net_usage.bytes_recv(),
        upload: net_usage.bytes_sent()
    };

    Some(net_usage)
}

pub fn calc_net_interval(
    first_net_usage: Option<NetUsageRaw>,
    second_net_usage: Option<NetUsageRaw>
) -> Option<NetUsage> {

    if first_net_usage.is_none() || second_net_usage.is_none() {
        return None;
    }

    let first_net_usage = first_net_usage.unwrap();
    let second_net_usage = second_net_usage.unwrap();

    let net_usage_download = second_net_usage.download - first_net_usage.download;
    let net_usage_upload = second_net_usage.upload - first_net_usage.upload;

    let net_usage = NetUsage {
        download: get_bytevalue_from(net_usage_download),
        upload: get_bytevalue_from(net_usage_upload)
    };

    Some(net_usage)
}