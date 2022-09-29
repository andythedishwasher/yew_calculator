# yew_calculator
A calculator app built with the Yew front end framework in Rust.

# How It Works
The dist (distribution) directory is built using the Trunk crate which bundles your rust code into web assembly and Javascript code that work together to grab the necessary data from the DOM in html for use in the web assembly logic generated from our Rust code with the wasm-pack and wasm-bindgen crates. Then, you just take the contents of the dist directory and place them at the root of whatever directory you're serving your content from. If you're running it from localhost, make sure Trunk is installed and run trunk serve on the command line from this repo's root directory after you clone it. I've hosted it here on a [subdomain of my company's website](rust.thecodekitchen.xyz).

# Why A Calculator?
Since I only recently began working with Rust as a sort of escape route from the rather messy ecosystem of Javascript frameworks for web development, I decided it would be interesting to see if the memory safety advantages and slightly more granular data structuring methods in Rust could help with the logic issues I was having with a similar implementation in React. 

# Why Rust?
It turns out I was totally correct! It took a bit more code, but the extra code needed was basically handed to me by the compiler in the form of extremely helpful error messages. In addition, the ownership system it provides allowed me to specify in a very granular fashion exactly which values were to be mutable in which scope. In JS, you more or less have to implicitly type data to match the mutability you need for a given implementation of its value, which can lead to some dizzying spaghetti coercions when dealing with complex state management. In Rust, you still have to coerce things a bit in order to get data to operate dynamically, but every line of code specifies exactly what it's coercing without a background knowledge of the implied mutability rules.

# Why Yew?
Mainly because it seemed initially like the easiest one to get started with given the state of its development. The build directory ends up being pretty hefty on account of the headaches involved in packing Rust logic into Web Assembly in a way that JS can process it and deliver data back and forth between the DOM and our wasm code. Rust seems top pack into wasm on its own just fine, but it feels like everything spaghettifies when it has to do a Javascript. Thankfully, however, some intrepid Rustaceans on the Yew team have tackled a good bit of that spaghetti in the background for us, but we still must store the rather hefty  fruits of their labor.

# So What Now?
Now that I've learned the basics of web development with Rust, I'm going to try and get more directly involved in the wider Rust community to see what sort of developments need more heads at the moment, so if anyone has suggestions toward that end, those are very welcome. For now, I may go back to developing in more stable Javascript frameworks for the sake of employment opportunities, but I will be keeping a close eye out for any potential careers available in Rust web development.
