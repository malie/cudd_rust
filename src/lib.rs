#[link(name="cudd")]
#[repr(C)]
pub struct Mn { ___x:u32 }

#[repr(C)]
pub struct Node { ___x:u32 }

extern {
    pub fn Cudd_Init(numVars:u64, numVarsZ:u64, numSlots:u64,
                     cacheSize:u64, maxMemory:u64) -> *mut Mn;
    pub fn Cudd_ReadLogicZero(mn:*mut Mn) -> *mut Node;
    pub fn Cudd_bddIthVar(mn:*mut Mn, n:u64) -> *mut Node;
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
                           n:u64, pr:u64) -> u64;
}

pub fn not(mn:*mut Mn, a:*mut Node) -> *mut Node {
    // TODO: should be "toggle lowest bit in pointer"
    unsafe {
        Cudd_bddXnor(mn, a, Cudd_ReadLogicZero(mn))
    }
}

#[test]
fn it_works() {
    unsafe {
        let mn = Cudd_Init(0, 0, 256, 262144, 0);
        let z = Cudd_ReadLogicZero(mn);
        let a = Cudd_bddIthVar(mn, 0);
        let b = Cudd_bddIthVar(mn, 1);
        let c = Cudd_bddIthVar(mn, 2);
        let d = Cudd_bddIthVar(mn, 3);

        let ab = Cudd_bddOr(mn, a, not(mn, b));
        let bc = Cudd_bddNand(mn, b, c);
        let ad = Cudd_bddOr(mn, not(mn, a), not(mn, d));
        let res = Cudd_bddAnd(mn,
                              Cudd_bddAnd(mn, ab, bc),
                              ad);
        
        Cudd_PrintDebug(mn, res, 1000, 2);
        // TODO: prints "4.6e300 minterms", what?

        println!("zero: {:?}", z);
        println!("ok");
        // panic!();
    }
}

