use crate::second::List;

#[test]
fn basics() {
    let mut list = List::new();
    assert_eq!(list.pop(), None);
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.pop(), Some(3));

    assert_eq!(list.pop(), Some(2));

    list.push(4);
    list.push(5);

    assert_eq!(list.pop(), Some(5));

    assert_eq!(list.pop(), Some(4));

    assert_eq!(list.pop(), Some(1));

    assert_eq!(list.pop(), None);
}
#[test]
fn peek() {
    let mut new_list = List::new();
    assert_eq!(new_list.peek(), None);
    assert_eq!(new_list.peek_mut(), None);

    new_list.push(1);
    new_list.push(2);
    new_list.push(3);

    assert_eq!(new_list.peek(), Some(&3));
    assert_eq!(new_list.peek_mut(), Some(&mut 3));
}

#[test]
fn into_iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
}

#[test]
fn iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));

    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}
#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));

    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}
