
pub const N: usize = 5;
pub const MASK: u8 = 2_u8.pow(N as u32) - 1;

pub const fn factorial(n: usize) -> usize {
    if n == 1 || n == 0 {
        return 1
    }

    n * factorial(n-1)
}

pub const CYCLES: usize = factorial((N as usize) - 1);

pub const MAX: usize = factorial(N as usize) 
    + factorial((N-1) as usize) 
    + factorial((N-2) as usize) 
    + factorial((N-3) as usize) + (N as usize) - 3;
