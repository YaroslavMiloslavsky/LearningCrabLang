mod common;

#[test]
fn test_process_successful() {
    let processor = common::create_processor_email_max_len_20();
    let mut test_string = "example@example.com".to_string();

    let result = processor.process(&mut test_string);
    println!("{result:?}");
    assert!(result.is_ok());
}

#[test]
fn test_process_fail_too_long_email() {
    let processor = common::create_processor_email_max_len_20();
    let mut test_string = "exampasdsadasdasdle@example.com".to_string();

    let result = processor.process(&mut test_string);
    println!("{result:?}");
    assert!(result.is_err());
}

#[test]
fn test_process_fail_empty_string() {
    let processor = common::create_processor_email_max_len_20();
    let mut test_string = "".to_string();

    let result = processor.process(&mut test_string);
    println!("{result:?}");
    assert!(result.is_err());
}

#[test]
fn test_process_pass_too_many_spaces_bu_ok_email() {
    let processor = common::create_processor_email_max_len_20();
    let mut test_string = "                                     asdle@example.com                          ".to_string();

    let result = processor.process(&mut test_string);
    println!("{result:?}");
    assert!(result.is_ok());
}

#[test]
fn test_process_fail_not_an_email() {
    let processor = common::create_processor_email_max_len_20();
    let mut test_string = "                                     asdlexample.com                          ".to_string();

    let result = processor.process(&mut test_string);
    println!("{result:?}");
    assert!(result.is_err());
}
