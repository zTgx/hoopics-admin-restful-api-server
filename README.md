# hoopics admin restful api server
前段时间使用ReactJs + Eggjs写了一个图片分享的网站[hoopics](http://www.hoopics.cn), 顺带实现了一个后台管理系统.  
去年开始使用Rust写项目, 于是想着继续使用Rust语言, 在web端的一个技术尝试.  
hoopics-admin-restful-api的主要功能:  
* 数据统计
* 图片审核
* 用户管理
* 首页推荐
---
* 相关框架: actix-web + Diesel ORM + JWT + Mariadb.  
* 代码结构: 学习eggjs, 即router -> controller -> service -> model的结构.
