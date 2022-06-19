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

pub fn get_net_usage(netio: &mut NetIoCountersCollector) -> NetUsageRaw {

    let net_usage = netio.net_io_counters().unwrap();

    NetUsageRaw {
        download: net_usage.bytes_recv(),
        upload: net_usage.bytes_sent()
    }
}

pub fn calc_net_interval(first_net_usage: NetUsageRaw, second_net_usage: NetUsageRaw) -> NetUsage {

    let net_usage_download = second_net_usage.download - first_net_usage.download;
    let net_usage_upload = second_net_usage.upload - first_net_usage.upload;

    NetUsage {
        download: get_bytevalue_from(net_usage_download),
        upload: get_bytevalue_from(net_usage_upload)
    }
}