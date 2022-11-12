# terminal-draw
this sprint in a nutshell
![Programmers Credo: We do these things not because they are easy, but because we thought they were going to be easy](https://i.redd.it/hnax90lyudz91.jpg)

## Overview

This sprint, I was trying to write a graphics library for a terminal-based game engine. However, this didn't work very well because Termion (a very useful crate which I used heavily) does not have practical examples on how to use such features as asyncronous input detection.
Although the graphics library works pretty well (If I say it myself), lacking the ability to obtain input from the user puts a bit of a damper on the possibilities of video games, so I looked into some fine `println!()` of the goal I set for myself and decided that I could fulfill it adequately by showing off some little ball-like shapes bouncing around the screen with randomly set positions and velocities.

I've always been interested in making video games, but I haven't been able to make them as frequently as I want to because I am not good at graphic design. 
There is a reason that I am not an art major (i.e. I suck at art).
Therefore, when I first got the idea for a terminal-based engine, I was intrigued. I said to myself, "Here is a way to make games without having to do art or graphic design. Just make the main character a 'W' or something, build some walls out of '|', '+', and '-', and you've got a neat little top-down dungeon crawler." 
I chose Rust as my language to write the engine in because it is really fast and I didn't want the performance of games being dragged down by a slow method of drawing the frames.

{Provide a link to your YouTube demonstration.  It should be a 4-5 minute demo of the software running and a walkthrough of the code.}

[Software Demo Video](https://youtu.be/JB4zND8rSRE)

## Development Environment

To develop this project, I used VScode and the suggested Rust extensions.

According to the official Rust website (found at [rust-lang.org](https://www.rust-lang.org/)):  
"Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages."
This is exactly what I wanted in the engine. I thought it would be cool to write the graphics in Rust and then call them from Python, which would handle all the actual game logic.
As for libraries, I used: 
* [termsize](https://crates.io/crates/termsize) (very light crate for finding terminal size)
* [termion](https://crates.io/crates/termion) (very complicated crate for super low-level terminal tasks)
* [rand](https://crates.io/crates/rand) (powerful crate for random number generation)

## Useful Websites

* [Visual Studio Code Rust page](https://code.visualstudio.com/docs/languages/rust) (taught me how to use the debugger)
* [FolksTalk](https://www.folkstalk.com/2022/06/how-to-convert-float-to-integer-and-int-to-float-in-rust.html) (gave me some invaluable information on how to convert between int and float in Rust)
* [Wikipedia.org](https://en.wikipedia.org/wiki/Gettysburg_Address) (gave me the Gettysburg Address, which I used in testing)
* [termion GitHub page](https://github.com/redox-os/termion/tree/master/examples) (included some helpful examples, just not enough of them for me to figure out how to do what I wanted to do)
* [Official Rust Documentation](https://doc.rust-lang.org/book/ch10-02-traits.html) (you can't go far wrong with official documentation. This source taught me about traits, which allowed me to do some pSeudo-Object-Oriented-Programming (I chose this algorithm very carefully. Don't think about it too hard))
* [(of course) Stack Overflow](https://stackoverflow.com/questions/35671985/how-do-i-get-keyboard-input-without-the-user-pressing-the-enter-key) (I've listed one question that started me thinking on the right track, but there were many others)
* [Official Rust Forums](https://users.rust-lang.org/t/flush-the-standard-output-on-terminal/1018) (helped me figure out why the ball kept disappearing early in testing and how to fix it)
* [Programming-Idioms.org](https://programming-idioms.org/idiom/45/pause-execution-for-5-seconds/484/rust) (I used this in the last Rust sprint but I forgot how it was done so I referenced it again)

## Future Work

* ADD INPUT (making this an actual game engine) and make an actual game
* Maybe do some computer-controlled pong? That could be fun
* I was running into some issues where objects that moved too fast would crash the program when they were close to the edges. I figured out why this was happening, but I didn't come up with a good solution. I'd like to fix that.

