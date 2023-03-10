use csv::ReaderBuilder;
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{prelude::*, OwnedRepr};
use ndarray_csv::Array2Reader;

fn main() {
    // Load dataset
    let x = array![[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]];
    let y = array![6., 15., 24.];

    // Create linear regression model
    let mut model = LinearRegression::default();

    // Fit model to data
    model.fit(&x, &y).unwrap();

    // Make predictions on new data
    let new_data = array![[10., 11., 12.], [13., 14., 15.]];
    let predictions = model.predict(&new_data).unwrap();

    println!("Predictions: {:?}", predictions);
}
