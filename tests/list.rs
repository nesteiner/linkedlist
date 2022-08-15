use list::list::List;

#[test]
fn test_list() {
    let mut list = List::new();
    for i in 1..=9 {
	list.pushback(i);
    }

    for i in list.iter() {
	println!("{}", i);
    }

    for i in 10..=19 {
	list.pushfront(i);
    }

    for i in list.iter() {
	println!("{}", i);
    }

    assert_eq!(list.popfront(), Some(19));
    assert_eq!(list.popfront(), Some(18));
}