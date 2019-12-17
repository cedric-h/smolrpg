    use ::alloc::alloc::{GlobalAlloc, Layout};
    use winapi::shared::minwindef::{DWORD, LPVOID};
    use winapi::um::winnt::{HEAP_ZERO_MEMORY, MEMORY_ALLOCATION_ALIGNMENT as MIN_ALIGN};
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::heapapi::{
        HeapAlloc,
        HeapReAlloc,
        HeapFree,
        GetProcessHeap,
    };

    pub struct Allocator;

    #[global_allocator]
    static A: Allocator = Allocator;

    #[alloc_error_handler]
    fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
        panic!("allocation error: {:?}", layout)
    }

    // Shamelessly stolen from here.
    // https://github.com/rust-lang/rust/blob/master/src/libstd/sys_common/alloc.rs
    pub unsafe fn realloc_fallback(
        alloc: &Allocator,
        ptr: *mut u8,
        old_layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        use core::{ptr, cmp};

        // Docs for GlobalAlloc::realloc require this to be valid:
        let new_layout = Layout::from_size_align_unchecked(new_size, old_layout.align());

        let new_ptr = GlobalAlloc::alloc(alloc, new_layout);
        if !new_ptr.is_null() {
            let size = cmp::min(old_layout.size(), new_size);
            ptr::copy_nonoverlapping(ptr, new_ptr, size);
            GlobalAlloc::dealloc(alloc, ptr, old_layout);
        }
        new_ptr
    }
    
    // Shamelessly stolen from here, but converted to winapi.
    // https://github.com/rust-lang/rust/blob/master/src/libstd/sys/windows/alloc.rs
    #[repr(C)]
    struct Header(*mut u8);

    unsafe fn get_header<'a>(ptr: *mut u8) -> &'a mut Header {
        &mut *(ptr as *mut Header).offset(-1)
    }
    
    unsafe fn align_ptr(ptr: *mut u8, align: usize) -> *mut u8 {
        let aligned = ptr.add(align - (ptr as usize & (align - 1)));
        *get_header(aligned) = Header(ptr);
        aligned
    }

    #[inline]
    unsafe fn allocate_with_flags(layout: Layout, flags: DWORD) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            return HeapAlloc(GetProcessHeap(), flags, layout.size()) as *mut u8;
        }

        let size = layout.size() + layout.align();
        let ptr = HeapAlloc(GetProcessHeap(), flags, size);
        if ptr.is_null() { ptr as *mut u8 } else { align_ptr(ptr as *mut u8, layout.align()) }
    }

    unsafe impl GlobalAlloc for Allocator {
        #[inline]
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            allocate_with_flags(layout, 0)
        }

        #[inline]
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            allocate_with_flags(layout, HEAP_ZERO_MEMORY)
        }

        #[inline]
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            if layout.align() <= MIN_ALIGN {
                let err = HeapFree(GetProcessHeap(), 0, ptr as LPVOID);
                debug_assert!(err != 0, "Failed to free heap memory: {}", GetLastError());
            } else {
                let header = get_header(ptr);
                let err = HeapFree(GetProcessHeap(), 0, header.0 as LPVOID);
                debug_assert!(err != 0, "Failed to free heap memory: {}", GetLastError());
            }
        }

        #[inline]
        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            if layout.align() <= MIN_ALIGN {
                HeapReAlloc(GetProcessHeap(), 0, ptr as LPVOID, new_size) as *mut u8
            } else {
                realloc_fallback(self, ptr, layout, new_size)
            }
        }
    }
