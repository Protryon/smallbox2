use smallbox2::*;

struct TestStruct {}

struct TestStruct2 {
    _data: [u8; 64],
}

trait TestTrait {
    fn thing(&self);
}

impl TestTrait for TestStruct {
    fn thing(&self) {
        println!("good");
    }
}

impl TestTrait for TestStruct2 {
    fn thing(&self) {
        println!("good");
    }
}

#[test]
fn test_stack_alloc() {
    let small_box = SmallBox32::new([0u8; 16]);
    assert_eq!(&*small_box, &[0u8; 16]);
}

#[test]
fn test_stackbox_alloc() {
    let small_box = StackBox32::new([0u8; 16]);
    assert_eq!(&*small_box, &[0u8; 16]);
}

#[test]
fn test_heap_alloc() {
    let small_box = SmallBox32::new([0u8; 48]);
    assert_eq!(&*small_box, &[0u8; 48]);
}

#[test]
fn test_stack_coerce() {
    let small_box = SmallBox32::new(TestStruct {});
    assert!(!small_box.is_heap());
    let small_box: SmallBox32<dyn TestTrait> = small_box;
    small_box.thing();
}

#[test]
fn test_stackbox_coerce() {
    let small_box = StackBox32::new(TestStruct {});
    assert!(!small_box.is_heap());
    let small_box: StackBox32<dyn TestTrait> = small_box;
    small_box.thing();
}

#[test]
fn test_heap_coerce() {
    let small_box = SmallBox32::new(TestStruct2 { _data: [0u8; 64] });
    assert!(small_box.is_heap());
    let small_box: SmallBox32<dyn TestTrait> = small_box;
    small_box.thing();
}
