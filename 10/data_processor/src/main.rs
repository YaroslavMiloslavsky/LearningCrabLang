#![warn(clippy::all, clippy::pedantic)]
#![allow(unused)]

use crate::data_pipeline::DataProcessor;
use crate::datasource::DataSource;
use crate::pipeline::{
    CollectVecAggregator, EvenNumberFilter, MultiplyByTwoTransformer, StringContainsFilter,
    StringToUpperTransformer, SumAggregator,
};

fn main() {
    println!("====================== Numbers Pipeline ======================");
    let some_data = [1, 2, 3, 4];
    let datasource = DataSource::new(&some_data);
    let data_filter = DataProcessor::new(
        EvenNumberFilter::new(),
        MultiplyByTwoTransformer::new(),
        SumAggregator::new(),
    );
    match data_filter.process_data(&datasource) {
        Ok(data) => println!("Data processor has the output: {data}"),
        Err(e) => println!("Could not process the data. Reason: {e:?}"),
    }
    println!("\n\n====================== Strings Pipeline ======================");
    let some_data = ["abba", "abca", "cacd", "asab", "afav"];
    let datasource = DataSource::new(&some_data);
    let data_filter = DataProcessor::new(
        StringContainsFilter::new("ab"),
        StringToUpperTransformer::new(),
        CollectVecAggregator::new(),
    );
    match data_filter.process_data(&datasource) {
        Ok(data) => println!("Data processor has the output: {data:?}"),
        Err(e) => println!("Could not process the data. Reason: {e:?}"),
    }
}

mod errors {
    #[derive(Debug)]
    pub enum PipelineError {
        TransformError(String),
        AggregatorError(String),
    }
}

// This would sit in a separate file like data_pipeline.rs or data_pipeline/mod.rs
mod data_pipeline {
    use crate::datasource::DataSource;
    use crate::errors::PipelineError;
    use crate::pipeline::{Aggregator, Filter, Transformer};
    use std::fmt::Debug;
    use std::marker::PhantomData;

    pub struct DataProcessor<T, F, U, V, A, P>
    where
        T: Debug,
        U: Debug,
        F: Filter<T> + Debug,
        P: Transformer<T, U> + Debug,
        A: Aggregator<U, V> + Debug,
    {
        filter: F,
        transformer: P,
        aggregator: A,
        _phantom: PhantomData<(T, U, V)>,
    }

    impl<T, F, U, V, A, P> DataProcessor<T, F, U, V, A, P>
    where
        T: Debug,
        U: Debug,
        F: Filter<T> + Debug,
        P: Transformer<T, U> + Debug,
        A: Aggregator<U, V> + Debug,
    {
        pub fn new(filter: F, transformer: P, aggregator: A) -> Self {
            Self {
                filter,
                transformer,
                aggregator,
                _phantom: PhantomData,
            }
        }

        fn filter<'a>(&self, data: &'a [T]) -> Vec<&'a T> {
            let data_len: usize = data.len();
            let mut filtered_data: Vec<&T> = Vec::with_capacity(data_len);

            for entry in data {
                if self.filter.filter(entry) {
                    filtered_data.push(entry);
                }
            }

            filtered_data
        }

        fn transform(&self, data: &[&T]) -> Result<Vec<U>, PipelineError> {
            let mut transformed_data: Vec<U> = Vec::with_capacity(data.len());
            for entry in data {
                let Some(transformed_entry) = self.transformer.transform(entry) else {
                    return Err(PipelineError::TransformError(format!(
                        "Could not transform {entry:?} using {:?}",
                        self.transformer
                    )));
                };
                transformed_data.push(transformed_entry);
            }

            Ok(transformed_data)
        }

        fn aggregate(&self, data: &[U]) -> Result<V, PipelineError> {
            if let Some(output) = self.aggregator.aggregate(data) {
                Ok(output)
            } else {
                Err(PipelineError::AggregatorError(format!(
                    "Couldn't aggregate data {data:?} using {:?}",
                    self.aggregator
                )))
            }
        }

        // 0 clone calls ;)
        pub fn process_data(&self, source: &DataSource<T>) -> Result<V, PipelineError> {
            let data = source.get_data();
            let filtered_data = self.filter(data);
            println!("Filtered data with {:?}: {:?}", self.filter, filtered_data);

            let transformed_data = self.transform(&filtered_data)?;
            println!(
                "Transformed data with {:?}: {:?}",
                self.transformer, transformed_data
            );

            self.aggregate(&transformed_data)
        }
    }
}

// This would sit in a separate file like pipeline.rs or pipeline/mod.rs
mod pipeline {

    use std::fmt::Debug;
    use std::marker::PhantomData;

    pub trait Filter<T: Debug> {
        fn filter(&self, item: &T) -> bool;
    }

    pub trait Transformer<T, U> {
        fn transform(&self, item: &T) -> Option<U>;
    }

    pub trait Aggregator<U, V> {
        fn aggregate(&self, data: &[U]) -> Option<V>;
    }

    #[derive(Debug)]
    pub struct CollectVecAggregator<T: Clone> {
        _marker: PhantomData<T>,
    }

    impl<T: Clone> CollectVecAggregator<T> {
        pub fn new() -> Self {
            Self {
                _marker: PhantomData,
            }
        }
    }

    impl<T: Clone> Aggregator<T, Vec<T>> for CollectVecAggregator<T> {
        fn aggregate(&self, data: &[T]) -> Option<Vec<T>> {
            Some(data.to_vec())
        }
    }

    #[derive(Debug)]
    pub struct StringToUpperTransformer {}

    impl StringToUpperTransformer {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Transformer<&str, String> for StringToUpperTransformer {
        fn transform(&self, item: &&str) -> Option<String> {
            if item.is_empty() {
                None
            } else {
                Some(item.to_uppercase().to_string())
            }
        }
    }

    #[derive(Debug)]
    pub struct StringContainsFilter<'a> {
        pattern: &'a str,
    }

    impl<'a> StringContainsFilter<'a> {
        pub fn new(pattern: &'a str) -> Self {
            Self { pattern }
        }
    }

    impl<'a> Filter<&'a str> for StringContainsFilter<'a> {
        fn filter(&self, item: &&'a str) -> bool {
            item.contains(self.pattern)
        }
    }

    #[derive(Debug)]
    pub struct EvenNumberFilter {}

    impl EvenNumberFilter {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Filter<u32> for EvenNumberFilter {
        fn filter(&self, item: &u32) -> bool {
            (*item & 0b0001) == 0
        }
    }

    #[derive(Debug)]
    pub struct MultiplyByTwoTransformer {}

    impl MultiplyByTwoTransformer {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Transformer<u32, u32> for MultiplyByTwoTransformer {
        fn transform(&self, item: &u32) -> Option<u32> {
            Some(item.saturating_mul(2))
        }
    }

    #[derive(Debug)]
    pub struct SumAggregator {}

    impl SumAggregator {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Aggregator<u32, u32> for SumAggregator {
        fn aggregate(&self, data: &[u32]) -> Option<u32> {
            if data.is_empty() {
                None
            } else {
                let mut sum_aggregation = 0;
                for num in data {
                    sum_aggregation += *num;
                }
                Some(sum_aggregation)
            }
        }
    }
}

// This would sit in a separate file like data.rs or data/mod.rs
mod datasource {
    use std::fmt::Debug;

    pub struct DataSource<'a, T>
    where
        T: Debug,
    {
        data: &'a [T],
    }

    impl<'a, T> DataSource<'a, T>
    where
        T: Debug,
    {
        pub fn new(data: &'a [T]) -> Self {
            Self { data }
        }

        // Third rule of elision, thus not needing to declare explicit lifetime 'a
        pub fn get_data(&self) -> &[T] {
            self.data
        }
    }
}

/*
Conceptual Questions
The code speaks for itself
*/