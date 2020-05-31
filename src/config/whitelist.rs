use std::collections::HashMap;

// 需要API key的path
lazy_static! {
    pub static ref WHITELIST: HashMap<&'static str, bool> = {
        let white_list: HashMap<&'static str, bool> = [
            // dashboard 模块
            ("/api/v2/dashboard/general/data", false),

            // 审核模块
            ("/api/v2/photo/unapproved/get", true),
            ("/api/v2/photo/approved/get",   true),

        ].iter().cloned().collect();
                                    
        white_list
    };
}
