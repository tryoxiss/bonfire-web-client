We follow our Beautiful Code Style for all source code. Below is a basic list of what that means. 

Below are some very basics: 
```rust
fn main(param1, param2) { // do not space pad brackets.
    println!("Cookies");
} 
```

Our code is also minimally nested. Only 4 layers or less of indentation will be allowed to merge. 4 layers deep is only sometimes acceptable, in some cases, and generally it is best to apply nesting minimisation starting at 2 or 3 levels deep. 

This is the absolute most code can be nested. 
```py
# layer 0

def main(): 
    # layer 1
    
    if cookies == True: 
        # layer 2
        
        if milk == True: 
            # layer 3
            
            if water == False: 
                # layer 4.
                
                # your code block here
```

We understand that it can be hard to avoid sometimes, but try and keep it to as few layers as possible. We have a few tips on how to reduce nesting below. 

Technique 1: Instead of if x if y if z etc, we can just return after each failed condition. Like so: 
```py
# layer 0

def main(): 
    # layer 1
    
    if cookies == False: 
        return
        
    if milk == False: 
        return
            
    if water == False: 
        return
    
    # your code block here
```

See how much more readable that is? But that won't always work, so enter tech 2! Extrapolate code to its own function. Any module that you expect to use repeadedly should be its own function anyway, but even if its only used in one place it can sometimes help for clarity. 

Yes, function calls have a small amount of overhead, but that is so minor its not worth thinking about. Additionally: if you make good use of functions our code becomes more modular and faster to expand! Which is very important to us. Especially with the power of the macros system in rust, we can entirely remove the function overhead at only minimal cost to our developement and compile time!

Nesting limitations do not apply to HTML, XML, XHTML, or any simillar markup languages as heavy nesting is unavoidable in those cases, and the nesting helps visual clarity. 

If you are still confused, [this video should help explain how to prevent code nesting](https://www.youtube.com/watch?v=CFRhGnuXG-4).

We indent with 4 spaces, not tabs. No more than 4 spaces, and no less. 

## Weird Cases
In the case of an HTML macro, don't indent the root element. This creates an entirely unnecesary layer of nesting. 

For `<style>` elements, used for performance opposed to downloading a seperate stylesheet for the webapp (seperate stylesheets are used for the client still), do not indent the elements. Write within the style element as if that was its own external file. Additionally, `<style>` elements always go directly after the `<head>`, and directly before the `<body>`. Similarly, do not indent `<head>` and `<body>` elements, only thier contents. They are functionally root elements. 

## Naming Conventions

Variables are in `snake_case`. 

1. Do not abbrivate names. `mouse_x_coordinate` or `mouse_x`, not `mX`. If your function is explictly a coordinate handler, and only deals with one set of coordinates, x, y, and z are fine. 
2. Do not prefix variable type. `is_passing`, not `bool_is_passing` or `b_is_passing`. 
3. If it uses a specific unit, add the unit. `delay_seconds`, not `delay`. 
4. No "Utils" or "Helpers": Specify further. It can always be sorted into more neat sets. 
5. Avoid "base_x", etc. Instead: create "x" as the base class, and over specify the name for children. 
    - Not "Truck of BaseTruck"
    - Instead use: "TrailerTruck of Truck"
6. One letter variable names are allowed for the following cases: 
    1. explicit one way mathmatical functions that take in one value, where you can use n. 
    2. x, y, and z coordinates in a coordinate handler where you only deal with one set of coordinates. 
    3. For i loops with only one nest. for j is not allowed, and if you need a nested for loop you cannot use i, you must name both variables according to other conventions
7. Abbreviations and initalisms are fine sometimes, for example `guid` is fine as its clear what it is, despite being an initalism. 

## Comments
Comments starting with //# are header comments. This means they tell you information about the block of code until the next empty line. This is mostly useful in Structs where we can use them to show our categorisation of variables. 
