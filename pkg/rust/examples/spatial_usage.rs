#[cfg(feature = "sled-storage")]
mod spatial_usage {
    use {
        gluesql::prelude::{Glue, JsonStorage},
        gluesql_core::ast_builder::{calc_distance, col, get_x, get_y, num, point, table, Build},
    };

    pub fn spatial_api() {
        let storage = JsonStorage::new("data/spatial-api").unwrap();
        let mut glue = Glue::new(storage);

        let drop_table_if_exists = table("Foo").drop_table_if_exists().build();
        let drop_table_if_exists_2 = table("Bar").drop_table_if_exists().build();

        let create_table_query = table("Foo")
            .create_table()
            .add_column("id INTEGER NULL")
            .add_column("name TEXT")
            .add_column("location Point")
            .add_column("city_id INTEGER")
            .build();

        let create_table_location_name = table("Bar")
            .create_table()
            .add_column("id INTEGER NULL")
            .add_column("city_name TEXT")
            .build();

        let insert_city_data = table("Bar")
            .insert()
            .columns("id, city_name")
            .values(vec![
                "1, 'New York'",
                "2, 'Los Angeles'",
                "3, 'San Francisco'",
            ])
            .build();

        let insert_data_query = table("Foo")
            .insert()
            .columns("id, name, location, city_id")
            .values(vec![
                "1, 'John', POINT(1, 2), 1",
                "2, 'Jane', POINT(3, 4), 2",
                "3, 'Joe', POINT(10, 40), 3",
            ])
            .build();

        let select_all_data_query = table("Foo").select().project("*").build();

        let select_get_data_where_distance_in_5 = table("Foo")
            .select()
            .left_join("Bar")
            .on("Foo.city_id = Bar.id")
            .filter(calc_distance(point(num(1), num(2)), col("location")).lte(num(5)))
            .project(vec![
                get_x("location"),
                get_y("location"),
                col("name"),
                col("city_name"),
            ])
            .build();

        let sqls = [
            drop_table_if_exists,
            drop_table_if_exists_2,
            create_table_query,
            create_table_location_name,
            insert_city_data,
            insert_data_query,
            select_all_data_query,
            select_get_data_where_distance_in_5,
        ];

        for sql in sqls {
            match sql {
                Ok(sql) => {
                    let result = glue.execute_stmt(&sql).unwrap();
                    println!("{:?}", result);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}

fn main() {
    #[cfg(feature = "json-storage")]
    spatial_usage::spatial_api();
}
