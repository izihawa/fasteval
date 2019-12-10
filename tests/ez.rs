use al::ez_eval;

use kerr::KErr;

use std::collections::BTreeMap;

#[test]
fn ez() {
    assert_eq!(ez_eval("3+3-3/3", &mut BTreeMap::new()), Ok(5.0));
    assert_eq!(ez_eval("3abc+3-3/3", &mut BTreeMap::new()), Err(KErr::new("unparsed tokens remaining")));
    assert_eq!(ez_eval("z+z-z/z", &mut {let mut m=BTreeMap::new(); m.insert("x".to_string(),1.0); m.insert("y".to_string(),2.0); m.insert("z".to_string(),3.0); m}), Ok(5.0));
}

