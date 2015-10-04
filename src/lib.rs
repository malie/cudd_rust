#[link(name="cudd")]

use std::mem;

#[allow(unused_imports)]
use std::ptr::null_mut;

#[repr(C)]
pub struct Mn { ___x:u32 }

#[repr(C)]
pub struct Node { ___x:u32 }

#[repr(C)]
pub struct Gen { ___x:u32 }

extern {
    pub fn Cudd_Init(numVars:u32, numVarsZ:u32, numSlots:u32,
                     cacheSize:u32, maxMemory:u32) -> *mut Mn;
    pub fn Cudd_ReadLogicZero(mn:*mut Mn) -> *mut Node;
    pub fn Cudd_bddIthVar(mn:*mut Mn, n:u32) -> *mut Node;
    pub fn Cudd_bddAnd(mn:*mut Mn,
                       a:*mut Node, b:*mut Node) -> *mut Node;
    pub fn Cudd_bddOr(mn:*mut Mn,
                      a:*mut Node, b:*mut Node) -> *mut Node;
    pub fn Cudd_bddXor(mn:*mut Mn,
                       a:*mut Node, b:*mut Node) -> *mut Node;
    pub fn Cudd_bddNand(mn:*mut Mn,
                        a:*mut Node, b:*mut Node) -> *mut Node;
    pub fn Cudd_bddNor(mn:*mut Mn,
                       a:*mut Node, b:*mut Node) -> *mut Node;
    pub fn Cudd_bddXnor(mn:*mut Mn,
                        a:*mut Node, b:*mut Node) -> *mut Node;
    pub fn Cudd_bddIte(mn:*mut Mn, a:*mut Node,
                       b:*mut Node, c:*mut Node) -> *mut Node;
    pub fn Cudd_PrintDebug(mn:*mut Mn, a:*mut Node,
                           n:u32, pr:u32) -> u32;
    pub fn Cudd_ReadSize(a:*mut Mn) -> u32;
    pub fn Cudd_DagSize(a:*mut Node) -> u32;
    pub fn Cudd_Ref(a:*mut Node) -> ();
    pub fn Cudd_RecursiveDeref(mn:*mut Mn, a:*mut Node) -> ();
    pub fn Cudd_ReadSlots(a:*mut Mn) -> f64;
    pub fn Cudd_ReadUsedSlots(a:*mut Mn) -> f64;

    pub fn Cudd_FirstCube(mn:*mut Mn, n:*mut Node, cube:*mut *mut u32,
                          value:*mut f64) -> *mut Gen;
    pub fn Cudd_IsGenEmpty(gen:*mut Gen) -> u32;
    pub fn Cudd_GenFree(gen:*mut Gen) -> u32;
    pub fn Cudd_NextCube(gen:*mut Gen, cube:*mut *mut u32,
                         value:*mut f64) -> u32;
}

pub fn not(a:*mut Node) -> *mut Node {
    unsafe {
        // Cudd_bddXnor(mn, a, Cudd_ReadLogicZero(mn))
        let p:u64 = mem::transmute(a);
        let q = p ^ 1;
        let res:*mut Node = mem::transmute(q);
        res
    }
}

#[test]
fn it_works() {
    unsafe {
        let mn = Cudd_Init(0, 0, 256, 262144, 0);
        let z = Cudd_ReadLogicZero(mn);
        println!("\nzero: {:?}", z);
        
        let a = Cudd_bddIthVar(mn, 0);
        let b = Cudd_bddIthVar(mn, 1);
        let c = Cudd_bddIthVar(mn, 2);
        let d = Cudd_bddIthVar(mn, 3);

        let ab = Cudd_bddOr(mn, a, not(b));
        Cudd_Ref(ab);
            
        let bc = Cudd_bddNand(mn, b, c);
        Cudd_Ref(bc);
        
        let ad = Cudd_bddOr(mn, not(a), not(d));
        Cudd_Ref(ad);
        
        let cd = Cudd_bddOr(mn, not(c), not(d));
        Cudd_Ref(cd);

        let res = Cudd_bddAnd(mn,
                              Cudd_bddAnd(mn, ab, bc),
                              Cudd_bddAnd(mn, ad, cd));
        Cudd_RecursiveDeref(mn, ab);
        Cudd_RecursiveDeref(mn, bc);
        Cudd_RecursiveDeref(mn, ad);
        Cudd_RecursiveDeref(mn, cd);
        
        Cudd_PrintDebug(mn, res, 1000, 2);
        // TODO: prints "4.6e300 minterms", what?

        let numvars = Cudd_ReadSize(mn);
        let mut cube : *mut u32 = null_mut();
        let mut value = 0.0f64;
        let mut gen = Cudd_FirstCube(mn, res, &mut cube, &mut value);

        let mut i = 0;
        loop {
            if Cudd_IsGenEmpty(gen) != 0 {
                Cudd_GenFree(gen);
                break}

            let cub = std::slice::from_raw_parts(cube,
                                                 numvars as usize);
            print!("\ncube {}:  {:?}", i, cub);
            
            Cudd_NextCube(gen, &mut cube, &mut value);
            i += 1;
        }
        
        println!("\nnum vars: {:?}", Cudd_ReadSize(mn));
        println!("num nodes: {:?}", Cudd_DagSize(res));
        println!("num slots: {:?}", Cudd_ReadSlots(mn));
        println!("used slots: {:?}", Cudd_ReadUsedSlots(mn));
        Cudd_RecursiveDeref(mn, res);
        println!("used slots: {:?}", Cudd_ReadUsedSlots(mn));
        
        println!("ok");
        panic!();
    }
}

