-- Add up migration script here
-- 创建部门表
CREATE TABLE IF NOT EXISTS `departments` (
                                             `id` INT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '部门ID',
                                             `name` VARCHAR(64) NOT NULL UNIQUE COMMENT '部门领导人',
                                             `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                                             `updated_at` DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                                             PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='部门表';
-- 创建用户表
CREATE TABLE IF NOT EXISTS `users` (
                                       `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '用户ID',
                                       `account` VARCHAR(11) NOT NULL UNIQUE COMMENT '账号',
                                       `nickname` VARCHAR(64) NOT NULL COMMENT '昵称',
                                       `level` INT UNSIGNED DEFAULT 1 COMMENT '等级',
                                       `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                                       `gender` ENUM('male', 'female', 'unknown') NOT NULL DEFAULT 'unknown' COMMENT '性别',
                                       `birthday` DATE DEFAULT '1970-01-01' COMMENT '生日',
                                       `is_opened` BOOLEAN NOT NULL DEFAULT TRUE COMMENT '是否开放',
                                       `reference` VARCHAR(64) NOT NULL DEFAULT 'Qiqianily' COMMENT '推荐人',
                                       `department_id` INT UNSIGNED DEFAULT NULL COMMENT '所属部门ID',
                                       `create_by` VARCHAR(128) NOT NULL DEFAULT 'Qiqianily' COMMENT '创建人',
                                       `update_by` VARCHAR(128) NOT NULL DEFAULT 'Qiqianily' COMMENT '更新人',
                                       `last_login_at` DATETIME NOT NULL DEFAULT '1970-01-01 00:00:00' COMMENT '最后登录时间',
                                       `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                                       `avatar` VARCHAR(255) DEFAULT '' COMMENT '头像',
                                       `password` VARCHAR(128) NOT NULL DEFAULT '' COMMENT '密码',
                                       `deleted_at` DATETIME NULL DEFAULT NULL COMMENT '删除时间',
                                       PRIMARY KEY (`id`),
                                       INDEX `idx_account` (`account`),
                                       INDEX `idx_nickname` (`nickname`),
                                       FOREIGN KEY (`department_id`) REFERENCES `departments`(`id`) ON DELETE SET NULL ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户表';
