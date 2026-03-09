# Chip-8 

First ROM running! ibm-logo.ch8 (https://github.com/Timendus/chip8-test-suite/blob/main/README.md) <br>
<img src="pictures/ibm_logo.png" alt="IBM Logo" width="800"/>
<br><br>
## What is chip-8?
From Wikipedia, the free encyclopedia

CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker on his 1802 microprocessor. It was initially used on the COSMAC VIP and Telmac 1800, which were 8-bit microcomputers made in the mid-1970s.

CHIP-8 was designed to be easy to program for and to use less memory than other programming languages like BASIC.

Interpreters have been made for many devices, such as home computers, microcomputers, graphing calculators, mobile phones, and video game consoles.

Full description on [wikipedia](https://en.wikipedia.org/wiki/CHIP-8).
<br>

## Main objective of this project

My goal in implementing Chip-8 in Rust is to learn more about the Rust programming language and how CPUs work. <br>

Some lessons learned were:
- Organization of modules and responsibilities
- Encapsulation
- How to build a virtual hardware (very basic virtual machine)
- Seeing the fine line between organization and overengineering
- Modeling system states and ensuring safe transitions between them, preventing invalid states.
- CPU fetch-decode-execute cycle
- What is an ISA (Instruction Set Architecture)

## How to use
You must have rustup and cargo installed. <br>
To use it, download the source code and download the ROMs you're interested in (Note: only ROMs with the .ch8 extension will work) and run the command `cargo run -- your_rom.ch8`


## Observations
If I were starting the Chip-8 implementation today with what I learned from this project, I would have done a few things differently:
1. **Getters and setters more conscious of their use:**
    - The set_pc, for example, is a method that only ensures that the PC will not go to an invalid state, but I ended up having to create a method called increment_pc because the PC increment would become verbose and difficult to verify as I implemented each instruction. Today I would create the get_pc, set_pc, and skip_instr methods so that each possible state of the PC could be accessed safely and consciously.

2. **Diagrams and a better description of the system to be implemented:** I thought I could implement it directly, considering that chip-8 is already well detailed on the internet, but I was wrong. I became too dependent on LLMs to understand the requirements and what each instruction had to do, for example. If I were starting today, I would do:
    - A class diagram describing the virtual hardware, each attribute and method such as `private pc: u16` and `increment_pc` for example, would give me a better direction for developing the virtual hardware and would make the programming practically mechanical.

    - An instruction table describing the opcode, objective, and pseudocode of the instruction (showing a step-by-step of which states the instruction must modify to function correctly). This would have made planning the division of modules and responsibilities much simpler and would have made me more autonomous in the implementation of the project by not having to resort to an LLM all the time to know what each instruction does.

3. **Better planning of modules and responsibilities:** with everything already planned, I would have a panoramic view of the entire project before starting and could better plan how to organize the code to avoid overengineering while still maintaining the security and control of state transitions and code organization.


## Conclusion
This project was a success, I learned a lot from it and I feel more prepared for future projects! In the next project, I will do it the way I would have liked to have started this project; it will be more structured and without overengineering.

This project is the foundation and introduction I needed to build interpreters and emulators in Rust; the learning I gained from it will be carried forward!