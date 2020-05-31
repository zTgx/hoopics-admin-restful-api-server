//业务层面的错误码,在body中反应,区别系统的错误码
pub const SUCCESS: u32 = 200;


//消息提示
pub const MESSAGE_USE_GET_SUCCESS : &str = "Users get successfully";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

// entry 模块
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGOUT_SUCCESS: &str  = "Logout successfully";

// dashboard 模块
pub const MESSAGE_DASHBOARD_GENERAL_FAILED      : &str = "Get dashboard data failed";
pub const MESSAGE_DASHBOARD_FAILED_USERS_GET    : &str = "Get user count failed";
pub const MESSAGE_DASHBOARD_FAILED_PHOTOS_GET   : &str = "Get photo count failed";
pub const MESSAGE_DASHBOARD_FAILED_DOWNLOADS_GET: &str = "Get photo downloads failed";
