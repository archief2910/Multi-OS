![Multi-OS Overview](https://github.com/user-attachments/assets/c3d1a75b-6813-440d-82d5-8015a5c18f9a)

# Multi-OS 

Multi-OS is a minimal operating system kernel designed specifically for x86_64 systems. The project involves transforming a freestanding binary and integrating it with a BIOS-based bootloader. This undertaking required the creation of a custom target tailored to the x86_64 architecture, ensuring compatibility and efficient hardware management. Key decisions included disabling SIMD instructions and the red-zone to prevent stack corruption, which simplified the initial design of the kernel. This careful focus on hardware initialization through the BIOS laid a robust foundation for a minimal yet functional operating system kernel.

## CPU Exceptions

![CPU Exceptions](https://github.com/user-attachments/assets/8e5bdb67-51f7-40cd-aeb9-497ca1656226)

A significant part of the project involved leveraging knowledge gained from the Microprocessor Interface and Programming course (PC-EC-MIP207) to handle CPU exceptions with precision. One of the challenges addressed was the development of stack-switching mechanisms to manage double faults caused by stack overflows, ensuring the system remained stable during critical faults. Additionally, we implemented the master-slave architecture of the 8259 Programmable Interrupt Controller (PIC), which was crucial for effective management of hardware interrupts and preventing deadlocks, allowing the system to handle multiple interrupts seamlessly.

Over 15 CPU interrupts were successfully implemented, utilizing carefully crafted error handler functions designed to address various scenarios, thus enhancing system robustness. Special attention was given to handling breakpoint exceptions; mechanisms were developed to catch these exceptions and allow the system to resume normal execution, which is vital for debugging purposes. Furthermore, the Interrupt Descriptor Table (IDT) was stored statically to ensure efficient and reliable access during interrupt handling, contributing to the overall efficiency of the system.

## Paging Mechanism and Heap Allocation

![Paging Mechanisms](https://github.com/user-attachments/assets/40d58f40-1835-4177-b450-1331c6b9a2b6)

In terms of memory management, two types of paging mechanisms were implemented: recursive paging and mapping of the entire physical memory. Recursive paging simplifies page table management by creating a self-referential structure that aids in virtual-to-physical address translation and debugging. On the other hand, mapping the entire physical memory provides a direct mapping into the virtual address space, allowing kernel-level operations to access physical memory directly.

The project also introduced two heap allocation schemes. The first, fixed block size allocation, divides memory into fixed-size blocks, minimizing fragmentation and simplifying allocation. This approach is particularly efficient for handling objects of consistent sizes. The second scheme, slab allocation with bitmaps, manages memory for same-sized objects by utilizing bitmaps to track free and occupied slots. This method offers rapid allocation and deallocation, reduces fragmentation, and ensures thread safety through atomic operations.

Together, these implementations create a robust and efficient memory management system, significantly enhancing the performance and reliability of the Multi-OS kernel.
