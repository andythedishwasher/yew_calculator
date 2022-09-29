# yew_calculator
A calculator app built with the Yew front end framework in Rust.

# How It (sort of) Works
My original plan was to serve this on a subdomain of my website called rust.thecodekitchen.xyz. Right now, you'll see a blank page there with a little css reference in the head. That's because during testing, I was using the Trunk crate with wasm-pack and wasm-bindgen as dependencies to build, bundle, and serve the code. Trouble is, I don't actually know how to install trunk on a web server via cPanel in order to serve the code from my subdomain! So unfortunately, until I figure out how to wade through that quagmire, you will have to clone this repo, install trunk in your local environment via [these instructions](https://trunkrs.dev/), and run the trunk serve command from the cloned directory in your command line. That should serve it to your browser on localhost:8080.

# Why A Calculator?
Since I only recently began working with Rust as a sort of escape route from the rather messy ecosystem of Javascript frameworks for web development, I decided it would be interesting to see if the memory safety advantages and slightly more granular data structuring methods in Rust could help with the logic issues I was having with a similar implementation in React. 

# Why Rust?
It turns out I was totally correct! It took a bit more code, but the extra code needed was basically handed to me by the compiler in the form of extremely helpful error messages. In addition, the ownership system it provides allowed me to specify in a very granular fashion exactly which values were to be mutable in which scope. In JS, you more or less have to implicitly type data to match the mutability you need for a given implementation of its value, which can lead to some dizzying spaghetti coercions when dealing with complex state management. In Rust, you still have to coerce things a bit in order to get data to operate dynamically, but every line of code specifies exactly what it's coercing without a background knowledge of the implied mutability rules.

# Why Yew?
Mainly because it seemed initially like the easiest one to get started with given the state of its development. The build directory ends up being pretty hefty on account of the headaches involved in packing Rust logic into Web Assembly in a way that JS can process it and deliver data back and forth between the DOM and our wasm code. Rust seems top pack into wasm on its own just fine, but it feels like everything spaghettifies when it has to do a Javascript. Thankfully, however, some intrepid Rustaceans on the Yew team have tackled a good bit of that spaghetti in the background for us, but we still must store the rather hefty  fruits of their labor.

# So What Now?
Now, I would really like to figure out how you're supposed to serve apps with this wasm model to my website instead of just hosting them locally. If anyone has any advice to that end, I'd love to hear from you. I'm also going to try and get more directly involved in the Rust community to see what sort of developments need more heads at the moment, so if anyone has suggestions toward that end, those are also very welcome. Until then, I think I'm going to explore the possibilities for desktop apps and game development with Rust.
