pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

extern "C" {
    pub fn net_perf_echo_server() -> i32;
    pub fn net_perf_ping_pong_client() -> i32;
    pub fn net_perf_init() -> i32;
}
