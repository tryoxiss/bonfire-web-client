We follow our Beautiful Code Style for all source code. Below is a basic list of what that means. 

Below are some very basics: 
```rust
fn main(param1, param2) { // do not space pad brackets.
    println!("Cookies")
} 
```

Our code is also minimally nested. Only 4 layers or less of indentation will be allowed to merge. 

This is the absolute most code can be nested. 
```py
// layer 0

def main(): 
    // layer 1
    
    if cookies == True: 
        // layer 2
        
        if milk == True: 
            // layer 3
            
            if water == False: 
                // layer 4.
                
                // your code block here
```

We understand that it can be hard to avoid sometimes, but try and keep it to as few layers as possible. We have a few tips on how to reduce nesting below. 

Teqnique 1: Instead of if x if y if z etc, we can just return after each faled condition. Like so: 
```py
// layer 0

def main(): 
    // layer 1
    
    if cookies == False: 
        return
        
    if milk == False: 
        return
            
    if water == False: 
        return
    
    // your code block here
```

See how much more readable that is? But that won't always work, so enter tech 2! Extrapalte code to its own function. Any module that you expect to use repeadedly should be its own function anyway, but even if its only used in one place it can sometimes help for clairty. 

Yes, function calls have a small amount of overhead, but that is so minor its not worth thinking about. Additioanlly: if you make good use of functions our code becomes more modular and faster to expand! Which is very important to us. Especially with the power of the macros system in rust, we can entirely remove the function overhead at only minimal cost to our developement and compile time!

If you are still confused, [this video should help explain how to prevent code nesting](https://www.youtube.com/watch?v=CFRhGnuXG-4).
