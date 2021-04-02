//! A rust implementation of MurmurHash3.

mod hash128 {
    use core::ptr;
    use core::{hash::Hasher, slice};

    const C1: u64 = 0x87c37b91114253d5;
    const C2: u64 = 0x4cf5ad432745937f;
    const C3: u64 = 0x52dce729;
    const C4: u64 = 0x38495ab5;

    pub fn murmurhash3_x64_128(bytes: &[u8], seed: u32) -> (u64, u64) {
        let nblocks = bytes.len() / 16;

        let mut h1 = seed as u64;
        let mut h2 = seed as u64;

        let mut start = bytes.as_ptr();
        for _ in 0..nblocks {
            let (k1, k2) = unsafe {
                let k1 = ptr::read_unaligned(start as *const u64);
                start = start.add(8);
                let k2 = ptr::read_unaligned(start as *const u64);
                start = start.add(8);
                (u64::from_le(k1), u64::from_le(k2))
            };
            let res = feed128(h1, h2, k1, k2);
            h1 = res.0;
            h2 = res.1;
        }

        unsafe {
            finish_tail128(
                start as *const u8,
                bytes.as_ptr().add(bytes.len()),
                bytes.len() as u64,
                h1,
                h2,
            )
        }
    }

    #[inline]
    fn fmix64(mut k: u64) -> u64 {
        k ^= k >> 33;
        k *= 0xff51afd7ed558ccd;
        k ^= k >> 33;
        k *= 0xc4ceb9fe1a85ec53;
        k ^ (k >> 33)
    }

    #[inline]
    fn feed128(mut h1: u64, mut h2: u64, mut k1: u64, mut k2: u64) -> (u64, u64) {
        k1 *= C1;
        k1 = k1.rotate_left(31);
        k1 *= C2;

        h1 ^= k1;
        h1 = h1.rotate_left(27);
        h1 += h2;
        h1 = h1 * 5 + C3;

        k2 *= C2;
        k2 = k2.rotate_left(33);
        k2 *= C1;

        h2 ^= k2;
        h2 = h2.rotate_left(31);
        h2 += h1;
        h2 = h2 * 5 + C4;

        (h1, h2)
    }

    #[inline]
    unsafe fn finish_tail128(
        mut tail: *const u8,
        end: *const u8,
        total: u64,
        mut h1: u64,
        mut h2: u64,
    ) -> (u64, u64) {
        if tail != end {
            let mut k1: u64 = 0;
            for i in 0..8 {
                k1 ^= ((*tail) as u64) << (8 * i);
                tail = tail.add(1);
                if tail == end {
                    break;
                }
            }
            k1 *= C1;
            k1 = k1.rotate_left(31);
            k1 *= C2;
            h1 ^= k1;

            if tail != end {
                k1 = 0;
                for i in 0..8 {
                    k1 ^= ((*tail) as u64) << (8 * i);
                    tail = tail.add(1);
                    if tail == end {
                        break;
                    }
                }
                k1 *= C2;
                k1 = k1.rotate_left(33);
                k1 *= C1;
                h2 ^= k1;
            }
        }

        h1 ^= total;
        h2 ^= total;
        h1 += h2;
        h2 += h1;
        h1 = fmix64(h1);
        h2 = fmix64(h2);
        h1 += h2;
        h2 += h1;
        (h1, h2)
    }

    #[repr(C)]
    pub struct Hasher128 {
        h1: u64,
        h2: u64,
        buf: [u8; 16],
        len: usize,
        consume: u64,
    }

    impl Hasher128 {
        pub fn with_seed(seed: u32) -> Hasher128 {
            Hasher128 {
                h1: seed as u64,
                h2: seed as u64,
                buf: [0; 16],
                len: 0,
                consume: 0,
            }
        }

        #[inline]
        fn feed(&mut self, k1: u64, k2: u64) {
            let (h1, h2) = feed128(self.h1, self.h2, k1, k2);

            self.h1 = h1;
            self.h2 = h2;
            self.consume += 16;
        }

        pub fn finish128(&self) -> (u64, u64) {
            unsafe {
                finish_tail128(
                    self.buf.as_ptr(),
                    self.buf.as_ptr().add(self.len),
                    self.consume + self.len as u64,
                    self.h1,
                    self.h2,
                )
            }
        }
    }

    impl Hasher for Hasher128 {
        fn write(&mut self, mut bytes: &[u8]) {
            if self.len + bytes.len() < 16 {
                unsafe {
                    ptr::copy_nonoverlapping(
                        bytes.as_ptr(),
                        self.buf.as_mut_ptr().add(self.len),
                        bytes.len(),
                    );
                }
                self.len += bytes.len();
                return;
            } else if self.len != 0 {
                let (n1, n2) = unsafe {
                    let cnt = 16 - self.len;
                    ptr::copy_nonoverlapping(
                        bytes.as_ptr(),
                        self.buf.as_mut_ptr().add(self.len),
                        cnt,
                    );
                    bytes = slice::from_raw_parts(bytes.as_ptr().add(cnt), bytes.len() - cnt);
                    let n1 = ptr::read(self.buf.as_ptr() as *const u64);
                    let n2 = ptr::read(self.buf.as_ptr().add(8) as *const u64);
                    self.len = 0;
                    (u64::from_le(n1), u64::from_le(n2))
                };
                self.feed(n1, n2);
            }
            let mut start = bytes.as_ptr();
            for _ in 0..bytes.len() / 16 {
                let (n1, n2) = unsafe {
                    let n1 = ptr::read_unaligned(start as *const u64);
                    start = start.add(8);
                    let n2 = ptr::read_unaligned(start as *const u64);
                    start = start.add(8);
                    (u64::from_le(n1), u64::from_le(n2))
                };
                self.feed(n1, n2);
            }
            unsafe {
                let len = bytes.len() % 16;
                if len > 0 {
                    ptr::copy_nonoverlapping(start, self.buf.as_mut_ptr(), len);
                }
                self.len = len;
            }
        }

        fn finish(&self) -> u64 {
            self.finish128().0
        }
    }
}

mod hash32 {
    use core::hash::Hasher;
    use core::{ptr, slice};

    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const C3: u32 = 0xe6546b64;
    const C4: u32 = 0x85ebca6b;
    const C5: u32 = 0xc2b2ae35;

    #[inline]
    fn fmix32(mut h: u32) -> u32 {
        h ^= h >> 16;
        h *= C4;
        h ^= h >> 13;
        h *= C5;
        h ^ (h >> 16)
    }

    #[inline]
    fn feed32(mut h: u32, mut k: u32) -> u32 {
        k *= C1;
        k = k.rotate_left(15);
        k *= C2;

        h ^= k;
        h = h.rotate_left(13);
        h * 5 + C3
    }

    #[inline]
    unsafe fn finish_tail32(mut tail: *const u8, end: *const u8, total: u64, mut h: u32) -> u32 {
        if tail != end {
            let mut k: u32 = 0;
            for i in 0..3 {
                k ^= ((*tail) as u32) << (8 * i);
                tail = tail.add(1);
                if tail == end {
                    break;
                }
            }
            k *= C1;
            k = k.rotate_left(15);
            k *= C2;
            h ^= k;
        }
        h ^= total as u32;
        fmix32(h)
    }

    pub fn murmurhash3_x86_32(bytes: &[u8], seed: u32) -> u32 {
        let nblocks = bytes.len() / 4;
        let mut h = seed;
        let mut start = bytes.as_ptr();

        for _ in 0..nblocks {
            let k = u32::from_le(unsafe { ptr::read_unaligned(start as *const u32) });
            h = feed32(h, k);
            start = unsafe { start.add(4) };
        }

        unsafe {
            finish_tail32(
                start as *const u8,
                bytes.as_ptr().add(bytes.len()),
                bytes.len() as u64,
                h,
            )
        }
    }

    pub struct Hasher32 {
        h: u32,
        buf: [u8; 4],
        len: usize,
        consume: u64,
    }

    impl Hasher32 {
        pub fn with_seed(seed: u32) -> Hasher32 {
            Hasher32 {
                h: seed,
                buf: [0; 4],
                len: 0,
                consume: 0,
            }
        }

        #[inline]
        fn feed(&mut self, k: u32) {
            self.h = feed32(self.h, k);
            self.consume += 4;
        }

        pub fn finish32(&self) -> u32 {
            unsafe {
                finish_tail32(
                    self.buf.as_ptr(),
                    self.buf.as_ptr().add(self.len),
                    self.consume + self.len as u64,
                    self.h,
                )
            }
        }
    }

    impl Hasher for Hasher32 {
        fn write(&mut self, mut bytes: &[u8]) {
            if self.len + bytes.len() < 4 {
                unsafe {
                    ptr::copy_nonoverlapping(
                        bytes.as_ptr(),
                        self.buf.as_mut_ptr().add(self.len),
                        bytes.len(),
                    );
                }
                self.len += bytes.len();
                return;
            } else if self.len != 0 {
                let n = unsafe {
                    let cnt = 4 - self.len;
                    ptr::copy_nonoverlapping(
                        bytes.as_ptr(),
                        self.buf.as_mut_ptr().add(self.len),
                        cnt,
                    );
                    bytes = slice::from_raw_parts(bytes.as_ptr().add(cnt), bytes.len() - cnt);
                    let n = ptr::read(self.buf.as_ptr() as *const u32);
                    self.len = 0;
                    u32::from_le(n)
                };
                self.feed(n);
            }
            let mut start = bytes.as_ptr();
            for _ in 0..bytes.len() / 4 {
                let n = unsafe {
                    let n = ptr::read_unaligned(start as *const u32);
                    start = start.add(4);
                    u32::from_le(n)
                };
                self.feed(n);
            }
            unsafe {
                let len = bytes.len() % 4;
                if len > 0 {
                    ptr::copy_nonoverlapping(start, self.buf.as_mut_ptr(), len);
                }
                self.len = len;
            }
        }

        fn finish(&self) -> u64 {
            self.finish32() as u64
        }
    }
}

pub use hash128::{murmurhash3_x64_128, Hasher128};
pub use hash32::{murmurhash3_x86_32, Hasher32};
