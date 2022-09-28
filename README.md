# yew_calculator
A calculator app built with the Yew front end framework in Rust.

# Why A Calculator?
Since I only recently began working with Rust as a sort of escape route the rather messy ecosystem of Javascript frameworks for web development, I decided it would be interesting to see if the memory safety advantages and slightly more granular data structuring methods in Rust could help with the logic issues I was having with a similar implementation in React. 

# Why Rust?
It turns out I was totally correct! It took a bit more code, but the extra code needed was basically handed to me by the compiler in the form of extremely helpful error messages. In addition, the ownership system it provides allowed me to specify in a very granular fashion exactly which values were to be mutable in which scope. In JS, you more or less have to implicitly type data to match the mutability you need for a given implementation of its value, which can lead to some dizzying spaghetti coercions when dealing with complex state management. In Rust, you still have to coerce things a bit in order to get data to operate dynamically, but every line of code specifies exactly what it's coercing without a background knowledge of the implied mutability rules.

# Why Yew?
Mainly because it seemed initially like the easiest one to get started with given the state of its development. The build directory ends up being pretty hefty on account of the headaches involved in packing Rust logic into Web Assembly in a way that JS can process it and deliver data back and forth between the DOM and our wasm code. Rust seems top pack into wasm on its own just fine, but it feels like everything spaghettifies when it has to do a Javascript. Thankfully, however, some intrepid Rustaceans on the Yew team have tackled a good bit of that spaghetti in the background for us, but we still must store the rather hefty  fruits of their labor.

# So What Now?
Now, I think I'm going to explore some other frameworks that operate similarly to Yew but perhaps go a different route with their JS interfacing. This appeared to run fast on localhost, but we will have to see how it performs remotely. As previously stated, these build directories are unfortunately large, but it may end up being the best route in the end. My plan is to just recycle the logic for this calculator for use in several Rust frameworks in alpha or beta stage and branch them out from this repo as examples of how different frameworks are handling the issues of the current html ecosystem with Rusty solutions. If you can't tell, I'm reaching the fanboy stage with Rust. I'll let you know if or when I start to seriously question that life choice.
