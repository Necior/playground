macro_rules! list_compr {
    ($e:expr; for $id:ident in $collection:ident) => {{
        $collection.iter().map(|$id| $e).collect()
    }};
    ($e:expr; for $id:ident in $collection:ident if $cond:expr) => {{
        $collection
            .iter()
            .filter(|&&$id| $cond)
            .map(|&$id| $e)
            .collect()
    }};
    ($e:expr; for $id:ident in $collection:ident if $cond:expr; else $e2:expr) => {{
        $collection
            .iter()
            .map(|&$id| match $cond {
                true => $e,
                false => $e2,
            })
            .collect()
    }};
}

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4];

    let result: Vec<_> = list_compr![y*y; for y in numbers];
    assert_eq!(result, vec![1, 4, 9, 16]);

    let result: Vec<_> = list_compr![y; for y in numbers if y % 2 == 0];
    assert_eq!(result, vec![2, 4]);

    let result: Vec<_> = list_compr![y*y; for y in numbers if y % 2 == 0];
    assert_eq!(result, vec![4, 16]);

    let result: Vec<_> = list_compr![y*y; for y in numbers if y >= 3; else y];
    assert_eq!(result, vec![1, 2, 9, 16]);
}
