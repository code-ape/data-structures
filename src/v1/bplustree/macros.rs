
macro_rules! void_ptr_to_ref {
    ($x:expr) => ({
        &*mem::transmute::<*mut Void, *mut _>($x)
    });
}

macro_rules! void_ptr_to_mut_ref {
    ($x:expr) => ({
        &mut *mem::transmute::<*mut Void, *mut _>($x)
    });
}

macro_rules! mut_ptr_to_void_ptr {
    ($x:expr) => ({
        mem::transmute::<*mut _,*mut Void>($x)
    });    
}


macro_rules! debug {
    ($($arg:tt)*) => (
        if cfg!(test) {
            println!($($arg)*);
        }
    )
}
