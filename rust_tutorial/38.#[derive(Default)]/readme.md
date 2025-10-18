目标：
1. 理解#[derive(Default)]的作用


内容：
1. 作用

The #[derive(Default)] attribute in Rust automatically implements the Default trait for the struct. This means you can create a new StepComputations instance with default values for all fields by calling StepComputations::default(). The default values are determined by the default for each field's type (e.g., 0 for numbers, false for booleans)


