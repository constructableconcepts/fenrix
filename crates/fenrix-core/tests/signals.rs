use fenrix_core::create_signal;

#[test]
fn signal_creation_and_initial_value() {
    let (count, _) = create_signal(0);
    assert_eq!(count(), 0, "Signal should be created with the correct initial value.");
}

#[test]
fn signal_setter_updates_value() {
    let (count, set_count) = create_signal(0);
    set_count(10);
    assert_eq!(count(), 10, "Signal value should be updated by the setter.");
}

#[test]
fn signal_with_string_type() {
    let (text, set_text) = create_signal("hello".to_string());
    assert_eq!(text(), "hello");
    set_text("world".to_string());
    assert_eq!(text(), "world");
}

#[test]
fn multiple_cloned_signals_share_state() {
    let (count, set_count) = create_signal(0);
    let count2 = count.clone();
    let set_count2 = set_count.clone();

    set_count(5);
    assert_eq!(count(), 5, "Original getter should see the update.");
    assert_eq!(count2(), 5, "Cloned getter should also see the update.");

    set_count2(10);
    assert_eq!(count(), 10, "Update via cloned setter should be reflected in original getter.");
    assert_eq!(count2(), 10, "Update via cloned setter should be reflected in cloned getter.");
}