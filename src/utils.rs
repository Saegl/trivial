use sdl2::cpuinfo;
use windows::Win32::UI::WindowsAndMessaging::SetProcessDPIAware;

pub fn print_all_cpu_info() {
    println!("CPU INFO:");
    println!("cache line size \t\t{}", cpuinfo::cpu_cache_line_size());
    println!("cpu count \t\t{}", cpuinfo::cpu_count());
    println!("has_3d_now \t\t{}", cpuinfo::has_3d_now());
    println!("has_alti_vec \t\t{}", cpuinfo::has_alti_vec());
    println!("has_avx \t\t{}", cpuinfo::has_avx());
    println!("has_avx2 \t\t{}", cpuinfo::has_avx2());
    println!("has_avx512f \t\t{}", cpuinfo::has_avx512f());
    println!("has_mmx \t\t{}", cpuinfo::has_mmx());
    println!("has_rdtsc \t\t{}", cpuinfo::has_rdtsc());
    println!("has_sse \t\t{}", cpuinfo::has_sse());
    println!("has_sse2 \t\t{}", cpuinfo::has_sse2());
    println!("has_sse3 \t\t{}", cpuinfo::has_sse3());
    println!("has_sse41 \t\t{}", cpuinfo::has_sse41());
    println!("has_sse42 \t\t{}", cpuinfo::has_sse42());
    println!("system ram \t\t{}", cpuinfo::system_ram());
}

pub fn dpi_aware() {
    unsafe {
        SetProcessDPIAware();
    }
}
