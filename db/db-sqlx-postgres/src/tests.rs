// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

#![cfg(test)]

use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::*;

use db_core::tests::*;

#[actix_rt::test]
async fn everyting_works() {
    const EMAIL: &str = "postgresuser@foo.com";
    const NAME: &str = "postgresuser";
    const PASSWORD: &str = "pasdfasdfasdfadf";
    const SECRET1: &str = "postgressecret1";
    // captcha config
    const CAPTCHA_SECRET: &str = "postgrescaptchasecret";
    const CAPTCHA_DESCRIPTION: &str = "postgrescaptchadescription";
    const CAPTCHA_DURATION: i32 = 30;
    // notification config
    const HEADING: &str = "testing notifications get db postgres";
    const MESSAGE: &str = "testing notifications get message db postgres";

    // easy traffic pattern
    const TRAFFIC_PATTERN: TrafficPattern = TrafficPattern {
        avg_traffic: 500,
        peak_sustainable_traffic: 5_000,
        broke_my_site_traffic: Some(10_000),
    };

    const LEVELS: [Level; 3] = [
        Level {
            difficulty_factor: 1,
            visitor_threshold: 1,
        },
        Level {
            difficulty_factor: 2,
            visitor_threshold: 2,
        },
        Level {
            difficulty_factor: 3,
            visitor_threshold: 3,
        },
    ];

    const ADD_NOTIFICATION: AddNotification = AddNotification {
        from: NAME,
        to: NAME,
        message: MESSAGE,
        heading: HEADING,
    };

    let url = env::var("POSTGRES_DATABASE_URL").unwrap();
    let pool_options = PgPoolOptions::new().max_connections(2);
    let connection_options = ConnectionOptions::Fresh(Fresh {
        pool_options,
        url,
        disable_logging: false,
    });
    let db = connection_options.connect().await.unwrap();

    db.migrate().await.unwrap();
    let p = Register {
        username: NAME,
        email: Some(EMAIL),
        hash: PASSWORD,
        secret: SECRET1,
    };

    let c = CreateCaptcha {
        duration: CAPTCHA_DURATION,
        key: CAPTCHA_SECRET,
        description: CAPTCHA_DESCRIPTION,
    };
    database_works(&db, &p, &c, &LEVELS, &TRAFFIC_PATTERN, &ADD_NOTIFICATION).await;
}
