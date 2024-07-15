# Multi-OS Overview
                            ![Screenshot 2024-07-15 094525](https://github.com/user-attachments/assets/c986cdf9-3de6-43f5-899b-1cd734103648)

Multi-OS is a minimal operating system kernel for x86_64 systems, created by transforming a freestanding binary and integrating it with a BIOS-based bootloader. The project involves designing a custom target specifically tailored for the x86_64 architecture, ensuring compatibility and efficient hardware management. Critical steps include disabling SIMD instructions and the red-zone to prevent stack corruption and simplify the kernel's initial design. This focus allows for successful hardware initialization through BIOS, providing a solid foundation for a minimal yet functional OS kernel.

## CPU Exceptions

Leveraging knowledge from the Microprocessor Interface and Programming course (PC-EC-MIP207), we focused on handling CPU exceptions with precision and efficiency. Key challenges included:

- **Stack-Switching Mechanisms**: Developed to manage double faults caused by stack overflows, ensuring system stability during critical faults.
  
- **Master-Slave Architecture of the 8259 PIC**: Utilized for effective hardware interrupt management and preventing deadlocks, allowing seamless handling of multiple interrupts.

- **CPU Interrupts**: Successfully implemented over 15 interrupts with error handler functions designed to manage various scenarios, enhancing system robustness.

- **Breakpoint Exceptions**: Implemented mechanisms to catch and resume normal execution after breakpoint exceptions, crucial for debugging.

- **Interrupt Descriptor Table (IDT)**: Stored as static for efficient and reliable access during interrupt handling, contributing to overall system efficiency.

## Paging Mechanism and Heap Allocation

In the project, two types of paging mechanisms were implemented:

1. **Recursive Paging**: Simplifies page table management by creating a self-referential structure, aiding in virtual-to-physical address translation and debugging.

2. **Mapping the Entire Physical Memory**: Creates a direct mapping of physical memory into the virtual address space, facilitating direct access for kernel-level operations.

### Heap Schemes

1. **Fixed Block Size Allocation**: Memory is divided into fixed-size blocks to minimize fragmentation and simplify allocation, efficiently handling consistent-sized objects.

2. **Slab Allocation with Bitmaps**: Manages memory for same-sized objects using bitmaps to track free and occupied slots. This method offers fast allocation/deallocation, reduces fragmentation, and ensures thread safety through atomic operations.

These implementations provide a robust and efficient memory management system, significantly enhancing the performance and reliability of the operating system.





