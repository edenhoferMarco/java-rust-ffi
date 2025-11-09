pub fn hello_from_rust() {
    println!("Hello from Rust!");
}

pub enum CalculationType {
    Add,
    Subtract,
}

pub enum CalculationHandle {
    AddHandle(Vec<i32>),
    SubtractHandle(Vec<i32>),
}
pub fn get_calculation_handle(calc_type: CalculationType, inputs: Vec<i32>) -> CalculationHandle {
    match calc_type {
        CalculationType::Add => CalculationHandle::AddHandle(inputs),
        CalculationType::Subtract => CalculationHandle::SubtractHandle(inputs),
    }
}

pub fn perform_calculation(handle: CalculationHandle) -> i32 {
    match handle {
        CalculationHandle::AddHandle(inputs) => inputs.iter().sum(),
        CalculationHandle::SubtractHandle(inputs) => {
            let mut iter = inputs.into_iter();
            let first = iter.next().unwrap_or(0);
            iter.fold(first, |acc, x| acc - x)
        }
    }
}

