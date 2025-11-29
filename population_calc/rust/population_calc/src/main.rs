//! This class is used to simulate the german population over the years since 2005.

use std::io;

fn main() {
    println!("How many years do you want to simulate?");
    println!("integers only !!!");

    let mut answer: String = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line.");

    let years: u128 = answer.trim().parse().expect("Please enter an Integer!");

    println!("Do you want an output for each year? [y/N]");

    let mut answer: String = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line.");

    let answer: &str = answer.trim();

    let raw: bool = answer == "y" || answer == "Y";
    println!("{answer}");
    println!("{raw}");

    sim(years, raw);
}

/// This function prints the values for each given year.
///
/// # Arguments
///
/// * `years` – Total number of years to simulate.
/// * `raw` – If `true`, prints the values for every individual year.
///           When `false`, only summary output is produced.
///
/// # Returns
///
/// Nothing. Output is written directly to the console.
///
/// # Notes
///
/// The exact behavior depends on the internal state of the struct.
/// Adjust the printing logic if the simulation model changes.
fn sim(years: u128, raw: bool) {
    let start_year: u128 = 2005;

    // Starting values in Million
    let mut values: (f64, f64, f64, f64) = (12.3, 39.1, 15.5, 16.3);

    if raw {
        println!("Year 2005");

        println!("0-14 years:     {0} million", values.0);
        println!("15-49 years:    {0} million", values.1);
        println!("50-64 years:    {0} million", values.2);
        println!("over 65 years:  {0} million", values.3);
        println!("");
    }
    for year in 0..years {
        if raw {
            println!("Year: {0}", start_year + year + 1);
        }

        values = calc(values);

        if raw {
            println!("0-14 years:     {0} million", values.0);
            println!("15-49 years:    {0} million", values.1);
            println!("50-64 years:    {0} million", values.2);
            println!("over 65 years:  {0} million", values.3);
            println!("");
        }
    }
    if raw != true {
        println!("Year: {0}", start_year + years);
        println!("0-14 years:     {0} million", values.0);
        println!("15-49 years:    {0} million", values.1);
        println!("50-64 years:    {0} million", values.2);
        println!("over 65 years:  {0} million", values.3);
        println!("");
    }
}

/// Computes the next year's values based on the previous year's data,
/// following a fixed update schema.
///
/// # Arguments
///
/// * `old_values` – Values from the previous year. Must follow the expected tuple format.
///
/// # Returns
///
/// A tuple containing the computed values for the next year, in the same field order as `old_values`.
///
/// # Notes
///
/// The update schema is deterministic. If the schema changes, both input
/// and output interpretation must be adjusted accordingly.
fn calc(old_values: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    let g_0_14 = old_values.0;
    let g_15_49 = old_values.1;
    let g_50_64 = old_values.2;
    let g_65 = old_values.3;

    // Group 0-14
    let mut new_0_14 = g_0_14 * 0.93;
    new_0_14 += g_15_49 * 0.02;

    // Group 15-49
    let mut new_15_49 = g_15_49 * 0.97;
    new_15_49 += g_15_49 * 0.066;

    // Group 50-64
    let mut new_50_64 = g_50_64 * 0.925;
    new_50_64 += g_15_49 * 0.029;

    // Group over 65
    let mut new_65 = g_65 * 0.972;
    new_65 += g_50_64 * 0.066;

    let new_values: (f64, f64, f64, f64) = (new_0_14, new_15_49, new_50_64, new_65);

    return new_values;
}
