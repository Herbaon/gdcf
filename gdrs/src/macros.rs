macro_rules! endpoint {
    ($php:expr) => {
        concat!("http://https://stable.supergd.ga/database/", $php, ".php")
    };
}

macro_rules! check_resp {
    ($data:expr) => {{
        if $data == "-1" {
            return Err(ApiError::NoData)
        }
    }};
}
