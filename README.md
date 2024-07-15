<img width="377" alt="image" src="https://github.com/user-attachments/assets/c3d1a75b-6813-440d-82d5-8015a5c18f9a">
Multi-OS is a minimal operating system kernel for x86_64 systems by transforming a freestanding binary and integrating it with a BIOS-based bootloader. This project involves designing a custom target tailored for the x86_64 architecture, ensuring compatibility and efficient hardware management. Critical steps include disabling SIMD instructions and the red-zone to prevent stack corruption and simplify the kernel's initial design. By focusing on these key aspects, it successfully initializes hardware through BIOS and provides a robust foundation for a minimal yet functional OS kernel.

#   CPU-EXCEPTIONS
Leveraging the extensive knowledge gained from the Microprocessor Interface and Programming course (PC-EC-MIP207), we focused on handling CPU exceptions with a high degree of precision and efficiency. One of the key challenges tackled was the development of stack-switching mechanisms designed to manage double faults caused by stack overflows. This involved creating robust procedures to ensure that the system remains stable even when critical faults occur.

A significant aspect of the project was utilizing the master-slave architecture of the 8259 Programmable Interrupt Controller (PIC). This architecture was critical for managing hardware interrupts effectively and preventing deadlocks, ensuring that the system could handle multiple interrupts seamlessly without entering a state of deadlock. By understanding and implementing this architecture, we were able to enhance the system's interrupt handling capabilities significantly.

In addition to this, we successfully implemented over 15 CPU interrupts using carefully crafted error handler functions. These functions were designed to handle various interrupt scenarios, thereby enhancing the overall robustness and reliability of the system. This implementation allowed the system to respond to different types of interrupts quickly and efficiently, maintaining system stability and performance.

At the kernel level, special attention was given to handling breakpoint exceptions. We implemented mechanisms to catch these exceptions and enable the system to resume normal execution afterward. This capability is crucial for debugging and ensuring that the system can recover gracefully from unexpected conditions.

Moreover, the Interrupt Descriptor Table (IDT) was stored as static, ensuring efficient and reliable access for handling interrupts. This static storage of the IDT allowed for faster and more reliable interrupt handling, as the table could be accessed quickly whenever an interrupt occurred. This approach contributed to the overall efficiency and reliability of the interrupt management system.

# paging mechanism and heap allocation 
In my project, I implemented two types of paging mechanisms—recursive paging and mapping of the entire physical memory—using the x86_64 crate’s paging functionality.

Paging Mechanisms:
Recursive Paging:
Recursive paging simplifies page table management by mapping the last entry of a page table to the table itself, creating a self-referential structure. This allows easy traversal and modification of page tables, aiding in virtual-to-physical address translation and debugging.
Mapping the Entire Physical Memory:
This technique creates a direct mapping of the entire physical memory into the virtual address space, facilitating direct access to any physical address with a fixed offset. It’s beneficial for kernel-level operations requiring direct physical memory access.
Heap Schemes:
Fixed Block Size Allocation:

Memory is divided into fixed-size blocks, minimizing fragmentation and simplifying allocation. This is efficient for consistent-sized objects, with a free list for quick allocation and deallocation.
Slab Allocation with Bitmaps:

Slab allocation efficiently manages memory for same-sized objects using bitmaps to track free and occupied slots. It offers fast allocation/deallocation, reduces fragmentation, and ensures thread safety through atomic operations. The allocator initializes with a contiguous memory block partitioned into data and bitmap segments.
These implementations provide a robust and efficient memory management system, enhancing the operating system's performance and reliability.






