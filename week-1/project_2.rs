fn  main() {
	// Sales record data
	let sales_record = [

		("Toshiba",450_000.00),
		("HP",750_000.00),
		("Dell", 2_850_000.00),
		("Lenovo", 250_000.00),
	];

	// Calculate the sum
	let mut sum = 0.0;
	for (_, amount) in sales_record {
		sum += amount;
	}

	// Calculate the average
	let average = sum / sales_record.len() as f32;

	//Print the results 
	println!("Sum of  sales: {:.2}",sum);
	println!("Average sale : {:.2}",average);

}