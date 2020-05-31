use crate::controller::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) { 
    cfg.service(
        web::scope("/api/v2")
        .service(
            web::scope("/entry")
            .service(
                web::resource("/login").route(web::post().to(entry::login))
            )
            .service(
                web::resource("/logout").route(web::post().to(entry::logout))
            )
        )
        .service( // 一般统计模块
            web::scope("/dashboard")
                .service(
                    web::resource("/general/data").route(web::get().to(dashboard::get_general_data))
                )
        )
        .service( //审核模块
            web::scope("/photo")
            .service(
                web::resource("/unapproved/get/{start}/{step}").route(web::get().to(photo::get_unapproved_photos))
            )
            .service(
                web::resource("/approved/get/{start}/{step}").route(web::get().to(photo::get_approved_photos))
            )
            .service(
                web::resource("/reject/get/{start}/{step}").route(web::get().to(photo::get_reject_photos))
            )
            .service(
                web::resource("/all/get/{start}/{step}").route(web::get().to(photo::get_all_photos))
            )
            .service(
                web::resource("/approve/status/update").route(web::put().to(photo::update_approve_status))
            )
            .service(
                web::resource("/delete/{photo_id}").route(web::delete().to(photo::delete_photo))
            )
        )
        .service( // 用户管理
            web::scope("/user")
            .service(
                web::resource("/get/{user_id}").route(web::get().to(user::get_user_by_id))
            )
            .service(
                web::resource("/get/{start}/{step}").route(web::get().to(user::get_all_users))
            )
            .service(
                web::resource("/blacklist/add").route(web::put().to(user::add_user_to_blacklist))
            )
            .service(
                web::resource("/blacklist/remove").route(web::put().to(user::remove_user_from_blacklist))
            )
            .service(
                web::resource("/delete").route(web::delete().to(user::delete_user))
            )
        )
        .service( //推荐图片到cover / 首页 , 推荐收藏夹
            web::scope("/recommand")
            .service(
                web::resource("/cover/{photo_id}").route(web::put().to(recommand::recommand_to_cover))
            )
            .service(
                web::resource("/homepage/photo/{photo_id}").route(web::put().to(recommand::recommand_photo_to_homepage))
            )
        )
    );
}