// tests/main.rs
use practice::add;
use log::info;  // Only import 'info' since 'debug' is not being used

#[test]
fn test_add() {
    // Initialize logger
    let _ = env_logger::builder().is_test(true).try_init();

    info!("Starting test_add function...");

    let result = add(2, 3);

    info!("Result of add(2, 3): {}", result);

    assert_eq!(result, 5);

    info!("Test passed!");
}
