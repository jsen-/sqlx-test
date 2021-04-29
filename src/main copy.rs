#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;

#[macro_use]
extern crate std;

use sqlx::{
    postgres::PgPoolOptions,
    types::chrono::{NaiveDateTime, Utc},
};

use std::time::Duration;

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let pool = if let Ok(pool) = PgPoolOptions::new().max_connections(5).connect_timeout(Duration::from_secs(5)).connect("postgres://root@localhost:26257/sensors").await {
                pool
            } else {
                return;
            };
            if let Ok(x) = {
                {
                    {
                        use::sqlx::Arguments as _ ;
                        let query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                        ::sqlx::query_with::<sqlx::postgres::Postgres, _>("
                            CREATE TABLE IF NOT EXISTS boxes(
                                id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
                                name STRING,
                                time_last_received TIMESTAMP,
                                imei STRING,
                                phone_number STRING
                            );", query_args)
                    }
                }
            }.execute(& pool).await {
                x
            } else {
                return ;
            };
            let now = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
            if let Ok(row) = {
                {
                    {
                        use::sqlx::Arguments as _;
                        let arg0 = &("aaa");
                        let arg1 = &(now) ;
                        let arg2 = &("0000");
                        let arg3 = &("jozo");
                        if false {
                            use::sqlx::ty_match::{WrapSameExt as _ ,MatchBorrowExt as _};

                            let _expr = ::sqlx::ty_match::dupe_value(arg0);
                            let ty_check = ::sqlx::ty_match::WrapSame::<&str, _>::new(&_expr).wrap_same();
                            let(mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(ty_check, &_expr);
                            _ty_check = match_borrow.match_borrow();
                            {
                                return
                            } ;
                        }
                        if false {
                            use::sqlx::ty_match::{ WrapSameExt as _ , MatchBorrowExt as _ } ;
                            let _expr =::sqlx::ty_match::dupe_value(arg1) ;
                            let ty_check =::sqlx::ty_match::WrapSame::< sqlx::types::chrono::NaiveDateTime , _ >::new(& _expr).wrap_same() ;
                            let(mut _ty_check , match_borrow) =::sqlx::ty_match::MatchBorrow::new(ty_check , & _expr) ;
                            _ty_check = match_borrow.match_borrow() ;
                            {
                                return
                            } ;
                        }
                        if false {
                            use::sqlx::ty_match::{ WrapSameExt as _ , MatchBorrowExt as _ };
                            let _expr =::sqlx::ty_match::dupe_value(arg2) ;
                            let ty_check =::sqlx::ty_match::WrapSame::< & str , _ >::new(& _expr).wrap_same() ;
                            let(mut _ty_check , match_borrow) =::sqlx::ty_match::MatchBorrow::new(ty_check , & _expr) ;
                            _ty_check = match_borrow.match_borrow() ;
                            {
                                return
                            };
                        }
                        if false { use::sqlx::ty_match::{ WrapSameExt as _ , MatchBorrowExt as _ } ;
                            let _expr =::sqlx::ty_match::dupe_value(arg3) ;
                            let ty_check =::sqlx::ty_match::WrapSame::< & str , _ >::new(& _expr).wrap_same() ;
                            let(mut _ty_check , match_borrow) =::sqlx::ty_match::MatchBorrow::new(ty_check , & _expr) ;
                            _ty_check = match_borrow.match_borrow() ;
                            {
                                return
                            };
                        }
                        let mut query_args = < sqlx::postgres::Postgres as::sqlx::database::HasArguments >::Arguments::default() ;
                        query_args.reserve(4usize, 0 + ::sqlx::encode::Encode::< sqlx::postgres::Postgres >::size_hint(arg0) + ::sqlx::encode::Encode::< sqlx::postgres::Postgres >::size_hint(arg1) +::sqlx::encode::Encode::< sqlx::postgres::Postgres >::size_hint(arg2) +::sqlx::encode::Encode::< sqlx::postgres::Postgres >::size_hint(arg3)) ;
                        query_args.add(arg0) ;
                        query_args.add(arg1) ;
                        query_args.add(arg2) ;
                        query_args.add(arg3) ;
                        struct Record {
                            id: i32,
                        }
                        impl::core::fmt::Debug for Record {
                            fn fmt(& self , f : & mut::core::fmt::Formatter) ->::core::fmt::Result {
                                match * self {
                                    Record { id : ref __self_0_0 } => {
                                        let debug_trait_builder = & mut::core::fmt::Formatter::debug_struct(f , "Record") ;
                                        let _ =::core::fmt::DebugStruct::field(debug_trait_builder , "id" , & &(* __self_0_0)) ;
                                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                                    }
                                }
                            }
                        }
                        ::sqlx::query_with::< sqlx::postgres::Postgres , _ >("insert into boxes(name, time_last_received, imei, phone_number) values($1, $2, $3, $4) returning id" , query_args).try_map(|row: sqlx::postgres::PgRow| {
                            use::sqlx::Row as _ ;
                            let id = row.try_get_unchecked::< i32 , _ >(0usize) ? ;
                            Ok(Record { id : id , })
                        })
                    }
                }
            }.fetch_one(& pool).await {
                row
            } else {
                return;
            } ;
        })
}
