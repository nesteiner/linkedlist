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

    for i in list.iter_mut() {
	*i *= 2;
    }

    for i in list.iter() {
	println!("{}", i);
    }

}

#[test]
fn test_advancefunc() {
    let mut list = List::new();
    for i in 1..=9 {
	list.pushback(i);
    }

    println!("{}", list.iter().fold(0, |r, x| r + x));

    for i in list.iter().map(|x| x * 2) {
	println!("{}", i);
    }
}