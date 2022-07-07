use super::*;
/*fn different_each_regen() {
	println!("Helloo!");
	println!("Helloo!");
}*/
#[rustfmt::skip]
pub fn body_0(db: &mut diesel::pg::PgConnection) {
	let q =
		schema::some_table_1::table.inner_join(
			schema::some_table_2::table.inner_join(
				schema::some_table_3::table.inner_join(
					schema::some_table_4::table.inner_join(
						schema::some_table_5::table/*.inner_join(
							schema::some_table_6::table.inner_join(schema::some_table_7::table),
						)*/,
					)
				),
			)
		);
}
