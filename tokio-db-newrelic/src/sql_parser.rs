use sqlparser::{ast::*, dialect::Dialect, parser::Parser};

#[derive(Debug)]
pub struct ObserverPostgresSqlDialect {}

impl Dialect for ObserverPostgresSqlDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z')
            || (ch >= 'A' && ch <= 'Z')
            || (ch == '@')
            || ch == '$'
            || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z')
            || (ch >= 'A' && ch <= 'Z')
            || (ch >= '0' && ch <= '9')
            || (ch == '@')
            || ch == '$'
            || ch == '_'
    }
}

#[allow(dead_code)]
fn split_query_by_where(query: &str) -> String {
    let query = query.to_lowercase();
    let sql: Vec<&str> = query.split("where").collect::<Vec<&str>>();
    (*sql.first().unwrap_or(&query.as_str())).to_string()
}

#[allow(dead_code)]
pub fn parse_sql(sql: &str) -> (String, String) {
    match Parser::parse_sql(&ObserverPostgresSqlDialect {}, &split_query_by_where(sql)) {
        Ok(ast) =>
        {
            #[allow(clippy::never_loop)]
            for x in ast {
                match x {
                    Statement::Query(query) => {
                        match query.body {
                            SetExpr::Select(select) => {
                                let mut table_name = vec![];
                                for x in select.from {
                                    table_name.push(x.relation.to_string());
                                    for join in x.joins {
                                        table_name.push(join.relation.to_string());
                                    }
                                }
                                return ("select".to_string(), table_name.join("__"));
                            }
                            _ => return ("unknown".to_string(), "unknown".to_string()),
                        };
                    }
                    Statement::Update { table_name, .. } => {
                        return ("update".to_string(), table_name.to_string());
                    }
                    Statement::Insert { table_name, .. } => {
                        return ("insert".to_string(), table_name.to_string());
                    }
                    Statement::Copy { table_name, .. } => {
                        return ("copy".to_string(), table_name.to_string());
                    }
                    Statement::Delete { table_name, .. } => {
                        return ("delete".to_string(), table_name.to_string());
                    }
                    Statement::CreateView { name, .. } => {
                        return ("create_view".to_string(), name.to_string());
                    }
                    Statement::CreateTable { name, .. } => {
                        return ("create_table".to_string(), name.to_string());
                    }
                    Statement::AlterTable { name, .. } => {
                        return ("alter".to_string(), name.to_string());
                    }
                    Statement::Drop { names, .. } => {
                        return (
                            "drop".to_string(),
                            names
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join("__"),
                        );
                    }
                    _ => {
                        return ("unknown".to_string(), "unknown".to_string());
                    }
                }
            }
        }
        Err(_err) => {
            #[cfg(debug_assertions)]
            println!("Err : {:?}", _err);
            return ("unknown".to_string(), "unknown".to_string());
        }
    };

    ("unknown".to_string(), "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_sql;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse_sql("select abc from employee, abc1 where name = asgief"),
            ("select".to_string(), "employee, abc1".to_string())
        );
        assert_eq!(
            parse_sql("select * from supplier join orders on supplier.id=orders.id;"),
            ("select".to_string(), "supplier, orders".to_string())
        );
        assert_eq!(
            parse_sql(
                r#"
                SELECT customer.customer_id FROM customer
                INNER JOIN payment ON payment.customer_id = customer.customer_id
                INNER JOIN payment1 ON payment1.customer_id = customer.customer_id;
            "#
            ),
            (
                "select".to_string(),
                "customer, payment, payment1".to_string()
            )
        );

        assert_eq!(
            parse_sql("update employee set name = asgief"),
            ("update".to_string(), "employee".to_string())
        );

        assert_eq!(
            parse_sql("insert into employee(id, name) values(1, 23)"),
            ("insert".to_string(), "employee".to_string())
        );

        assert_eq!(
            parse_sql("delete from employee where name = asgief"),
            ("delete".to_string(), "employee".to_string())
        );

        assert_eq!(
            parse_sql(
                r#"
                CREATE TABLE account(
                user_id serial PRIMARY KEY,
                username VARCHAR (50) UNIQUE NOT NULL,
                password VARCHAR (50) NOT NULL,
                email VARCHAR (355) UNIQUE NOT NULL,
                created_on TIMESTAMP NOT NULL,
                last_login TIMESTAMP);
            "#
            ),
            ("create table".to_string(), "account".to_string())
        );

        assert_eq!(
            parse_sql("drop table employee, employee1;"),
            ("drop".to_string(), "employee, employee1".to_string())
        );

        assert_eq!(
            parse_sql(
                r#" SELECT "users_skill"."id", "users_skill"."name", "users_skill"."description",
            "users_skill"."allocation_logic" FROM "users_skill" WHERE "users_skill"."id" > $1"#
            ),
            ("select".to_string(), "\"users_skill\"".to_string())
        );

        assert_eq!(
            parse_sql(
                r#"
            SELECT "ackore_policy"."id", "ackore_policy"."data", "ackore_policy"."created_on",
            "ackore_policy"."updated_on", "ackore_policy"."plan_id", "ackore_policy"."user_id",
            "ackore_policy"."output", "ackore_policy"."sort_on", "ackore_policy"."payment_id",
            "ackore_policy"."insurance_data", "ackore_policy"."intermediary_id",
            "ackore_policy"."policy_number", "ackore_policy"."refund_id"
            FROM "ackore_policy" WHERE "ackore_policy"."id" = $1 LIMIT $2 -- binds: [143343871, 1]
        "#
            ),
            ("select".to_string(), "\"ackore_policy\"".to_string())
        );

        assert_eq!(
            parse_sql(
                r#"
        SELECT customer.customer_id FROM customer
        INNER JOIN payment ON payment.customer_id = customer.customer_id
        INNER JOIN payment1 ON payment1.customer_id = customer.customer_id;
        "#
            ),
            (
                "select".to_string(),
                "customer, payment, payment1".to_string()
            )
        );

        //                assert_eq!(
        //                    parse_sql(r#"
        //                        BEGIN;
        //                            UPDATE accounts SET balance = balance - 100.00
        //                                WHERE name = 'Alice'
        //                        COMMIT;
        //                    "#),
        //                    ("transaction".to_string(), "employee, employee1".to_string())
        //                );

        //                assert_eq!(
        //                    parse_sql("CREATE VIEW view_name AS query;"),
        //                    ("create view".to_string(), "employee".to_string())
        //                );

        //        assert_eq!(
        //            parse_sql("ALTER TABLE table_name ADD COLUMN new_column_name varchar"),
        //            ("create view".to_string(), "employee".to_string())
        //        );
    }

    #[test]
    fn split_sql_test1() {
        let sql = r#"
            SELECT "ackore_policy"."id", "ackore_policy"."data", "ackore_policy"."created_on",
            "ackore_policy"."updated_on", "ackore_policy"."plan_id", "ackore_policy"."user_id",
            "ackore_policy"."output", "ackore_policy"."sort_on", "ackore_policy"."payment_id",
            "ackore_policy"."insurance_data", "ackore_policy"."intermediary_id",
            "ackore_policy"."policy_number", "ackore_policy"."refund_id"
            FROM "ackore_policy" WHERE "ackore_policy"."id" = $1 LIMIT $2 -- binds: [143343871, 1]
            "#;
        assert_eq!(
            r#"
            select "ackore_policy"."id", "ackore_policy"."data", "ackore_policy"."created_on",
            "ackore_policy"."updated_on", "ackore_policy"."plan_id", "ackore_policy"."user_id",
            "ackore_policy"."output", "ackore_policy"."sort_on", "ackore_policy"."payment_id",
            "ackore_policy"."insurance_data", "ackore_policy"."intermediary_id",
            "ackore_policy"."policy_number", "ackore_policy"."refund_id"
            from "ackore_policy" "#,
            super::split_query_by_where(sql)
        );
    }

    #[test]
    fn split_sql_test2() {
        let sql = r#"
        SELECT customer.customer_id FROM customer
        INNER JOIN payment ON payment.customer_id = customer.customer_id
        INNER JOIN payment1 ON payment1.customer_id = customer.customer_id;
        "#;
        assert_eq!(sql.to_lowercase(), super::split_query_by_where(sql));
    }
}
