// network in & out (download, upload)

use psutil::network::NetIoCountersCollector;
use crate::byteutils::{ByteValue, convert_value_from};

pub struct NetUsage {
    pub download: ByteValue,
    pub upload: ByteValue
}

pub fn get_net_usage(netio: &mut NetIoCountersCollector) -> NetUsage {

    let net_usage = netio.net_io_counters().unwrap();

    NetUsage {
        download: convert_value_from(net_usage.bytes_recv()),
        upload: convert_value_from(net_usage.bytes_sent())
    }
}
