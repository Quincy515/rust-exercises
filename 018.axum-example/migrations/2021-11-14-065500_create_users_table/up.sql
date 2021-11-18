-- Your SQL goes here
SET character_set_client = utf8mb4;
CREATE TABLE users
(
    `id`           INTEGER AUTO_INCREMENT PRIMARY KEY,
    `uuid`         VARCHAR(36)  NOT NULL COMMENT '用户UUID',
    `username`     VARCHAR(255) NOT NULL COMMENT '用户登录名',
    `password`     VARCHAR(255) NOT NULL COMMENT '用户登录密码',
    `email`        VARCHAR(255) NOT NULL DEFAULT '' COMMENT '用户邮箱',
    `phone`        VARCHAR(255) NOT NULL DEFAULT '' COMMENT '用户手机',
    `nick_name`    VARCHAR(255) NOT NULL DEFAULT '系统用户' COMMENT '用户昵称',
    `head_img`     VARCHAR(255) NOT NULL DEFAULT 'http://qmplusimg.henrongyi.top/head.png' COMMENT '用户头像',
    `authority_id` VARCHAR(120) NOT NULL DEFAULT '888' COMMENT '用户角色ID',
    `created_at`   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY `udx_uuid` (`uuid`),
    UNIQUE KEY `udx_username` (`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户表';

CREATE TRIGGER `users_before_insert` BEFORE INSERT ON `users` FOR EACH ROW 
BEGIN
IF new.uuid is NULL THEN
		SET new.uuid = UUID();
END IF; 
END;