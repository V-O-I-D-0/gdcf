macro_rules! endpoint {
    ($php:expr) => {
        concat!("http://absolllute.com/gdps/gdapi/", $php, ".php")
    };
}

macro_rules! check_resp {
    ($data:expr) => {{
        if $data == "-1" {
            return Err(ApiError::NoData)
        }
    }};
}
