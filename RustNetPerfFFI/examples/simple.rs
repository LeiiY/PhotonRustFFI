fn main() {
    unsafe {
        RustNetPerfFFI::net_perf_init();
        // println!("photon::init returned: {}", init_ret);
        // let ret = RustNetPerfFFI::net_perf_echo_server();
        // println!("echo_server returned: {}", ret);
    }
} 