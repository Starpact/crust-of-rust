fn bar<T>(_: u32) {}

fn accept_fn_pointer(f: fn(u32)) {
    f(1);
}

fn accept_fn<F: Fn(u32)>(f: F) {
    f(1);
}

fn accept_fn_mut<F: FnMut(u32)>(mut f: F) {
    f(1);
}

fn accept_fn_once<F: FnOnce(u32)>(f: F) {
    f(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_item_is_zero_size() {
        println!("-------type: {}", std::any::type_name_of_val(&bar::<i32>)); // bar<i32>
        assert_eq!(std::mem::size_of_val(&bar::<i32>), 0);
        assert_eq!(std::mem::size_of_val(&bar::<u32>), 0);
    }

    #[test]
    fn fn_pointer_is_not_zero_size() {
        let f: fn(u32) = bar::<i32>;
        println!("-------type: {}", std::any::type_name_of_val(&f)); // fn(u32)
        assert_eq!(std::mem::size_of_val(&f), 8);

        accept_fn_pointer(f);
        accept_fn_pointer(bar::<i32>);
        accept_fn(bar::<i32>);
        accept_fn_mut(bar::<i32>);
        accept_fn_once(bar::<i32>);
    }

    #[test]
    fn closure_capturing_nothing() {
        let f = |_| {
            let _ = 1;
        };
        println!("-------type: {}", std::any::type_name_of_val(&f)); // _::{{closure}}
        assert_eq!(std::mem::size_of_val(&f), 0);

        accept_fn_pointer(f);
        accept_fn(f);
        accept_fn_mut(f);
        accept_fn_once(f);
    }

    #[test]
    fn closure_capturing_shared_reference() {
        let s = "x".to_owned();
        let f = |_| {
            let _ = s;
        };
        println!("-------type: {}", std::any::type_name_of_val(&f)); // _::{{closure}}
        assert_eq!(std::mem::size_of_val(&f), 8);

        // accept_fn_pointer(f); // cannot accept closure.
        accept_fn(f);
        accept_fn_mut(f);
        accept_fn_once(f);
    }

    #[test]
    fn closure_capturing_mutable_reference() {
        let mut s = "x".to_owned();
        let mut f = |_| {
            s.clear();
        };
        println!("-------type: {}", std::any::type_name_of_val(&f)); // _::{{closure}}
        assert_eq!(std::mem::size_of_val(&f), 8);

        // accept_fn_pointer(f); // cannot accept closure.
        // accept_fn(f); // not impl Fn.
        accept_fn_mut(&mut f);
        accept_fn_once(f);
    }

    #[test]
    fn closure_capturing_ownership_consume() {
        let s = "x".to_owned();
        let f = |_| drop(s);
        println!("-------type: {}", std::any::type_name_of_val(&f)); // _::{{closure}};
        assert_eq!(std::mem::size_of_val(&f), 24);

        // accept_fn(f);
        // accept_fn_mut(f);
        accept_fn_once(f);
    }

    fn only_use_reference_but_need_to_capture_ownership() -> impl Fn() {
        let s = "".to_owned();
        move || println!("{}", s)
    }
}

