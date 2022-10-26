# mandelbrot_rust
## Premise
Mandelbrot rust is a program I've created to learn the basics of rust. It has a simple premise: to provide an API that can be queried to determine if a number is part of the Mandelbrot set. It's designed to be used to render the Mandelbrot set, which is why it contains functions for mapping a screen to the complex plane. More information on the mandelbrot set can be found [here](https://en.wikipedia.org/wiki/Mandelbrot_set).
## Design
All of the library source code is in lib.rs. The runtime configuration is stored in a struct called MandelbrotCalculator. I chose to store it in a struct instead of using static variables because it allows the API to be used to create multiple instances of mandelbrot calculator with different configurations within the same program.
## API
### new(width: u32, height: u32, iterations: u32) -> Self
new returns a MandelbrotCalculator with the width, height and number of iterations that's inputted. Note that the iterations is not the number of iterations that are guaranteed to occur, but the maximum number of iterations that will be performed if the complex number doesn't grow to infinity.
Here's an example use if you wanted to output to a panel that was 500 x 500 and wanted a maximum number of iterations of 100:
```rust
    use MandelbrotCalculator

    //...

    let calc = MandelbrotCalculator::new(500, 500, 100);
```
### change_config(width: u32, y: u32, iterations: u32)
change_config alters the configuration parameters of the MandelbrotCalculator. Here is an example of changing the dimensions to 1000 x 1000 and the number of iterations to 50:
```rust
    calc.change_config(1000, 1000, 50)
```
No matter what parameters are set in new and change_config the library will always ensure that the entire Mandelbrot set remains on screen and centered.
### pos_part_of_set(x: u32, y: u32) -> bool
pos_part_of_set takes in an x and y coordinate and returns whether true if that position corresponds to a complex number that is part of the Mandelbrot set, and false if not. Bear in mind that the origin is in the top left corner and the positive y axis is downwards.
Here is an example usage using all 3 functions:
```rust
    use MandelbrotCalculator

    //...

    let calc = MandelbrotCalculator::new(500, 500, 100);

    print_ln!("{}", calc.pos_part_of_set(250, 250));

    calc.change_config(1000, 1000, 50);

    print_ln!("{}", calc.pos_part_of_set(0, 0));

    //...
```
Output:
```bash
    True
    False
```