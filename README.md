## Overview
In project 3, I developed a logistic regression training model, and upload training dataset on AWS S3 for data file storage.

I also try to deploy the program on Amazon SageMaker Studio, but found it only support more commonly used data analysis language like R and Python.


### Rust ML Program: Fit Logistic Regrssion Classifier on Wine Quality

#### 1.Linfa - A Meta-crate for ML

Linfa is a higher-level meta-crate that includes common helpers for data processing and algorithms for many areas of machine learning, including:

- Linear regression
- Data clustering
- Kernel methods
- Logistic regression
- Bayes
- SVMs

#### 2. Dataset

In the code of main.rs, I use the build in dataset in linfa to classify quality of wine into good and bad
```
let(train,valid) = linfa_datasets::winequality()
```

There are also two relatively big datasets in ```data``` folder: ```winequality-white.csv``` and ```winequality-red.csv```.


#### 3. Run the Training Model Locally

To run the training model, type:

```
cargo run
```
And the result is something like



### Upload Data Files on AWS S3
Create a bucket

Upload the file on AWS S3
