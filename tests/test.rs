
extern crate retain_mut;

use retain_mut::RetainMut;

#[test]
fn retain_mut_unordered(){

    let mut v=vec!(0,1,4,2,5,8,9,8);

    v.retain_mut_unordered(|&mut a|a%2==0);
    assert_eq!(v,vec!(4,5,2));
}