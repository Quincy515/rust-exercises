-- Your SQL goes here
CREATE TABLE users
(
    id           SERIAL PRIMARY KEY                     NOT NULL,
    created_at   TIMESTAMP    DEFAULT current_timestamp NOT NULL,
    updated_at   TIMESTAMP    DEFAULT current_timestamp NOT NULL,
    uuid         VARCHAR(255) DEFAULT NULL COMMENT '用户UUID',
    username     VARCHAR(255) DEFAULT NULL COMMENT '用户登录名',
    password     VARCHAR(255) DEFAULT NULL COMMENT '用户登录密码',
    email        VARCHAR(255) DEFAULT NULL COMMENT '用户邮箱',
    phone        VARCHAR(255) DEFAULT NULL COMMENT '用户手机',
    nick_name    VARCHAR(255) DEFAULT '系统用户' COMMENT '用户昵称',
    head_img     VARCHAR(255) DEFAULT 'http://qmplusimg.henrongyi.top/head.png' COMMENT '用户头像',
    authority_id VARCHAR(120) DEFAULT '888' COMMENT '用户角色ID'
);

CREATE UNIQUE INDEX username_idx ON users (username);
CREATE UNIQUE INDEX uuid_idx ON users (uuid);