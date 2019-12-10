use al::{Evaler, Slab, EvalNamespace, EmptyNamespace, FlatNamespace, ScopedNamespace, parse};
use al::evaler::bool_to_f64;

use kerr::KErr;

use std::mem;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn eval() {
    let mut slab = Slab::new();
    let mut ns = BTreeMap::<String,f64>::new();
    ns.insert("x".to_string(), 1.0);
    ns.insert("y".to_string(), 2.0);
    ns.insert("z".to_string(), 3.0);

    // Sanity check:
    assert_eq!(parse(&mut slab.ps,"3+3-3/3").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 5.0);

    assert_eq!(parse(&mut slab.ps,"x+y+z").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 6.0);

    assert_eq!(parse(&mut slab.ps,"x+y+z+a").unwrap().from(&slab.ps).eval(&slab, &mut ns), Err(KErr::new("variable undefined: a")));
}

#[test]
fn aaa_util() {
    assert_eq!(bool_to_f64(true), 1.0);
    assert_eq!(bool_to_f64(false), 0.0);
}

#[test]
fn aaa_aaa_sizes() {
    eprintln!("sizeof(Slab):{}", mem::size_of::<Slab>());
    assert!(mem::size_of::<Slab>()<2usize.pow(18));  // 256kB

}

#[test]
fn aaa_aab_single() {
    let mut slab = Slab::new();
    let mut ns = EmptyNamespace;
    assert_eq!(parse(&mut slab.ps, "123.456").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap(), 123.456f64);
}

#[test]
fn aaa_basics() {
    let mut slab = Slab::new();

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "12.34 + 43.21 + 11.11").unwrap().from(&slab.ps).var_names(&slab),
        BTreeSet::new());

    let mut ns = EmptyNamespace;
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "12.34 + 43.21 + 11.11").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(66.66));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "12.34 + 43.21 - 11.11").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(44.44));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "11.11 * 3").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(33.33));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "33.33 / 3").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(11.11));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "33.33 % 3").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.3299999999999983));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1 and 2").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.0));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1 && 2").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.0));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "2 or 0").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.0));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "2 || 0").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.0));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1 > 0").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.0));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1 < 0").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.0));

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "+5.5").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(5.5));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "-5.5").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(-5.5));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "!5.5").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.0));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "!0").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.0));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "(3 * 3 + 3 / 3)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(10.0));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "(3 * (3 + 3) / 3)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(6.0));

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "4.4 + -5.5").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(-1.0999999999999996));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "4.4 + +5.5").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(9.9));

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "x + 1").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Err(KErr::new("variable undefined: x")));

    let mut ns = FlatNamespace::new(|_,_| Some(3.0));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "x + 1").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.0));

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + int(3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.2));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + ceil(3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(5.2));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + floor(3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.2));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + abs(-3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.6));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + log(1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.2));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + log(10)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.2));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + log(0)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(std::f64::NEG_INFINITY));
    assert!(parse({slab.clear(); &mut slab.ps}, "1.2 + log(-1)").unwrap().from(&slab.ps).eval(&slab, &mut ns).unwrap().is_nan());
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + round(3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.2));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + round(0.5, 3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.7));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + round(-3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(-1.8));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + round(0.5, -3.4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(-2.3));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + min(1,2,0,3.3,-1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.19999999999999996));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + min(1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.2));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + max(1,2,0,3.3,-1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(4.5));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "1.2 + max(1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.2));

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, r#"12.34 + print ( 43.21, "yay" ) + 11.11"#).unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(66.66));

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "e()").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.718281828459045));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "pi()").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(3.141592653589793));

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "sin(pi()/2)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.0));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "cos(pi()/2)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.00000000000000006123233995736766));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "tan(pi()/4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.9999999999999999));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "asin(1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.5707963267948966));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "acos(0)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(1.5707963267948966));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "atan(1)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.7853981633974483));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "sinh(pi()/2)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.3012989023072947));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "cosh(pi()/2)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(2.5091784786580567));
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "tanh(pi()/4)").unwrap().from(&slab.ps).eval(&slab, &mut ns),
        Ok(0.6557942026326724));
}


#[derive(Debug)]
struct TestEvaler;
impl Evaler for TestEvaler {
    fn _var_names(&self, _slab:&Slab, _dst:&mut BTreeSet<String>) {}
    fn eval(&self, _slab:&Slab, ns:&mut impl EvalNamespace) -> Result<f64,KErr> {
        match ns.get_cached("x",vec![]) {
            Some(v) => Ok(v),
            None => Ok(1.23),
        }
    }
}

#[test]
fn aaa_evalns_basics() {
    let slab = Slab::new();
    let mut ns = ScopedNamespace::new(|_,_| Some(5.4321));
    assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 5.4321);
    ns.create_cached("x".to_string(),1.111).unwrap();
    assert_eq!(ns.eval_bubble(&slab, &TestEvaler{}).unwrap(), 1.111);
}

#[test]
fn corners() {
    let mut slab = Slab::new();
    let mut ns = EmptyNamespace;
    assert_eq!(
        format!("{:?}", parse({slab.clear(); &mut slab.ps}, "(-1) ^ 0.5").unwrap().from(&slab.ps).eval(&slab, &mut ns)),
        "Ok(NaN)");
}

fn my_evalns_cb_function(_:&str, _:Vec<f64>) -> Option<f64> { None }
#[test]
fn evalns_cb_ownership() {
    let _ns = FlatNamespace::new(my_evalns_cb_function);
    let _ns = FlatNamespace::new(my_evalns_cb_function);
    // Conclusion: You can pass a function pointer into a function that receives ownership.

    let closure = |_:&str, _:Vec<f64>| None;
    let _ns = FlatNamespace::new(closure);
    let _ns = FlatNamespace::new(closure);

    let x = 1.0;
    let closure = |_:&str, _:Vec<f64>| Some(x);
    let _ns = FlatNamespace::new(closure);
    let _ns = FlatNamespace::new(closure);

    let mut x = 1.0;
    let closure = |_:&str, _:Vec<f64>| {
        x+=1.0;
        Some(x)
    };
    let _ns = FlatNamespace::new(closure);
    //let _ns = FlatNamespace::new(closure);  // Not allowed.

    // Conclusion: Functions and Closures that don't mutate state are effectively Copy.
    //             Closures that mutate state aren't Copy.
    //             Note that the argument type (FnMut vs Fn) doesn't actually matter,
    //             just the implementation matters!
}

#[test]
fn custom_func() {
    let mut slab = Slab::new();
    let mut ns = FlatNamespace::new(|name,args| {
        eprintln!("In CB: {}",name);
        match name {
            "x" => Some(1.0),
            "y" => Some(2.0),
            "z" => Some(3.0),
            "foo" => {
                Some(args.get(0).unwrap_or(&std::f64::NAN)*10.0)
            }
            "bar" => {
                Some(args.get(0).unwrap_or(&std::f64::NAN) + args.get(1).unwrap_or(&std::f64::NAN))
            }
            _ => None,
        }
    });
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "x + 1.5").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns}),
        Ok(2.5));

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "x() + 1.5").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns}),
        Ok(2.5));

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "x(1,2,3) + 1.5").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns}),
        Ok(2.5));

    eprintln!("I should see TWO x lookups, 1 y, and 1 z:");
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "x(x,y,z) + 1.5").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns}),
        Ok(2.5));

    eprintln!("I should see TWO x lookups:");
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "x(x,x,x) + 1.5").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns}),
        Ok(2.5));

    eprintln!("I should see TWO x lookups:");
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "x(1.0) + x(1.1) + x(1.0) + x(1.1)").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns}),
        Ok(4.0));

    eprintln!("---------------------------");

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "foo(1.23)").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns}),
        Ok(12.3));

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "bar(1.23, 3.21)").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns}),
        Ok(4.4399999999999995));

    assert_eq!(
        format!("{:?}", parse({slab.clear(); &mut slab.ps}, "bar(1.23)").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns})),
        "Ok(NaN)");
}

#[test]
#[cfg(feature="unsafe-vars")]
fn unsafe_var() {
    let mut slab = Slab::new();

    let mut ua = 1.23;
    let mut ub = 4.56;
    unsafe {
        slab.ps.add_unsafe_var("ua".to_string(), &ua);
        slab.ps.add_unsafe_var("ub".to_string(), &ub);
    }

    let mut ns = EmptyNamespace;

    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "ua + ub + 5").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns}),
        Ok(10.79));

    ua+=1.0;
    ub+=2.0;
    assert_eq!(
        parse({slab.clear(); &mut slab.ps}, "ua + ub + 5").unwrap().from(&slab.ps).eval(&slab, {ns.clear_cached(); &mut ns}),
        Ok(13.79));

    let _ = (ua,ub);  // Silence compiler warnings about variables not being read.
}

