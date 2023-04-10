use crate::{prelude::*, storage::Static, LocalRb};

fn head_tail(observer: &impl crate::raw::RawBase) -> (usize, usize) {
    (observer.read_end(), observer.write_end())
}

#[test]
fn capacity() {
    const CAP: usize = 13;
    let rb = LocalRb::<Static<i32, CAP>>::default();
    assert_eq!(rb.capacity(), CAP);
}
#[test]
fn split_capacity() {
    const CAP: usize = 13;
    let mut rb = LocalRb::<Static<i32, CAP>>::default();
    let (prod, cons) = rb.split_ref();

    assert_eq!(prod.capacity(), CAP);
    assert_eq!(cons.capacity(), CAP);
}

#[test]
fn try_push() {
    let mut rb = LocalRb::<Static<i32, 2>>::default();
    let (mut prod, _) = rb.split_ref();

    assert_eq!(head_tail(&prod), (0, 0));

    assert_eq!(prod.try_push(123), Ok(()));
    assert_eq!(head_tail(&prod), (0, 1));

    assert_eq!(prod.try_push(234), Ok(()));
    assert_eq!(head_tail(&prod), (0, 2));

    assert_eq!(prod.try_push(345), Err(345));
    assert_eq!(head_tail(&prod), (0, 2));
}

#[test]
fn pop_empty() {
    let mut rb = LocalRb::<Static<i32, 2>>::default();
    let (_, mut cons) = rb.split_ref();

    assert_eq!(head_tail(&cons), (0, 0));

    assert_eq!(cons.try_pop(), None);
    assert_eq!(head_tail(&cons), (0, 0));
}

#[test]
fn push_pop_one() {
    const CAP: usize = 2;
    let mut rb = LocalRb::<Static<i32, CAP>>::default();
    let (mut prod, mut cons) = rb.split_ref();

    const MOD: usize = 2 * CAP;
    let values = [12, 34, 56, 78, 90];
    assert_eq!(head_tail(&cons), (0, 0));

    for (i, v) in values.iter().enumerate() {
        assert_eq!(prod.try_push(*v), Ok(()));
        assert_eq!(head_tail(&cons), (i % MOD, (i + 1) % MOD));

        assert_eq!(cons.try_pop().unwrap(), *v);
        assert_eq!(head_tail(&cons), ((i + 1) % MOD, (i + 1) % MOD));

        assert_eq!(cons.try_pop(), None);
        assert_eq!(head_tail(&cons), ((i + 1) % MOD, (i + 1) % MOD));
    }
}

#[test]
fn push_pop_all() {
    const CAP: usize = 2;
    let mut rb = LocalRb::<Static<i32, CAP>>::default();
    let (mut prod, mut cons) = rb.split_ref();

    const MOD: usize = 2 * CAP;
    let values = [(12, 34, 13), (56, 78, 57), (90, 10, 91)];
    assert_eq!(head_tail(&cons), (0, 0));

    for (i, v) in values.iter().enumerate() {
        assert_eq!(prod.try_push(v.0), Ok(()));
        assert_eq!(head_tail(&cons), (CAP * i % MOD, (CAP * i + 1) % MOD));

        assert_eq!(prod.try_push(v.1), Ok(()));
        assert_eq!(head_tail(&cons), (CAP * i % MOD, (CAP * i + 2) % MOD));

        assert_eq!(prod.try_push(v.2).unwrap_err(), v.2);
        assert_eq!(head_tail(&cons), (CAP * i % MOD, (CAP * i + 2) % MOD));

        assert_eq!(cons.try_pop().unwrap(), v.0);
        assert_eq!(head_tail(&cons), ((CAP * i + 1) % MOD, (CAP * i + 2) % MOD));

        assert_eq!(cons.try_pop().unwrap(), v.1);
        assert_eq!(head_tail(&cons), ((CAP * i + 2) % MOD, (CAP * i + 2) % MOD));

        assert_eq!(cons.try_pop(), None);
        assert_eq!(head_tail(&cons), ((CAP * i + 2) % MOD, (CAP * i + 2) % MOD));
    }
}

#[test]
fn empty_full() {
    let mut rb = LocalRb::<Static<i32, 1>>::default();
    let (mut prod, cons) = rb.split_ref();

    assert!(prod.is_empty());
    assert!(cons.is_empty());
    assert!(!prod.is_full());
    assert!(!cons.is_full());

    assert_eq!(prod.try_push(123), Ok(()));

    assert!(!prod.is_empty());
    assert!(!cons.is_empty());
    assert!(prod.is_full());
    assert!(cons.is_full());
}

#[test]
fn len_remaining() {
    let mut rb = LocalRb::<Static<i32, 2>>::default();
    let (mut prod, mut cons) = rb.split_ref();

    assert_eq!(prod.occupied_len(), 0);
    assert_eq!(cons.occupied_len(), 0);
    assert_eq!(prod.vacant_len(), 2);
    assert_eq!(cons.vacant_len(), 2);

    assert_eq!(prod.try_push(123), Ok(()));

    assert_eq!(prod.occupied_len(), 1);
    assert_eq!(cons.occupied_len(), 1);
    assert_eq!(prod.vacant_len(), 1);
    assert_eq!(cons.vacant_len(), 1);

    assert_eq!(prod.try_push(456), Ok(()));

    assert_eq!(prod.occupied_len(), 2);
    assert_eq!(cons.occupied_len(), 2);
    assert_eq!(prod.vacant_len(), 0);
    assert_eq!(cons.vacant_len(), 0);

    assert_eq!(cons.try_pop(), Some(123));

    assert_eq!(prod.occupied_len(), 1);
    assert_eq!(cons.occupied_len(), 1);
    assert_eq!(prod.vacant_len(), 1);
    assert_eq!(cons.vacant_len(), 1);

    assert_eq!(cons.try_pop(), Some(456));

    assert_eq!(prod.occupied_len(), 0);
    assert_eq!(cons.occupied_len(), 0);
    assert_eq!(prod.vacant_len(), 2);
    assert_eq!(cons.vacant_len(), 2);

    assert_eq!(prod.try_push(789), Ok(()));

    assert_eq!(prod.occupied_len(), 1);
    assert_eq!(cons.occupied_len(), 1);
    assert_eq!(prod.vacant_len(), 1);
    assert_eq!(cons.vacant_len(), 1);
}
